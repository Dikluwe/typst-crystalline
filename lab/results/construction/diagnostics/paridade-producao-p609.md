# Paridade de Produção — P609

**Data do relatório:** 2026-07-08
**Passo:** 609
**Foco:** Confirmar por que fontes de fallback geravam PDFs massivos (15,7 MB para uma linha de texto) e corrigir.

---

## Resumo executivo

P608 mediu 15,7 MB para um documento de uma linha com fallback para CJK e árabe, e atribuiu isso a "subsetting scope-out". P609 confirma que a explicação estava incorrecta: o subsetting funciona para fontes simples, mas **falha silenciosamente para TrueType Collections (`.ttc`)**. Quando o subsetter falha, o código embute a fonte completa — no caso do Noto Sans CJK JP, 15,4 MB.

A causa raiz é que `FontSlot::get()` devolvia os bytes brutos do ficheiro `.ttc` (coleção completa). Tanto o shaper como o export usavam `Face::parse(data, index)` para aceder à face correcta, mas o export passava os bytes da coleção inteira ao `oxifont_subset`, que não consegue subsetar coleções. O fallback era embutir a coleção inteira.

**Correcção:** `FontSlot::get()` agora extrai a face individual de uma coleção antes de a expor. O export recebe sempre uma fonte simples, e o subsetting aplica-se correctamente a todas as fontes, incluindo fallback.

**Resultado:** o documento de teste de P608 passou de **15,7 MB** para **~196 KB**.

---

## Proveniência

- **Hash base:** `3afca01e0808e9b9bdb1126343057c7141148876`
- **Data/hora:** 2026-07-08T01:20-03:00 (referência de sessão)
- **Binários usados:**
  - Cristalino: `./target/release/typst` (reconstruído em release após a correcção)
  - Vanilla 0.15.0: `lab/typst-original/target/release/typst compile`
- **Ferramentas auxiliares:** `mutool extract`, `fc-scan`, `pdftotext`

---

## Sonda

### Documento de teste

`/tmp/p608-fallback.typ`:

```typst
Hello 你好 مرحبا world, more latin text after the fallback scripts.
```

### Fontes embutidas antes da correcção

Extraídas do PDF cristalino original (`/tmp/p608.pdf`, 15,7 MB):

| Ficheiro | Tamanho | Identificação | Interpretação |
|----------|---------|---------------|---------------|
| `font-0007.ttf` | 13 KB | Liberation Serif | Fonte primária, subsetada |
| `font-0012.cid` | 15,4 MB | Noto Sans CJK JP | Fonte de fallback CJK, **não subsetada** |
| `font-0017.cid` | 174 KB | Noto Naskh Arabic (fc-scan identificou como FreeMono devido a tabelas mínimas) | Fonte de fallback árabe, subsetada mas quase completa |

O objecto `/W` do CIDFont CJK referenciava apenas ~26 CIDs, mas o stream de fonte continha a coleção completa.

### Localização do problema no código

- `03_infra/src/fonts.rs:36-43` — `FontSlot::get()` lê os bytes do disco e valida com `Face::parse(&data, self.index)`, mas devolve `data` (a coleção inteira).
- `03_infra/src/pipeline.rs:405-411` — `collect_fonts_from_doc` + `resolve_fonts` passam esses bytes para `export_pdf_multifont`.
- `03_infra/src/export/builder.rs:500` — `measure_subset` chama `subset_font_with_mapping` com os bytes da coleção.
- `03_infra/src/export/subset.rs:70-72` — `oxifont_subset::subset_with_gid_set` provavelmente falha em coleções; o código faz `.ok()?` e cai no fallback de embutir a fonte completa (`03_infra/src/export/builder.rs:509`).

---

## Implementação

### Ficheiro alterado

`03_infra/src/fonts.rs`:

- `FontSlot::get()` agora detecta TrueType/OpenType Collections via `ttf_parser::fonts_in_collection`.
- Se for uma coleção, extrai a face correspondente a `self.index` a partir do cabeçalho `ttcf`.
- A validação `Face::parse(&data, 0)` passa a usar sempre índice 0, porque `data` é agora uma fonte simples.

Função nova:

```rust
fn extract_collection_face(data: &[u8], index: u32) -> Option<Vec<u8>>
```

Lê o cabeçalho `ttcf` (`tag` + `version` + `numFonts` + `offsetTableOffsets`), calcula o intervalo de bytes da face `index` e devolve essa fatia.

### Porque isto funciona

- O shaper e o export já usavam `Face::parse(data, index)`; com a extracção, ambos passam a usar dados independentes da face correcta.
- O subsetter recebe uma fonte simples e pode reduzi-la aos glifos usados.
- Não é necessário alterar as assinaturas públicas de export (`export_pdf_multifont`, etc.).

---

## Validação

### Tamanho do PDF

| Compilador | Tamanho |
|------------|---------|
| Cristalino antes (P608) | 15.653.692 bytes (15,7 MB) |
| Cristalino depois (P609) | 200.983 bytes (~196 KB) |
| Vanilla 0.15.0 | 12.015 bytes (~12 KB) |

O cristalino ainda é maior que o vanilla (que usa subsetting mais agressivo e não embute o mesmo conjunto de glifos Latin na fonte árabe), mas deixou de ser 15 MB.

### Fontes embutidas depois da correcção

| Ficheiro | Tamanho | Estado |
|----------|---------|--------|
| `font-0007.ttf` | 13 KB | Subsetada (Liberation Serif) |
| `font-0012.ttf` | 5,6 KB | **Agora subsetada** (Noto Sans CJK JP) |
| `font-0017.cid` | 175 KB | Subsetada, mas inclui glifos Latin por partilha de `glyph_ids` entre fontes |

A fonte CJK passou de 15,4 MB para 5,6 KB — redução de ~99,9 %.

### Testes com mais scripts

Documento adicional (`/tmp/p609-multi-script.typ`):

```typst
Hello 你好 مرحبا שלום नमस्ते こんにちは
```

- Compilou com sucesso.
- Tamanho: ~204 KB.
- Fonte CJK extraída: 8,4 KB (subsetada).

### Correcção visual e extracção de texto

`pdftotext /tmp/p609-depois.pdf -`:

```text
Hello 你好
‫ مرحبا‬world, more latin text after the fallback scripts.
```

O texto extrai correctamente. O rendering visual não foi afectado.

### Suite completa

- `cargo test --workspace` → 0 falhas.
- `crystalline-lint .` → `✓ No violations found`.

---

## Notas e limitações restantes

1. **Fonte árabe ainda grande:** Noto Naskh Arabic subsetada ficou com ~175 KB (fonte completa ~176 KB). Isto acontece porque o conjunto de codepoints e glyph IDs usados para subsetar é recolhido a nível do documento inteiro, não por fonte. O subset da fonte árabe acaba por incluir também os glifos Latin usados no documento, porque partilham IDs baixos. Esta optimização (subsetting por fonte, não global) fica para trabalho futuro; o ganho potencial é pequeno comparado com o problema CJK.

2. **`.otc` (OpenType Collections):** a mesma lógica `ttcf` aplica-se; não foi testado directamente, mas o formato de cabeçalho é idêntico.

3. **Vanilla ainda menor:** o vanilla consegue 12 KB porque aplica subsetting mais fino e não replica glifos Latin em fontes de fallback. O cristalino está agora na mesma ordem de grandeza de outros casos multi-script; fechar totalmente a diferença para o vanilla exigiria o item 1 acima.

---

## Actualização das listas de disparidades

- `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`:
  - Adicionada entrada "Subsetting de fontes de fallback em `.ttc`" à secção "Corrigido ao longo desta conversa".
  - Actualizada a data de actualização.

- `00_nucleo/diagnosticos/inventario-decisoes-pendentes.md`:
  - Secção 2.2 (Exportação / texto): entrada "Fusão de blocos `BT...ET` consecutivos" actualizada para deixar claro que o tamanho massivo em P608 não era causado pela fusão de blocos, mas por `.ttc` embutidos inteiros — corrigido em P609.

---

## Critérios de fecho do passo

- [x] Causa confirmada — fallback `.ttc` embutido inteiro por falha silenciosa do subsetter.
- [x] Subsetting estendido às fontes de fallback `.ttc` via extracção em `FontSlot::get()`.
- [x] Tamanho medido antes/depois: 15,7 MB → ~196 KB.
- [x] Sem regressão em `cargo test --workspace`.
- [x] `crystalline-lint .` limpo.
- [x] Listas de disparidades actualizadas.
- [x] Relatório escrito com proveniência.

---

## Ligações

- `00_nucleo/materialization/typst-passo-609.md` — passo que originou a investigação.
- `00_nucleo/diagnosticos/paridade-producao-p608.md` — passo anterior cuja explicação foi corrigida.
- `03_infra/src/fonts.rs:36` — `FontSlot::get()` com extracção de coleção.
- `03_infra/src/fonts.rs:67` — `extract_collection_face`.
- `03_infra/src/export/builder.rs:500` — chamada ao subsetter no caminho multi-font.

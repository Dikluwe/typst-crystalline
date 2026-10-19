# Relatório de Paridade de Produção — Passo 533

| Campo | Valor |
|-------|-------|
| Passo | P533 |
| Foco | Citações bibliográficas (`@key`) e formatação CSL |
| Data | 2026-07-02 |
| Status | Concluído |

---

## Contexto

O Passo 531 (Grupo 6) confirmou dois sintomas:

1. **`@key1` no corpo do texto não resolve** — o PDF mostrava "See ." em vez de "See [1].".
2. **Formatação CSL corrompida** — aspas tipográficas viravam `?` e o título "Bibliography" aparecia truncado para "Bibliograph".

Este passo sondou as duas causas e implementou o fix da sub-tarefa A. A sub-tarefa B não se reproduziu no estado actual do repositório; as hipóteses foram verificadas e documentadas abaixo.

---

## Sonda

### Sub-tarefa A — `@key1` não resolve

**Perguntas e respostas:**

1. **`@key1` é reconhecido como citação ou como referência cruzada?**
   - O parser markup reconhece `@target` como `SyntaxKind::Ref` (`01_core/src/engine/parse/markup.rs:188`). O eval converte `Expr::Ref` em `Content::reference(name)` (`01_core/src/engine/eval/mod.rs:691-694`). Não há distinção sintáctica entre citação bibliográfica e referência cruzada.

2. **O layout faz lookup na `BibStore`?**
   - Não. `references.rs::layout_ref` (`01_core/src/engine/layout/references.rs:39`) trata todo o `Content::Ref` como referência cruzada: procura label/counter e, se não encontrar, renderiza "?" (ou espaço vazio quando o layout de "?" produz um item invisível).

3. **O resultado (`[1]`) é descartado nalgum sítio?**
   - Não chega a ser produzido. A cadeia eval → layout nunca identifica `@key1` como citação bibliográfica.

4. **O problema depende da ordem de `bibliography()`?**
   - Testes com `bibliography()` antes e depois da citação mostraram o mesmo sintoma ("See ."), confirmando que a causa não é ordem de processamento, mas sim a falta de distinção semântica entre `Ref` e `Cite`.

### Sub-tarefa B — Formatação CSL corrompida

**Hipóteses testadas:**

1. **Truncagem de "Bibliography" para "Bibliograph"**
   - O título "Bibliography" é gerado como `Content::heading(1, "Bibliography")` (`01_core/src/engine/stdlib/structural.rs:1467`) e renderizado pelo layout de heading. Em páginas de largura normal (A4), aparece completo.
   - Reproduziu-se truncagem apenas forçando uma página artificialmente estreita (`#set page(width: 50pt)`), onde a palavra não cabe na linha. Este é um problema geral de *overflow* de palavras longas no `layout_word`, não um bug específico de bibliografia nem corte UTF-8.

2. **Aspas tipográficas viram `?`**
   - Testes com títulos que contêm aspas tipográficas (`"A Sample Paper"`) renderizaram as aspas correctamente com o fallback Helvetica.
   - Os `?` observados no corpus `cite-bibliography.typ` (acentos em "introdutório", "citação") são devido a **font fallback** — a fonte usada não possui glifos para caracteres acentuados. Este é o focus do Passo 534.

3. **Corte a meio de carácter UTF-8**
   - Não foi encontrado nenhum corte por índice de byte no caminho CSL (`01_core/src/engine/layout/bib_csl.rs`), no shaper (`03_infra/src/shaper.rs`) nem no export PDF (`03_infra/src/export/builder.rs`). Todos os pontos de divisão de strings usam fronteiras de carácter (`char_indices`, `chars().take()`, `get(byte_idx..)?.chars()`).

**Conclusão da sonda B:** os sintomas descritos em P531 não se reproduzem no estado actual do repositório para documentos de largura normal. A truncagem é um efeito colateral de overflow de linha; os `?` são falta de glifos na fonte (P534).

---

## Implementação

### Ficheiros alterados

- `01_core/src/engine/introspect.rs` — adicionada função pública `convert_bib_refs_to_cites` que converte `Content::Ref` em `Content::Cite` quando o nome coincide com uma key do `Content::Bibliography`. Usada antes do walk de introspecção e no helper de teste local.
- `01_core/src/engine/layout/references.rs` — `layout_ref` delega para `cite::layout` quando a key existe no `BibStore`, cobrindo o caminho do layout mesmo para content que não passou pela conversão prévia.
- `01_core/src/engine/layout/bib_csl.rs` — `build_cache` / `build_cache_with_style` aceitam `citation_order` opcional e reordenam as entries antes de renderizar a bibliografia (necessário para estilos numéricos como `ieee`).
- `01_core/src/engine/layout/mod.rs` — passa `introspector.citation_order()` para o cache CSL.
- `03_infra/src/pipeline.rs` — aplica `convert_bib_refs_to_cites` ao content de introspecção e ao content de layout antes das respectivas fases.
- `01_core/src/engine/layout/tests.rs` — adicionado teste `layout_ref_bibliografico_renderiza_como_cite`.
- `01_core/src/engine/layout/bib_csl.rs` — adicionado teste `build_cache_ieee_reordena_por_citation_order`; actualizados call sites de `build_cache` para o novo parâmetro.
- `01_core/src/engine/introspect.rs` — adicionado teste `p533_bib_refs_convertidos_contam_citation_order`.

### Lógica do fix

O fix tem duas camadas:

1. **Conversão prévia (introspecção)** — antes do walk de introspecção, todo o `Content::Ref` cujo nome é uma key bibliográfica conhecida é convertido em `Content::Cite`. Isto garante que:
   - o contador `"citation"` avança;
   - `citation_order` e `back_refs` ficam correctos;
   - a bibliografia numérica (`ieee`) pode ser reordenada pela ordem de primeira citação.

2. **Fallback no layout** — `references.rs::layout_ref` verifica `bib_entry_for_key`. Se a key for bibliográfica, constrói um `CiteElem` e delega a `cite::layout`. Isto cobre callers directos de `layout_with_introspector` que ainda possam passar `Content::Ref` sem conversão prévia.

A reordenação do cache CSL é feita apenas quando `citation_order` é fornecido. Entries não citadas ficam no final, na ordem original do ficheiro `.bib`/`.yaml`.

---

## Validação

### Citação inline

```typst
#bibliography("refs.bib", style: "ieee")
See @key1.
```

| Cenário | Cristalino | Vanilla 0.15.0 |
|---------|------------|----------------|
| `bibliography()` antes da citação | `See [1].` | `See [1].` |
| `bibliography()` depois da citação | `See [1].` | `See [1].` |
| Múltiplas citações (`@key2`, `@key1`) | `[1]`, `[2]` por ordem de aparição | `[1]`, `[2]` por ordem de aparição |
| Bibliografia ordenada por citação | `[1] S. Author`, `[2] T. Author` | `[1] S. Author`, `[2] T. Author` |

### Formatação CSL

| Sintoma P531 | Reproduzido? | Causa verificada |
|--------------|--------------|------------------|
| "Bibliography" → "Bibliograph" | Não (A4) / Sim (página 50pt) | Overflow de palavra longa; não é corte UTF-8 |
| Aspas tipográficas → `?` | Não (Helvetica tem os glifos) | Font fallback para caracteres acentuados (P534) |

### Testes

| Comando | Resultado |
|---------|-----------|
| `cargo test -p typst-core` | 3553 passed; 0 failed |
| `cargo test -p typst-infra` | 565 passed; 0 failed |
| `cargo test --workspace` | all passed |
| `crystalline-lint .` | zero violations |

---

## Limitações

- O output textual extraído por `pdftotext` pode mostrar espaços extra entre caracteres (ex.: `[ 1 ]` em vez de `[1]`). Isto é um gap pré-existente no export PDF: cada `FrameItem::Text` vira um `BT...ET` separado, e `pdftotext` insere espaços entre objectos de texto. Visualmente o PDF está correcto.
- A formatação CSL ainda depende da fonte ter os glifos necessários. Caracteres acentuados ou símbolos sem glifo na fonte activa produzem `?` — este é o focus do Passo 534 (fallback de fonte por carácter).
- A truncagem de títulos longos em páginas muito estreitas é um problema geral de *line overflow*, não tratado neste passo.

---

## Decisão

A sub-tarefa A (**citação inline `@key`**) está **fechada em P533**.

A sub-tarefa B (**formatação CSL corrupta**) **não se reproduziu** no estado actual para documentos de largura normal; os sintomas observados foram decompostos em:
- overflow de linha (não específico de bibliografia);
- font fallback (Passo 534).

---

## Reprodução

```bash
# Compilar
cargo build --release --bin typst

# Documento de teste
cat > /tmp/test-cite.typ <<'EOF'
#bibliography("refs.bib", style: "ieee")
See @key1.
EOF
cat > /tmp/refs.bib <<'EOF'
@article{key1,
  title = {A Sample Paper},
  author = {Author, Test},
  year = {2024},
}
EOF
./target/release/typst /tmp/test-cite.typ /tmp/cite.pdf
pdftotext /tmp/cite.pdf -

# Testes e linter
cargo test -p typst-core
cargo test -p typst-infra
crystalline-lint .
```

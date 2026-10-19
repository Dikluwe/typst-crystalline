# Passo 146 — Relatório (multi-font per document; ADR-0055 decisão 5)

**Data**: 2026-04-24
**Natureza**: passo **substantivo** (L3 + L0 + ADR anotação +
DEBT.md + README ADR). Decisão 5 de ADR-0055 (multi-font per
document) materializada voluntariamente pós-fecho de DEBT-1.
**Sem ADR nova**; sem revisão de ADR-0055 (status permanece
`IMPLEMENTADO`). **Sem crates novas**.
**Precondição**: Passo 144 encerrado; gap 7 fechado; 1104
tests; zero violations; 57 ADRs.

---

## 1. Sumário executivo

`compile_to_pdf_bytes` deixou de ser single-font: `collect_fonts_from_doc`
acumula todas as `FontList` distintas no documento;
`resolve_fonts` map-filter de `resolve_font` (silent drop);
nova função `export_pdf_multifont` constrói resource dict com
N entradas `/F1..N` apontando para `/CrystallineFont1..N` e
emite cada `FrameItem::Text` com a font correcta. Single-font
preserva o caminho `export_pdf_with_font` por dispatch.

Documento com 2 spans `#set text(font:)` em famílias
diferentes (ambas resolvíveis) agora produz PDF com 2
`/Subtype /Type0` — a primeira não é silenciosamente
descartada como pré-146.

**Tests**: 1104 → **1113** (+9: 4 unit `collect_fonts_from_doc_*`
+ 3 unit `resolve_fonts_*` + 2 integração `font_wiring_multifont_*`).
Test 140B `font_wiring_segunda_font_diferente_primeira_vence`
renomeado para `..._ambas_embebidas` com assertion `1 → 2`
(regressão deliberada do MVP).

---

## 2. Decisão de numeração: 146 (não 142A)

Notas operacionais de 142, 143 e 145 referem "Passo 142A"
como candidato para multi-font. **Não usado** neste passo:

- 142 já foi executado como passo administrativo de fecho de
  DEBT-1.
- Sufixo `A` na grelha actual significa
  **diagnóstico-primeiro** associado ao mesmo número de
  trabalho substantivo (precedente: 131A→131B; 132A→132B;
  140A→140B). Cada par é "diagnóstico + materialização".
- "142A" para multi-font seria ambíguo: associado a 142
  administrativo (que não tem materialização pendente)? a
  algum diagnóstico futuro?
- 146 é o seguinte número natural após 145, sem colisão com
  144 (já executado como passo de hyphenation).

Decisão registada na história. Notas operacionais futuras
devem referir-se a "Passo 146 (multi-font)" e não "142A".

---

## 3. Inventário pré-materialização

### 3.1. API actual (sub-passo 146.1)

- `first_font_from_doc(doc) -> Option<FontList>` em
  `03_infra/src/pipeline.rs:96` (Passo 140B).
- `resolve_font(font_list, font_book, world) -> Option<Vec<u8>>`
  em `03_infra/src/pipeline.rs:142` (Passo 141 — itera famílias).
- `export_pdf_with_font(doc, font_data: &[u8]) -> Vec<u8>` em
  `03_infra/src/export.rs:30`.
- `build_cidfont` em `03_infra/src/export.rs:423` (108 linhas).
- `FontList` deriva `Debug, Clone, Eq, PartialEq, Hash` em
  `01_core/src/entities/font_list.rs:56` — **dedup trivial via
  `Vec::contains`** sem alteração em L1.

### 3.2. Usos externos de `export_pdf_with_font` (sub-passo 146.1.A.2)

3 chamadas detectadas:
1. `pipeline.rs:87` — dispatch principal (substituído por 146).
2. `integration_tests.rs:137` — test directo do export (preservado).
3. `integration_tests.rs:620` — test directo do export (preservado).

**Decisão**: opção A pragmática — `export_pdf_with_font`
**preservada** no API público; novo `export_pdf_multifont`
paralelo. Single-font passa por `export_pdf_with_font` quando
`vec.len() == 1`. Convergência total (opção B do spec) recusada
porque o nome canónico `/CrystallineFont` (sem sufixo numérico)
do single-font path tem testes que dependem dele
(`font_wiring_set_text_font_existente_embute_cidfont` etc.).
Multi-font usa `/CrystallineFont1..N`.

### 3.3. Estrutura de `build_cidfont`

5 objectos PDF emitidos por font: Type0 (`/F1`), CIDFontType2,
FontDescriptor, FontFile2 stream, ToUnicode CMap. Refactor
**não** foi feito — `build_cidfont` permanece intacto para o
caminho single-font. `build_multifont` (novo) duplica a lógica
generalizando para N fonts. Trade-off: ~140 linhas
duplicadas em troca de zero risco em código existente.

---

## 4. `collect_fonts_from_doc`

**Assinatura**:

```rust
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList>
```

Itera `doc.pages → items` recursivamente (atravessa
`FrameItem::Group`); dedup estrutural por `Vec::contains`
sobre `FontList` (que deriva `PartialEq`). Ordem preservada
por primeira ocorrência.

**4 unit tests**:

| Test | Cenário | Assert |
|------|---------|--------|
| `collect_fonts_from_doc_documento_vazio_devolve_vazio` | doc vazio | `Vec::new()` |
| `collect_fonts_from_doc_uma_font_devolve_unitario` | 1 FrameItem com font "Inria" | `vec!["inria"]` |
| `collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem` | 2 spans com fonts diferentes | `["primeira", "segunda"]` |
| `collect_fonts_from_doc_duas_iguais_dispersas_dedup` | A e B intercalados em 2 páginas | `["a", "b"]` (dedup) |

**Localização**: L3 (`pipeline.rs`). Junto a
`first_font_from_doc` para coerência semântica.

---

## 5. `resolve_fonts`

**Assinatura**:

```rust
fn resolve_fonts(
    font_lists: &[FontList],
    font_book:  &FontBook,
    world:      &dyn World,
) -> Vec<(FontList, Vec<u8>)>
```

Map-filter sobre `resolve_font` (Passo 141). Silent drop para
entries que não resolvem.

**3 unit tests**:

| Test | Cenário | Assert |
|------|---------|--------|
| `resolve_fonts_todos_resolvem` | 2 FontList; ambas presentes em FontBook | 2 entradas com bytes correctos |
| `resolve_fonts_alguns_nao_resolvem_filtrados` | 2 FontList; apenas 1 em FontBook | 1 entrada |
| `resolve_fonts_nenhum_resolve_devolve_vazio` | 2 FontList; nenhuma em FontBook | `Vec::new()` |

---

## 6. `export_pdf_multifont` + `build_multifont` + page stream

### 6.1. API público

```rust
pub fn export_pdf_multifont(
    doc:   &PagedDocument,
    fonts: &[(FontList, Vec<u8>)],
) -> Vec<u8>
```

Parse de cada bytes via `Face::parse(...)`; se algum falha,
fallback para `export_pdf` (Helvetica). Caso contrário invoca
`PdfBuilder::new().build_multifont(doc, fonts, &faces)`.

### 6.2. `build_multifont` (novo, em `PdfBuilder`)

~140 linhas. Emite por font 5 objectos com IDs consecutivos
(`fonts_start + 5*fi`); BaseFont nameado `/CrystallineFont{i+1}`.
Resource dict por página: `/F1..N` apontando para os Type0
correspondentes.

Por-font:
- `mappings = map_chars_to_glyphs(face, &chars)` (chars
  partilhados entre fonts).
- Math glyph variants adicionadas via
  `build_math_glyph_reverse_map` (paridade com `build_cidfont`).
- `widths = widths_array(face, &mappings)`.
- `to_unicode_cmap(&mappings)` cria CMap.

**`build_cidfont` actual permanece intacto** (decisão 7 do
spec). Refactor mínimo para extrair função interna recusado
em favor de duplicação contida em `build_multifont`. Aceita-se
~100 linhas de duplicação; alternativa (refactor) tinha
risco de afectar single-font path.

### 6.3. `build_page_stream_multifont` (novo helper)

~120 linhas (cópia de `build_page_stream_cidfont` com font
selection no arm `FrameItem::Text`). Para cada
`FrameItem::Text`:

```rust
let fi = style.font.as_ref()
    .and_then(|fl| fonts.iter().position(|(stored, _)| stored == fl))
    .unwrap_or(0);  // default font 0 se None ou sem match
ops.push_str(&format!(
    "BT\n/F{} {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
    fi + 1, style.size.val(), pos.x.val(), pdf_y
));
```

`hex_str` usa `per_font_char_to_gid[fi]` (mapping específico
da face seleccionada).

`FrameItem::Glyph` (variantes matemáticas) emite sempre em
`/F1` — math typesetting tipicamente usa apenas uma font. Se
multi-font math for necessário no futuro, seria endereçado
em passo dedicado.

`FrameItem::Line/Image/Shape/Group` arms são cópia byte-exacta
de `build_page_stream_cidfont` (sem dependência de font).

### 6.4. Trade-off de Helvetica fallback dentro de multi-font

Spec linha 252-255 sugere que spans sem `style.font` em
documento multi-font emitem com Helvetica. **Decisão divergente**:
spans sem match ou sem `style.font` usam **font 0** (primeira
font embebida). Razão:
- Resource dict em `build_multifont` tem `/F1..N` (CIDFont)
  mas **não** tem `/F1..F3` (Helvetica Type1) do `build_helvetica`.
  Adicionar Helvetica ao multi-font causaria conflito de
  nomes ou exigiria refactor maior.
- Default-to-font-0 é consistente com o caminho single-font
  (`build_cidfont`): todos os spans usam `/F1` independentemente
  de `style.font`.
- Se utilizadores reportarem confusão ("o meu texto sem font
  declarada renderizou em Inria Serif"), abrir trabalho
  dedicado.

Registado como divergência aceite face ao spec.

---

## 7. Dispatch em `compile_to_pdf_bytes`

```rust
let font_lists = collect_fonts_from_doc(&doc);
let resolved = resolve_fonts(&font_lists, world.book(), world);
let pdf = match resolved.as_slice() {
    []         => export_pdf(&doc),
    [(_, b)]   => export_pdf_with_font(&doc, b),
    many       => export_pdf_multifont(&doc, many),
};
```

Match exaustivo. Single-font (`[(_, b)]`) preserva caminho
140B/141 com nome canónico `/CrystallineFont` (verificado por
`font_wiring_multifont_regressao_single_font_inalterado`).

---

## 8. Tests adicionados; revisão do test do 140B

### 8.1. Unit (7) em `pipeline.rs::tests`

`collect_fonts_from_doc_documento_vazio_devolve_vazio`,
`collect_fonts_from_doc_uma_font_devolve_unitario`,
`collect_fonts_from_doc_duas_distintas_devolve_par_em_ordem`,
`collect_fonts_from_doc_duas_iguais_dispersas_dedup`,
`resolve_fonts_todos_resolvem`,
`resolve_fonts_alguns_nao_resolvem_filtrados`,
`resolve_fonts_nenhum_resolve_devolve_vazio`.

### 8.2. Integração L3 (2 novos)

| Test | Assert |
|------|--------|
| `font_wiring_multifont_uma_resolve_outra_falla_silent_drop` | doc com 3 fonts (1 inexistente) → 2 `/Subtype /Type0` |
| `font_wiring_multifont_regressao_single_font_inalterado` | doc single-font → 1 `/Subtype /Type0` + nome canónico `/CrystallineFont` (sem sufixo) |

### 8.3. Test do 140B renomeado

```diff
- fn font_wiring_segunda_font_diferente_primeira_vence()
+ fn font_wiring_segunda_font_diferente_ambas_embebidas()
```

Assertion ajustada:
```diff
- assert_eq!(n_type0, 1, "MVP single-font: exactamente 1 Type0");
+ assert_eq!(n_type0, 2, "Pós-146 multi-font: exactamente 2 Type0");
```

Documento com duas fonts distintas agora produz 2 Type0.
Regressão **deliberada** do MVP do 140B (decisão 3) para a
materialização da decisão 5. Documentada aqui e no diff do
teste.

### 8.4. Test do spec não-implementado

Spec lista `font_wiring_multifont_dois_fonts_distintos_ambos_embebidos`
como teste 1. Cobertura essencialmente equivalente ao test
renomeado `font_wiring_segunda_font_diferente_ambas_embebidas`
— **sem duplicação**: o test renomeado herda o mesmo nome
semântico (rastreabilidade preservada) e o spec é cumprido em
intenção.

---

## 9. Anotação em ADR-0055

Adicionada linha no cabeçalho:

```markdown
**Anotação Passo 146**: decisão 5 (multi-font per document)
materializada — ver
[`00_nucleo/materialization/typst-passo-146-relatorio.md`](...).
Modelo análogo a ADR-0019 + nota factual de 140A: anotação
factual sem revisão; status permanece `IMPLEMENTADO`.
```

Secção "Materialização" estendida com entrada Passo 146.
Parágrafo de scope-out actualizado: decisão 5 movida de
"scope-out" para "materializada"; decisão 6 referenciada por
Passo 144 (gap 7); decisão 7 (rustybuzz) permanece scope-out.

**Status `IMPLEMENTADO` preservado**. Sem revisão. Modelo
ADR-0019 + 140A.

---

## 10. Edição L0 + hash propagado

`prompts/infra/pipeline.md`: secções "Pipeline" e "Helpers
privados" reescritas. Novo helper `collect_fonts_from_doc`
documentado; `first_font_from_doc` marcado como preservado
em `#[allow(dead_code)]`. Dispatch novo (3 arms) descrito.

**Hash recalculado**: `00e4ebd3 → 1b030acd`. Propagado a
`03_infra/src/pipeline.rs`. (Outros consumers do prompt:
inexistentes — apenas pipeline.rs cita-o.)

`crystalline-lint .`: ✓ zero violations.

---

## 11. DEBT-52 actualizado

`00_nucleo/DEBT.md` Secção 2 — entrada DEBT-52 ENCERRADO ganha
"Actualização Passo 146 — Multi-font per document (decisão 5)".
**DEBT-52 não reabre**: padrão consistente com 144 (gap 7).
Contagem de DEBTs abertos: **inalterada (10)**.

---

## 12. README dos ADRs actualizado

Nova entrada "Passo 146" em "Passos-chave da história dos
ADRs". **Sem mudança** na tabela "Estado por ADR" (ADR-0055
permanece `IMPLEMENTADO`); sem mudança na distribuição de
status; sem mudança no total (57 ADRs).

---

## 13. Limitações registadas

1. **Variant-aware ainda ausente** (ADR-0055bis candidata).
   `FontVariant::default()` é usada em `resolve_font`;
   `weight: 700` continua a usar faux-bold do Passo 139.
2. **Subsetting ausente** (ADR-0056 candidata). Multi-font
   embute font inteira por entrada; PDF com N fonts é N×
   maior.
3. **Shaping ausente** (DEBT-53 candidato XL). Cada font
   usa CMAP directo sem rustybuzz.
4. **Gap 8 (font dict) opcional** — ADR-0054bis condicional.
5. **Helvetica fallback dentro de multi-font não disponível**
   — divergência face ao spec (registada em §6.4). Spans sem
   match ou sem `style.font` usam font 0 do multi-font.
6. **`build_multifont` duplica lógica de `build_cidfont`**
   (~100 linhas). Refactor para função partilhada é candidato
   futuro de baixa prioridade.
7. **`build_page_stream_multifont` duplica
   `build_page_stream_cidfont`** (~120 linhas). Mesmo
   trade-off.
8. **Reprodutibilidade tests CI** — limitação 7 do 142
   permanece. Tests `font_wiring_multifont_*` dependem de
   fonts no sistema; graceful skip via
   `discover_any_system_fonts`.
9. **Optimização O(N²) → O(N)** em `collect_fonts_from_doc`
   é trivial via `HashSet<FontList>` (FontList deriva Hash).
   Por agora, `Vec::contains` é suficiente para documentos
   típicos (<10 fonts).
10. **Math glyph emit em `/F1`**: variantes matemáticas
    (`FrameItem::Glyph`) sempre na primeira font. Multi-font
    math seria trabalho dedicado.

---

## 14. Verificação final

| Item | Estado |
|------|--------|
| `collect_fonts_from_doc` + 4 unit tests | ✅ |
| `resolve_fonts` + 3 unit tests | ✅ |
| `export_pdf_multifont` materializada | ✅ |
| `build_multifont` em `PdfBuilder` | ✅ |
| `build_page_stream_multifont` helper | ✅ |
| Dispatch em `compile_to_pdf_bytes` (3 arms) | ✅ |
| 2 tests integração L3 (multi-font + regressão) | ✅ |
| Test do 140B renomeado + assertion 1→2 | ✅ |
| ADR-0055 anotada (`IMPLEMENTADO` preservado) | ✅ |
| DEBT-52 secção 2 com actualização Passo 146 | ✅ |
| README dos ADRs com entrada P146 | ✅ |
| L0 `prompts/infra/pipeline.md` actualizado; hash `1b030acd` | ✅ |
| L1 de domínio intacto (apenas L3 + L0 + ADRs + DEBT.md + README) | ✅ |
| `cargo test --workspace --lib` | 874 + 215 + 24 = **1113 passed** (+9 vs P144) |
| `crystalline-lint .` | ✅ zero violations |
| Sem ADR nova; sem revisão de status | ✅ |
| Sem crates novas; sem mudança em `Cargo.toml` ou `crystalline.toml` | ✅ |

**Pós-146**: Fase C de DEBT-1 está completa **incluindo
multi-font**. Limitações remanescentes (mapa do relatório 142
§9):
- 1. Variant-aware (ADR-0055bis candidata).
- 2. ~~Multi-font~~ — fechada por 146.
- 3. Subsetting (ADR-0056 candidata).
- 4. Shaping rustybuzz (DEBT-53 candidato XL).
- 5. ~~Lang/hyphenation~~ — fechada por 144.
- 6. Font dict (ADR-0054bis condicional, gap 8).
- 7. Reprodutibilidade tests CI.

**Restam 5 candidatos** não-bloqueantes.

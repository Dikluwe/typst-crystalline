# Pipeline — L3 orquestração
Hash do Código: 7de28a14

## Módulo
`03_infra/src/pipeline.rs`

## Propósito

Orquestra o pipeline completo de compilação em L3. Esconde o
boilerplate `comemo` (Routines, Traced, Sink, Route) e expõe
APIs alto-nível à L4 (04_wiring) e aos testes.

Materializado no Passo 113 (ADR-0046) a partir de helpers
test-only em `integration_tests.rs`.

## Contrato

### `eval_to_module_with_sink`

```rust
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>);
```

- Chama `eval()` com o boilerplate `comemo` completo.
- Warnings drenados do `Sink` e devolvidos separadamente.
- Não formata — retorna `Vec<SourceDiagnostic>` cru.
- Caller (CLI ou testes) decide formatação via
  `diagnostic_format::drain_diagnostics_to_stderr`.

### `compile_to_pdf_bytes`

```rust
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>);
```

- Pipeline `eval → introspect → layout → (dispatch export)`.
- **P617** — variantes `_with_document_id` aceitam um `Option<[u8; 16]>`
  externo e propagam-no ao `PdfBuilder`. Quando `Some`, esse valor
  fixa o `xmpMM:DocumentID`; `xmpMM:InstanceID` continua aleatório.
  Quando `None`, mantém o comportamento de P615 (aleatório). As
  funções públicas sem sufixo mantêm `None` e preservam a API
  existente.
- Dispatch font-aware multi-font (Passos 140B + 141 + 146,
  ADR-0055 `IMPLEMENTADO`; decisão 5 anotada por 146):
  - **Colecciona** todas as `FontList` distintas no
    `PagedDocument` em ordem de primeira ocorrência (atravessa
    `FrameItem::Text` e `FrameItem::Group` recursivamente).
    Dedup estrutural via `Vec::contains` (O(N²); N tipicamente
    pequeno).
  - Para cada `FontList`, **itera todas as famílias** em ordem
    (Passo 141): consulta
    `world.book().select(name, &FontVariant::default())` →
    índice, depois `world.font(index)` → bytes. **Primeira
    família a completar ambos os passos vence**. Cenário
    patológico (índice stale) não curto-circuita.
  - Filtra entries que não resolvem (silent drop).
  - Dispatch:
    - `[]` (vec vazio) → `export_pdf(&doc)` (fallback
      Helvetica Type1).
    - `[(_, bytes)]` (uma única) →
      `export_pdf_with_font(&doc, &bytes)` (caminho preservado
      do 140B/141 — output `/CrystallineFont` único).
    - `many` (2+) → `export_pdf_multifont(&doc, many)` —
      resource dict `/F1..N` com `/CrystallineFont1..N`; cada
      `FrameItem::Text` selecciona `/F{i+1}` por match
      estrutural contra a sua `style.font` (default `/F1`
      quando `style.font` é `None` ou não casa).
- **Multi-font per document** (ADR-0055 decisão 5,
  materializada no Passo 146): N fonts distintas → N
  `/Subtype /Type0` no PDF. Single-font como caso particular
  (preservado por dispatch).
- **Instrumentação de benchmark** (Passo 518): a struct
  `Timings` expõe `shape_ms` e `subset_ms` para permitir a
  análise de gargalos do pipeline de produção real. O tempo
  de `render_ms` passa a ser o tempo de export PDF *após* o
  subsetting.
- **Array fallback chain** (ADR-0055 decisão 4, Passo 141):
  dentro de uma `FontList`, todas as famílias são tentadas
  até resolver.
- Selecção usa `FontVariant::default()` (regular, normal, normal
  stretch). `weight`/`style` no documento continuam a ser
  renderizados via faux-bold/faux-italic (Passo 139) — selecção
  variant-aware é candidato ADR-0055bis.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

### `expand_context_blocks` (P506, corrigido P711)

```rust
pub fn expand_context_blocks(
    content: Content,
    intr: &TagIntrospector,
    world: &dyn World,
    source: &Source,
) -> SourceResult<Content>;

fn collect_context_blocks(
    content: &Content,
    chain: &StyleChain,
) -> HashMap<u64, (Arc<ContextBlockElem>, StyleChain)>;
```

- Passo intermédio do pipeline, entre `introspect` e `layout`
  (`compile_to_pdf_bytes_full_error`, §P506): resolve cada
  `Content::ContextBlock` chamando a sua closure via `apply_func`
  e substituindo o nó pela `Content` resultante
  (`value_to_content`).
- **P711 — `StyleChain` da posição, não `default_chain()`.**
  Paridade com o vanilla (`typst-library/foundations/context.rs`
  `CONTEXT_RULE`): o `context` block é um show rule que recebe o
  `styles: StyleChain` **da posição onde aparece no documento**
  (o `StyleChain` acumulado por `#set` léxicos/`Content::Styled`
  ancestrais), não uma cadeia de defaults isolada. `collect_context_blocks`
  acumula essa cadeia durante o walk: parte de
  `StyleChain::default_chain()` e aplica `.push_styles(styles)`
  a cada `Content::Styled(inner, styles)` atravessado, associando
  a cadeia resultante (não só o `id`/elem) a cada `ContextBlockElem`
  encontrado. `expand_context_blocks` usa essa cadeia (não
  `default_chain()`) como `engine.styles` ao invocar a closure.
  Antes da correção, `.to-absolute()` (e qualquer resolução
  dependente de estilo) dentro de `context {...}` ignorava
  silenciosamente `#set text(size: ...)` e outros `#set`
  ancestrais, usando sempre os defaults (`size: 11.0`).
- `collect_context_blocks` continua um walk parcial (Sequence,
  Styled, Strong, Emph, Heading) — `ContextBlock` não é esperado
  aninhado dentro de Grid/Table/etc. neste subset (scope-out
  pré-existente, inalterado por P711).
- Fora do escopo de P711, medido mas não corrigido aqui (passos
  próprios): `repr_value` formata `Value::Length`/`Ratio`/`Angle`/
  `Color`/`Stroke`/`Align` com `{:?}` do Rust em vez do repr Typst
  quando embutidos directamente em markup (bug pré-existente,
  independente de `context`); `measure()` devolve sempre `0pt`
  (dentro e fora de `context`) e não tem o gate "can only be used
  when context is known" do vanilla.

## Helpers privados de dispatch (Passos 140B + 141 + 146)

```rust
fn collect_fonts_from_doc(doc: &PagedDocument) -> Vec<FontList>;
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
fn resolve_fonts(
    font_lists: &[FontList],
    font_book:  &FontBook,
    world:      &dyn World,
) -> Vec<(FontList, Vec<u8>)>;

// Helper preservado do 140B (test-only no pipeline.rs):
#[allow(dead_code)]
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
```

- `collect_fonts_from_doc` (Passo 146): itera `doc.pages →
  items` recursivamente (atravessa `FrameItem::Group`) e
  devolve **todas** as `FontList` distintas em ordem de
  primeira ocorrência. Dedup estrutural via `Vec::contains`.
- `resolve_font` (Passo 141): itera `font_list.as_slice()` em
  ordem. Para cada família, consulta
  `font_book.select(name, &FontVariant::default())`; se devolve
  `Some(index)`, chama `world.font(index)`; se devolve
  `Some(font)`, devolve os bytes. **Primeira família a completar
  ambos os passos vence**. Cenário patológico (índice stale)
  não curto-circuita.
- `resolve_fonts` (Passo 146): map-filter de `resolve_font`
  sobre `&[FontList]`. Devolve `(FontList, bytes)` por entrada
  resolvida (silent drop para entries que não resolvem;
  consistente com 140B).
- `first_font_from_doc` (Passo 140B, **preservado em
  `#[allow(dead_code)]`**): historicamente o entry-point do MVP
  single-font (devolvia primeira `FontList`). Substituído pelo
  dispatch multi-font no Passo 146; permanece como referência
  e para os testes unitários do 140B continuarem activos.
- Funções privadas ao módulo e cobertas por testes unitários
  em `#[cfg(test)] mod tests`.

Multi-font materializada no Passo 146 (ADR-0055 decisão 5
anotada). Variant-aware (ADR-0055bis, candidata) e subsetting
(ADR-0056, candidata) permanecem fora do escopo actual.

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).

## Conversão de `layout_warnings` em diagnósticos

Após `layout_with_introspector_and_metrics`, a pipeline converte cada
entrada de `doc.layout_warnings` (strings puras produzidas em L1 — ex.:
aviso de body de footnote que excede a página/coluna, ver
`compiler/footnote_overflow_columns.md`) em
`SourceDiagnostic::warning(Span::detached(), msg)` e adiciona-a ao `Vec`
de `warnings` que a pipeline já retorna. L1 permanece data-only (ADR-0043):
nenhum `SourceDiagnostic` é construído em L1.

## P906 — `collect_fonts_in_items` cego a `FrameItem::Glyph`

**Contexto**: mecanismo de esticamento horizontal (`compiler/layout.md`
§P906) — achado mais fundo, na SELECÇÃO de quais fontes embutir no PDF
(distinto do embedding/subsetting, `export/builder.md` §P906).

**Achado, confirmado por instrumentação directa** (não inferido):
`collect_fonts_in_items` (usada por `collect_fonts_from_doc`, que decide
Cidfont-vs-Multifont e QUAIS fontes resolver) tinha o braço
`FrameItem::Glyph { .. } => {}` — ignorava por completo qualquer glifo de
esticamento matemático. Como a decisão Cidfont/Multifont é feita
EXCLUSIVAMENTE a partir de `style.font` visto em `Text`/`TextShaped`, uma
equação cujo ÚNICO conteúdo a precisar da fonte companion MATH fosse um
glifo de esticamento (`underbracket(a+b+c)` sem mais texto itálico
matemático à volta — caso isolado por outro achado incidental, P906
também, em `apply_math_default` não recursar `MathUnderover`) escolhia só a
fonte de corpo como candidata Cidfont única. O `glyph_id` do esticamento,
correcto na fonte MATH (confirmado via `FallbackFontMetrics`, que já
resolve correctamente para efeitos de LAYOUT), caía fora do subset embutido
— glifo `.notdef` no PDF real, apesar do mecanismo de layout estar
inteiramente correcto e testado.

**Correcção**: `collect_fonts_from_doc`/`collect_fonts_in_items` ganham
`world: &dyn World`, constroem um `FallbackFontMetrics` local, e o braço
`FrameItem::Glyph { style, base_char, .. }` chama o novo método
`FallbackFontMetrics::resolve_font_combo(base_char, style)` (`infra/
font_metrics.md` §P906 — mesmo mecanismo `covering`/`resolve_primary_with_
math_fallback` já usado internamente para LAYOUT, agora reaproveitado para
devolver a IDENTIDADE da fonte, não dados de glifo) — o resultado entra na
mesma lista `seen`/dedup, como mais um span de "texto" para efeitos de
selecção.

## P836 — chave de fonte com variações explícitas

`collect_fonts_from_doc`/`resolve_fonts` passam a chavear por
`(FontList, FontVariant, FontVariations)` (o terceiro componente vem de
`TextStyle::variations`, `unwrap_or_default`): dois runs com o mesmo
`FontVariant` mas `variations:` distintas embutem fontes instanciadas
distintas — paridade vanilla (`FontInstance` chaveado por variações
completas).

O gate `needs_variable_font_instancer` e o desvio single-font→multi-font
passam a usar `axis_variations_for_text_style` (fusão derivados +
explícitos), de modo que um documento cuja única variação é explícita
(ex. `wght: 250` com weight regular) também dispara a instanciação.

---

## P844 (achado #54 de P831) — re-introspecção pós-expansão

- Nova função pública `expand_context_blocks_and_reintrospect` (expansão + `introspect_with_introspector` do conteúdo expandido). Sonda: o `ContextBlock` é locatable no walk de introspecção, mas a expansão substitui-o por conteúdo não-locatable; o Locator do walk de layout (invariante P185C) desfasava face ao introspector pré-expansão e `CounterRegistry::value_at` devolvia o snapshot anterior (headings renumeravam a partir do bloco: `1.|1.|2.` em vez de `1.|2.|3.`). A pipeline de produção usa a nova função e re-injecta os styles CSL no `BibStore` reconstruído; o introspector reconstruído também alimenta `intr_for_positions` (P535). Probes que confirmaram o mecanismo: `#metadata(1)` e `#counter(heading).step()` entre headings (locatable que permanece) não dessincronizam.


## P956 — `stream_mode` nas entry points PDF

ADR-0126 (emendada P956): todas as variantes `compile_to_pdf_bytes*`
(`compile_to_pdf_bytes`, `_with_timings`, `_with_timings_full_error`,
`_with_timings_full_error_and_document_id`, `_full_error`,
`_full_error_and_document_id`) ganham `stream_mode: StreamMode` como **último
parâmetro**, propagado ao dispatch de export (`export_pdf*` —
`infra/export/mod.md` §P956). Quebra de assinatura deliberada (mesma razão de
`mod.md` §P956: caller nenhum fica ambíguo sobre o modo).

- O dispatch font-aware (`export_pdf` / `export_pdf_with_font` /
  `export_pdf_multifont`) passa o modo recebido a qualquer dos três ramos.
- `compile_to_png_bytes*` / `compile_to_svg_string*` **inalterados** — a flag
  `--compact` não tem significado fora do PDF (ver `shell/cli.md` §P956).

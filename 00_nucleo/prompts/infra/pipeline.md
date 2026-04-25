# Pipeline — L3 orquestração
Hash do Código: 88dcf9b6

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

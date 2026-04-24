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
- Dispatch font-aware (Passo 140B, ADR-0055):
  - Procura o primeiro `TextStyle.font` (`FontList`) não-`None`
    no `PagedDocument` (itens `FrameItem::Text` e `Group`
    recursivos).
  - Tenta resolver a primeira família via
    `world.book().select(name, &FontVariant::default())` → índice.
  - Se o índice resolve e `world.font(index)` retorna `Some(Font)`,
    invoca `export_pdf_with_font(&doc, font.as_slice())`.
  - Caso contrário (sem `font` no documento, família não conhecida
    pelo `FontBook`, ou slot vazio), fallback `export_pdf(&doc)`
    (Helvetica Type1).
- **MVP single-font per document** (ADR-0055 decisão 3): apenas
  a primeira font encontrada é usada; spans subsequentes com font
  diferente são silenciosamente ignorados. Fallback chain array
  (ADR-0055 decisão 4) é Passo 141.
- Selecção usa `FontVariant::default()` (regular, normal, normal
  stretch). `weight`/`style` no documento continuam a ser
  renderizados via faux-bold/faux-italic (Passo 139) — selecção
  variant-aware é candidato ADR-0055bis.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

## Helpers privados de dispatch (Passo 140B)

```rust
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
```

- `first_font_from_doc`: itera `doc.pages → items` recursivamente
  (atravessa `FrameItem::Group`) e devolve o primeiro
  `TextStyle.font` com `Some(FontList)`. `None` se nenhum item
  tem font definida.
- `resolve_font`: pega na primeira família de `font_list`,
  consulta `font_book.select(name, &FontVariant::default())`;
  se match, devolve `Some(world.font(index)?.as_slice().to_vec())`.
  Apenas a primeira família é tentada — array fallback é Passo
  141.
- Ambas as funções são privadas ao módulo e cobertas por testes
  unitários em `#[cfg(test)] mod tests`.

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).

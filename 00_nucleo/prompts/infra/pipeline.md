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
- Dispatch font-aware (Passos 140B + 141, ADR-0055
  `IMPLEMENTADO`):
  - Procura o primeiro `TextStyle.font` (`FontList`) não-`None`
    no `PagedDocument` (itens `FrameItem::Text` e `Group`
    recursivos).
  - **Itera todas as famílias** da `FontList` em ordem; para
    cada uma, tenta
    `world.book().select(name, &FontVariant::default())` → índice
    e depois `world.font(index)` → bytes. **Primeira família a
    completar ambos os passos vence** (semântica vanilla
    "primeira-que-resolve").
  - Se alguma família resolve, invoca `export_pdf_with_font(&doc,
    font.as_slice())`.
  - Caso contrário (sem `font` no documento, ou nenhuma família
    da lista conhecida pelo `FontBook`, ou todos os slots vazios),
    fallback `export_pdf(&doc)` (Helvetica Type1).
- **Single-font per document** (ADR-0055 decisão 3): a primeira
  `FontList` encontrada no `PagedDocument` é usada para o
  documento inteiro; spans subsequentes com `FontList` diferente
  são silenciosamente ignorados. Multi-font per document (ADR-0055
  decisão 5) é Passo 142 opcional.
- **Array fallback chain** (ADR-0055 decisão 4) materializada
  no Passo 141: dentro de uma `FontList`, todas as famílias são
  tentadas até resolver. Cenário patológico (`select` devolve
  `Some` mas `world.font` devolve `None` — índice stale) **não**
  curto-circuita: continua a tentar as famílias seguintes.
- Selecção usa `FontVariant::default()` (regular, normal, normal
  stretch). `weight`/`style` no documento continuam a ser
  renderizados via faux-bold/faux-italic (Passo 139) — selecção
  variant-aware é candidato ADR-0055bis.
- Warnings sempre devolvidos (mesmo em erro) — caller decide se
  os imprime.
- Módulo sem `content` (AST puramente executivo) produz
  `Ok(Vec::new())` — não é erro.

## Helpers privados de dispatch (Passos 140B + 141)

```rust
fn first_font_from_doc(doc: &PagedDocument) -> Option<FontList>;
fn resolve_font(
    font_list: &FontList,
    font_book: &FontBook,
    world:     &dyn World,
) -> Option<Vec<u8>>;
```

- `first_font_from_doc` (Passo 140B): itera `doc.pages → items`
  recursivamente (atravessa `FrameItem::Group`) e devolve o
  primeiro `TextStyle.font` com `Some(FontList)`. `None` se
  nenhum item tem font definida.
- `resolve_font` (Passo 141): itera `font_list.as_slice()` em
  ordem. Para cada família, consulta
  `font_book.select(name, &FontVariant::default())`; se devolve
  `Some(index)`, chama `world.font(index)`; se devolve
  `Some(font)`, devolve os bytes. **Primeira família a completar
  ambos os passos vence**. Se nenhuma completa, devolve `None`
  (pipeline cai em fallback Helvetica). Cenário patológico
  (índice stale: `select` devolve `Some` mas `world.font` devolve
  `None`) **continua** a tentar famílias seguintes — não
  curto-circuita. Paridade com vanilla: semântica
  "primeira-que-resolve" do `#set text(font: (...))`.
- Ambas as funções são privadas ao módulo e cobertas por testes
  unitários em `#[cfg(test)] mod tests`.

Array fallback materializado no Passo 141. Multi-font per
document (Passo 142, opcional) e variant-aware (ADR-0055bis,
candidata) permanecem fora do escopo actual.

## Integração com ADR-0045

As funções deste módulo **não formatam** diagnósticos. O caller
usa `diagnostic_format::format_diagnostic` /
`drain_diagnostics_to_stderr` para converter
`Vec<SourceDiagnostic>` em texto gcc/clang-compatível.

Separação alinhada com ADR-0043 (L1 data-only) e ADR-0045
(formatação em L3 — num módulo próprio).

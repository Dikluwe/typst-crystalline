# L0 — Semantic title carrier
Hash do Código: 62f1a7d9

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/title.rs`

## Contract


`TitleElem` retains public body: Content. `TitleElem::new(body)` preserves it;
plain text delegates to body. Maps transform body and preserve Content::Title.
`get_field("body")` returns Value::Content(body); unknown fields return None.
The entity performs no I/O, context queries or default selection. Language
construction, document.title fallback, presentation, introspection and HTML
are outside scope. Construction belongs to
`00_nucleo/prompts/compiler/stdlib/structural/title.md` and rendering to
`00_nucleo/prompts/compiler/layout/title.md`.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

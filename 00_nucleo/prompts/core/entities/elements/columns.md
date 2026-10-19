# Prompt L0 — `entities/elements/columns` — `ColumnsElem`
Hash do Código: 3801768a

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/columns.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical upstream `e0e8ca4d`, crates/typst-library/src/layout/columns.rs:
28–35 distinguishes page-level columns from wrapping content in a layout
container and identifies pagebreaks, footnotes, and line numbers as affected
observables. page.rs:233–236 specifies page columns with default one and
points to columns() for a container; columns.rs:6–13 describes equal-sized
columns filling sequentially to container or remaining page height.
Full literal quotations remain archived. This evidence supports the semantic
distinction, not an unmeasured exact footnote arrangement; see
`00_nucleo/prompts/core/compiler/layout/set_page.md` for its remaining inference.
Historical Content anchors are 1834,1665,2143,2020 for text/emptiness/maps/equality.

## Contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnsElem {
    pub count: usize,
    pub gutter: Option<Length>,
    pub body: Content,
    pub page_columns: bool,
}
```

Preserve Content::Columns(Arc<ColumnsElem>) and
Content::columns(count, gutter, body), with page_columns false. The flag is
true when wrap_page_columns transforms #set page(columns:N), false for
#columns(N)[...]. That transformation belongs to its owning consumer.
Plain text and emptiness delegate to body. Both maps recurse into body and
preserve count/gutter/page_columns. Field/kind/payload use defaults;
non-locatable. Derived equality compares all four fields.
Hash is manual through Debug because Length contains f64; the bool participates
automatically. Document the signed-zero caveat; the existing allowance assumes
this element is not a map key and must be revisited if it becomes one.
Verify body transparency, maps preserving all fields, flag distinction,
constructor default, structural equality and specified hash behavior.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/columns.md`. Historical observations are not a new parity certification.

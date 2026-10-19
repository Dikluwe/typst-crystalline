# L0 — `LayouterRuntimeState` (`entities/layouter_runtime_state.rs`)
Hash do Código: 808e5a19

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/layouter_runtime_state.rs`

## Responsibility

Own state populated during layout, not derived by Content pre-pass or the
walk/from_tags introspection pipeline. Page semantics belong here rather than
polluting CounterStateLegacy or mixing them into TagIntrospector. Historical
page-tracking and outline-freeze decisions remain in the archive.

## Data contract

```rust
pub struct LayouterRuntimeState {
    pub label_pages: HashMap<Label, usize>,
    pub known_page_numbers: HashMap<Label, usize>,
    pub is_readonly: bool,
    pub figure_page_numbers: Vec<usize>,
    pub table_page_numbers: Vec<usize>,
    pub known_figure_page_numbers: Vec<usize>,
    pub known_table_page_numbers: Vec<usize>,
}
```

label_pages records labels resolved during the current layout; references
writes it and finish exposes extracted_label_pages. known_page_numbers carries
the prior fixpoint iteration for outline reads; the coordinator updates it
between iterations. is_readonly brackets outline/TOC rendering.
Current figure/table vectors record counted page numbers in document order,
written by their respective layout owners; finish exposes the figure vector
as extracted_figure_page_numbers. The known figure/table vectors are injected
at iteration start and read only by list-of-figures/list-of-tables layout.

## Acceptance and deferred scope

Keep the seven declared fields, distinct current/previous state, document
order, read-only outline usage and finish propagation. The pure RAM carrier
has no I/O and is available to tests through Default::default().
The original optional lang field remains deferred pending walk-signature
work; a later historical description of pattern replication does not itself
authorize adding it. This owner does not implement the layout/fixpoint
algorithms that populate or consume its fields.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/layouter_runtime_state.md`. Historical observations are not a new parity certification.

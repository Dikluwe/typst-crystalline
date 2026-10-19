# Prompt L0 — `entities/elements/footnote_entry` — `FootnoteEntryElem`
Hash do Código: e87fdc04

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/footnote_entry.rs`

## Evidence and approved scope

Historical upstream `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
typst-library/src/model/footnote.rs:223–296 supplies the reference. Human
approval was recorded on 2026-09-21 after L0-only commit `3ac60a578`;
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1473.md`
preserves the separate implementation/test/integration history. The scope is
public construction without footnote realization; full measurements and
language construction belong to
`00_nucleo/prompts/core/compiler/stdlib/structural/footnote.md`.

## Carrier contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteEntryElem {
    pub note: Content,
    pub separator: Option<Content>,
    pub clearance: Option<Length>,
    pub gap: Option<Length>,
    pub indent: Option<Length>,
}
```

Preserve Content::FootnoteEntry(Arc<FootnoteEntryElem>). note carries the actual
footnote content without stronger validation than Content. The declared
Content::footnote_entry constructor accepts note, separator, clearance, gap,
indent with the field types/order above. Its old return annotation says
FootnoteEntryElem despite naming a Content helper; reconcile that annotation
with the separate construction owner rather than changing an API here.
The four optional fields preserve unresolved defaults: separator is a 30%-wide
LineElem with 0.05em stroke; clearance 1em; gap 0.5em; indent 1em. None means
use that default if/when realization is implemented, not materialize it here.

Text and emptiness delegate to note. Both maps recurse into note and also
into separator when Some, preserving the remaining fields. Derived PartialEq
compares fields. Hash is manual via Debug because Length contains f64, as in
the existing list/enum-item strategy; do not derive Hash merely to simplify.

## Acceptance and explicit non-goals

Keep local tests for all five fields, maps, text/emptiness, equality/hash,
and optional/default transport. The historical integration added five local
tests for simultaneous V2 verification; its green result is not a fresh run.
Do not implement realize (link/superscript through Counter::of(FootnoteElem)
or note.location()), ShowSet automatic leading/size, or modify footnote layout.
Reopening those phase boundaries requires its own measurement and ADR-0127 gate.
This carrier does not participate in actual footnote layout within this slice.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/footnote_entry.md`. Historical observations are not a new parity certification.

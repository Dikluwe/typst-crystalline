# Prompt L0 — `entities/elements/grid` — `GridElem`
Hash do Código: d032e282

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/grid.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1648`, `content.rs:1795`, `content.rs:1925`, `content.rs:2237`, `content.rs:2490`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GridElem {
    pub columns: Vec<TrackSizing>,
    pub rows:    Vec<TrackSizing>,
    pub cells:   Vec<Content>,
    pub gutter:  Option<Length>,
    pub align:   Option<Align2D>,
    pub inset:   Sides<Length>,
    pub header:  Option<Content>,
    pub footer:  Option<Content>,
    pub stroke:  Option<Stroke>,
    pub fill:    Option<Color>,
}
```

## Behavior and acceptance

Preserve `Content::Grid(Arc<GridElem>)` and `Content::grid(...)`.
Manual Hash uses Debug because TrackSizing, Length, Align2D, Sides<Length>,
Stroke, and Color contain f64; PartialEq compares all ten fields.
Plain text joins cell plain texts with one space. Emptiness checks only whether
the cells vector is empty. Both maps recurse through cells, header, and footer,
preserving columns, rows, gutter, align, inset, stroke, and fill; map_content
collects SourceResult. Header/footer may themselves be Content::GridHeader or
GridFooter, with ordinary transparent element dispatch. Field/kind/payload
retain defaults; non-locatable. Verify all fields, cell order/text joining,
vector-based emptiness, nested header/footer mapping, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/grid.md`. Historical observations are not a new parity certification.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Add the exact public metadata:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridChildOrder { Cell(usize), Header, Footer, HLine(usize), VLine(usize) }
```
GridElem additionally has pub source_order: Option<Vec<GridChildOrder>>. Existing hlines/vlines and all other fields remain. Some retains original accepted positional order; None is the explicit legacy synthetic cells/header/footer order, never reconstructed source order. Maps preserve descriptors and transform each actual child exactly once in their existing cells/header/footer order. Hash/equality include source_order. The feature-owned semantic resolver validates all descriptors before visiting children (complete indices exactly once, Header/Footer exactly iff present); malformed Some errors `invalid grid source order` at the Grid source span. The entity adds no rendering/phase/validation behavior and no duplicated Content snapshot.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

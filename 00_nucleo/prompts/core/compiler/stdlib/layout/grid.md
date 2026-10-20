# L0 — stdlib/layout/grid — grid e tracks
Hash do Código: 78463430

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/grid.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

Tracks preserve auto, length, fraction and integer expansion; grid validates named arguments, cells, gutter, fill, inset and stroke and constructs the same language content.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Exhaustive argument decisions and measured rejection

### Evidence before decision

At baseline `91ea6ee91105b97b1aedf172e34780dddde14c94`, the exact source/L0
hashes and UTC are retained in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/initial-audit-state.json`.
The baseline eval/query receipts in that directory bind the rebuilt product
SHA to the ratified oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
They measure acceptance, diagnostic meaning, supplied-value provenance and
geometry, not Rust representation or rendered bytes. The initial audit separates
source evidence from inferred intent and records refuters.

`layout/grid.rs:11-33,60-61` silently drops unsuccessful track projections;
`structural/table_grid.rs:71-72` shares the operation. Pinned
`layout/grid/mod.rs:455-463` casts each array member fallibly; baseline mixed
`(1pt, true, 2pt)` succeeds locally but errors upstream. Count 0/-1 also wrongly
falls through to omitted-track defaults. The upstream scalar versus array-member
expected-type diagnostics are distinct.

### Contract and limits

Keep the existing crate-private `parse_track_sizing` projection exhaustive over
Value: Float, Length absolute component, Fraction, Auto and the legacy string
`auto` retain their results. Other strings and all other present variants return
None explicitly; future variants require a new decision. Make the existing
crate-private `extract_tracks` fallible and propagate its diagnostics from both
Grid and Table. This is an internal helper ABI, not an external Rust contract;
the native public signatures, entities and phase do not change.

Omission and empty arrays preserve their current Auto defaults. Positive integer
counts expand in order; nonpositive counts report `number must be positive`.
A malformed scalar reports `expected auto, relative length, fraction, integer,
or array, found {type}`; a malformed array member reports `expected auto, relative
length, or fraction, found {type}`. Use vanilla type vocabulary, retaining legacy
Float point tracks and string `auto` even though upstream rejects these extensions.
Validate columns then rows at their existing read sites, before gutter/children.
Use the last matching named occurrence's value_span, then occurrence.span, then
Args.span; array-member errors retain that array value span because Args carries
no per-member provenance. Do not fabricate a narrower location.

Ratio/Relative omissions and Length.em truncation remain explicit RM1 carrier
debt. Preserve their measured existing output, including omission inside arrays,
without reclassifying valid dimensions as malformed. This bounded repair does
not certify dimensional track parity, cardinality for these unsupported forms,
or a new TrackSizing carrier. Every other failed projection must report an error
instead of being silently filtered. A newly added Value must force local review.

Tests cover malformed mixed arrays/scalars, nonpositive counts, supplied-value
spans, columns-before-rows precedence, preserved valid ordering/defaults, and
legacy extensions. Table runs the same shared boundary. RED precedes code; full
tests, paths and lineage follow. No lint suppression or wildcard evasion.

The array-member rejection decision inside the fallible extractor is also
nominal and exhaustive over Value. Ratio/Relative retain only the explicitly
inventoried dimensional omission. Every other variant reports its actual type
if the preceding projection failed; already-successful variants are precluded
by that local invariant but still named so future edits cannot create an open
rejection policy. Add a green-only future-Value E0004 witness at this new match
in addition to the selected parse_track_sizing witness.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

While doing existing Grid positional extraction, record Some(Vec<GridChildOrder>) in original accepted argument order. Every accepted cell/line descriptor has an in-bounds index; Header/Footer exactly once iff present. Recognize transparent Source wrappers for classification without discarding source/provenance or child. Preserve existing line auto row/column behavior, duplicate-header/footer restrictions, footer validation phase/errors, all dimensional parsing/paint/layout contracts. No duplicated source Content snapshot, silent metadata repair, or new invalid-layout acceptance.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

# L0 — decorations
Hash do Código: 4b7b4467

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/decorations.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/decorations.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The 12 SVG pairs in `bibliography-hanging-2026-10-18/deco-before.json.gz`, with corrected line/path primitive reading in `deco-inspection.json`, use the same binary/freeze as the initial 9026 suite and pinned bd5ebec oracle. In overline h10, second-line text is at x18 while the stroke is at x8; vanilla starts the stroke at x18. h0 already has y/thickness/tail differences, which this horizontal correction neither hides nor closes. Local source: `decorations.rs:43–45,76–78` uses global cursor/line_start_x; `line_commit.rs:615–619` and `sub_frame.rs:350–354` capture global start. This is language geometry, without an obligation to copy vanilla buffers. The negative preview may be less than the real cursor before any admission; therefore end > preview alone does not prove a segment.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the body under a temporary decoration collector and restore the outer collector. Preserve wrap segments and the final still-pending segment. Resolve offset and extent against the active font size; defaults are underline `0.10em`, strike `-0.25em`, overline `-0.80em`, thickness `max(0.05em, 0.4pt)`. Explicit stroke color takes precedence over active fill. Emit the existing butt-capped lines with matching segment geometry. Do not create a parallel line-commit or page algorithm.

Capture start_x_initial through the private cursor_start preview. After wraps, the last segment uses the effective left edge from `00_nucleo/prompts/core/compiler/layout/paragraph_indent.md`, not global line_start_x. Do not advance baseline/admission, alter offset/thickness/extent or rerun body; preserve nested collectors and a terminal segment without flush.

Expose only a private read-only applied query in the paragraph-indent owner. Here, when preview differs from the actual initial cursor and no segment has been flushed, the final segment requires an origin actually applied. Do not start a scope/admission to satisfy decoration or create paint/height from an empty body. This private parity correction follows continuous flow under ADR-0127, with no new phase/API.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Refutation cases include duplicated origin, premature compensation consumption, and changes to baseline, end, thickness, text or region outside the prescribed horizontal delta. Add RED tests for empty decoration after a break with negative h, including empty body, metadata and empty Par. This horizontal refinement certifies LTR only; complete decoration geometry and RTL remain unmeasured, without a universal claim.


## Planned source-child transport

The approved physical-page protocol is recorded in `lab/results/set-page-header-2026-10-21/review.md`; baseline source/owner evidence and actual semantic RED are preserved in `lab/results/set-page-header-implementation-2026-10-21/`. This consumer already dispatches the source child named below; ordinary layout_content intentionally isolates generated content and therefore cannot transport its source packet.

In a planned pass, descend by UnderlineBody, StrikeBody or OverlineBody selected by the actual decoration constructor. Preserve one actual source occurrence, its source authority and inherited deferred anchor. Known generated render output remains unplanned and cannot acquire source membership. Measurements and repeated visual copies do not execute or admit the source again. Preserve all existing geometry, wrappers, diagnostics, cache behavior, zero-render cases and unplanned entrypoints. No new public API/default or phase is authorized here.


## Measured nonexpanding Repeat alignment — 2026-10-22

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
retains exact source/font/binary/build inputs, UTC2026-10-22T12:02:34.973885+00:00
through12:02:51.345977+00:00, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227` and its dirty-file stat. Product
SHA-256 `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
and ratified oracle commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
are pinned. The eight-source manifest and full matrix are in the adjacent
`agents/repeat/` evidence. Nested fixed-child center/right and unequal-width
multiline center/right fail; decorated multiline also loses line paint.
Direct Shape, natural text and an explicit inner-left override are GREEN.
Repeat clears inherited alignment before composing its piece; pinned upstream
`typst-layout/src/repeat.rs:19,55–59` preserves lexical styles and separately
places copies. `shape.rs:251–283` emits flow geometry without line alignment,
refuting global translation. Nested-only inheritance does not fix the measured
multiline defect. Language geometry/paint are required; data structures and
iteration mechanics are free under ADR-0107. Classification and pre-edit hashes
are preserved in `agents/repeat/classification-before-L0.json`.

### Private contract

Transport the private source-line association from collected and final decoration segments to their actual emitted Line items in an active piece scope. Final-line segments use the pending actual line association; previous-line segments retain their recorded association. Do not guess by coordinates or label strings, move source effects, or change stroke metrics.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


## Approved font-backed underline metrics — 2026-10-22

### Evidence and public-contract gate

The fresh paired sources and exact source/font/binary/UTC/dirty-state provenance
are in `lab/results/remaining-parity-2026-10-22/underline-before-font-scoped/measurement.json.gz`
(SHA-256 `db81849f43eaa92212629aeaddce587ba5fff8465fd878a22890cd05e894a838`).
DejaVu at 11pt requires downward offset 0.21484375pt and thickness
0.4833984375pt; Nimbus at 11pt requires 1.386pt and 0.55pt. Both 17pt
witnesses preserve the actual table scaling. These measured differences
refute a universal fixed-font coefficient. Noto's numeric equality does not
waive the separately measured SVG open-line fill failure. Exact font-table
hashes and reads are retained in the sibling
`agents/inline/underline-font-table-measurement.json`; rendered pairs and
source language geometry govern, rather than a required Rust representation.

The human approved the exact additive contract and bounded integration in
`lab/results/remaining-parity-2026-10-22/agents/inline/underline-contract-proposal.md`,
SHA-256 `e7f3db142865646f13f9041f36f572582802cd302f1aebb7f8b42bb0610f2c6f`.
Approval is recorded in `lab/results/remaining-parity-2026-10-22/underline-approval.json`.
This satisfies ADR-0127 section 2.1 for exactly the method below; it supersedes
only this addition's earlier no-new-public-method restriction. No new entity,
FrameItem field, phase, dependency, mode or unrelated API is authorized.

### Bounded actual-run Underline integration

Only Underline adopts the approved method; Strike and Overline retain their
existing policy. After exactly one actual body dispatch, positively certify
a no-flush inline result: newly committed items must all be existing neutral
markers, a material exterior prefix must remain unflushed, and no unresolved
fraction/repeat queue may remain. Decorate only the exact newly appended
current-line slice. Recurse through its actual local Group/Link/Semantic/
TextReplacement envelopes; insert line paint beside the corresponding Text
so ancestor transforms transport both consistently. Respect actual Box
baseline and its frame region; preserve link target and annotation bounds.

Coalesce text runs only through represented separator advances at equal
typography/baseline; unrepresented H-space gaps are not underline ink. Use
actual run text, font size/style and injected underline metrics. Explicit
offset and color retain priority; extent resolves at that actual run size.
Default color remains the text's existing optional fill. Unknown/multiline
results retain the existing collector route and explicit limitations. Preserve
outer collector restoration and the Repeat owner's private line association
metadata; never fabricate a new source or paint marker for this correction.

Require a meaningful consumer RED with an injected non-default offset and
thickness, explicit signed offset/color/extent, adjacent text plus real
separator coalescing, separated fractional Box body runs with no line over
free space, and unchanged prefix/source events. Full immutable decorated-link
source must pass complete glyph/paint/annotation assertions at the unchanged
1e-8 bound. Preserve all existing multiline/Grid/Place/nested collection tests.


### Declared-size test precondition correction

The exact stage-2 receipt/log in
`lab/results/remaining-parity-2026-10-22/red2-core-underline/` records both
new consumer tests failing an earlier width assertion. Inspection of
`layout/mod.rs:954–976` establishes that Layouter::new intentionally ignores
the historical font-size argument and installs the default 11pt chain.
The tests expected their declared 12pt input without installing it, so these
failures do not count as consumer semantic RED. Install the actual typed
12pt style/chain explicitly in this test fixture before dispatch. Preserve
every expected width, metric, offset, extent, color and tolerance unchanged;
rerun the corrected tests before changing the consumer implementation.


## Uniform actual multiline Underline metrics and initial baseline

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
and the exact `agents/repeat/refuters/decorated-multiline-center.typ` record
first stroke y11.1 versus native18.572265625 and second34.96484375 versus
34.0796875, with old thickness0.55 versus actual0.483398438. The existing
collector overwrites its first settled text baseline with the initial
geometric top. These are actual paint coordinates and remain separate from
Repeat's causal horizontal line alignment.

Keep the same collector and private piece-line metadata. If no material
exterior prefix existed, and the exact newly emitted flat ASCII text runs
across committed/pending output all have one actual TextStyle, resolve
Underline metrics with the approved provider and that actual style. Unknown,
mixed, transformed/opaque or nested-painted output retains its prior route.
When initial baseline was pending, preserve the collector's actual settled
first baseline rather than overwriting it with the former geometric top;
ordinary already-installed midline baseline handling remains. Never replay
body, infer source identity from paint or alter line positions/height/callbacks.

Require an injected-metric multiline RED: WIDE, explicit linebreak, X, with
actual12pt style. Each stroke follows its corresponding actual text baseline
by the injected offset, has the exact injected thickness and word advance,
and retains both lines. The existing complete measured multiline source must
retain its full paint criteria; no fill or geometry waiver. Strike/Overline
remain unchanged. This is within the approved Underline method integration
and continuous-flow parity scope, without any further public/phase change.

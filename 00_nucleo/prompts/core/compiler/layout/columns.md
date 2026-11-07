# L0 — columns
Hash do Código: 6f8bfb70

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/column-note-regions.toml sha256:ce40781bf12cee5ae49390769448b1ca10968af4eaa5a109399374beab99e619
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

## Ownership and evidence

Layer: L1. Sole productive consumer: `01_core/src/compiler/layout/columns.rs`.
ADRs: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.
Current source evidence precedes the contracts: layout/geometry at `:310`,
segment splitting/direction at `:192` and `:244`, sibling regions at `:407`,
continuous flow at `:638`, marginal patterns at `:36`, and physical column
operations at `:715`. The producer/transform boundary is
`01_core/src/entities/content/page_structure.rs:181`, not this consumer.

This owner specifies the column layout entrypoint and local geometry. Segmented sibling regions, continuous flow and physical transitions are owned by `00_nucleo/prompts/core/compiler/layout/columns/segmented.md`, `00_nucleo/prompts/core/compiler/layout/columns/flow.md` and `00_nucleo/prompts/core/compiler/layout/columns/transitions.md` respectively.
ColumnsElem and page_columns are existing input data owned by
`00_nucleo/prompts/core/entities/elements/columns.md`; page-body transformation
belongs to `00_nucleo/prompts/core/entities/content/page_structure.md`.
Cursor decides when to advance; `00_nucleo/prompts/core/compiler/layout/columns/transitions.md` owns close_current_column, merge_column_items, finish_columns, start_next_column and start_column, with descendant visibility for layout coordination.
Footnote body fitting/overflow is owned exclusively by
`00_nucleo/prompts/core/compiler/layout/footnote_flush.md`.

## Additional supporting evidence

Historical scope-tracking evidence is in `lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` and `summary.json`: 30 sources/90 PDFs, with commands, sources and exact state in `initial.json` and `before-pdf/provenance.json`, against vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. Repeated equal scopes accumulate approximately 0.4pt and nested scopes 0.8pt; positive wrap changes the line. `text_tracking.rs:50` compares only resolved TextStyle. Vanilla `inline/collect.rs:287-295` preserves segments from distinct chains; `inline/shaping.rs:1269-1295` adds tracking only within each run. This measures observable text boundaries without requiring PartialEq or the chain representation to be copied. Hide/Box are also boundaries: `hide.rs:21-24` and `boxed.rs:157-208` replace/restore buffers without preserving that distinction.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

paint-root-2026-10-14/layout-first.log records tests looking for Text only in page.items and missing descendants of a new Block paint root. before-extra-test-owners preserves source/L0/HEAD/UTC/diffstat before adjustment. Group.paint_root was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; only test observation changes here.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

## Geometry and dispatch

Use the existing page margins as sides, not an obsolete scalar assumption:

```text
usable_width = page_width - margins.horizontal()
count = max(input_count, 1)
gutter = explicit_length.resolve_pt(text_size)
         or page_width * COLUMNS_DEFAULT_GUTTER_RATIO
column_width = (usable_width - (count - 1) * gutter) / count
column_region_width = column_width + margins.horizontal()
column_x[i] = margins.left + i * (column_width + gutter)
dx = column_x[i] - margins.left
```

Start each local working region at the left margin. Its mini-page width
includes the margins so the inline right bound reaches the full usable column
width. Translation applies the horizontal delta once; do not narrow text again
or edit layout_word/layout_chunk to compensate.

Split explicit Colbreaks at the inspected body Sequence, looking through one
outer Styled and reapplying that same Styles once to each segment.
Pad missing segments with Empty; fold excess segments into the final one
without dropping content. A detected separator selects segmented layout;
otherwise continuous flow uses the body and the existing column/page transitions.

Direction discovery descends through Sequence/Styled, preferring an inner
direction over its outer wrapper and ignoring wrappers without visible body.
The first relevant child direction wins; absent body direction falls back to
the active chain's text.dir. Only Value::Dir participates.
RTL reverses physical offsets, not item order inside a column; LTR/absence
retain their order. Page sections must arrive with the producer's own
Pagebreak segmentation; this module does not acquire ownership of that transform.

## Vertical boundaries and sibling regions

Participate in the existing auto-Block margin protocol. Flush an open inline
line before establishing pending par.spacing from the previous baseline;
defer initial top-to-baseline conversion until real content arrives.
Collapse adjacent entry/bottom margins by maximum rather than addition.
At exit retain par.spacing for the next block/paragraph.
At a region top do not add a strong margin. Respect active par.spacing,
par.leading and text size rather than historical measured coordinates.

Each explicit segment starts at column_start_y, without adding ascender twice,
and receives the same entry snapshot of logical baseline and collapse state.
A sibling's resulting state never initializes the next sibling.
Only regions with material items contribute the deepest output
baseline-plus-descent; cursor output uses the greatest consumed bottom.
An empty or HTML-carrier-only segment cannot invent a material baseline;
retain its carriers for extraction and preserve behavior when real items exist.
SpaceFromNewline is non-visible like Space without normalizing its origin.

Pass each segment to the dedicated cursor-relative fractional Sequence region.
Fixed content and fr weights are local to that segment; use its current
page_bottom_limit. The single outer style transport does not license a recursive
rewrite of nested Styled content or the use of page_columns as an overflow
sentinel to open fractional layout in arbitrary sublayouts/footnotes.

## Footnote coordination

Numbering comes from the shared introspector, never a saved/restored local
footnote_counter. Segmented page columns (page_columns true) flush pending
notes at the end of the referencing column and translate them with its items.
Segmented explicit containers (false) accumulate their notes and flush once
below the container's deepest consumed content at the first logical column
offset. That offset need not mean the physical left edge in RTL.
Preserve the exterior pending-note buffer when entering/leaving sibling regions.

The received continuous flow flushed per physical column through
close_current_column for both forms. The measured 2026-11-01 correction below
supersedes that policy for actual note-bearing continuous flow: standalone
columns retain page-owned notes until the physical page closes, while page
columns retain per-column note ownership. Non-note segmented behavior remains
under its existing contract. The body flush accepts optional bottom_y; its fitting policy,
warnings and remainder handling are child-owned by the overflow L0.

## Marginal line-number consolidation

Complete exterior pending width fixups before replacing the exterior item
buffer. Drain each column's own fixups before extraction/translation, both in
segmented mode and close_current_column; never resolve a sibling's indexes
against another buffer.

Segmented page columns carry a private index/count for marginal numbering only;
it does not activate fitting/overflow transitions. The last physical column
forces the end margin. Explicit containers instead retain the entry
counter/page snapshot across siblings, suppress the already emitted ordinal
prefix and keep maximum ordinal progress rather than summing siblings.

For string patterns in segmented explicit containers, defer realization and
consolidate siblings jointly in stable ascending baseline order. Accept a
candidate only when its baseline is not below previous_bottom.
Use number height at least 1 pt and advance only accepted candidates according
to the existing formatter/scope handling. Capture common outer TextStyle for
the number; marginal properties remain those of the occurrence's chain.
Candidate buffers participate in layout rollback and are drained on exit.

For page columns, consolidate each physical column separately before translation:
reset its vertical suppression boundary, not the document counter/scope.
The first available candidate captures the shared immutable TextStyle for that
composition. In continuous close_current_column, if the first physical column
closes without a candidate, capture active TextStyle once before translating
and advancing; a later candidate must not replace it.
Explicit containers keep joint sibling consolidation instead.
Restore private contexts/buffers at exit without additional callbacks.

## Additional current state and observation boundaries

In each segmented sibling-column composition, save tracking State with the exterior line and restore them together. Sibling text provenance must not contaminate exterior prefix; all other states/counters/widths remain.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Only the affected test observer traverses the frame tree and projects coordinates by Group.pos/matrix through the layout test helper. Preserve content, expected values and tolerances; no productive algorithm changes. `00_nucleo/prompts/core/compiler/layout/tests.md` owns that helper; test-sibling visibility is not product API. Revalidate tests and package renders.

Publish allocated Columns usable causal width, never global width inferred from paint in a single column.

## Acceptance and evidence limits

Verify full usable width, asymmetric margins, segmented/continuous transitions,
LTR/RTL offsets, independent page-section direction, both segmented footnote
policies, unchanged note numbering, and warning transport via the overflow owner.
Verify auto-Block boundaries, sibling logical origins, empty/material output
state, independent fr weights and HTML carrier transport.
Check region-local width fixups, shared marginal style, stable baseline
suppression, last-column margin and counter scope/progress.

These are language/layout observables, not fixed page totals for a particular
font or proof from PDF bytes. No global formatting, new public API or new
default is authorized. Historical scope-outs are not evidence of current
absence. Nested wrappers/columns, multi-region fractional cases, heterogeneous
callbacks, local par.line divergence and RTL marginal-numbering parity remain
outside demonstrated coverage; do not claim them complete or universally
unsupported. Producer support for mid-document page columns and page gutter
must be audited separately, as recorded in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.

Tracking correction follows continuous-flow ADR-0127, with RED→GREEN, bilateral text/position/line controls and preserved uniform tracking; error-free build/lint and restricted resealing. All other open limits remain.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Continuous planned flow uses the original ColumnsBody source child, while unplanned legacy callers retain their current constructed segment. Explicit colbreak segmentation records original child indices in the same construction pass that clones/pads/merges segments. Generated padding has no source packet. Preserve the original Source/at-most-one-Styled/Sequence spine and emit its source Root/PostChildren phases once around the sibling-region composition. Each actual segment child routes through its original source packet address, not through the index of a generated Sequence. Generated style wrappers keep existing geometry and do not duplicate source facts. The sequence owner supplies an optional exact per-part cursor route without changing grouping, fraction, direction, spacing or strategy.

Retain parallel footnote cursor queues together with the existing body queues at sibling-region entry/exit. Existing packet geometry carriers carry deferred child events to their accepted physical page in the same final projection pass. Do not infer source identity from copies, values, hashes, pointers or Locations, reconstruct global semantics from paint, activate new callback phases or alter any existing column/footnote geometry policy.

## Borrow the existing segmented-content slice directly

Measurement before correction: `lab/results/set-page-report-blockers-2026-10-21/agents/domain/before.json.gz` retains the
exact source/L0 bytes, UTC, HEAD and dirty-file inventory. The retained
`columns.rs:196` already borrows `split.segments`; line 227 takes an additional
reference at the `layout_segmented` call, and warnings-denied Clippy reports
`needless_borrow`. This is reference coercion mechanics, not a language change.

Pass the existing borrowed segments directly to their owner. Preserve segment
order, original source routes, geometry, sibling-region state and note queues.
No signature, allocation policy, public contract or phase changes. This internal
ADR-0127 continuous-flow correction requires existing segmented columns and
physical-page route tests plus warnings-denied Clippy.


## R8 — measured column note geometry

Evidence before decision: `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/before/receipt.json`
measures Beta atx156 instead of155.2 and container notes immediately under the
columns rather than in the page note area. Pinned library/layout/columns.rs:73
uses4% of the available region; current columns.rs:170 applies4% to full page
width. At300pt with20pt side margins the correct gutter is10.4pt, not12pt.
These are physical language coordinates, not traversal mechanics. ADR-0127
continuous parity correction applies, with unchanged public signatures and modes.
Resolve the default gutter against usable_width, preserving explicit length
resolution. Segmented explicit-container notes remain deferred to the containing
page's bottom note area at full outer width; page-columns retain per-column notes.
This supersedes the old below-container policy only for this measured behavior.

Tests first: default and explicit gutter, container/page columns, note text/position and exterior queue preservation; bilateral0.001pt and full validation.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Pass the existing source e.page_columns boolean to the private continuous layout_flow function. Preserve segmented dispatch, original source plan and container behavior.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.

## Ratified A/B direction defaults — 2026-11-01

### Measurement and approval before code

Source anchors and runtime counterexamples are preserved in
`lab/results/construction/diagnostics/issue-8990-residuals-2026-10-31/`
before receipts, pinned-documentation.json and gate-default-direction-proposal.md.
Current source/owner bytes, exact concurrent diffstat and UTC are fixed in
`lab/results/construction/diagnostics/issue-8990-ab-2026-11-01/before-state.json`.
The same causal paths still read only explicit text.dir or discard Auto; the
operator delegate does not establish a local mathematical base. Those are
language defaults/semantic bidi inputs; font IDs/IR/byte equality are mechanics.
The human explicitly approved A and B on 2026-11-01, preserved in approval.json,
for proposal SHA-256 a2a9737c7960cb400b2899ac60c3b02e04f0823f6a9cc388ceee385663f214ca.
ADR-0127 default gate is satisfied for this bounded package only.

### Exclusive responsibility

Approved A supersedes Only Value::Dir participates for automatic lexical resolution. Use body-direction discovery under the active chain and otherwise the paragraph-owned resolver. Preserve first relevant visible child and inner-style precedence, region order and all other column geometry.

### Acceptance and limits

Require tests first and functional RED before implementation, then GREEN;
measure he/ar/en/absent language, explicit LTR/RTL, Auto reset, inherited
language and scope restoration, geometry/paint and existing RTL #8990 controls.
Keep the six prior physical RTL-Hebrew punctuation negatives separate; this
package does not ratify a new fallback-font policy or general RTL mathematics.
Revalidate core/math/shaper tests, RAM-only build, lint and paired lineage,
with exact source/UTC/diffstat provenance and restricted resealing after a
valid V15/V26 graph. Preserve all concurrent changes and every other owner
responsibility; no commit/push, new public API, Nucleus or phase is authorized.


## Footnote column and lexical-numbering correction — 2026-11-01

### Measurement before decision

`lab/results/construction/diagnostics/footnote-columns-numbering-2026-11-01/{before.json.gz,baseline-build.json.gz,baseline-tekt.json.gz,baseline-vanilla.json.gz,general-before.json.gz}` binds HEAD3c6b0ce7, uncommitted tree/exact diffstat and source/L0 bytes, UTC, argv and executable SHA. Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17, binary22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34. Global/alias/local set is ignored at the call (H1/ref1 instead of Ha/refa); query numbering field is missing. Short standalone columns already match; long explicit columns use three product pages versus two oracle pages, and Box notes occupy the wrong column while main rows ignore note area. Existing pending queues and page_column_notes distinguish ownership, but refresh refuses columns, flow selects fixed segments on any colbreak and final flow guesses page bottom. Native physical text/coordinates and lexical values govern acceptance; buffer/match shape is mechanics. No intent is inferred from source inspection alone.

### Exclusive obligation

Detect selected Footnote content or existing queued notes. Route that note-bearing body, including explicit colbreak, through the existing continuous ColumnsBody source route and physical column allocator. Do not use a generated split segment as the original body. Preserve existing segmented strategy and source-route/fraction/line-number behavior for non-note content. This supersedes only the old unconditional segmented choice for note-bearing content.

This is internal convergence/restoration of existing language behavior under ADR-0127 continuous flow: no public entity/trait/signature, user mode/default, compatibility break or eval/layout phase change. Preserve the constructor's raw morphology and all #8935 identity, measurement and header replay corrections. Nucleus obligations are shared only through these full pins; each source retains its canonical sole owner. Require minimal real RED before production code, GREEN, prior14 #8935, footnote/subframe/column suites, bilingual source controls, full PDF extraction and SVG paint checks. Use only the task's exclusive /dev/shm target, one compilation job, 12GiB process limit, preserve concurrent work and do not commit/push. Global combined qualification remains separate. Any new public carrier or phase need stops at a concrete proposal for human approval.

Clippy refinement measured in clippy-libs.json.gz: the added eighth flow argument exceeds the existing seven-argument limit. Carry the same private note-flow intent through the already owned optional flow-end accumulator: the Columns caller installs/restores it, and flow derives the intent from presence. Remove the extra boolean argument without changing behavior or suppressing the warning.

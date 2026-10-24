# L0 — core/compiler/layout/footnote_flush
Hash do Código: 0bc82b46

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/link-frame-scopes.toml sha256:91cb1c097fc694c138acf2d044a3fc46dc7c41ec608ac557f086e1dc41ec0f57
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


## Ownership and evidence

Layer: L1. Sole productive consumer:
`01_core/src/compiler/layout/footnote_flush.rs` (lineage header at `:2`).
The flush body at `:40`, safe boundary at `:91`, greedy fitting at `:128`
and placement at `:188` implement the current behavior.
`01_core/src/compiler/layout/mod.rs:2187` forwards warning strings;
`01_core/src/entities/layout_types.rs:989` declares the existing document
warning carrier; `03_infra/src/pipeline/paged.rs:439` converts it to diagnostics.
Those files are evidence of integration, not additional consumers owned here.

Column placement/translation is specified in
`00_nucleo/prompts/core/compiler/layout/columns.md`. Layouter state, document fields and
pipeline warning conversion retain their own L0s; this prompt does not authorize
new fields or changes to those modules.

## Additional supporting evidence

Source→PDF RED probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and inspection of this consumer's FrameItem handlers precede textual-separator changes. Public-contract approval in `00_nucleo/prompts/core/entities/layout_types.md` was received on 2026-09-28 and recorded in materialization provenance.

Bilateral probes/exact state in `lab/results/construction/diagnostics/identidade-hifen-discricionario-2026-09-28/initial.json` and before/ measure U+002D versus SHY with coincident geometry in eight controls. This consumer's FrameItem traversal is archived in approved-before/. The owner confirmed `00_nucleo/prompts/core/entities/layout_types.md`.

The ratified bidi-carrier inventory `lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json` identifies this unit's references and 1:1 owners; manifest `1ae7513e` was confirmed.

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}` distinguishes regular GIDs from Oblique selection. The human confirmed public TextShaped.font: Option<Font> in `00_nucleo/prompts/core/entities/layout_types.md` (SHA `f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`). This round's snapshot/inventory are in `lab/results/construction/diagnostics/identidade-fonte-shaped-implementacao-2026-10-05/`.

`lab/results/construction/diagnostics/links-paginacao-2026-10-14.md` and receipts under `posicoes-compostas-2026-10-14/automatic-boundary-probes` record source, HEAD/dirty tree/UTC, binaries and vanilla pin. On a 160×40pt page with 10pt margins, ordinary text crosses the break; Link URL, Ref and Ref Page abort with a stale index after buffers are emptied. `references.rs:270` appears in the stack; `link.rs:22-32` uses the same mechanism. `outline.rs:28-50` also saves indices before the body. Clamping an index proves neither prefix/order/fragment/destination preservation nor geometry. Earlier snapshot: `lab/results/construction/diagnostics/links-captura-2026-10-14/before.json.gz`. Content, destination, regions and bounds are observables; private boundaries are mechanics.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [293, 300]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

## Flush and bounded overflow

Preserve flush_pending_footnote_bodies(bottom_y: Option<f64>).
It consumes pending numbered bodies, measures their marker plus body in a local
sub-frame of the current usable page/column width, and emits fitting bodies in
order. bottom_y optionally supplies the container's footnote-area bottom;
otherwise use the region height minus bottom margin.

Compute top_safe as the maximum of top margin, current cursor_y, the current
positioned-item Y values and the open line's item Y values, using the existing
FrameItem projection (including Semantic children).
available_h is max(area_bot - top_safe, 0).
This measured projection is not a general bounding-box/ink-extent proof.

Fit greedily by cumulative measured height. At the first non-fitting body,
defer it and subsequent bodies in order for the caller's next flush.
Force the first body only when its own measured height exceeds the full usable
page/column height, not merely the remaining current space.
This defensive escape prevents non-progress on a body that cannot fit anywhere.

Place accepted bodies top-down from
max(area_bot - accumulated_height, top_safe), compensating the sub-frame's
ascender in physical item translation. Supply the actual left origin to the
sub-frame and do not add it again as a full horizontal translation.
Retain decoration and orphan alignment-fixup rebasing, distinguishing logical
origins from ascender-adjusted physical baselines. Preserve causal numbering,
remainder order and pending-note coordination by the caller.

## Warning and purity contract

On forced emission, append the existing pure string, substituting the number:

```text
footnote body [N] exceeds available column/page height and may be truncated or overlap content
```

L1 writes no Sink, file, environment or clock. Existing Layouter warnings move
to PagedDocument.layout_warnings; the paged pipeline drains them into its
returned warnings vector as SourceDiagnostic::warning with detached span.
This consumer owns warning production, not the carrier declaration or conversion.

The user must be able to recover the note or be warned of placement failure;
mere presence of bytes in a PDF is not success. A fitting/deferred note must
not silently disappear. The forced oversized fallback explicitly permits
truncation/overlap with a warning: do not promise impossible no-overlap or
vanilla-style note splitting/continuation for that case.

## Current transport and lexical isolation

Transport TextSeparator position, consumed advance, style and origin without new paint/baseline/advance; preserve child order and exclude it from safe vertical bound.

TextReplacement's safe bound consults visual children; translation preserves text and shifts children by placement delta without new advance. The shared nucleus supplies invariants only, never extra ownership.

Ratified carrier migration uses None for uncertified constructors and `..` in read patterns, preserving language observables. Transformations obey the pinned origin nucleus without broadening phase/API.

Migrate only this owner's shaped-face references: legacy constructors use None, transformations preserve font, and shaped comparison/join discriminates face/axes. The shaper owner's emission provides effective-font clone also on cache hit, without changing semantics/style to induce selection. Collectors/subset/CMap/PDF bind the same original face with style coordinates; PNG/SVG prepare local bindings and instantiate once per face/request before render, keeping original resource index distinct from instantiated bytes. None retains legacy resolution; existing public API/FontKey remain. No artificial fonts, unsafe, globals or new phase. Private functions may receive explicit font/bindings.

Shared shaped-face-identity invariants govern these cross-owner boundaries: associate original face bytes plus axis request rather than name/slot. Export-local tables preserve supplied resource order and append missing faces; multifont distributes items once by binding before per-font collection, never sharing global GIDs/CMap across faces. This owner remains 1:1 and materializes only its own references; the shared nucleus owns common claims.

Deferred note emission must not inherit the flushing main-flow link. Suspend ambient scopes and close/reopen their committed segment around emission, retaining explicit links inside the note. Do not change note splitting/overflow policy.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Group reconstruction transports paint_root unchanged, never true→false during projection/marker removal/rebase.

## Acceptance and boundaries

Verify empty queues, notes that fit, deferral when only remaining space is too
small, inclusion of unflushed line/cursor state in top_safe, oversized-body
termination and exact warning production, optional bottom_y, and single-origin
translation with nested placement/alignment fixups.
Integration checks cover preserved warning forwarding and existing small-note
column placement. No new source edits are implied by these checks.

The old quarantined-vanilla observation did not establish that current vanilla
lacks page(columns:). Its incomplete provenance is historical, not an exception
to the active parity target or a new default. Relevant history and outstanding
evidence limits are in
`lab/results/construction/diagnostics/sanitization-introspection-columns-history.md`.

Preserve observable text/geometry, adjacency controls and pinned invariants. TextReplacement acceptance covers text/boxes, causal tests, recursive transport and literal-hyphen controls, never PDF bytes or new selection.

Require RED→GREEN for breaks before/within body; preserve prefix/suffix, regions, destinations/bounds, rich body/nesting/columns, deferred content and context/rollback, with no private boundaries in output. Revalidate no-break controls, suite, build and lineage.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

## Source-owned physical page child transport

### Measurement and approved scope

At the implementation baseline, `footnote_flush.rs:45–48, :126–169 and :188–222` selects the
actual body/subframe or deferred-body operation. Exact source/L0 texts, full
SHA-256, UTC, HEAD and working-tree state before this amendment are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/child-adapters/before.json`.
The approved review SHA-256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
and human `approval.json` under the implementation evidence ratify this finite
physical-page source-routing phase. Existing geometry is preservation, not a new
claim of upstream intent, global page parity or completed historical proposals.

### Exclusive routing contract

Retain each queued FootnoteBody cursor by its existing queue index, including aligned remainder cursors when a note defers. The generated numeric prefix remains unplanned; render it then the exact source body under its retained cursor through the bounded generated-prefix subframe adapter, preserving the original combined content for geometry classification. A nonfitting tentative frame rolls back its transcript/request/consumed state and retains the cursor for later physical placement. Accepted frame packet markers follow the same existing decoration/item translation and physical-page projection. Do not turn combined generated Sequence into the source body plan or discard genuine no-paint operations.

Use private parent-descendant protocol helpers with explicit typed source edges;
no new public API, entity field/variant, generic mapper, callback execution,
source re-evaluation, result cache or layout-from-Frame semantic reconstruction.
Retain legacy behavior when the planned cursor is absent. Actual source packets,
source parentage and physical placement remain distinct; measurement cannot
publish actions, consume transcripts or leak tentative diagnostics. Preserve
existing current styles, original/visual identity, per-store position sealing and
all stated geometry/effect/error limits. In particular, deferred footnote/float
causality versus later physical placement requires source-bound language controls;
this routing amendment does not declare that universal paint order is semantics.

Require actual nested source Context/Figure/label/counter cases, repeated pages,
no-paint children, wrong-edge/foreign cursor rejection, measurement inertness,
retained/deferred/rollback and existing geometry/baseline/clip/float/note tests.
No old assertion, fixture, diagnostic or explicit pending limitation is weakened.
L0-first source-current RED→GREEN, scoped lineage/graph/seals and full established
gates remain mandatory before completion.


## Deferred-note fresh physical region availability

### Measurement before correction

The exact original source `note-child-on-later-page` and pinned-oracle page text
are preserved under `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/`.
`lab/results/set-page-deferred-corrections-2026-10-21/fit-3/receipt.json` and
`fit-3/tests.log` bind executable/source state, UTC and the actual pure observation:
body height193.75pt, full useful physical height200pt, fresh pages2/3 remaining
height192.3pt because top_safe67.7pt includes a prepared unused text baseline.
Both fresh pages have no material items and no causal line. The body consequently
fits neither the remaining area nor the oversized escape, and its unchanged
remainder survives the terminal drain. Required source coverage correctly fails.
This is an internal geometry defect, not authority to weaken that guard.

### Exclusive correction and boundaries

On a positively certified unused physical region, top_safe is its actual useful
top margin rather than its prepared text baseline. Certify all of: ordinary
physical page (no subframe/column mode), no causal items or line, no explicit
flow-placement witness, no previous line advance, and cursor exactly equal to
the top margin plus its retained assumed ascent (or the unfixed top when absent).
Do not infer emptiness merely from missing paint: literal-space lines and explicit
VSpace remain occupied causal flow. Do not change force-emission thresholds,
warning text, body geometry, source anchors or pending ownership.

This amendment removes only the impossible fresh-area gap. It does not certify
the separately observed vanilla note continuation/reservation behavior: the
original oracle places prefix note text on page1 and the later Figure on page2.
Preserve that pending obligation explicitly until its actual source-bound
fragmentation is implemented and validated.


## Source-owned note continuation correction

Measurement: `lab/results/set-page-deferred-corrections-2026-10-21/fit-3/receipt.json`, `fit-3/tests.log`, and unchanged pinned-oracle page text under `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/` precede this amendment. Exact ownership and source-only records are in `agents/layout/` under the correction root.

The captured frame path consumes each source body once and retains certified geometric continuations without replaying source, callbacks or semantic effects. Fit the complete current frame if it fits; otherwise choose a certified prefix within actual remaining height and retain the rebased exact tail. On a split, defer subsequent notes in order. If no certified cut applies, preserve whole-body deferral and the original oversized warning/termination policy. Current-pin original page text requires prefix START/NOTE TEXT on page1 and NOTE Figure plus SECOND on page2. Source phases keep callsite semantic anchors while final marker projection supplies physical pages. Never discard a frame, marker, decoration or orphan alignment range. This supersedes whole-body-only deferral solely within the certified private fragment envelope.


## Unsplit private frame recovery

The Clippy RED at `candidate-2/validation-remainder/clippy.log:125–132` under
`lab/results/set-page-deferred-corrections-2026-10-21/` identifies an oversized
private split-helper error carrier. Exact before state is archived in
`lab/results/set-page-report-blockers-2026-10-21/agents/layout/before.json.gz`.
The fragment owner may box only the unchanged unsplit error frame. Recover that
frame by moving it out of the Box, then retain the same oversized-warning,
deferral, capture and rollback branches. No public signature, accepted geometry,
warning, source authority or existing regression criterion changes.


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

Preserve decoration metadata through the existing deferred-note geometric translation; completed independent frame tokens are resolved before crossing scopes. Do not alter note fitting or source admission.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


## R1 — retain the formatted entry marker (2026-10-23)

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/baseline.json` and `before/receipt.json` bind exact HEAD, uncommitted changed-file list, source archive, UTC, executable and fixed oracle identities. Successful note and note-successor probes expose generated `[1]` instead of `1`; custom exposes decimal bracket footers despite correctly formatted inline `a` and `(ii)`. Source `footnote.rs:45–48` formats the call; `footnote_fragments.rs:286` and `footnote_flush.rs:156` independently synthesize brackets. Oracle `typst-library/src/model/footnote.rs:305–338` realizes the original note numbering as superscript in its entry. This is visible language morphology; internal queue representation is mechanics.

Classification: ADR-0127 continuous parity correction, private layout metadata only, no public contract, default mode, pipeline phase or compatibility change.

Exclusive obligation: Consume the stored entry prefix in both captured and legacy/column flush paths. Retain it with its body through every remainder/overflow transfer. Do not reinterpret numbering or strip user brackets. Preserve existing diagnostic strings including bracketed diagnostic numbers.

Acceptance: tests first must demonstrate actual marker failure before implementation and pass afterward. Cover default, custom and intentionally bracketed numbering, literal body brackets, captured and uncaptured emission, and checkpoint preservation. Reuse generated Content rather than export text replacement. Preserve the existing body styles and note-area model, source admissions, diagnostic behavior and unrelated tests. Full build, tests, lineage and linter gates are required. This does not certify broader entry typography, column placement, nested-note body completeness, call/body migration or oversized-note continuation; measured prior limits remain open.


## R8 — note entry composition and continuation

Measurement before decision: `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/before/receipt.json`
records the exact dirty source state and ten current-pin controls. Entry text is
11pt instead of9.35pt, marker x20 instead of29.35; nested loses INNER, split loses
END, and container notes are placed directly under columns. The fixed oracle's
model/footnote.rs and layout/rules.rs resolve entry text at85%, leading0.5em,
indent1em and marker gap0.05em. Queue representation is mechanics; complete text,
source effects, geometry and physical page membership are language observables.
The approved metrics API is bound by the adjacent approval.json; all changes here
are private parity corrections under ADR-0127 continuous flow. This supersedes
only the corresponding older unresolved behavior clauses, not acceptance bounds.
Place normalized captured frames relative to their top rather than subtracting
an unrelated current main-text ascender. Use0.5em gap between entries and the
resolved note separator/clearance in fitting and placement. Uncaptured legacy
fallback uses the same capture geometry policy, with transactional rollback kept.
Do not discard nested bodies queued while draining: preserve their body/plan/frame
alignment and emit after their containing entry. Split only at certified boundaries
and retain every tail item on subsequent physical pages. Preserve oversized warnings
for actually uncertified oversized bodies, source effects once and rollback.

Require module RED→GREEN, original ten paired controls at0.001pt, nested/split/refused/source-once/rollback tests and full workspace/lineage validation.

R8 reference refinement: use the private captured outer_size for clearance1em,
per-entry gap0.5em and separator thickness0.05em; entry size is85% of that
reference. Nested entries retain the root reference. Frame head and tail keep
that exact outer reference. This is private geometry metadata, not a source ID.


R8 reentrant-drain evidence: candidate4/receipt.json and svg-candidate4/receipt.json
move END back onto page1 after terminal clearance is corrected. finish.rs drains
once, then next_region drains the same region again before commit. The newly
normalized short tail now fits below its own head on that repeated drain, which
is not a new page. Add only private DeferredFootnoteFrame.deferred_region recording
(pages.len(), current_column, segmented_par_line_column) for a split tail. The
flush owner marks the actual region at acceptance of the head and refuses that
tail and following queued notes while the same region is still active. A genuine
page/column sibling permits the tail; checkpoint/split transport preserves the
stamp. Initialize unmeasured/root captures with None. This stamp is physical
progress metadata, never source identity or a new pipeline phase/public contract.
The mod owner stores it; fitting remains exclusively in footnote_flush. Require
END on exactly the next region, no duplicate/lost body, reentry/rollback and
page-column controls plus all final gates.


## R9 — long notes with explicit line breaks (2026-10-24)

Measurement precedes this correction: long-footnotes-r9-2026-10-24/baseline.json and before/receipt.json under lab/results/construction/diagnostics/ bind HEAD, exact dirty stat, UTC,2162 inputs and current R8/pinned oracle binaries. h80-n5 places1/2/2 body lines versus0/2/3; both-zero instead requires1/3/1; orphan-zero requires1/2/2. h80-n6 through n9 lose late words because finish.rs:75 bounds drain by queued note count. following-flow incorrectly reserves a non-admitted first fragment and sends TAIL to another page. Original bytes and failed controls remain immutable.
Pinned flow/collect.rs:196–239 resolves strictly positive widow/orphan costs, protects first two/last two and whole-three, ignoring empty neighboring lines. flow/compose.rs:650–730 budgets the first entry against current flow+separator+gap, while757–777 inserts already laid out future-region spill. These source structures are not mandated; physical words/pages, source effects and geometry are language observables.
Classification: private vanilla-parity/correctness correction, ADR-0127 continuous flow. No public field, trait, signature, mode/default, eval/layout phase or compatibility break. Preserve prior gates, callbacks once, static exhaustive dispatch, source anchors and pure injected metrics; no arbitrary painted-frame semantic inference or hard-coded font/page fixtures.

### Exclusive owning obligation

Own current/future admission and physical emission. First entries retain current available minus real outer clearance/gap. Only source-line-certified continuations use full future body capacity; on a causally empty fresh region their pre-separator clearance may extend above the usable top, as in the pinned spill observation, while body/separator placement stays bottom anchored. Main-flow safety and ordinary/uncertified/spacing-frame clamps are unchanged. If the first protected group cannot fit an empty initial entry region but can progress in a future body region, retain the same captured payload, emit its empty initial separator/gap once and mark continuation+actual deferred_region; do not synthesize/replay source or discard the marker. Ordinary non-fitting initial notes defer without reserving main-flow area. Preserve queue order, source/plan/frame alignment and same-region reentry barrier.
Provide a private terminal-force path preserving existing ordinary flush signature/callers. It is used only after repeated actual non-progress on fresh regions and emits retained content with the existing oversized-note warning; it must not bypass a same-region tail barrier or silently lose payload. No new public warning carrier/string, pipeline/default, or blanket opaque splitting.

Acceptance: source-backed module RED→GREEN before implementation, independent positive/zero/negative cost refuters, complete long-note words/markers, no-progress warning escape, actual PDF/SVG physical geometry at unchanged0.001pt, R8 and A–H/R1–R7 preservation, resource recipe, complete workspace/Clippy/fmt/Rustdoc/lint and both lineage gates. Preserve failed attempts and all received unrelated changes.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Own one private shared pending-work drain used by finish and by active page-column flow. Whole-document draining accepts all pending bodies; finite page-column draining stops when the first queued frame is uncertified, preserving legacy opaque paths. Retain the actual progress rank and warning escape. On a fresh active page-column region, if an initial certified protected group cannot fit the initial entry budget, defer through remaining columns without emitting repeated empty entries; only the last column emits the one empty initial separator/gap and transitions to continuation. Actual continued lines use the future body budget as already specified.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.


## R9 — measured following-prose occupancy and frame origin

Evidence before decision: candidate4-flow/receipt.json under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 records four real-output failures with full inputs/UTC/binaries: h80-n5-following-lines interleaves TAIL1–8 before the deferred entry, and h100-n8/h120-n20 lose note capacity to a prepared following baseline. All words survive, but physical page membership differs. svg-candidate4/receipt.json passes66/68; Nimbus and Liberation separators differ by0.10285/0.360668945pt while every PDF word coordinate passes. native-overhang-evidence.json refutes glyph-overhang inference: those exact shifts equal (sTypoAscender−sCapHeight)*9.35/upem for the actual faces. sub_frame.rs:327–340 installs an ascender, line_commit.rs:547–590 reconciles the actual text edge, sub_frame.rs:490–515 already normalizes height, and footnote_fragments subtracts the same origin from line witnesses again. parbreak.rs:55–110 records real flow end independently of weak spacing. This is geometry/source admission, not a new language mode or eval/layout phase. ADR-0127 private continuous-flow correction applies.

Exclusive owning obligation: Add a private early-placement mode for the source-before-dispatch following-prose admission. Use the existing real flow_placement_y for certified-note current occupancy when there is no material current line; weak next-baseline/parbreak preparation is not painted main flow, while explicit VSpace remains in that real flow witness. Early placement preserves the real occupied note area in footnote_reserved_height (clamped to full usable area), rather than discarding it on flush. A one-time empty initial fragment occupies its entire logical region. A tail already deferred in the same actual region cannot be early-flushed again or erase its prior occupied reservation. Normal close/finish flushing keeps the existing clearance reset.

Require new module RED→GREEN for following prose and one-origin capture, all72 actual PDF/SVG controls at unchanged0.001pt, original R8/refusal/effect-once and A–H/R1–R7 preservation, both build profiles, resources and full workspace/lineage validation.


## R9 refinement — manual singleton drain and zero-paint carriers

Evidence before decision: candidate5 source/commands and receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve six GREEN domain tests,68/68 SVG, two of four following-prose controls and a failed original R8 numbering SVG gate. The broad origin adjustment is refuted for one-line entries: sub_frame.rs:409–515 manually drains its final line without the earlier line_commit reconciliation. candidate5-flow confirms remaining prepared-marker occupancy at100/120pt. These are internal corrections under ADR-0127 continuous flow, with no API/default/phase changes.

Exclusive owning obligation: Exclude existing context/position/link marker-only trees from the material item-top maximum, using their existing owner predicate. Preserve those actual carriers and their coordinates; only their zero-paint footprint is excluded. candidate5-flow still defers h100 initial entry and cuts h120 after three rather than five because the completed Parbreak source carrier at a weak prepared baseline re-enters top_safe through the item maximum, despite the real flow-end correction. Add a direct actual-note marker-neutrality RED→GREEN, not a source-ID reconstruction.

R9 source-end refinement: candidate6-flow retains two RED controls although all original R8 gates and68 SVG controls pass. place.rs:116–119 deliberately filters flow_placement_y by equality to a prepared cursor. A subsequent weak baseline may invalidate that getter while the recorded material end remains the valid certified-note witness. For a certified note with no material current line, use the raw private flow_placement_end payload (which explicit VSpace updates) and the material item maximum; do not count the weak prepared cursor. Preserve the getter and all other owners unchanged. This is an internal formula correction, with the existing two real-output tests RED before this change.

R9 captured-queue correction: gdb-prose5.json/log observes the finite preflight executing but NOTE CAPACITY still uses the prepared49.91484375pt rather than the committed carrier. footnote_flush.rs:104–111 has already moved pending_footnote_frames into local frames before the later certificate predicate. Read that actual local captured queue, not the now-empty pending field, when choosing the real flow-end witness. No further algorithm, public contract or phase changes; preserve all failed controls and run72-case PDF/SVG plus full gates.


## R9 — preserve deferred stacking after an intermediate paragraph

Evidence before decision: candidate8-queued/receipt.json and candidate8-queued-output.log under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 retain three RED font controls: Alpha#footnote[FIRST], then MIDDLE, then Beta#footnote[SECOND]. The eager branch paints FIRST before MIDDLE, then a new queue/reservation forgets that prior placement and puts SECOND at the same footer baseline. This is a regression against the received deferred collector, not an allowed rendering change. The earlier72-case cohort passing does not certify this boundary. The correction stays private within layout under ADR-0127 continuous flow; no public/default-mode/phase changes. It supersedes the early-placement obligation above.

Exclusive owning obligation: Remove the early-placement/reserve_placed path and its one-off occupied-area state. Keep normal flush resetting its reservation as received. Own a lean private advance_footnote_region(force) coordinating the existing-warning force flush only on an empty region after caller-observed non-progress, then the genuine existing new_page transition. Normal fitting-note stacking, separator count, queued order and later note admission remain deferred until closing the region. Keep local captured-frame flow-end selection, zero-marker footprint correction and source tails unchanged.

Require the new stacked-entry module RED→GREEN and all75 PDF/SVG controls at unchanged0.001pt, all original R8 and A–H/R1–R7 controls, full workspace/lint/lineage/resource gates. Keep failed eager-placement receipts immutable.

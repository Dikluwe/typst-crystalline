# L0 — footnote
Hash do Código: 3d4bfcc6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/footnote.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/footnote.rs:31`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/footnote-composicao-2026-09-30/results.json` fixes historical vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, crystalline binary, HEAD and probe's uncommitted tree. In defer.typ, vanilla places `Alpha¹ omega.` and the note on the next page; crystalline leaves the call on the prior page and defers only body. `footnote.rs:47-51` emits call/queues body; `footnote_flush.rs:128-169` decides fitting too late to move the source line. Causality is inferred from these points and would be refuted by a path already moving the full line before flush.

The footnote.rs prototype was refuted by source→PDF defer.typ: the prepared paragraph has already committed the line and the local queue cannot move it.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Read the current causal footnote counter from the injected introspector, with the existing fallback of one. Format the supplied pattern or default `1`, falling back to the decimal value, and dispatch a superscript marker without square brackets. Queue the numbered body for deferred emission. Footnote-body drain and region progress remain in `00_nucleo/prompts/core/compiler/layout/cursor.md`; terminal coordination remains in `00_nucleo/prompts/core/compiler/layout/mod.md`.

The call-and-first-body admission proposal remains pending and is not current behavior. It requires future coordination with `00_nucleo/prompts/core/compiler/layout/line_commit.md` and `00_nucleo/prompts/core/compiler/layout/cursor.md`; this text does not authorize changing this consumer as though the correction were complete.

Proposed future behavior: before emitting a call, when the complete note fits a fresh region but not the current region together with the carrier line, move the entire active line to the next region. Preserve items, surrounding text, horizontal advance, styles and causal number; do not move only the superscript. Keep the note pending for ordinary flush in the call's region. Do not migrate without regional progress or if the note exceeds the complete useful height; oversized-note continuation requires separate materialization. This would be a private parity correction under ADR-0127 continuous flow, with no new public field/signature/default/phase/compatibility.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Future proposal acceptance, not certification of completion: defer places call/body on the same page as vanilla; fit remains one page; carrier-line text stays ordered; a note exceeding the entire region does not loop through pages. Prove through text/pages/morphology, never PDF bytes.

## Source-owned physical page child transport

### Measurement and approved scope

At the implementation baseline, `footnote.rs:46–51` selects the
actual body/subframe or deferred-body operation. Exact source/L0 texts, full
SHA-256, UTC, HEAD and working-tree state before this amendment are archived in
`lab/results/set-page-header-implementation-2026-10-21/agents/realization/child-adapters/before.json`.
The approved review SHA-256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
and human `approval.json` under the implementation evidence ratify this finite
physical-page source-routing phase. Existing geometry is preservation, not a new
claim of upstream intent, global page parity or completed historical proposals.

### Exclusive routing contract

The superscript marker is generated paint and stays unplanned. Capture the exact FootnoteBody child cursor at the source call and retain it in the owning private pending-note protocol alongside the unchanged numbered body tuple. Do not match a later body by equality/hash/text, bind the marker as a source child or admit a deferred body merely when queued. The pending-note owner and checkpoints retain queue/cursor alignment through every rollback and column transfer.

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

### Deferred source anchor is independent of physical Location.page

Measurement before decision: the current pinned oracle receipts oracle-extended
and oracle-crosspage under the approved physical-page implementation are audited
in agents/realization/footnote-causal-review.json. Three successful compile/query
controls bind exact source and binary identities. Same-page note order is Header,
FIRST, NOTE, SECOND. In both two-page controls NOTE has physical Location.page2
and counter3, preceding Header2 counter4 and SECOND counter5. Sorting those facts
by their relocated physical page would contradict the observed language values.
This does not establish universal paint ordering or settle historical call/body
regional progress.

At Footnote queue capture, pass the exact FootnoteBody cursor through the private
defer_planned_cursor helper before storing it. This reserves the current logical
frontier and semantic anchor page, inheriting any enclosing deferred anchor. It
executes and admits no child at queue time. Subsequent accepted body traversal
uses that retained causal anchor, while the existing position markers independently
record actual page/point. Queue/remainder/checkpoint transfers preserve the same
cursor; nonfit rollback must not retain tentative admissions. No generated
superscript becomes a source packet and no Location page is used as counter order.


## Source-owned note continuation correction

Measurement: `lab/results/set-page-deferred-corrections-2026-10-21/fit-3/receipt.json`, `fit-3/tests.log`, and unchanged pinned-oracle page text under `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/` precede this amendment. Exact ownership and source-only records are in `agents/layout/` under the correction root.

At an ordinary planned physical-page call, capture the actual numbered body exactly once into the private DeferredFootnoteFrame with authenticated source phases/markers and certified fragment boundaries. Store it alongside the original body and retained causal cursor; nested/column/legacy calls retain their existing whole-body path. Reserve its current note area before subsequent main flow, without moving the actual call or reconstructing semantic facts from paint. Queue/checkpoint transfers preserve the frame, cuts and reserve. The previous blanket no-admission-at-queue clause is superseded only for this source-owned actual deferred-frame capture, analogous to existing Place capture. These obligations follow the original source/paint RED and current-pin page text documented in footnote_fragments.md; historical whole-body/call migration proposals do not override these measurements.


## R1 — retain the formatted entry marker (2026-10-23)

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/baseline.json` and `before/receipt.json` bind exact HEAD, uncommitted changed-file list, source archive, UTC, executable and fixed oracle identities. Successful note and note-successor probes expose generated `[1]` instead of `1`; custom exposes decimal bracket footers despite correctly formatted inline `a` and `(ii)`. Source `footnote.rs:45–48` formats the call; `footnote_fragments.rs:286` and `footnote_flush.rs:156` independently synthesize brackets. Oracle `typst-library/src/model/footnote.rs:305–338` realizes the original note numbering as superscript in its entry. This is visible language morphology; internal queue representation is mechanics.

Classification: ADR-0127 continuous parity correction, private layout metadata only, no public contract, default mode, pipeline phase or compatibility change.

Exclusive obligation: Generate the entry prefix from the same already formatted superscript marker as the inline call, followed by the existing word separator. Queue that exact Content with the original body and diagnostic number. Pass it to actual planned capture without broadening capture eligibility.

Acceptance: tests first must demonstrate actual marker failure before implementation and pass afterward. Cover default, custom and intentionally bracketed numbering, literal body brackets, captured and uncaptured emission, and checkpoint preservation. Reuse generated Content rather than export text replacement. Preserve the existing body styles and note-area model, source admissions, diagnostic behavior and unrelated tests. Full build, tests, lineage and linter gates are required. This does not certify broader entry typography, column placement, nested-note body completeness, call/body migration or oversized-note continuation; measured prior limits remain open.


### R1 test exhaustivity and guard gate

Measurement before decision: `lab/results/footnote-markers-r1-2026-10-23/full-lint-complete.log` records V16 for the new test collector and V17 for the column test guard after all functional gates and replays passed. This amendment owns a test-form correction, not a runtime layout change. Enumerate every ignored FrameItem leaf explicitly in the test text collector; no silent wildcard may hide future domain variants. Revalidate final lineage, full lint and the established suite without suppressions or weakened observations.


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
Compose the generated entry prefix with explicit horizontal indent1em and gap
0.05em in entry size. Capture each original body exactly once with its original
planned cursor, including legacy and nested calls, before deferred flush loses
its lexical style. Keep queue/body/plan/frame alignment across recursive capture.
Nested entry text uses the root note entry size, not another85% reduction.
Keep the source marker and formatted numbering, counter and literal brackets.

Require module RED→GREEN, original ten paired controls at0.001pt, nested/split/refused/source-once/rollback tests and full workspace/lineage validation.

R8 reference refinement: use the private captured outer_size for clearance1em,
per-entry gap0.5em and separator thickness0.05em; entry size is85% of that
reference. Nested entries retain the root reference. Frame head and tail keep
that exact outer reference. This is private geometry metadata, not a source ID.


## R8 source-before-paint carrier admission

Measurement before decision: R8 candidate2/deferred.typ keeps Alpha/call on page1
and moves only LINE/TAIL to page2, whereas the fixed oracle retains an empty page1
from explicit45pt spacing and puts the complete carrier and entry on page2. The
existing footnote-local proposal was already refuted: admission after paragraph
paint is too late. A preflight at the original sibling frontier can measure before
that source child is dispatched. Geometry/page membership is language; queue and
preflight mechanics need not mirror vanilla. This remains layout, with no public
contract/default-mode/compatibility change (ADR-0127 continuous).
Own a private positive preflight for a finite single-line text carrier followed
by its first footnote. Inspect original borrowed Source/Located wrappers and
causal Location for numbering; do not identify source by equality/paint. Require
finite ordinary page geometry, no subframe/columns/active bidi, no earlier pending
notes, no floats, no open carrier line, no tracking, and a provably fitting inline
prefix. Refuse opaque/ambient-dependent/effectful note bodies. Certify only plain
text, spaces, explicit linebreak and their transparent source/location/sequence
wrappers. Measure the entry in an isolated injected layouter with no source cursor,
using the same actual prefix/font/85%-size/leading geometry. It must publish no
source admission or side effect. Move to a fresh physical page only when the
complete entry plus carrier and separator/gap fits fresh capacity but not current
capacity. Otherwise leave ordinary dispatch unchanged; oversized entries use
certified continuation. Preserve the once-only actual body route and callback/
counter/Location semantics. This finite certificate does not certify arbitrary
opaque or multiline carrier migration.

Require actual deferred-carrier RED→GREEN, fits/oversized/refused/effect-once controls, all ten0.001pt PDF/SVG cases and full validation.

R8 Empty admission refuter: Empty is source-transparent and must not acquire the
Space width in the finite carrier certificate. Use an actual injected-metric
near-width-boundary carrier with Empty before its note and verify admission before
dispatch, alongside the original full-width source control. This is the existing
zero-content invariant, not permission to certify an opaque effectful sibling.

The source-bound red-empty.log confirms pages0 versus required1 for the valid
near-width certificate. Treat Empty as zero width, keeping Space/Newline spacing
and every refusal guard unchanged. This is a private formula correction.

## R8 test iterator lint
Measurement: clippy-first-pipeline6.log reports double_ended_iterator_last in the new entry-position test. Select the same final matching marker row with reverse find rather than exhausting a forward filter. Preserve the identical marker, geometry assertions and runtime implementation; rerun Clippy without suppression.


## R9 — source page-column admission and drain

Evidence before decision: candidate2 and candidate3 receipts under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 preserve exact inputs, dirty stat, UTC and binaries. The first passes40/41; the wider68-case matrix refutes retaining context only at segmented exit: page-columns follows columns/flow.rs:119–126, which closes active columns before terminal draining. cols2-h80-n2 also measures Alpha atx20 versus155.2pt; cols2-h100-n3 unnecessarily defers a fitting whole-three entry. Pinned flow/compose.rs:650–777 measures admission at the source carrier and retains future column regions. These physical positions/pages are language, private state/dispatch is mechanics. This supersedes the refuted column follow-up saved in l0-column-followup.json, whose source changes were withdrawn without changing received code.

Classification: private parity/correctness, ADR-0127 continuous flow; no public field/signature/default/phase/compatibility change.

Exclusive owning obligation: Extend the finite source-before-dispatch carrier certificate to active page-column note scope, keeping all other refusal guards. Measure once in an isolated effect-free injected probe. For certified source lines use the actual first protected group height (or first actual line) as admission need; ordinary page admission keeps its previous whole-entry rule. If the carrier+first group cannot fit current column, advance genuine existing regions until it fits or reaches the last column when it cannot fit any fresh column; do not advance beyond that last column merely to repeat an impossible request. A fitting-fresh group may advance across a physical page from a partially occupied last column. Preserve numbering/source identity and actual body evaluation once.

Require actual carrier-column RED→GREEN before implementation, the wider68-case PDF/SVG contract at unchanged0.001pt, original R8 and A–H/R1–R7 controls, unchanged effects, resources, full workspace validation, both lineage gates and preservation audits.


## R9 — measured following-prose occupancy and frame origin

Evidence before decision: candidate4-flow/receipt.json under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 records four real-output failures with full inputs/UTC/binaries: h80-n5-following-lines interleaves TAIL1–8 before the deferred entry, and h100-n8/h120-n20 lose note capacity to a prepared following baseline. All words survive, but physical page membership differs. svg-candidate4/receipt.json passes66/68; Nimbus and Liberation separators differ by0.10285/0.360668945pt while every PDF word coordinate passes. native-overhang-evidence.json refutes glyph-overhang inference: those exact shifts equal (sTypoAscender−sCapHeight)*9.35/upem for the actual faces. sub_frame.rs:327–340 installs an ascender, line_commit.rs:547–590 reconciles the actual text edge, sub_frame.rs:490–515 already normalizes height, and footnote_fragments subtracts the same origin from line witnesses again. parbreak.rs:55–110 records real flow end independently of weak spacing. This is geometry/source admission, not a new language mode or eval/layout phase. ADR-0127 private continuous-flow correction applies.

Exclusive owning obligation: Own finite source-before-dispatch admission for an ordinary following paragraph with a certified first pending note. Borrow its complete Parbreak-delimited source siblings; require plain Source/Located/Sequence text, Empty, spaces and explicit linebreaks, each nonempty line fitting injected actual width; reject literal controls, local styles/opaque/effectful children, columns/subframes/cells/floats/active transactions, tracking/math/scripts/RTL. Derive strictly-positive widow/orphan first-group need and actual ordinary edges/leading. Use actual baseline admission and the feature early-placement mode, advancing genuine regions until that first group fits alongside the already placed note reservation; do not dispatch source children or repeat body evaluation. Preserve full-region continuation/empty-entry occupancy and reuse the warning/progress escape. A lone following line may stay on the call page when it fits.

Require new module RED→GREEN for following prose and one-origin capture, all72 actual PDF/SVG controls at unchanged0.001pt, original R8/refusal/effect-once and A–H/R1–R7 preservation, both build profiles, resources and full workspace/lineage validation.

The following-prose preflight is further finite: require ASCII source text, nonnegative finite leading and positive finite ordinary edges. Non-ASCII/fallback/script ambiguity retains the existing path; it is not certified by the four new controls.

R9 V16 gate: full-lint-v16-red.log identifies the new finite certificate wildcard false branch in this owner. Enumerate every currently refused Content variant explicitly, preserving the exact finite acceptance set and normal dispatch fallback. Do not suppress V16 or hide rejection behind a helper. Future enum additions must require a deliberate certificate classification. This is internal information-preservation hygiene under ADR-0127, with identical behavior for existing variants; full lint must become GREEN.


## R9 — preserve deferred stacking after an intermediate paragraph

Evidence before decision: candidate8-queued/receipt.json and candidate8-queued-output.log under lab/results/construction/diagnostics/long-footnotes-r9-2026-10-24 retain three RED font controls: Alpha#footnote[FIRST], then MIDDLE, then Beta#footnote[SECOND]. The eager branch paints FIRST before MIDDLE, then a new queue/reservation forgets that prior placement and puts SECOND at the same footer baseline. This is a regression against the received deferred collector, not an allowed rendering change. The earlier72-case cohort passing does not certify this boundary. The correction stays private within layout under ADR-0127 continuous flow; no public/default-mode/phase changes. It supersedes the early-placement obligation above.

Exclusive owning obligation: The queued-note preservation refuter demonstrates that early painting is too broad. Following-prose preflight must perform baseline/group admission and genuine region transitions only, leaving fitting entries queued so later notes are stacked by the existing close-region collector. Do not flush a fitting whole entry in anticipation of prose. If first group does not fit the reserved region, advance normally; retain pending-work rank, and request the existing warning escape only after repeated no-progress in a genuinely empty region. Preserve a fitting final tail alongside main prose through its existing reservation, rather than painting it and forgetting its area.

Require the new stacked-entry module RED→GREEN and all75 PDF/SVG controls at unchanged0.001pt, all original R8 and A–H/R1–R7 controls, full workspace/lint/lineage/resource gates. Keep failed eager-placement receipts immutable.

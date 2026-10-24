# L0 — repeat
Hash do Código: d03bc3db

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/repeat.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/repeat.rs:17`, `01_core/src/compiler/layout/repeat.rs:27`, `01_core/src/compiler/layout/repeat.rs:58`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Measured Repeat correction — 2026-10-22

The frozen baseline is `lab/results/nested-fractions-repeat-2026-10-21/repeat-before/measurement.json.gz`
(SHA-256 `e6d3c085c28062ad22fdb3b1575108d3a778842a50bf0e5c7fa8381b445e88e0`),
UTC `2026-10-22T01:15:47.337768+00:00`–`2026-10-22T01:16:29.201630+00:00`, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227`. It preserves exact code/L0 and dirty-file state,
all source/font/driver/binary hashes, every command and complete stderr. Product
`94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`
was compared with ratified vanilla commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
The forty rows have unchanged measured inputs. `original-before/` in that result
root preserves the earlier nested-fraction/repeat source witnesses.

Observed language differences precede this decision: naked prefix/repeat/suffix
is one local inline line but three oracle flow units; multiline body has one
local U/V versus seven complete oracle pieces; zero-width body paints local X
but no oracle X; unbounded width succeeds locally but oracle emits exactly
`repeat with no size restrictions`; negative gap is rejected locally and accepted
by the oracle. The oversized nominal 100pt Box in a 40pt region still paints one
oracle piece, not zero: do not confuse nominal child paint width with prepared
frame width. `count-zero-oracle/receipt.json` and
`agents/repeat/count-zero-qualification.json` preserve a separate successful
finite negative-gap witness with ten surrounding BEFORE/TAIL glyph uses and
no X copy; source hash `7a5dcc19834795506bbffe3549ef1d6d063c3f4cbdacb8004c13cb0b4ae50d85`.

Pinned library `layout/repeat.rs:4–12` documents available-space fill and the
unbounded error. Layout `rules.rs:743–745` realizes Repeat as a flow Block;
`flow/collect.rs:240–288` supplies actual region and ordinary weak paragraph
margins. Layout `repeat.rs:19–30` composes one nonexpanding piece and returns
full-region width with actual piece height/baseline; `:32–65` defines count,
gap, alignment and visual cloning. `flow/distribute.rs:651` bounds returned
nonexpanding frame size by its region even when paint overflows. Language
content/geometry/errors are the requirement; no upstream global, layout-pass
mechanics, or data-structure identity is required. A fresh differing oracle
result would refute the corresponding bounded inference; no global parity or
performance claim follows.

## Ownership and current contract

Own Repeat's complete prepared piece, arithmetic, local alignment and cloned
visual/decorations. Source body is dispatched exactly once by the existing
planned `RepeatBody` edge through a geometric local subframe at the **actual
containing region width**. Preserve complete committed and pending body lines,
wrappers, first baseline, positions and actual height. Use prepared logical
piece width, bounded by the finite region, rather than overflowing inner paint
width. No source body or callbacks execute to make each copy.

Repeat is a full-width flow frame, not inline spacing to its next sibling.
Use the private `block::admit_prepared_frame`/`commit_prepared_frame` helpers
owned by `00_nucleo/prompts/core/compiler/layout/block.md` for the existing
causal admission, ordinary weak paragraph margin collapse, atomic region
progress, final origin and flow end. Preserve the genuine RepeatBody source
authority; never invent a source Block or dispatch it as BlockBody. Before body
composition, an already-proven private fractional Box width can define its
local region; no unproved sibling estimate or source re-evaluation is allowed.
The new prepared-frame helpers and assigned-width scope are internal to the
existing layout phase, not a new public API or evaluation/realization phase.

For finite full region width F, positive prepared piece width w and resolved
Length gap g, use N=floor((F+g)/(w+g)), bounded only by the existing maximum of
1000 rendered copies after the source arithmetic/cast. Do not force N to at
least one. If piece width is zero, emit no visual copies. Preserve frame
height and full logical width even when no copy is emitted. Invalid/unbounded
result size is an actual layout error with message `repeat with no size restrictions`
and current source span, transported through the existing layout error channel;
do not silently succeed or substitute finite coordinates.

Resolve gap against actual styles, including accepted negative Length gaps.
For justify, add source remainder/(N-1) to gap; for N=1 or justify=false,
apply the inherited horizontal alignment to the remainder. Avoid nonfinite
placement of an actual rendered copy; no output is made from invalid/empty
iteration ranges. Clone the complete original prepared piece, preserving
order/transform/paint and source-neutral visual copies. Copy local decoration
segments to the same actual origins, using the helper's returned final origin
after possible region progress. Apply known local alignment fixups against
settled piece dimensions, not parent indices. No empirical offsets, per-body
shape tests or fixture special cases.

The old PendingRepeat/expand_pending range API remains temporarily available
for existing line-close clients and its prior guards/count policy; do not remove
it or rewrite its unrelated caller contract silently. Actual Repeat source
layout uses the new prepared full-width flow path. Private state/hub storage
remains owned by `00_nucleo/prompts/core/compiler/layout/mod.md` and rollback
by its current owners. No entity field, public signature, default justify,
flag, World capability, compatibility or pipeline-phase change is authorized.

## Acceptance and limits

L0 precedes meaningful RED on unchanged Repeat source, then implementation and
exact bilateral revalidation. Add synthetic-metric tests whose text advance,
ascent/descent and paragraph spacing are independent declared inputs: full
region/flow boundaries, positive gap/justify, right alignment, multiline full
piece, zero width, finite count zero/negative gap, and the exact unbounded
error. Add the native negative-gap acceptance test under its own owner before
removing only that measured rejection. Preserve all other constructor errors,
input validation, default justify and entity maps/Hash contracts.

The immutable twenty-source corpus, count-zero witness, original nested cases
and source-effect controls must preserve every piece/suffix/position, source
counter/label occurrence and observable diagnostics at existing precision.
Keep the known 1000 cap, one actual planned source admission, neutral render
copies, complete decorations and atomic rollback. Page count or matching an
outer separator alone cannot close Repeat parity. No warning suppression,
new ignore, threshold increase, expected-output fitting or contract weakening.
Build/full tests, Hash, rustdoc, strict Clippy/fmt, full unsuppressed Tekt and
both lineage directions remain root-managed validation gates.

## Planned source-child transport

The approved physical-page protocol is recorded in `lab/results/set-page-header-2026-10-21/review.md`; baseline source/owner evidence and actual semantic RED are preserved in `lab/results/set-page-header-implementation-2026-10-21/`. This consumer already dispatches the source child named below; ordinary layout_content intentionally isolates generated content and therefore cannot transport its source packet.

In a planned pass, descend by RepeatBody. Preserve one actual source occurrence, its source authority and inherited deferred anchor. Known generated render output remains unplanned and cannot acquire source membership. Measurements and repeated visual copies do not execute or admit the source again. Preserve all existing geometry, wrappers, diagnostics, cache behavior, zero-render cases and unplanned entrypoints. No new public API/default or phase is authorized here.


## Measured zero-copy physical membership — 2026-10-22

### Evidence before decision

`lab/results/nested-fractions-repeat-2026-10-21/zero-source-oracle/receipt.json`
preserves UTC `2026-10-22T01:32:01.324993+00:00`–`2026-10-22T01:32:01.800881+00:00`,
uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, its exact dirty
changed-file list, source/font/command hashes and ratified oracle identity.
Oracle binary SHA-256 is
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`,
commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`. The immutable zero-copy
source hash is `8435c4215fa7e04160234b290328d012545f73bf8ac39da7f190ca42f313c3b2`.
Both PDF and SVG compilation succeed without warnings. The PDF hash is
`d8cb79e4426cff98701e65a0081c6649c65d467164631a3e0725b8a236c87d89`;
its retained extraction is `BEFORE TAIL STATE0 COUNTER0 LABELS0`. Positive-copy
controls retain STATE1/COUNTER1/LABELS1 in the independently archived baseline
source-effects measurement. Thus composition/evaluation once cannot be used
to infer final physical body membership when the copy count is zero. This
refutes preserving a physical source carrier solely because preparation ran.

### Scoped private integration and acceptance

Prepare the actual planned RepeatBody exactly once so its size, errors and
render frame remain causal. Preserve that child's private cursor before the
prepared dispatch. After successful preparation/counting, when no visual
copy is emitted, call the existing-layout private interface
`discard_empty_repeat_body(child: Option<&Cursor>)` on Layouter. The protocol
owner records only the exact already-prepared child contribution as discarded
from final physical membership. Missing or invalid required source admission
remains an error; this is not blanket source-complete success, source replay,
callback re-evaluation, marker substitution or a new public capability.
Positive copies retain one source membership and source-position authority;
subsequent render copies remain neutral.

Require zero-copy STATE0/COUNTER0/LABELS0 and positive-copy1/1/1 at the same
language-level query surfaces, preserving existing strict admission, causal
coverage, diagnostics and all unrelated source contributions. The private
optional-cursor helper and discarded-contribution transport remain within the
existing layout phase with unchanged public entities/signatures/defaults.
Implementation of this call follows the separate genuine protocol RED; this
L0 supplement does not declare that gate already implemented or passed.


## Prepared baseline independent of visual copies — 2026-10-22

### Evidence before decision

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/zero-source-after/measurement.json.gz`
(SHA-256 `c845b0915e3263d647c5034f2302f1baeee52d7613485580178103b1fd730f8c`)
retains exact source/font/HEAD/dirty-state/command/build provenance, unchanged
measured inputs, and complete PDF/SVG trees. Its product zero-copy row runs
UTC `2026-10-22T01:45:02.661709+00:00`–`2026-10-22T01:45:03.998401+00:00`;
its oracle row runs `2026-10-22T01:45:04.021705+00:00`–`2026-10-22T01:45:04.322210+00:00`.
Product SHA-256 is `380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`;
oracle commit/tree/binary remain the pinned identities above. The first BEFORE
glyph's retained global SVG origin is product `(8,24)` versus oracle
`(8,16.357421875)`. This measured displacement is evidence of lost geometry,
never a formula/offset to reproduce. Final zero-copy membership0/0/0 remains
its own independent contract; restoring a baseline must not restore source ink
or body membership.

Pinned `typst-layout/src/repeat.rs:19–29` prepares one source piece and transfers
its actual baseline before the rendered-copy loop, including count0.
`inline/box.rs:32–47,79–89` preserves the body frame baseline through resizing
and applies only explicit alignment/shift. `flow/distribute.rs:689–699` takes
the first in-flow frame's baseline decision, including explicit absence; later
paint cannot replace that first decision. Exact pinned source hashes were
rechecked unchanged in `agents/repeat/zero-baseline-audit/source-revalidated.json`.
Local Box's paint-only first_baseline scan cannot recover a real empty frame
baseline, so its box-height fallback incorrectly changes the enclosing line.

### Private Repeat baseline contract

Immediately after the one planned source-body subframe completes, capture
`last_sub_frame_baseline: Option<Option<f64>>` in its local completed-frame
coordinates, before any later subframe can replace that result. Outer None
means unknown/unadmitted; Some(None) means the first actual frame explicitly
has no baseline; Some(Some(y)) is its actual local baseline. No font strut,
box-height substitution, painted-leaf guess or fake Text is authorized.

Pass the captured complete tri-state unchanged to the existing private
`block::commit_prepared_frame` argument AFTER height and BEFORE items.
Commit records a known first-frame decision at the actual final physical
origin, independently of rendered-copy count. Outer None stays unknown and
must not become Some(None): an uninstrumented producer is not proof of an
absent baseline. Some(None) remains a genuine first-frame absence and cannot
fall through to a later painted Text. Thus count0 retains its known actual
frame geometry while its exact prepared source-body physical contribution is
still discarded by the separately proven helper. A positive copy uses the
same baseline result, not a separate typography path. Do not add a new paint
fallback or claim complete baseline coverage for unknown producers here;
preserve their existing legacy path under the receiving owner's certificate.
Parent scope isolation, tri-state first-frame semantics and rollback remain
owned by their current hub/subframe/checkpoint/line-close owners.

This private metadata transport stays in current layout, with unchanged public
FrameItem/entity fields, signatures, defaults and source preparation count.
ADR-0127 continuous flow applies after genuine baseline RED and L0 first.
Require independent declared-metric zero/positive, earlier-text, unpainted-first,
nested-inset and empty-Box controls plus actual pinned zero-source geometry at
unchanged precision; do not claim source/paint restoration from aggregate
counts or alter the membership0/0/0 gate.


### Unknown-versus-absent transport refinement

Read-only source review at UTC2026-10-22T02:20 identifies an explicit loss of
information: Repeat's `baseline.flatten()` and generic Block forwarding turn
unknown outer None into a known absent baseline. `placement.rs:416–417` publishes
flow end/width but not baseline, so existing opaque Align/Text producers can
legitimately return unknown while carrying actual visible text. Box already
distinguishes unknown via its legacy affine observer. The private commit must
preserve all three states rather than manufacture absence. This is a causal
metadata correction within the already specified protocol, with no new public
contract, producer certification or fallback policy. Root/Box owners retain
actual independent opacity-refutation tests and source-state provenance; this
consumer changes only its argument transport after L0 first.


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

Own a private per-piece line-alignment scope. Keep inherited lexical alignment for child frames. Compose the source once, record only actual outer piece line operations, and settle them against the completed finite natural logical width before adding Repeat residual alignment. Direct flow items are never shifted by a guessed whole-frame correction. Store exact line/item and decoration associations; apply each once before link resolution and visual cloning.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.


### Direct-flow source carrier regression control

Read-only review after the five directed alignment tests passed is recorded in
`lab/results/remaining-parity-2026-10-22/agents/repeat/marker-control-before.json`
with exact UTC, HEAD, dirty stat and source/L0 hashes. `shape.rs:195–201` flushes
its preceding marker-only line; `line_commit.rs:369–370,655` distinguishes no
causal line from actual line fitting but still invokes Cursor.
`html_frame.rs:83` already defines source/context markers as causally neutral.
The new piece scope must record only an actual causal line after excluding its
causally associated decoration paint; a source marker alone cannot acquire an
independent text-line alignment. A genuine Locator-generated Located Shape
control must preserve its actual source anchor at the same Repeat residual
origin as the shape, without a hidden half-frame displacement. This is a
regression hypothesis until its new test runs before the guard, not a measured
closure claim. Preserve the five earlier alignment tests and all source/physical
coverage checks. The private visibility warning is repaired by keeping the
new hub state private to its actual layout descendants, never widening it or
adding a lint suppression.

### Explicit source-owned child alignment refuter

`lab/results/remaining-parity-2026-10-22/agents/repeat/source-align-regression-before.json` records the current candidate-2 and original-control evidence before new tests. The immutable explicit-child-left refuter regressed by8.732177734pt while the other seven Repeat controls are GREEN. Add actual bare-versus-Source textual Left contrasts inside a25pt child under inherited Center, with independently declared10pt glyph metrics. Both must receive only Repeat's outer residual, at x37.5pt in the80pt region with origin10pt; do not recenter the explicitly left child. This owner retains inherited alignment and the existing piece protocol unchanged. Classification and the narrow Source transparency correction belong solely to `00_nucleo/prompts/core/compiler/layout/placement.md`; no Repeat geometry compensation or source replay is authorized.

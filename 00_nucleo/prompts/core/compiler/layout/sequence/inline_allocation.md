# L0 — certified inline fractional allocation
Hash do Código: 16b97a92

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sequence/inline_allocation.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130, ADR-0131.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Measurement before specification

`lab/results/nested-fractions-repeat-2026-10-21/original-before/` and
`box-before/measurement.json.gz` retain exact source, injected font, binary,
HEAD, dirty stat, UTC, commands and diagnostics. Baseline product
`94208ba4526d7bda97d4519aa8eb12212bd13dc1f8932552f235940e73238485`
is bound to `baseline-source.json.gz` and its copied successful build receipt.
Oracle commit is `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

The original three nested allocation/repeat sources are RED while four
existing supported controls pass. In child-one-fraction, Tekt paints joined AB
at x8 versus oracle A at8 and B at185.98584; AFTER remains197.02881.
In parent-child-budget, Tekt AB starts83.13318 and SUFFIX171.80383 versus
oracle A95.39974, B136.91960 and SUFFIX193.53223. Child and exterior
fractions have incorrectly shared a budget. These are language positions and
content, not vector identity. The previously rejected full-body provisional
frame cannot be reused: it placed B beyond the page after settling the wrong
width. It remains archived in the prior task, not an accepted design.

Current `boxed.rs:196–305` appends child pending fractions/repeats to the
exterior flat queues; `line_commit.rs:163–281` allocates that exterior
queue. `word_layout.rs:31–44` already chooses wrapping before line close.
Pinned `inline/collect.rs:94–101,228–233` and
`inline/line.rs:577–591` establish the width dependency. Repeat is separately
a Block (`rules.rs:743–745`), with default collapsed paragraph spacing
(`flow/collect.rs:240–288`) and whole supplied region consumption
(`repeat.rs:18–68`). A horizontal-only prepared paint tree cannot promise
correct multiline height. Existing vertical Block preflight and whole-text
paragraph-planner certificates remain owned by their original components.


## Source-scope refinement measurement (2026-10-21)

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/scope-refuters/measurement.json.gz`
records exact source/font bytes, UTC, HEAD, dirty state, commands, diagnostics,
PDF/SVG and source-bound prototype binary
`380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`
against the same ratified oracle commit/tree/SHA above. All six sources compile
without errors. Global-flow-tail paints joined PAB and Q at134.05884 in the
prototype, versus PA and BQ at215.79541 in the oracle. Fraction-rounding-residual
paints joined AB and Q at226.88379, versus A at8, B at36.686947 and Q at225.4.
These are source-language positions; PDF token grouping alone does not certify
source deletion. Zero-width controls retain the visible source body and fixed
sibling, including coincident A/Q; an empty zero Box has its own real baseline.

`eval/markup_stream.rs:305` adds `eval.flow.scope` to a complete lexical tail;
`layout/paragraph_prepare.rs:65–99` already positively classifies bounded text
fields/channels. Rejecting every custom delta propagates Blocked into such a
complete tail. The initial arithmetic model in
`agents/box/rounding-certificate-analysis.json` is withdrawn as a causal proof:
Python's compensated builtin sum did not model the current Rust ordered sum.
`agents/box/rounding-ordered-fold-correction.json` preserves that refutation
and a separate provisional ordered-fold analysis; it is not compiler evidence.
`lab/results/nested-fractions-repeat-2026-10-21/corrected-certificate-red/`
retains the exact input/source state, UTC, full test log and receipt. Its
corrected scope test fails at actual29.8 versus expected133.2, while fourteen
controls, including the
rounding geometry control, pass. Preserve their exact test inputs/tolerances.
No arithmetic correction follows from the refuted model or its passing control;
retain the existing normalized ratios and strict cursor<=right proof unchanged.

### Refinement decision and refuters

Admit only positively reconstructed lexical deltas: existing paragraph
preparation's typed bold/italic/size/fill/weight/lang/font fields and its exact
inert text/custom channel class, including eval.flow.scope. Compare the full
delta with the permitted reconstruction so unknown or newly added fields
remain opaque. A marked complete tail participates in its original exterior
certificate; its marker never authorizes a partial restart or omits siblings.
Resolve hypothetical lexical scopes through text's actual pure chain/style
policy, preserving its custom font/style precedence and folds rather than
substituting TextStyle::from alone. Do not dispatch content or mutate effects.

Retain the current normalized ratio/clamp formula and strict ordered
cursor<=right proof unchanged. No binary search, representable-budget rewrite,
epsilon, relaxed tolerance, suffix snapping or preferred-fraction offset is
authorized by this refinement. Zero total budgets yield Some(0), while
zero/negative/nonfinite weights and uncertified scopes retain legacy handling.
Missing B, shifted suffix, repeated body, changed diagnostics or lexical
render/refusal regressions refute the refinement. Existing eight Box tests,
three initial nested tests and exact language tolerances remain unchanged.

## Exclusive responsibility

Privately construct an immutable assignment tree for a positively certified
complete no-wrap inline segment before actual dispatch. Inspect real Content
nominally and exhaustively; no Content/effect emission, callbacks, page or
locator/request mutation, identity allocation, source replay, output cache,
unknown-as-zero approximation or glyph-string/pointer identity key is allowed.
Use actual injected FontMetrics and the existing word/space measurement
policy under resolved lexical styles. A fractional Box body is opaque to
exterior natural-width calculation and must not be measured or dispatched.

Admit explicit fixed Box total widths without measuring their bodies; admit
single-word Text, known finite nonnegative absolute HSpace, ordinary weak
space and transparent Source/Located/Realized visual/Styled/Strong/Emph
transports only within their proved measurement/style conditions. A complete
segment must contain a fractional Box and have a positive finite no-wrap
proof from all its fixed occupancy and assigned widths. H-fr-only segments
retain the existing path. Unknown visual/contextual fixed width, ambiguous
lexical changes, internal text whitespace or break, unsupported direction/
tracking/justification, invalid dimensions or unproved wrapping refuse the
whole segment without mutation or changed diagnostics. Pure nonvisual source
markers may be zero only when their existing dispatcher proves no extent.
Opaque refusal is not semantic zero or a suppressed warning.

Explicit Parbreak/Linebreak and real flow objects (including Repeat/Par/Block)
delimit independent scopes; no plan spans them. Preserve the paragraph
planner's whole-scope rule. A rejected or fixed inline child receives a
private Blocked capability so a nested transparent Sequence cannot plan a
partial paragraph without its exterior suffix. Independent flow/subframe
scopes start with no exterior assignment. Transparent wrappers preserve the
assignment route; recursive Sequence nodes describe its actual original
children. The tree is geometry routing only, never source-event authority.

A private Node has Fraction assigned width, Sequence child assignments and
Blocked alternatives. Some(0) is a genuine assigned zero, distinct from no
assignment. For each admitted line compute nonnegative free width after
fixed occupancy, using the existing finite fraction ratio/clamp semantics;
prove the resulting total also fits. Preserve zero/negative/nonfinite legacy
handling by refusing uncertified cases, never changing its formula or flags.
Actual Box/H consumers take their own assignment once and do not enqueue
that same share for another allocation. A nested Box body starts an
independent region plan at its actual known width; fixed children retain
their own explicit region. Unknown/legacy paths remain unchanged.

Expose only the private Node, construction and assigned-width consumption
needed by existing layout descendants. Keep eligibility and arithmetic here;
Sequence owns scoped installation/original dispatch, Box owns composition,
HSpace owns its advance, hub owns storage, subframe owns isolation, checkpoint
owns restoration and Repeat owns its Block flow/copy semantics. No public
field/type/trait/function signature or new pipeline phase is authorized.

## Acceptance and limits

L0 first, meaningful RED→GREEN and exact bilateral measurement are mandatory.
Derive expected child/sentinel coordinates from declared glyph widths and
regional shares; include exterior spacers, unequal siblings, nested/fixed
children, inset and zero budgets. Require original source piece/Context
admission once, visible B and every sentinel, no duplicated parent/child
share, unchanged true body baseline/height/clip/positions and normal opaque
refusal without state mutation. Preserve all previous eight Box tests and
full prior original/twenty-plus-seven controls and their exact tolerances.

Assignments are immutable and scoped. Test restoration across nested
subframes, styled/source transports and real checkpoint rejection/replay,
including Some(0) versus absent. No global ledger or feature-name heuristic.
General wrapping, bidi/typographic scope and opaque fixed measurement not
positively certified remain explicit limits; do not claim their parity or
borrow a vertical allocator for them. Repeat and source callbacks must use
their actual owners, not a width formula guessed from fixture output.

This is ADR-0127 continuous-flow parity correction inside existing layout,
with source order/phases unchanged. A discovered need to move evaluation/
realization or add a public contract must receive its own review first.
Validate tests, source-bound parities, Hash, Clippy/rustdoc/fmt, unsuppressed
Tekt and direct/reverse ownership/Nucleus/hash lineage before completion.


## Actual page-style tree measurement (2026-10-21)

`lab/results/nested-fractions-repeat-2026-10-21/prototype-2/original-after/measurement.json.gz`
and `prototype-2/eval-content-probe/receipt.json.gz` retain the frozen candidate,
source/archive/rlib binding, HEAD/dirty state, UTC, exact programs/fonts/commands,
full diagnostics and actual Content trees. The original three main debts remain
RED despite synthetic core controls. `01-source.content.txt` is exactly root
Sequence[SetPage, Styled(page channels, Sequence[Space, Styled(text.font,
Sequence[Empty, BEFORE, Parbreak, fractional Box[A,hfr,B], Space, AFTER,
Space])])]. Its outer page delta contains four Float8 margins, Auto height,
Float240 width and eval.flow.scope=true. `02-source.content.txt` confirms the
same real page/text wrappers around parent and child fractions. Paragraph
preparation refuses Box/Repeat atomically; it does not authorize their dispatch
without this allocation owner. The current text-only Styled class rejects the
whole page-style tail and propagates Blocked after the actual SetPage installs
the new region. That is the measured source route missing from prior tests.

### Page-boundary decision and acceptance

A fresh complete Sequence scope may prepare its full remaining ORIGINAL suffix
again immediately after its existing direct typed SetPage dispatch has installed
actual geometry. Preserve original typed source indices and every preceding
source occurrence; a plan-base offset affects assignment lookup only. An
inherited subpath/Blocked capability may not restart a partial suffix and drop
exterior siblings. Inert preflight encountering SetPage must refuse geometry for
all following parts, including after a Parbreak, until that actual dispatch.
Admit only the explicit proved page.width/page.height/four page.margin-* style
transport channels alongside the existing positive text delta class. They carry
lexical metadata and cannot themselves predict or apply future page geometry.
Unknown page/style channels still refuse honestly; no page/phase/default changes.

Test the exact dumped tree with initial360pt page/10pt margin and actual SetPage
to240pt/8pt, and a prior OLD prefix on the old page. Derive final A/B/AFTER bounds
from declared11pt*0.6 metrics and the real224pt region, retaining the exact final
source weak newline. Existing metrics::line_content_right excludes weak
TextSeparator at the terminal edge; that source token is not a second fixed
suffix word. Preserve all prior tests and tolerances. These main-case tests must
fail before implementation and become green with the original three bilateral
sources; the page wrapper is not an uncertified-path waiver for those cases.

The combined actual-page RED receipt records both exact tree witnesses failing
with B at21.2 instead of192.4 before implementation. Terminal weak occupancy
is distinct from fixed content or an explicit H-space slot. At closure trim
only trailing weak occupancy; a later fixed or fractional H slot preserves the
earlier weak gap. Keep adjacent weak-token collapse state separate from this
occupancy order. Literal Text spaces are never trimmed by this certificate.
This follows pinned inline/line.rs233–249 and current separator provenance; it
does not change weak-space dispatch or any unknown-prefix eligibility rule.

## Owner-local verification

The source-bound full lint retained in
`lab/results/nested-fractions-repeat-2026-10-21/prototype-4/lint-candidate/after.sarif.gz`
reports V2 for this consumer: it lacks a local test module even though the
existing Box/source tests exercise its integration. Exact source/owner,
dirty HEAD/stat, UTC and lint SHA are archived in
`agents/box/inline-allocation-local-tests/` in that lab record. This is a
verification gap, not evidence for changing productive allocation behavior.

Keep three meaningful local tests using an injected declared-width font and
actual Content inputs. Unknown natural Box width must refuse the complete
certificate instead of receiving zero; an exactly exhausted known budget
must still produce Some(0), distinct from absent/Blocked assignment. Zero,
negative zero and negative fraction weights retain refusal. A weak terminal
newline is excluded from the fixed budget, while a weak gap preceding a real
absolute H slot remains included. Unequal exterior siblings and an independently
prepared child use their respective actual local widths; inspecting an opaque
fractional body must never be required for the exterior certificate. Derive
expected widths from the declared characters and region, preserve immutable
exterior assignments after child preparation, and leave all productive
functions, thresholds, source integration tests and prior criteria unchanged.
These tests supplement the existing full-path tests and cannot certify
source-language parity or replace final lint/lineage/build qualification.

The declared-width test adapter must implement the complete existing FontMetrics
contract, including text_edges with its declared 7pt top and -3pt bottom. This
test-only setup corrects the preserved E0046 attempt; it does not change
productive metrics or allocation acceptance.


## Remaining inline source correction — 2026-10-22

### Measurement before specification

`lab/results/remaining-parity-2026-10-22/baseline-nine/measurement.json.gz` (SHA-256 `320e226fc9eb1b606cb155c4bfcba26046c2398f6fe0339102f1fd870ebd2aa1`, UTC 2026-10-22T11:48:44.824207+00:00–2026-10-22T11:49:15.381299+00:00)
records the exact nine immutable sources, ratified oracle commit/tree/executable,
source-bound baseline product `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`,
HEAD/dirty stat/source archive, font bytes, commands and full PDF/SVG observations.
The exact inspected L0/source hashes are preserved in
`lab/results/remaining-parity-2026-10-22/agents/inline/investigation-state.json`.
These are source-language geometry/text observations, not required execution
mechanics. Preserve all historical refusals and failed candidates as evidence;
only the explicitly measured subsets below supersede their conservative limits.

### Actual block boundary and signed finite shares

The explicit Shape prefix has matching shape/page geometry but the following
fractional Box still joins A/B, with B displaced193.637207031pt. The current
certificate groups Shape among unknown inline occupancy; Shape's actual owner
closes the line and installs a flow boundary (`layout/shape.rs:196–279`).
Classify this known flow delimiter independently: close the preceding complete
inline segment and prepare the following complete segment from actual bounds.
Do not measure Shape as zero inline width or make an unknown natural inline
Box transparent. Source/Located/Styled transports preserve original routes.
Shape after a prefix and inside a subframe must retain actual dispatch and
current region ownership; no partial Sequence restart is permitted.

Finite zero and negative fractions are valid language values. The pinned
`typst-library/src/layout/fr.rs:54–61` returns nonnegative ratio×remaining when
the ratio/remaining are finite, else0. Therefore0/0 and finite cancelling
fractions yield assigned0; a standalone-1/-1 share receives full free width.
The current weight<=0/total<=0 refusal is superseded only for positively proved
complete finite no-wrap budgets. Use the existing ratio/clamp policy, including
its nonfinite-ratio0, and retain the strict final cursor<=right proof. Do not
change H-fr-only legacy semantics, arithmetic rounding, total budget, source
phases or unknown-width refusal. Assigned Some(0) still differs from absent
certification. Compose the actual fractional Box body once at its real share.

### Bounded Latin RTL allocation

The RTL witness has visible A/B in SVG at incorrect positions and missing A/B
in clipped PDF extraction. The current certificate refuses RTL explicitly;
its fallback composes the Box body with the exterior provisional width0.
Positively admit the measured horizontal Latin/ASCII no-wrap case under uniform
actual horizontal direction and no tracking/bidi override/unsupported text
features. Reuse the existing local-region Box composition and actual glyph
width policy. Keep Arabic/Hebrew/control characters, mixed direction scopes,
vertical directions and every unproved bidi surface opaque; do not synthesize
BidiSpan, reorder text or claim general RTL parity. Preserve the prior LTR
certificate and complete source/assignment routing.

### Acceptance

Add meaningful rendered RED tests before implementation for Shape followed by
P/fraction[A,hfr,B]/Q, a standalone negative-weight fractional Box with that
body, and the actual Latin RTL frame. Derive expected regional bounds from the
injected glyph widths, require A/B and all sentinels once, exact suffix and
independent child shares. Keep unknown-width/overflow/nonfinite refusals and
all existing source, effect, rollback, page-style and rounding controls. Add
owner-local signed/zero/cancelling share tests; replace only the superseded
mechanical conservative-refusal assertions with stronger language-share
assertions, retaining their inputs and separate nonfinite refusal controls.

This is internal ADR-0127 continuous-flow parity work, with no public field,
trait signature, new default or phase. Link/Underline geometry and font metrics
remain under their separately reviewed owner contracts; this clause does not
authorize their new public method. Full frozen corpus, prior controls, tests,
Hash, Clippy/rustdoc/fmt, unsuppressed lint and direct/reverse seals remain
mandatory without changed thresholds or ignored tests.


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

### Approved transparent wrapper routing

Link and Underline may transport the complete immutable assignment route to
their actual body, without measuring body effects or invoking callbacks.
This positive geometry traversal does not change typed source plan edges,
Context execution, source authority, annotation semantics or the no-partial-
restart rule. Unsupported decorators remain conservative until measured.
Require the actual nested decorated-link Box A/hfr/B and outside prefix/tail
coordinates, plus source effects once and no missing link region. Font-backed
Underline paint belongs to the separately approved decoration/metric owners,
not this width certificate.


## Known weak-boundary selection before fractional allocation

### Actual source evidence and classification

The fresh source and final prepared-content receipts above, supplemented by
`lab/results/remaining-parity-2026-10-22/red3-infra-local-repeat/`, record the
actual whole-flow fixed occupancy226.95556640625pt in a224pt region. Native
places LABELS1 on a second line; its first-line fixed occupancy is
174.947265625pt and the actual Box share49.052734375pt. Source-bound native
SVG/PDF has five X glyphs, not the initially miscounted six. The old strict
no-wrap proof correctly refused the overfull whole flow; rounding is refuted.
Pinned `inline/collect.rs:98–99,229–233` gives fractional slots their explicit
known zero natural width, deferring their actual Box body until line shares.
This is language word/line/region geometry; no vanilla buffer representation
is required.

### Bounded internal selection contract

Keep the existing no-wrap route unchanged. After a completely known valid
walk, an ordinary Simple/LTR/nonjustified/nonhyphenated/untracked scope may
select known weak word boundaries with the existing private
paragraph_select(Simple) owner. Create a width-only selection skeleton:
consecutive fixed/fractional items between actual weak separators belong to
one indivisible word; gaps retain actual known weak widths. Fractional slots
have known natural zero; arbitrary opaque content never does. Keep original
source child routes and the immutable assignment tree; do not emit prepared
text, replay source, dispatch callbacks, rebuild Content or restart a partial
suffix. The selector is only a shared pure width decision and receives no
fragment text that could be materialized.

Map complete selected word ranges back to their original occupancy slices;
allocate each fractional line only after its fixed occupancy and total share
fit the same actual regional bounds under unchanged strict arithmetic. No
epsilon, representable-budget rewrite, cursor snapping or threshold change.
A newly selected wrap must have a proved positive fractional Box on its
preceding line and an actual positive weak gap, so normal existing dispatch
closes the filled line at that boundary. Require the complete continuation
to fit its own known scope; reject unproved wrapping, opaque words, optimized
strategy, mixed/RTL direction, explicit hyphenation, absolute-H ambiguity,
regional transactions, columns/cells/subframes and source callbacks. Preserve
all earlier no-wrap/signed-zero/refusal controls and public contracts.

Require a declared-width source-shaped RED with P/fraction[A,hfr,B]/Q then
STATE+1, COUNTER+1, LABELS+1 separated by actual weak spaces. The indivisible
first word and Context text fragments stay intact; LABELS1 must start on the
next line, B must end at Q, first line must occupy its actual regional edge,
and every source piece/numeral appears once. The exact five-copy source RED
must become GREEN with its effects once. This supersedes only the bounded
no-wrap exclusion; it introduces no public API, default strategy or phase.


## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

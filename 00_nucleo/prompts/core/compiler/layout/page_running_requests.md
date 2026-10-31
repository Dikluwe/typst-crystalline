# L0 — Physical page running-content request protocol
Hash do Código: f14b94e1


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/page_running_requests.rs`
**Architecture:** ADR-0071, ADR-0073, ADR-0074, ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130, ADR-0131.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/bundle-numbering-input.toml sha256:2e87970ea88f2802436705618856cc6aff0563d0fdbe8724d3d0221e59d494d9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42


## Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, `layout/page_running.rs:78–106` and `layout/page_canvas.rs:17–55` render raw templates after body indexing. `layout/mod.rs:1237–1278` proves an inserted Located wrapper would alter causal identity rather than being neutral transport. `layout/flow_suffix.rs:15–129`, `layout/checkpoint.rs`, `layout/sub_frame.rs`, `layout/place.rs:43–77,176–236` and `layout/footnote_flush.rs:119–165` establish speculative, isolated and deferred ownership boundaries.

The pinned oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) binds exact sources, argv, times, source state and binaries: repeated headers/footers create physical semantic occurrences; all five page layers have ordered counter/query effects; overwritten templates remain inert. `ambient-baseline/receipt.json` in the same evidence root (SHA-256 `6291efca219a3558ad2192f99f1208d64ad3e70b38ec96db2d417527549fcb05`) refutes freezing creation10pt styles: actual page headers inherit20pt and30pt at insertion.

These are language values, errors and physical positions; the opaque plan and transcript representation are mechanics. The human approved the exact API and Paged discovery/realization/source-construction phase in `lab/results/set-page-header-2026-10-21/review.md`. Frozen declarations live in `agents/core/api-declarations-final.json` there (SHA-256 `4baaab61b3c3b247593433d18ab78d22ba5e2c275acab6e747c43dc0ef559196`). This approval supersedes legacy no-new-phase clauses only for this protocol, preserving all existing APIs and unrelated limits.

## Exclusive responsibility and public surface

Own opaque protocol representations, immutable store validation, private request matching and transactional accepted-packet trace. Source-plan construction/index population belong to `00_nucleo/prompts/core/compiler/introspect/page_running.md`; requested realization/rebinding belong to `00_nucleo/prompts/core/compiler/realize/page_running.md`. The layout hub owns the new facade and descendant state installation. No public setters, extra getters, trait methods, entity fields, Content variants or legacy signature changes are authorized.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageRunningSlot { Background, Header, Footer, Foreground }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageRunningKey {
    pub page: NonZeroUsize,
    pub slot: PageRunningSlot,
}

#[derive(Debug, Clone)]
pub struct PageRunningBody { /* private owned Content, typed plan and authority */ }
impl PageRunningBody {
    pub fn content(&self) -> &Content;
}

#[derive(Debug, Clone)]
pub struct PageRunningRequest { /* private authority, key, capture and geometry */ }
impl PageRunningRequest {
    pub fn key(&self) -> PageRunningKey;
    pub fn span(&self) -> Span;
}

#[derive(Debug, Clone)]
pub struct PageRunningResolution { /* private request authority, Content and plan */ }
impl PageRunningResolution {
    pub fn key(&self) -> PageRunningKey;
    pub fn content(&self) -> &Content;
}

#[derive(Debug, Clone)]
pub struct PageRunningTrace { /* private typed packet admission and sealed geometry */ }

pub struct PageRunningRequests {
    pub callbacks: LayoutCallbackRequests,
    pub running: Vec<PageRunningRequest>,
    pub trace: PageRunningTrace,
}

pub struct PageRunningDocument {
    pub document: PagedDocument,
    pub trace: PageRunningTrace,
}

pub struct PageRunningPassOutcome { /* private closed Pending/Complete state */ }
impl PageRunningPassOutcome {
    pub fn into_result(self)
        -> Result<PageRunningDocument, PageRunningRequests>;
}

#[derive(Debug, Clone)]
pub struct SealedPageRunning { /* private immutable resolutions */ }
impl SealedPageRunning {
    pub fn new(resolutions: Vec<PageRunningResolution>) -> SourceResult<Self>;
    pub fn resolutions(&self) -> &[PageRunningResolution];
}

pub struct PageRunningLayoutInputs<'a> {
    pub math: Option<&'a SealedMathCallbacks>,
    pub par_line: Option<&'a SealedParLineCallbacks>,
    pub running: Option<&'a SealedPageRunning>,
}
```

## Private authority, matching and trace

PageRunningBody retains unchanged Content and one immutable Arc<SourcePlan> authority. Private plan nodes use transient PacketId addresses and closed typed child edges, with separate source-parent relations and complete environment-frontier provenance. PacketId and environment addresses are never language Locations, hash keys inferred from Content, or exposed morphology. Private compiler-descendant access uses parent descent; no new pub(crate) helper is introduced.

A request retains the exact source plan, selected template packet, physical key, Source span, exact finite region, and effective page-environment authority. The environment is the accepted page-wide source frontier, not an arbitrary last child style or freshly allocated retry-local StyleChain pointer. Creation-time operational capture retains rules/guards/file/source/context floor while its ambient boundary rebases onto the effective page chain; explicit inner deltas and prepared aliases survive. Same source/key/geometry with a changed effective environment is not reusable.

A resolution retains its exact request and a prepared marginal body/plan. Binding creates a new immutable resolution; old resolutions remain unchanged. The resolution retains its original unexpanded contribution with the exact producer objects and plan authority. Reused realization exposes that retained unexpanded view; binding changes only the current expanded view. The existing Session may retain its hierarchy, frames and reads only after proving the exact original producer objects, never from key or Content equality. Retain the complete ordered realization diagnostics (severity, span, hints and trace) privately; reused realization replays them into the current existing Sink through its normal recording semantics without executing callbacks. Zero-producer results cannot silently lose their realization warnings. Sealing rejects mixed source generations, duplicate/conflicting keys, malformed finite geometry, invalid retained authority or incompatible template/environment bindings. Empty store is valid but proves no requested physical content exists. A pending membership change returns the complete current request set, including matched requests with a private retained-resolution proof established by L1. Requested realization may return that immutable proven resolution without execution. Replacing the store prunes obsolete physical keys without allowing L3 to infer authority from keys; an empty current set may replace an obsolete nonempty store. Layout consumes a valid matching resolution once per accepted physical insertion; absence, reuse or mismatching authority yields pending/diagnostic state rather than an invented empty success.

The private trace records source-authority, PacketId, Root/PostChildren phase, semantic anchor page and a hierarchical causal order token, including operations without Location and derived operations sharing one Location. It owns accepted coverage and sealed physical geometry, not semantic facts reconstructed from painted frames. Background/Header precede the accepted body segment and Footer/Foreground follow it; source parents remain separate from physical order. Captured templates are not ordinary body admissions. A deferred child retains its accepted source-callsite semantic anchor independently of its eventual geometric destination; neither isolated measurement nor paint order supplies semantic order.

## Transaction and isolation

Trace, pending requests, consumed resolutions, cursor/source-edge state, slot bindings, page-effective environment and page assignment form one causal transaction with existing layout state. Rollback restores mutations to prior entries as well as append-only tails. Flow-suffix replay retains exact planned packet addresses. Subframe isolation saves/restores caller plan state in LIFO order. A source-bearing child must descend by its declared typed edge; generated wrappers must preserve the actual source-child route. Isolated measurement or genuinely generated-only content carries explicit None and cannot consume running resolutions or append trace facts. No fake Location, Content wrapper, pointer search, postpaint page guess or matching by Debug/hash/equality is permitted.

## Acceptance and limits

Require semantic RED before implementation, then fixed expected physical query/counter/state/page order for repeated header/footer/all five layers, inherited and overridden templates, Context/read stability, exact diagnostics/source origins, nested PageRun restoration and explicit alias identity. Preserve explicit inner styles and insertion20/30pt ambient controls. Protocol controls cover mixed generations, duplicate keys, same geometry with changed environment, stale/rebound source, empty stores, complete body/slot coverage, rollback/replay, deferred float/note destination, measurement isolation, and math/par.line coexistence. Source plans and packet classifiers require independent exhaustive-domain compile-negative evidence; grouped dispatch is not proof for every class.

No direct Func marginal mode, source re-evaluation, separate Context solver, new arena, generic global-frame semantic introspection or public semantic store is introduced. Existing per-store sealing, retained five-attempt Context budget, legacy layout entrypoints and HTML semantic target remain. The old diagnostic-order, HTML ignored-page warning and unrelated layout limitations remain explicit unless separately corrected under their owners. Full lineage/ownership/Nucleus graph, strict lint, builds, suites, Rustdoc, fmt and Clippy must pass without suppressions; finite controls do not certify universal layout or language parity.

A feature constructor may retain an explicit finite typed path through known generated wrappers (for example AlignBody then BlockBody) to one actual source cursor. While any generated edge remains, dispatch emits no source phase, Location or coverage and never reads the target packet as if it were the wrapper. Each owning handler consumes exactly the next declared edge; mismatch is invalid and cannot complete. Only the exhausted path enters the actual source packet. The host must document its exact constructor path; no wrapper discovery by Content matching is permitted.


## Deferred source order and non-rendering semantic children

The pinned oracle cross-page receipt `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/receipt.json` records NOTE at physical page2 with counter3 before page2 HEADER counter4; FIRST is page1 counter2. `oracle-extended/receipt.json` there separately records a deferred note before SECOND on the same page despite later painting. These language observations refute sorting semantic events by final paint page or flush time. A deferred child reserves its semantic page and hierarchical source-order token at its actual enqueue frontier; it executes and admits facts only when the accepted deferred body is processed. Descendants and remainder retain that token. Final geometric positions remain separately sealed and may be later pages. Rollback restores reservations and admissions atomically. No public API, default or additional pipeline phase changes.

Source-only child admission preserves existing canonical traversal for zero-render FigureCaption and FootnoteEntry children without painting them. Skip operational templates and pruned original branches. A prospective citation child or auxiliary identity is admitted only under the actual bibliography-membership predicate used by the Ref owner; ordinary Ref candidates remain dormant. Preserve the existing source parents, aliases and zero-size geometry.


## Page-wide style frontier

`lab/results/set-page-header-implementation-2026-10-21/oracle-partial-style/receipt.json` measures three partial inline/block style forms retaining page20pt and a full-root set-rule tail changing to30pt. The actual round7 style assertion reports11pt, so the old Sequence-only observation is incomplete. `compiler/eval/markup_stream.rs:305` marks lexical set-rule tails with existing eval.flow.scope, unlike native local text styling.

At entry to an admitted source packet, observe its full stored environment only when the source-plan owner proves a page-wide root path. That private proof uses closed typed carrier/sequence/page-flow edges, permits StyledBody only for the actual lexical flow marker and rejects local feature bodies. Root singleton global-style tails must be observed even without a Sequence. Do not infer page style from the last painted leaf, local partial style, equality or a collapsed chain. Marginal/generated/subframe cursors cannot replace the page frontier. Preserve source style data and the measured20/30pt and explicit inner-style controls.


## Planned direct Equation source bridge

### Measurement before correction

`lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json`
records the source-bound diagnostic11 product and the pinned vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Actual public syntax
`math.equation(metadata("equation-child"))` and
`math.equation(figure([EQUATION CHILD], caption: none))` both succeed in the
oracle and fail source-plan verification in the product. Ordinary `$a + b$`
succeeds with equal typed observations in both. Exact before source/L0 bytes,
hashes, UTC and working-tree provenance are archived in
`agents/layout/equation-bridge-before.json.gz` under that implementation root.

The source plan already declares EquationBody and recursively plans the direct
body according to the existing semantic partition. The equation painter calls
MathLayouter without that cursor; its external bridge creates an isolated
unplanned Layouter. Therefore required direct-body facts cannot be admitted by
actual painting. This is a missing internal handoff within the human-approved
physical-page phase, not authorization to open terminal MathSequence subtrees.

### Exclusive correction

Own a compiler-private EquationBody cursor selector. Its required-subtree test
follows only already-declared source-plan children, never opaque entity fields or
terminal math descendants. The parent painter uses the exact cursor in the
existing planned subframe, retaining source authority, transaction, captures,
actual resources, queues and diagnostics. Expose only a compiler-private predicate
for the existing authenticated position-marker shape to the math composition
owner. Do not manufacture semantic facts, positions, Locations or new public
carrier fields/signatures.


## Source-owned note continuation correction

Measurement: `lab/results/set-page-deferred-corrections-2026-10-21/fit-3/receipt.json`, `fit-3/tests.log`, and unchanged pinned-oracle page text under `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage/` precede this amendment. Exact ownership and source-only records are in `agents/layout/` under the correction root.

While a private footnote capture is active, bracket an actual planned packet with the footnote fragment owner's certified atomic boundary before Root/Location admission and after PostChildren. The outer transparent source/style/location carrier participates in that same scope. Unsupported source variants disable cuts, preserving the prior whole-frame path; nested atomic packets cannot introduce cuts. This geometry boundary neither invents facts nor changes source authority, causal ordering, coverage, duplicate checks or public APIs.


## Accepted block frontier after a full note-area reservation

`lab/results/set-page-deferred-corrections-2026-10-21/green-4/receipt.json` and
`green-4/tests.log` bind the actual source state: NOTE and SECOND physical pages
are correct, but SECOND's Root remains on page1 before its own painter admits
page2. Consequently Header2 counter5 follows SECOND counter4, contradicting the
unchanged oracle Header2 counter4 / SECOND counter5. A Source-wrapped Figure Root
is currently recorded into the preceding carrier line before the feature flush.

When the actual current note reservation consumes all remaining main-flow area,
close the preceding actual carrier line and request ordinary region progress at
the next certified source-owned Figure/Block boundary, before installing/admitting
that block's Root or Location. Use the existing exhaustive boundary classifier
and actual region geometry. Do not retag an event after the fact, change semantic
source anchors, sort by paint, reserve guessed page numbers, or broaden this
bounded block-frontier rule to uncertified source classes.


## Source-consumed spacing and single frontier transition

The actual `break-5/receipt.json` and `break-5/tests.log` under
`lab/results/set-page-deferred-corrections-2026-10-21/` show that capture is enabled
and START has a certified6.6pt break; the remaining frame222.9pt still contains
the full180pt source VSpace. Pinned `flow/distribute.rs:219–227,391–394,650`
consumes absolute spacing in the current region, stops before the following block
and clamps the emitted frame size. Record a typed VSpace end witness only after
its actual source traversal. It retains the preceding occupied prefix height and
actual consumed gap end; transparent source/style carriers inherit that witness.
A cut inside that trailing zero-paint gap clamps emitted prefix height to actual
available area but rebases the tail by the full source-consumed gap end, without
reapplying or manufacturing spacing. Ordinary cuts and uncertified gaps retain
the previous rule. Source/VSpace facts and authenticated markers remain once.

The same measured run shows an extra third page after the block-frontier helper
flushes its carrier. The flush may itself perform the required ordinary region
transition. Capture its actual page/column/rollback generation before flush and
request a further transition only if all remain unchanged. Do not guess the page
number or skip actual source admission; the block Root is still admitted after
the accepted frontier.


## Opaque physical trace storage

The source-bound Clippy RED in
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/validation-remainder/clippy.log:83–113`
identifies oversized private pass variants and the public result's opaque trace
storage. Exact before texts, SHA-256, UTC and working-tree state are in
`lab/results/set-page-report-blockers-2026-10-21/agents/layout/trace-callers-before.json.gz`
and `agents/layout/before.json.gz` under that evidence root.

The trace owner retains every existing source, admission, request, sealed geometry
and completeness value behind one compiler-private owning payload Box. Access
those same fields through `trace.data`; preserve exact mutation/read order,
cloning, authority, geometry, verification and acceptance. This is private storage
indirection only: the public opaque PageRunningTrace and all public fields and
function signatures stay unchanged. No new semantic fact, source traversal, phase,
callback, diagnostic suppression or relaxed gate is authorized.

Keep the compiler-private PageRunningTraceData payload structurally identical to
the previous trace fields; PageRunningTrace::new initializes that payload once
and ordered() still sorts the same admissions by the same causal keys. Box both
private PassResult payloads so neither branch determines its stack size. The
public into_result signature remains exactly
`Result<PageRunningDocument, PageRunningRequests>`; move payloads out of their
Boxes on return. No new public field, type, trait or signature is introduced.


## Zero-copy Repeat dispatch and inactive source contribution

### Measured language boundary

`lab/results/nested-fractions-repeat-2026-10-21/zero-source-oracle/receipt.json`
records exact UTC/HEAD/dirty stat, immutable source/fonts and the ratified oracle
commit/tree/binary. The source preserves the earlier positive Repeat body and
changes only its known Box width to80pt and gap to−100pt. The oracle composes the
body but emits no X, and its final observable results are STATE0 COUNTER0 LABELS0.
The positive body source remains STATE1 COUNTER1 LABELS1 in the immutable
`acceptance-before/measurement.json.gz` receipt under that same task. Composition
once therefore does not imply active final semantic membership when no copy is
selected. These are language effects and query membership, not painted-frame
inference or integer identity.

### Private selection and strict acceptance

Retain the complete raw source-dispatch transcript, including each required
Root/PostChildren phase, source authority and order. A zero-copy Repeat may record
only an opaque private inactive-RepeatBody proof bound to the current actual
Repeat source, its exact typed RepeatBody child in the same Arc source plan, and
the completed one-body dispatch. Do not substitute bibliography conditional_ref,
source equality, packet-number intervals, absent ink or missing geometry for this
source-owned selection. A positive count retains one active source occurrence;
its repeated visuals cannot create more source membership. Parent and siblings
outside the selected body remain active. Nested inactive bodies compose by actual
source ancestry without losing any raw dispatch proof.

Validate source/slot authority, foreign-plan rejection, complete required phases
and duplicate phases against ALL raw events before any semantic filtering.
Validate each inactive proof, reject forged/wrong typed edges and duplicate or
incomplete proofs, then filter only that exact source subtree's semantic
contribution before bibliography-key selection and index population. A missing
or duplicated event cannot be excused by inactivity. Ordinary zero-paint metadata
and updates remain active unless this source-owned zero-copy Repeat selection
excludes them. Inactive source identities stay reserved; no reevaluation,
reallocation, extra Context solver or retry-budget change is authorized.

Keep ordered() as ALL raw events. Store private selection state inside the cloned
PlannedPass/trace transaction so checkpoint restore is atomic. Preserve normal
positive Repeat bodies, actual source/effect order, ordinary source positioning,
foreign capabilities, complete-candidate gates and all prior tests. Add causal
actual-layout tests for zero versus positive membership, nested source boundaries
and missing/duplicate/foreign raw corruption before implementation. Final
acceptance must reproduce both measured 1/1/1 and0/0/0 with all real copy counts,
page/text/position/diagnostic controls unchanged.

This is an ADR-0127 continuous internal parity correction within the existing
layout→source-index channel: only private implementation metadata/methods change.
No public entity field, public method/signature, product mode/default, compatibility
break or pipeline phase is authorized. Preserve the pinned physical-page nucleus
and its complete coverage, transactional admission and source-derived-index
obligations. Source-dispatch coverage and active final contribution are distinct;
inactivity never weakens coverage or reconstructs membership by walking paint.

The private proof also binds the actual child Root/PostChildren admission indices.
Its constructor runs while the parent Repeat is still dispatching; final proof
validation requires its unique enclosing parent PostChildren afterward. A stale
index, incomplete child/parent phase, wrong typed child, foreign source or duplicate
proof rejects. These addresses prove completed dispatch and never select a packet
number interval. Only this owner stores/builds/validates the proof; its index
consumer is `00_nucleo/prompts/core/compiler/introspect/page_running.md`.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## Stack child origin transport (2026-10-27)

Measurement before decision: `lab/results/construction/diagnostics/stack-image-flow-fix-2026-10-27/effects-candidate/context-binding.json` records the twentieth child with metadata location page 2, but both captured `here().page()` and the retained Here Location page 1; vanilla reports page 2 for all three. `page_running_requests.rs:990-1003` and `realized.rs:285-307` register descendant original identities only when absent. Ancestor registration can therefore prevent a local child carrier from being captured. This is observable location semantics, not a requirement to imitate vanilla mechanics.

Decision (ADR-0127 continuous flow): while entering the existing private Stack subframe, recapture the child's already-authorized original identities at the local origin, before dispatch. Reuse the selected source-plan traversal, operational/pruned/ref predicates and existing marker protocol; this anchoring mode must not admit source events a second time or execute callbacks. Without a plan, reuse the existing original-location collection, without introducing a new allocator/identity rule. Ordinary original registration remains only-if-absent. Later explicit placements remain authoritative and nested Stack frames repeat the same local transport. Geometry and semantic event ordering remain separate. No public contract, phase, global dispatcher or Context solver changes.

Acceptance: an ancestor-pre-registered realized child origin must follow that child's transported frame onto page 2; source-level stored Here Location, captured page integer and metadata Location must all match vanilla. Counter/footer source ordering and all current context stability gates remain mandatory.

The second candidate refutes eager observation of every descendant: `effects-candidate2/binding.json` in the same diagnostic records rotated Metadata/Here positions after the preliminary reanchor advanced the Locator for unvisited visual nodes. Local recapture must neither observe/allocate locations nor create an absent runtime position. It only attaches a local carrier to identities already admitted by an ancestor. Existing normal traversal remains the sole first admission/allocator authority. Preserve the failing candidate and add a RED test asserting that recapture leaves the Locator seed and unvisited membership unchanged.

## Approved document-local numbering input (2026-10-28)

Measure before decision: readiness-fixes-2026-10-28/physical-candidate-execution.json,
its source archive and full preservation log bind dirty HEAD212aed0449, exact
diffstat, inputs and UTC. p1486 keeps /St20 and /St22 but the last PDF paints10
instead of20, because run_planned consults global PageStore by local ordinal.
The review review-bundle-numbering-input.md SHA-256
d248aa88694a06da945a2036ec33887fcbba0f67adf3e3bd7ccb30757354670d was approved
in direct human reply, retained in bundle-numbering-input-approval.json.
This ratifies only the named additive method and its local transport, superseding
earlier no-new-API clauses only for that method; all other boundaries remain.

Own the approved public method on opaque PageRunningBody:
`pub fn with_numbering_input(&self, pages: &PageStore, first: NonZeroUsize,
total: NonZeroUsize) -> SourceResult<Self>`. Keep a compiler-private optional
PageStore input on the wrapper; cloning a projected body preserves its exact
Content and SAME Arc SourcePlan. Validate checked interval bounds and complete
slots before projecting through existing from_realized. Preserve all raw and
realized values including logical zero, absent values and present Empty Content.
An internal store certificate may inspect private vector coverage in its owning
entity module; no extra externally public method/field/trait is authorized.
Invalid/overflow/minimal/incomplete input returns SourceResult diagnostics with
no panic, truncation, callback or fabricated value. Old wrappers default to None.
Tests before implementation require correct non-first interval values, plan
identity, unchanged original wrapper and rejection of invalid/incomplete ranges.

Require functional RED→GREEN, original integrated controls, complete technical
gates and independent PDF/SVG/word/page/0.001pt validation with source provenance.
Finite tests do not certify general parity, geometry, performance or JPEG.

Functional RED before implementation: numbering-input-red.json/log and source
archive retain the approved API skeleton returning its unchanged clone. The
valid non-first interval has no local view, and an invalid4/1 interval is accepted;
both tests fail. outline-original3-green separately passes its corrected carrier
observer under the same frozen inputs. The p1486 original integrated RED remains
the independent transport acceptance gate; no expected values may be relaxed.


## Private source-plan Math provenance fixture integration

The owning source-plan contract now retains inherited Math as private PlanNode
provenance for nested Equation semantic membership. This consumer's two manually
constructed cfg(test) PlanNodes describe ordinary Flow document/template roots;
initialize that private field false. Keep every existing test expectation,
source authority, raw phase and layout/runtime field unchanged. No productive
trace/renderer/public contract change is authorized here. Actual data ownership
and measured parity remain in introspect/page_running.md and its pinned nuclei.

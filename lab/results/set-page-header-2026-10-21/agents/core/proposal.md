# SetPage running matter: concrete draft for API and phase review

**Status: reviewable for exact API/phase approval; not approved.** No productive
L0, source or test is changed. Public declarations are frozen below. The finite
inspection domains, source-derived construction, strict provenance, transaction
rules and independent language gates define the implementation boundary; they do
not authorize unknown later public interfaces. Private formulas and actual changed
owner clauses follow owner-first RED→GREEN after the decision.

## Evidence before the proposal

Clean HEAD is `63b7b96b99109b61655b373dc0d248beb7838f86`, on
`codex/tekt-mature-layout`. `source-evidence.json.gz` and its index bind UTC,
clean status/diffstat, full source/L0 text and SHA-256 for 25 inspected pairs,
independently verified forward/reverse seals, 41 effective Nuclei, root/core
guides and ADR-0073/0074/0107/0108/0109/0127/0129/0130/0131. This inspection is
not a new language execution. Parent-owned fresh baseline/oracle receipts are
the decisive runtime evidence; earlier F/G observations remain historical.

The original F source is:

```typ
#set page(width:200pt,height:200pt,margin:30pt,header:figure([H],caption:[HC]))
#figure([B],caption:[BC])
#context repr(counter(figure).final())
```

`figure-kind-implementation/target-controls-oracle/receipt.json` measured
`H Figure 1: HC B Figure 2: BC (2,)`; its frozen candidate control measured
`H Figure 1: HC B Figure 1: BC (1,)`. Its exact source SHA is
`9f93fc8d20015c656d366b0be1584089f0a0d2be483b79b12ea49bb6d66233b6`.
These are semantic counter/text observations, not a required Rust tree or PDF
byte sequence, and do not by themselves establish repeated-header identity.

Current source establishes the causal chain:

- `eval/rules/set/page.rs:287-303,569` stores raw header Content in a progressive
  SetPageDelta. `stdlib/layout/page.rs:341-385` instead creates lexical PageRun.
- `entities/content/traversal.rs:319,1273,1635` explicitly leaves SetPage opaque.
  The static Figure gate uses that visitor (`realize/figure.rs:21-34`), and
  `eval/mod.rs:917-939` therefore misses a Figure solely in a SetPage field.
  Entering realization for a body Figure does not reopen that leaf.
- PageRun has explicit static children (`traversal.rs:55-76,799-840`), semantic
  layer order (`introspect/walk.rs:1047-1066`) and demand order
  (`introspect/figure_demands.rs:73-94`). SetPage remains terminal in those
  latter two walkers (`walk.rs:1238`, `figure_demands.rs:385`).
- `entities/content/page_structure.rs:188-251` wraps columns only; it preserves
  SetPage and does **not** convert it to PageRun. Moving this call cannot fix
  the marginal omission and would reopen unrelated column timing.
- `layout/set_page.rs:65-68,114-120` installs the raw marginal after closing a
  preceding material page. `layout/page_running.rs:78-106` later lays it in an
  isolated finite subframe, called by `page_commit.rs:43-44` or
  `finish.rs:115-116`. No Engine/recipe execution belongs to that renderer.
- `layout/figure.rs:33-78` requires a prepared exact typed key/Location history.
  Its subsequent unprepared ordinal branch is explicit synthetic compatibility;
  source running matter reaching that branch is not productive parity.
- Figure L0 explicitly excludes static SetPage admission and full page-running
  timing (`prompts/core/compiler/realize/figure.md:69,218`). Generic traversal's
  SetPage leaf is an existing contract, not a claim of semantic emptiness.

Current invariants also rule out a wholesale replacement:

- `layout/page_run.rs:14-99` has lexical initial/terminal boundaries and restores
  prior config; SetPage is progressive. Converting each SetPage to PageRun can
  create pages/restoration that its source did not request.
- `layout/finish.md` forbids global introspection reconstruction from frames.
  ADR-0073/0074 retain tracked introspector and per-sub-store sealing; the global
  vanilla-style frame introspector was explicitly rejected as disproportionate.
- Existing pure callback protocol is `layout/par_line_callbacks.rs:18-62`,
  `layout/convergence.rs:130-190` and `pipeline/paged.rs:1197-1265`. It can carry
  Pending work across L1/L3, but its math/par.line-only transcript does not
  interleave marginal occurrences into body action/query order by itself.
- `CounterRegistry` already separates identity from causal ordinals/actions
  (`counter_registry.rs:43-104`). Numeric Location order, sorted coordinates,
  hashes or display counters cannot stand in for insertion order.

## Fresh baseline language evidence and retained gaps

`baseline-language-binding.json` binds the complete parent-owned 15-case matrix:
`baseline-parity/receipt.json`, SHA-256
`38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`.
Its clean HEAD is the baseline above; product SHA-256 is
`257c153b1044bf8bb773b05dad1a83c6a978ba6f9ce83506e929e4da3c11b89f`,
oracle SHA-256 is `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
The receipt binds exact source hashes, UTC start/end, build receipt, both binaries
and all inputs before/after, with no changed inputs. Raw PDF extraction order is
not used as a required semantic introspection order.

The measured two-page static header has four distinct oracle query Locations,
physical pages `[1,1,2,2]` and states `[1,2,3,4]`: header states 1/3, body states
2/4. Product has only two body Locations and final state 2; both are reported on
page 1. Static footer is symmetric, with body states 1/3 and footer states 2/4.
The three-page Context header records actual pages/current counters 1/2/3 and
final 3 in oracle, while product admits none of those metadata contributions.
All five explicit layers have oracle semantic order background/header/body/
footer/foreground; product only registers body. Reusing raw Content is not proof
of an already prepared alias. These measurements refute once-per-SetPage-template
admission and confirm physical raw insertion, page ownership and interleaving.

The overwritten-before-first-material panic template remains inert in both
products. This is a required negative gate against eager template realization,
not permission to skip ordinary argument validation. A reference to a repeated
header label is rejected by oracle as `occurs multiple times`; product instead
says `does not exist`. Equal exit code 1 does not preserve that language refusal.

Preserve explicit inherited limits: native PageRun text/counters match the
contrast, but product associates its second-page Figure with page 1. Direct Func
header is rejected by both, with differing cast-message type order. HTML does not
execute the header Context and both retain BODY, but product omits oracle's
`page set rule was ignored during HTML export` warning. The proposal does not
silently accept these position/diagnostic gaps as parity or resolve unrelated
column/baseline conflicts. Prepared alias repetition remains a separate gate;
raw physical repetition does not justify changing its nominal identity contract.

## Ambient-style refutation and the effective environment

The two separately frozen sources and copied worker are bound by
`ambient-preparation.json`; original baseline15 inputs and runner are unchanged.
`../../ambient-baseline/receipt.json`, SHA-256
`6291efca219a3558ad2192f99f1208d64ad3e70b38ec96db2d417527549fcb05`,
records the same clean HEAD/product/oracle above from
`2026-10-21T14:30:18.105357+00:00` to
`2026-10-21T14:30:22.980346+00:00`, with zero source/measurement drift and both
binaries/build receipt unchanged. Both sources compile in oracle and product.

Oracle observes header size **20pt** after creation under 10pt followed by the
body's 20pt set. The inherited second page observes **30pt** after the body changes
to 30pt, with header/body pages 1/2. Product has no header metadata and associates
the second body with page 1. Freezing the full creation-time 10pt chain as the
header environment is therefore refuted. Pinned `pages/run.rs:101–103` computes
page-wide root styles before marginal layout; this source is supporting evidence,
not a requirement to reproduce upstream implementation mechanics.

Preserve two separate private facts: the operational lexical capture retains raw
template/rules/guards/file/source/context floor, while the actual selected physical
page supplies its effective insertion-style environment. Rebase only that
operational capture's ambient root; preserve explicit inner Styled 18/12pt and
existing genuinely captured local recipe deltas/prepared aliases. Do not replace
locals with ambient page values, rebase prepared aliases, collapse raw style data,
or substitute the last body's style for the page-wide scope. Further local-rule/
partial-style cases are implementation RED gates; the measured 20/30 result is
already binding and no new public getter is needed.

## Minimal alternatives and gate classification

**Same-phase private call-path correction** would be continuous ADR-0127 only
if measurements and an emission proof show how existing realization can select
all and only actually used marginal insertions before the current semantic
construction, preserving overriding, physical repetition and styles. No such
proof currently exists. Unconditionally traversing all SetPage fields or
preparing each installed delta is not that proof: a template can be overwritten
before any physical page uses it. A PageRun clone is not an equivalent carrier.

**Selected candidate:** a narrow typed
running-matter discovery/resolution channel, alongside existing callback
channels, inside the current Paged contextual transaction. It requires an exact
ADR-0127 public-contract and phase approval. Existing Figure/Ref approvals do
not authorize it. It must not become a new global frame introspector, independent
context solver or source-evaluation pass.

The candidate below is intentionally complete about its proposed public Rust
surface. No field/variant is added to Content, SetPageDelta, PageConfig,
PageRunElem, PagedDocument, TagIntrospector, Introspector, LayoutCallbackRequests
or LayoutPassOutcome. All legacy signatures and external struct literals and
exhaustive matches remain valid. Existing legacy layout wrappers retain their
current behavior. The product's Paged coordinator selects the new façade after
approval; HTML does not acquire this Paged-only admission path.

## Candidate public surface

The selected scope is the four measured physical slots: Background, Header,
Footer and Foreground. It adds a typed Paged protocol; it adds no direct Func
marginal mode and does not change HTML admission. The declarations below replace
the initial draft's incomplete `Body(Location)` trace and mutable request fields.
They are exact candidate declarations for review, not approval of additional
future interfaces. Implementation-private types and helpers remain private.

New owning module `compiler/layout/page_running_requests.rs`, owning L0
`00_nucleo/prompts/core/compiler/layout/page_running_requests.md`:

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

The public layout hub reexports these names without changing any legacy type.
There is no public packet ID, event enum, mutable plan access, request constructor,
request field setter or Default that invents a completed store. The opaque trace
carries source-plan packet addresses, not only Locations. It also owns geometry
sealed by existing per-store APIs; it is not a public Frame/PDF event catalogue.

Request/Resolution authority is an immutable private Arc-owned source-plan and
captured-template representation plus actual page-effective environment authority,
scoped to one retained candidate generation.
This is a mechanical proof of stored protocol provenance, like the current arena
owner relation; it is neither language occurrence identity nor an assertion that
functions execute equivalently. Sealing rejects duplicate/conflicting keys,
mixed candidate authorities, malformed geometry and missing root provenance.
Matching requires those same source/capture and page-effective environment
authorities plus exact physical key and finite geometry. Same source plan, key and
geometry alone cannot prove unchanged effective styles after reflow or a different
accepted page-style frontier.
No Debug/hash, generic Func/Value equality, collapsed style chain, approximate
observer or stale `(page,slot)` alone proves a match. The immutable authorities
contain complete creation capture and actual page-effective environment separately,
so matching discards no opaque style/rule slot. A changed environment invalidates
its resolution even when the physical key and geometry are unchanged. Reflow reuses this exact owned plan; replacing the source plan
invalidates the whole transcript. Every accepted resolution is consumed once per
actual physical insertion, with consumption restored on rejected trials.

New façade in existing `compiler/layout/mod.rs`, owned by its current hub L0:

```rust
pub fn layout_with_page_running<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    body: &PageRunningBody,
    introspector: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
) -> PageRunningPassOutcome;
```

Existing layout functions continue to accept ordinary Content and retain their
behavior. Only the new façade installs the private planned dispatch protocol.
Pending returns requests/trace and no incomplete public document. Its Err is a
Pending work envelope, not a SourceDiagnostic. Existing math/par.line outcomes
and request struct literals/exhaustive matches remain unchanged.

New public module `compiler::introspect::page_running`, productive file
`compiler/introspect/page_running.rs`, owning L0
`00_nucleo/prompts/core/compiler/introspect/page_running.md`:

```rust
pub struct PageRunningIndexInput<'a> {
    pub body: &'a PageRunningBody,
    pub target: EvalTarget,
    pub running: Option<(&'a PageRunningTrace, &'a SealedPageRunning)>,
}

pub fn prepare_page_running_body(
    content: &Content,
    ctx: &mut EvalContext,
) -> SourceResult<PageRunningBody>;

pub fn introspect_with_page_running(
    documents: &[PageRunningIndexInput<'_>],
    features: Features,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)>;
```

The sole index facade accepts one ordered document slice, so a single document
uses one input rather than a separate algorithm. Paged inputs require the current
matching trace/store pair; HTML inputs require None and retain source-order HTML
construction without physical-slot admission. Empty input is valid. Validate the
whole target/provenance metadata before runtime effects, preserve global Bundle
counter/state/query order and document starts, and finalize only Paged implicit
Figure demands with the explicit Features. Existing Bundle metadata rejection
still precedes this protocol.

Preparation runs under the existing shared arena/frame installed in ctx. It
reserves existing aliases and assigns any unlocated body occurrences through that
allocation domain, not a standalone Locator whose identities are invisible to the
arena. Missing frame/ownership is a fallible invariant error, never a fallback
allocator. The stored plan retains its document seed. Style/Location context and
runtime reads are not manufactured for planning. Preparation constructs the source
plan and its source-parent relations without executing language closures or
publishing counter/history/query stores. The
Content stays unmodified: no fake Located wrapper, fake Span, hidden language
field, pointer/text/hash matching or new Content variant transports packet IDs.
The new layout façade consumes this owned Content/plan pair. Feature handlers
select source children by private closed typed edge slot/index; generated render
output has an explicitly classified generated edge rather than borrowing the
next source child accidentally. Each meaningful source operation has a packet,
including no-Location label/outline facts and Context execution registrations.
Derived records sharing a Location remain distinct source operations. Root-entry,
child and post-child work retain their existing causal ordering.

The admitted plan drives the existing source-derived incremental construction in
physical packet order, keeping ADR-0071's dependency: populate the candidate before
executing helpers that read it. It does not revive generic `from_tags::from_tags`,
copy already calculated body-only projections, replay closures or scan Frames.
Source parentage is retained independently of the flattened physical work order.
Normal postprocessors and demanded folds remain owned by their current units.

New public module `compiler::realize::page_running`, productive file
`compiler/realize/page_running.rs`, owning L0
`00_nucleo/prompts/core/compiler/realize/page_running.md`:

```rust
pub fn realize_page_running(
    request: &PageRunningRequest,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<PageRunningResolution>;

pub fn bind_page_running_content(
    resolution: &PageRunningResolution,
    expanded: Content,
    ctx: &mut EvalContext,
) -> SourceResult<PageRunningResolution>;
```

Realization runs only the requested raw insertion under the existing shared
ContextOccurrenceArena retained frame installed by the coordinator. It allocates
actual descendant Locations through that arena, never page-number-derived
locations. The operational resolution root is private plan authority, not an
injected Located wrapper or queryable Context/Figure. Figure supplement callbacks
retain known styles/no Location; actual Context producers and prepared aliases
retain their own real Locations. Physical placement records actual descendants,
not a synthetic ancestor that can force the first Figure/Context root identity. Deferred ContextBlocks are admitted to the same
existing L3 Session and its retained read/warning/error transactions.

`bind_page_running_content` is the explicit crossing back from that Session: it
retains the private immutable request authority, validates the actual retained
root/source ownership and rebuilds the complete typed source plan for the expanded
Content before sealing. Any newly unlocated expanded occurrence is assigned
through the same installed shared-arena retained plan frame in ctx; no private
standalone Locator manufactures a separate identity domain. It returns a new
Resolution, preserving the old value.
It rejects wrong/missing/conflicting retained root ownership. Sealing and layout
consumption reject mixed/stale candidate authorities against the current owned
body plan; Content alone is not treated as a candidate-version proof. It does not
claim to prove arbitrary closure execution from Content alone: the existing Session
causally owns producer execution, substitution and read validation. L3 already
transports Content; the function validates this typed protocol instead of exposing
mutable authority fields or silently adding an unknown setter during implementation.
No generic observation law, public Context capability or language field changes.

## Phase/order and ownership of actual occurrences

Before: evaluate source once → realize reached body → construct body semantic
index → contextual candidate/layout → install SetPage deltas → draw raw running
Content at page commit → seal positions/pages. The semantic index does not know
that raw header contribution.

Proposed: evaluate once → realize body and capture SetPage template environment
without executing it → existing contextual candidate discovers actual physical
pages/configuration → pure layout emits typed trace/requests → retained L3 frame
realizes only requested templates through L1 → source-derived semantic builders
consume the validated interleaving and current resolved templates → demanded
counter/state folds and normal context validation → replay layout with a sealed
resolution transcript → Complete only when trace, stores, positions/pages and
existing math/par.line transcripts agree. Replaced/provisional diagnostics stay
in their existing transaction sinks and are discarded normally.

Capture is an explicit bounded recipes-owner decision for a reached SetPage.
After its parent/root admission, wrap each raw marginal Content in an existing
ShowRecipe snapshot (`textual=false`) without descendant realization. Preserve
raw styles, rules, guards, file, context-id floor and nearest proven Source through
existing recipe fields, separately from the actual page-effective style authority; do not add an entity field or change generic traversal.
A capture-only presence predicate may select the eval coordination path when no
body Figure or deferred recipe exists; it must execute no closure, kind preview,
callback or counter/state effect. Preserve existing captured recipes and prepared
Located aliases rather than recapturing their environments. The current pending
recipe implementation at `realize/recipes.rs:609–660` installs/restores stored
environments on every Result path; a physical request delegates under the bounded
operational rebase described above rather than reinstalling refuted ambient 10pt.
The recipe L0 must expressly authorize this deferred operational-template capture
before code. Capturing is not admitting the dormant template as an occurrence.

PageRun requires the same operational-template boundary while retaining its
lexical body and restore policy. Its current static fields are not allowed to
prepare one raw header then repeat that Located object as an intentional alias.
Feature-specific recipe/Table preparation, finish_occurrences and the product's
provisional index/Context discovery must capture/reserve templates without their
emitted admission; only the lexical body is visited until actual requests. Do not
clear fields in an inspection clone and publish that changed Content as a queried
Figure/body snapshot. Generic entity traversal and legacy unplanned contracts
remain distinct. Pure normalization is not automatically an effect defect; amend
only a measured/required boundary, not every generic mapper.

A request is made at the same actual selected physical slot boundary used by
`page_running::explicit_layers`, not when a SetPage delta is merely installed.
Capture rules/styles/guards/file/context floor/Source using the existing immutable
ShowRecipe representation before the ordinary Engine disappears; preserve raw
templates. Renderers execute no callbacks. Override-before-emission produces no
request for the dormant template. Omitted/Auto/None/Empty Content retain their
present distinct semantics. Nested PageRun restores ordinary progressive config
through its existing layout code, never a synthetic lexical rewrite.

The trace must include **body** causal events, including zero-paint CounterUpdate,
StateUpdate, Metadata and Context markers, and canonical Realized.original roots
plus actually emitted visual children. It must use actual retained occurrence
Locations at registration/dispatch, not text/coordinates/pointer heuristics.
`layout/mod.rs:1183-1203,1237-1285` and `layout/realized.rs` are the current causal
registration boundaries. Flow/column/subframe replay must roll trace state back
with the corresponding existing transaction, so rejected trials contribute no
phantom event. Existing deliberate aliases keep their first semantic observation.

For each accepted page, Background then Header precede its body event block;
Footer then Foreground follow it. A trace consisting only of marginal requests is invalid. Body event coverage,
original/visual membership, existing relative causal order and page ownership
must be checked before applying any order-dependent fold. CounterRegistry's
ordinals/actions, StateRegistry order, kind/query vectors and metadata order all
consume the same admitted semantic order; inserting an action after histories
were already calculated is insufficient. Parent relations for body roots remain
source-derived; marginal roots are not made children of whichever body node was
open when a page happened to close. This is a typed construction input, not a
post-render global index reconstruction.

Use existing ContextOccurrenceArena/frame/evaluate APIs with one retained frame
per actual request key inside the same compilation/document session. Keys are
local to that document; Bundle documents keep their existing global occurrence
arena and document starts. Raw template instantiation and prepared aliases follow
the final measured identity rule, not a hardcoded assumption that all physical
copies are distinct or that all are one occurrence. No template result cache,
second arena, second context fold or fabricated filtered read is introduced.
The five-attempt contextual budget and existing callback/nonconvergence/error
precedence remain binding; new unresolved work joins the same retained candidate.

Existing positions and PageStore still seal at their own per-iteration boundary
and feed the next attempt through existing APIs. The new trace transports those
sealed values. `finish`/`page_commit` do not execute language code, create a global
introspector or expose private frame markers in PagedDocument. Layout requests
are pure data; L3 only orchestrates and owns I/O. Feature realization, selector
matching, kind inference and temporal action interpretation remain L1.

## Private source-plan construction and transaction boundaries

`agents/parity/index-driver.md` provides the source anchors for the current
incremental operations. The semantic unit cannot be only a Location: Heading
root/derived TOC/Bookmark facts share one; internal Context roots bypass Start;
nonlocatable labels and Styled outline metadata can have none. The owned plan
retains closed typed root/child/post-child operations and independent source-parent
relations. Physical admission schedules those operations into the existing
incremental populate/helpers sequence before runtime folds. It does not permute
already calculated counters, copy body-only query vectors or revive a generic
post-walk global tag reconstruction.

No Content is wrapped or otherwise altered. A private planned-child entrypoint
accepts an explicit closed source edge slot/index from its owning feature adapter;
its plan cursor identifies exactly that source operation. It does not infer the
next packet from pointer/borrow address, Location, text, hash or mechanical Eq.
The actual callsite inventory distinguishes source child, generated wrapper with
a source child, generated-only output, measurement-only work and deferred replay.
A generated caption container retains its explicit source-caption child edge.
Generated-only output preserves its current semantic boundary rather than being
misidentified as the next source packet. Legacy unplanned entrypoints remain.

Before actual occurrence allocation, reserve-only inspection includes existing
Located and Context identities captured in all four SetPage templates, including
ones later overridden. The feature-owned reservation does not execute a closure,
prepare a Figure, add a semantic element/action or change generic SetPage opacity.
It prevents collision with prepared aliases without admitting a dormant template.
The owning arena/reservation clauses must name this exact rule; equality/hash/span
cannot substitute for identity reservation.

`agents/contracts/proposal-review-v0.md` binds exact checkpoint/provenance findings.
Trace state, planned cursors, consumed resolution bits and tentative page ownership
are part of the existing checkpoint/restore transaction, not an interior-mutable
callback side channel left behind on rejection. Subframes return captured packet
segments; measured/discarded trials contribute none. Deferred floats and footnote
remainders carry those segments until actual placement; their earlier subframe
page does not prove final physical page. Page commit interleaves four selected
slots with accepted body segments. It does not reconstruct facts from FrameItems.

Optimized paragraph/planning paths must preserve packet admission even when they
bypass `layout_content`. A private source-plan proof may mark a measured/generated
subtree as containing no semantic packet. Otherwise accepted optimized line/page
placement carries the relevant explicit edges. The plan does not claim Located
wrapper neutrality: current `paragraph.rs:472–483` treats Located as opaque and
`paragraph_prepare.rs:217–239` rejects it, while `layout/mod.rs:1255–1278` changes
current Location, records geometry and can forward transported root identity.
The wrapper-free plan avoids introducing those changes.

The complete exact source/L0 manifest must enumerate these adapter/optimization/
checkpoint/deferred owners before ratification. It can preserve a proven unchanged
owner as inspected rather than automatically authorizing edits to every file.
The direct inventory alone is not advertised as the full graph. Private helper
formulas and L0 clauses may then follow normal post-approval owner-first flow;
additional public signatures/defaults/phases/compatibility changes still require
an exact proposal revision.

## Implementation obligations and functional RED gates

1. Raw/reused Content physical repetition and header/body/footer order are now
   measured above. Prepared aliases still need a separate measured control.
   Physical request keys are not public semantic Locations; raw occurrence
   instantiation uses the existing arena, while prepared aliases retain actual
   identities. The final document must explicitly bind that distinction.
2. The typed owned-plan design above and independent index-driver review define
   the incremental source-side schedule. Complete its finite exact owner/edge
   inventory, including original/visual and no-Location operations, as part of the
   owner-first implementation record; the finite domains below bound that work. A marginal-only trace or
   Location-only packet list is not an implementation of this protocol.
3. Trace rollback has to cover the existing columns, subframes, floats, footnotes
   and main-flow retry boundaries, rather than assuming a successful page count
   proves semantic order. Existing sealed-store and scope contracts remain.
4. Request provenance uses the exact private owned-plan authority described
   above, not widened generic observation. Independent review must check capture
   completeness, finite geometry, source-version invalidation and actual Session
   substitution/root validation. Opaque values cannot be dropped to converge.
5. The four-slot scope is explicitly selected from measured background/header/
   body/footer/foreground order. Geometry, accessibility decoration and explicit
   Auto/None/Content distinctions remain. Direct Func marginals are still rejected.
   Inherited PageRun position, HTML warning and cast-message gaps stay separately
   reported until a measured correction is explicitly included.

These are implementation/proof obligations under the frozen declarations, not
permission to add public APIs later. A necessary public field/signature, new
default, extra phase or compatibility break beyond this package still requires
an exact ADR-0127 revision. Failing functional acceptance cannot be waived as an
implementation detail.

## Fixed owner inspection and implementation domains

`candidate-owner-domains.json`, SHA-256
`a32eca2a829a350b1826bdd48a822b0383e70ac863fab4dee2fec5f6be9a6306`,
is the finite source/L0 domain manifest. It lists every existing path with its
canonical one-to-one owner and full current source/owner SHA. Full texts are in
`candidate-owner-domain-sources.json.gz`, SHA-256
`1eb425e4ea43683e8f3b2ebfaf35238c2dab5f3ba09bb9fe49e800aa447689a0`.
This is not a repo-wide exhaustive inventory or a promise that every inspected
file changes. The actual modified-owner manifest must name only the changed
subset and distinguish inspected unchanged owners. All changes remain within
these finite domains and the exact public declarations; an unlisted necessary
owner is separately reviewed with its L0 before code, never implicit permission
for more public API.

The three new one-to-one owners are fixed:

| Productive consumer | Canonical owning L0 |
|---|---|
| `01_core/src/compiler/layout/page_running_requests.rs` | `00_nucleo/prompts/core/compiler/layout/page_running_requests.md` |
| `01_core/src/compiler/introspect/page_running.rs` | `00_nucleo/prompts/core/compiler/introspect/page_running.md` |
| `01_core/src/compiler/realize/page_running.rs` | `00_nucleo/prompts/core/compiler/realize/page_running.md` |

The existing domains are explicitly enumerated in that JSON:

- The independently inventoried 56 direct dispatch/subframe owners and 118
  production sites from `agents/parity/descendant-call-inventory.json` (cfg tests
  excluded), including generated/measurement calls so None is a deliberate
  preservation decision. `overlay-feasibility.md` names all 56 basenames.
- Capture/evaluation/identity: eval hub, realization hub/recipes/Table preparation/
  occurrence arena/Ref owner; L3 pipeline, Paged coordinator and Context Session.
- Source construction: introspection hub, entrypoints/walk/populate, Heading/
  labelled/temporal/parent/outline/bibliography/typed Figure units, their five
  runtime postprocessor units and existing registry/introspector dependencies.
- Bypasses/geometry/transactions: checkpoint; columns/flow/segmented/transitions;
  region progress/cursor; page commit/finish/PageRun/canvas/geometry/position
  sealing; paragraph planning/preparation/emission/pagination/selection and bidi;
  Sequence inline-prefix/fractional preflight and Grid measurement cache.
- Equation/math handoff: Equation and each enumerated productive math-layout unit
  listed in the manifest. Prove an accepted atomic subtree/None path or carry the
  explicit typed edge; do not infer semantic emptiness from its different renderer.
- Domain preservation: Content/traversal/page structure, PageRun, running/canvas
  data, progressive set-page parser and native page constructor.

Source-child, generated-only, generated-wrapper, measurement and deferred paths
receive explicit classification. A successful generated wrapper containing an
actual caption/Cell child cannot discard that source edge. An owner requiring no
change retains its current contracts/formulas. Shared protocol invariants belong
in proposed `prompts/_nuclei/layout/physical-page-occurrences.toml`, consumed and
fully pinned by actual owning L0s; the Nucleus never becomes a productive @prompt
target. Preserve the existing page-running geometry/accessibility Nucleus.

### Duplicate-label reference correction

This scope includes the measured ordinary Ref duplicate-label refusal through
`compiler/realize/reference_supplements.rs` and, for its existing downstream
routes, `compiler/layout/references.rs`. LabelRegistry already offers
`lookup_all`/`count` while retaining its public first-lookup compatibility; do not
change that public contract. Once actual physical memberships are complete, normal
Ref selection must reject more than one actual target with the pinned duplicate
message/origin, retaining bibliography/numbering/source precedence and strict read
replay. Do not count an intentional same-Location alias as a fresh raw insertion.
Update the owning L0 first and require a genuine independent RED; this is a
bounded continuous parity correction with no extra API. Supersede only the
normal-language first-target behavior at this boundary; retain historical F
failures unchanged in evidence, and leave unrelated bibliography conflict debt
explicit. The header's duplicate refusal cannot be greened by its old missing-label
exit code.

## Acceptance before closure

Use the independent `agents/contracts/acceptance.md` gates, frozen fresh oracle
sources and exact baseline build. Required families include original header/body
numbers/query/positions; overridden dormant panic/warning; inherited and cleared
marginals; multiple physical pages; lexical PageRun restore; manual counter/state
updates including no-paint actions; hidden parent/original-versus-visual; typed
kind/supplement/set/show-set; source/rules/styles; here/page/context reads and
retained diagnostics. Do not assume footer timing from its geometric location.

API/phase review must bind the final exact manifest and document SHA before L0
and code. Thereafter use L0-first meaningful RED→GREEN, independent tests and
integration, old source/policy preservation, full suites/strict Clippy/Rustdoc/
fmt/canonical paths/1:1 ownership/Nucleus DAG and all forward/reverse seals.
Prove actual closed decisions by attributable real-enum extension witnesses;
no allow/expect/config/alert suppression, weaker expectations, synthetic ordinal
fallback, global Frame walk or invented contextual Location. Reuse the F/G
language/counter/query/diagnostic controls and repeat the full12 gates plus fresh
candidate CLI on frozen source/binaries. Report the bounded repaired scope and
all retained debts without claiming complete page-running parity.

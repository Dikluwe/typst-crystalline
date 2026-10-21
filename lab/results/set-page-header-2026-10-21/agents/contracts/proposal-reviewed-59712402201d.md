# SetPage running matter: concrete draft for API and phase review

**Status: draft, not approved and not yet approval-ready.** No productive L0,
source or test is changed. The public surface below is a concrete candidate,
not permission to implement unspecified later interfaces. The completed baseline language matrix settles actual physical raw repetition
and ordering. Private typed construction and rollback details below still require
review before approval can bind a final exact API/owner manifest. A one-page caption match is
insufficient.

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

## Minimal alternatives and gate classification

**Same-phase private call-path correction** would be continuous ADR-0127 only
if measurements and an emission proof show how existing realization can select
all and only actually used marginal insertions before the current semantic
construction, preserving overriding, physical repetition and styles. No such
proof currently exists. Unconditionally traversing all SetPage fields or
preparing each installed delta is not that proof: a template can be overwritten
before any physical page uses it. A PageRun clone is not an equivalent carrier.

**Recommended candidate if physical admission is required:** a narrow typed
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

New owning module: `compiler/layout/page_running_requests.rs`; owning L0:
`00_nucleo/prompts/core/compiler/layout/page_running_requests.md`.
The hub reexports only these new names. Types use the existing domain imports;
`NonZeroUsize`, Content, StyleChain, Span, Rect, Location, SealedPositions,
PageStore, PagedDocument and LocatorSeed keep their existing definitions.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageRunningSlot { Header, Footer }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageRunningKey {
    pub page: NonZeroUsize,
    pub slot: PageRunningSlot,
}

#[derive(Debug, Clone)]
pub struct PageRunningRequest {
    pub key: PageRunningKey,
    pub template: Content,
    pub styles: StyleChain,
    pub span: Span,
    pub region: Rect,
}

#[derive(Debug, Clone)]
pub struct PageRunningResolution {
    pub request: PageRunningRequest,
    pub location: Location,
    pub content: Content,
}

#[derive(Debug, Clone, Copy)]
pub enum PageRunningOrderItem {
    Body(Location),
    Marginal(PageRunningKey),
}

#[derive(Debug, Clone)]
pub struct PageRunningTrace {
    pub order: Vec<PageRunningOrderItem>,
    pub positions: SealedPositions,
    pub pages: PageStore,
}

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
    pub location_start: LocatorSeed,
}
```

The opaque outcome's private representation is not a public allocation promise.
Its public Result has exactly Complete/Ok and Pending/Err states; Pending is not
a SourceDiagnostic. No incomplete PagedDocument is returned in Pending. No
Default implementation fabricates a complete store. Store construction rejects
duplicate physical keys/conflicting resolutions; consumption requires complete
request provenance and consumes an accepted resolution at most once per actual
physical insertion. Matching must preserve full captured data, not Debug/hash,
names or a shortcut that erases opaque values. An unproven correspondence remains
Pending/fail-closed; no neutral Content/history is manufactured.

New façade in `compiler/layout/mod.rs`, owned by its current hub L0:

```rust
pub fn layout_with_page_running_from<M: FontMetrics + Clone, S: ImageSizer + Clone>(
    content: &Content,
    introspector: TagIntrospector,
    metrics: M,
    sizer: S,
    font_size: f64,
    inputs: PageRunningLayoutInputs<'_>,
) -> PageRunningPassOutcome;
```

New feature module `compiler/realize/page_running.rs`, owning L0
`00_nucleo/prompts/core/compiler/realize/page_running.md`:

```rust
pub fn realize_page_running(
    request: &PageRunningRequest,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<PageRunningResolution>;
```

It requires the host's existing shared ContextOccurrenceArena retained frame to
be installed. It returns a real arena-allocated transport root Location, never a
numeric derivation from page/slot. That root is not a queryable language element.
It is positioned by actual marginal composition; it must not masquerade as the
Location of the Figure supplement callback, whose known-style/no-Location
contract remains. Context descendants retain their own real Locations.

New index façade in `compiler/introspect/page_running.rs`, owning L0
`00_nucleo/prompts/core/compiler/introspect/page_running.md`, reexported by its hub:

```rust
pub fn introspect_with_page_running(
    body: &Content,
    trace: &PageRunningTrace,
    running: &SealedPageRunning,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<TagIntrospector>;
```

This builds a candidate through existing source-derived per-store construction
and runtime folds, with a validated typed insertion order. It must not inspect
FrameItem/PDF output or build vanilla-style global introspection from pages.
It does not mutate an already published tracked index or replace sealing APIs.

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
ShowRecipe snapshot (`textual=false`) without descendant realization. Capture
styles, rules, guards, file, context-id floor and nearest proven Source through
existing recipe fields; do not add an entity field or change generic traversal.
A capture-only presence predicate may select the eval coordination path when no
body Figure or deferred recipe exists; it must execute no closure, kind preview,
callback or counter/state effect. Preserve existing captured recipes and prepared
Located aliases rather than recapturing their environments. The current pending
recipe implementation at `realize/recipes.rs:609–660` installs/restores these
stored environments on every Result path; a physical request delegates there.
The recipe L0 must expressly authorize this deferred operational-template capture
before code. Capturing is not admitting the dormant template as an occurrence.

A request is made at the same actual selected header/footer boundary used by
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

For each accepted page, Header precedes its body event block and Footer follows
it. A trace consisting only of marginal requests is invalid. Body event coverage,
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

## Engineering/semantic points still preventing approval readiness

1. Raw/reused Content physical repetition and header/body/footer order are now
   measured above. Prepared aliases still need a separate measured control.
   Physical request keys are not public semantic Locations; raw occurrence
   instantiation uses the existing arena, while prepared aliases retain actual
   identities. The final document must explicitly bind that distinction.
2. A private typed body-event construction driver must be specified from the
   current walk/populate/store code, including zero-paint actions and
   original/visual roots. It must show how each dependent per-store builder
   consumes the admitted order without replaying effects or a global frame walk.
   A marginal-only transcript does not meet this requirement. The exact private
   algorithm/owner delta must be reviewed before the public façade is approved.
3. Trace rollback has to cover the existing columns, subframes, floats, footnotes
   and main-flow retry boundaries, rather than assuming a successful page count
   proves semantic order. Existing sealed-store and scope contracts remain.
4. Strict request provenance/capability matching and retained frame integration
   need their current observer/Session owners audited. An opaque value cannot be
   silently declared equal to make a transcript converge.
5. Footer semantics, canvas and dynamic header functions are not silently
   authorized. Header/Footer is a closed physical slot discriminant so existing
   footer geometry/order cannot be accidentally reordered. Any footer realization
   expansion must be explicitly measured and included in the final scope; this
   draft does not claim it is already implemented or certified.

If these points require another public field/signature, that exact revision must
be present in the final proposal hash. Approval of a vague phase must not be
represented as approval of an unknown future API.

## Exact owner map for the candidate

The three new files above each require their one new canonical L0 before code.
Current owners requiring a bounded clause/update if selected are:

| Productive consumer | Canonical owning L0 |
|---|---|
| `01_core/src/compiler/eval/mod.rs` | `00_nucleo/prompts/core/compiler/eval/mod.md` |
| `01_core/src/compiler/realize/recipes.rs` | `00_nucleo/prompts/core/compiler/realize/recipes.md` |
| `01_core/src/compiler/realize/context_occurrences.rs` (only if existing frame contract needs clarification) | `00_nucleo/prompts/core/compiler/realize/context_occurrences.md` |
| `01_core/src/compiler/realize/mod.rs` (module declaration) | `00_nucleo/prompts/core/compiler/realize/mod.md` |
| `01_core/src/compiler/introspect.rs` (new façade/module reexport) | `00_nucleo/prompts/core/compiler/introspect.md` |
| `01_core/src/compiler/introspect/walk.rs` | `00_nucleo/prompts/core/compiler/introspect/walk.md` |
| `01_core/src/compiler/introspect/populate.rs` (event construction boundary) | `00_nucleo/prompts/core/compiler/introspect/populate.md` |
| `01_core/src/compiler/introspect/figure_demands.rs` | `00_nucleo/prompts/core/compiler/introspect/figure_demands.md` |
| `01_core/src/compiler/introspect/figure_counters.rs` (only if its current order input needs migration) | `00_nucleo/prompts/core/compiler/introspect/figure_counters.md` |
| `01_core/src/entities/counter_registry.rs` | `00_nucleo/prompts/core/entities/counter_registry.md` |
| `01_core/src/entities/state_registry.rs` | `00_nucleo/prompts/core/entities/state_registry.md` |
| `01_core/src/entities/metadata_store.rs` (only if order data requires migration) | `00_nucleo/prompts/core/entities/metadata_store.md` |
| `01_core/src/compiler/layout/mod.rs` | `00_nucleo/prompts/core/compiler/layout/mod.md` |
| `01_core/src/compiler/layout/convergence.rs` | `00_nucleo/prompts/core/compiler/layout/convergence.md` |
| `01_core/src/compiler/layout/page_running.rs` | `00_nucleo/prompts/core/compiler/layout/page_running.md` |
| `01_core/src/compiler/layout/page_commit.rs` | `00_nucleo/prompts/core/compiler/layout/page_commit.md` |
| `01_core/src/compiler/layout/finish.rs` | `00_nucleo/prompts/core/compiler/layout/finish.md` |
| `01_core/src/compiler/layout/realized.rs` | `00_nucleo/prompts/core/compiler/layout/realized.md` |
| `03_infra/src/pipeline/paged.rs` | `00_nucleo/prompts/infra/pipeline/paged.md` |
| `03_infra/src/pipeline/context_stabilization.rs` | `00_nucleo/prompts/infra/pipeline/context_stabilization.md` |
| `03_infra/src/pipeline.rs` (only if private Session ownership moves) | `00_nucleo/prompts/infra/pipeline.md` |

The final manifest must narrow conditional entries and name every actual rollback
consumer changed; unspecified extra files are not approved. Existing entity
maps stay unchanged/SetPage-opaque. `set_page` adapter geometry/column/baseline
escrows are preservation constraints, not permission to resolve them. Its L0 or
shared nucleus changes require their own exact scope before code. New shared
invariants, if necessary for more than one owner, belong to a pinned Nucleus,
not a1:N materializable prompt.

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

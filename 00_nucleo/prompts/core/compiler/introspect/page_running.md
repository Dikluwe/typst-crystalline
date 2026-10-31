# L0 — Source plans and incremental physical-page introspection
Hash do Código: 1479d645


Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/page_running.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/bundle-numbering-input.toml sha256:2e87970ea88f2802436705618856cc6aff0563d0fdbe8724d3d0221e59d494d9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/math/equation-membership.toml sha256:ca9db29aceb31c748330eb69e474c4e6a6a5c2a2d474d78dc5db52b30380464d
- 00_nucleo/prompts/_nuclei/math/external-flow-body.toml sha256:2f215ea73f3cbfdeb8c8ae01cfb8fe41814609a80bf1a98b1d936599be6155b5


## Evidence before decision

The approved package is `lab/results/set-page-header-2026-10-21/agents/core/proposal.md`, SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`; its exact additive public declarations are frozen in `lab/results/set-page-header-2026-10-21/agents/core/api-declarations-final.json`. Human approval of review `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`, recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`, ratifies this exact public/phase scope under ADR-0127. Approval provenance and implementation receipts belong under `lab/results/set-page-header-implementation-2026-10-21/`; the reviewed package remains immutable.

The clean baseline is HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`. Exact before texts/hashes, UTC and working-tree state for this ownership domain are in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Fresh current-pin bilateral receipt `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json`, SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`, records four distinct raw header/body Figure occurrences across two pages, source query order background/header/body/footer/foreground, located per-page Context metadata, distinct footer updates, and an inert overwritten panic template. Product omissions and PageRun position debt remain explicit. Ambient evidence records page-effective 20/30pt styles rather than template-creation 10pt. These are language values and refusals, not Rust equality, allocation counts or frame-order mechanics.

`01_core/src/compiler/introspect/walk.rs:391-404,785-1011` and `01_core/src/compiler/introspect/populate.rs:19-311` record both ordinary roots and auxiliary nonquery execution facts. `01_core/src/compiler/introspect/heading.rs:152-280` creates derived facts sharing a Location; `01_core/src/compiler/introspect/labelled.rs:233-345` also records facts without a locatable root. `01_core/src/compiler/introspect/parent_index.rs:14-57` derives source parents from balanced source tags. These measurements require a complete source plan and independent source-parent relation; a marginal-only or Location-only list cannot schedule every semantic fact.

## Exact public contract

Expose only the ratified `PageRunningIndexInput<'a> { body: &'a PageRunningBody, target: EvalTarget, running: Option<(&'a PageRunningTrace, &'a SealedPageRunning)> }`, `prepare_page_running_body(content: &Content, ctx: &mut EvalContext) -> SourceResult<PageRunningBody>` and `introspect_with_page_running(documents: &[PageRunningIndexInput<'_>], features: Features, ctx: &mut EvalContext, engine: &mut Engine<'_>) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)>`. No additional public constructor, packet ID, event enum, getter/setter, entity field, trait method or legacy signature change is authorized.

## Source plan and authority

Preserve the original Content unchanged: no Located/Styled/metadata marker is inserted for packet addressing, and semantic fields are not cleared in a clone later exposed as a language snapshot. Own a complete closed, static typed Content partition and finite owner-typed child-edge/index plan. Existing original-versus-visual and selected Table/Cell/Outline/Ref rules remain authoritative. Source roots, no-Location facts, root/derived facts at the same Location, generated Context execution roots, labels, metadata, state/counter actions, bibliography and deferred derived projections have explicit source operations. Store complete chain/language/span/label/field snapshots and source-parent provenance at their causal source boundary.

Packet addresses are transient private plan identities, never Locations or language objects. The immutable Arc-owned plan is protocol authority; exact shared authority proves stored provenance only, not arbitrary callback equivalence. Retain complete immutable environment data separately from template creation capture. Environment IDs identify a declared source/capture/page-effective frontier in that plan; never deduplicate or prove styles by generic Value/Func equality, Debug/hash, collapsed chains or text. A changed effective environment invalidates its old resolution even when page/slot/geometry is unchanged.

Reserve existing Located and Context identities in all operational SetPage/PageRun templates, including overwritten templates, before assigning any raw occurrence. Reservation admits no element, action, producer or callback. Allocate every newly needed actual occurrence through the installed existing ContextOccurrenceArena retained frame and its allocator; obtain a Location without wrapping the publicly retained original Content. Preserve intentional Located aliases and first observation; repeated raw physical insertions have distinct retained occurrences. No second arena, standalone private Locator domain, numeric/page-derived Location or hash identity.

Operational templates remain deferred until actual requests. The ordinary PageRun body is planned; PageRun running fields and opaque SetPage templates are reserved/captured rather than indexed as emitted children. Existing entity traversal and legacy unplanned entrypoints retain their own contracts. Source evaluation, kind inference and callbacks do not run in preparation of this plan.

## Incremental semantic construction

Validate exact body/source/environment authority, complete admitted packet coverage, correct parent/child edge, selected four-slot request provenance, physical ownership, deliberate aliases and rollback state before Complete. Unknown, stale, foreign, duplicated/conflicting or missing admission never becomes an empty successful index. The trace is typed data, not FrameItem/PDF inspection or sorted coordinates.

Schedule source operations into the existing incremental population/helpers before any order-sensitive history/query projection is calculated. Do not permute counters after copying body-only kind/label/metadata/bibliography/state vectors. Install complete causal order independently of numeric Location identity, then populate root snapshots and actions in that order. Derived heading/label/outline facts read the correctly ordered candidate at their specified root/post-child boundary; retain source operands instead of stale preliminary numbers. Source parent relations remain independent from flattened physical operation order. Marginals are separate roots, not children of whichever body node spans their page.

For each accepted physical page use background → header → body → footer → foreground. Nonpaint CounterUpdate/StateUpdate/Metadata/Context facts and existing Realized original-only registrations are not omitted. Actual selected running plans join the body at the admitted slot; dormant templates contribute nothing. Preserve shared bundle arena/document starts and actual per-document Paged/HTML targets. HTML does not gain paged running-template admission.

Coordinate existing bibliography conversion, outline construction, Counter/State display and Equation/Heading callbacks, typed Figure demand and Page current/final snapshots only after the candidate's source-derived stores have the complete admitted causal order. Preserve demanded-only State behavior, callback diagnostics, strict contextual read replay and the existing five-attempt transaction. No source reevaluation, alternate solver, result cache, fabricated fold read or global observation weakening. Return a fresh private candidate and publish only after existing per-store sealing agrees with the accepted physical generation.

## Private cooperation and acceptance

Private compiler-descendant types/helpers may connect the approved layout/realize protocol by parent descent; do not add `pub(crate)` convenience bridges or reverse entity-to-compiler imports. The finite dispatch catalogue `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json` defines inspected source, generated-wrapper, generated-only, measurement and deferred paths. Uncertain rich/cached paths require their owning L0 proof before implementation Complete; this is not permission for unknown public APIs. Preserve immutable Content morphology and legacy None/unplanned rendering. Follow ADR-0109 static feature delegation and ADR-0073/0074 separate store sealing; do not resurrect global from_tags/Frame introspection. The source-planned physical schedule is the specifically approved extension of the intr-accessible incremental construction boundary.

Require actual functional RED before productive repair and independent source-language tests: repeated headers/footers and raw reuse, four-slot counter/query order, page-effective styles/local overrides, actual Context here/get/final locations, overwritten inactive templates, no-paint updates, repeated-label refusal and source origin, bundle starts/HTML isolation, original/visual snapshots, source parents across pages, packet authority/coverage/edge rejection, retained retries and aliases, measurement/deferred rollback. Preserve all prior tests and F/G evidence/policies. New Content alternatives must force owner review through attributable E0004 witnesses, without catchalls, suppressions or moved neutral fallbacks. Validate all existing suites/build/strict lint/Clippy/Rustdoc/fmt/forward and reverse lineage/Nucleus DAG. Retained unrelated geometry, full caption/HTML and diagnostic gaps remain separate measured obligations; no complete page-running parity claim.

## Provisional discovery boundary

The existing ordinary source-realization owner may privately request a source-order body-only provisional index after its canonical occurrence finishing, when operational templates require physical discovery. This is not the public Paged index facade and is never a completed physical result. Reuse the same typed source plan/incremental facts, preserve all already canonical body identities and exclude operational templates without modifying the retained Content. This private path assigns no new source identity, executes no callback or demanded runtime fold, and manufactures no position/page/history. A missed unlocated productive body root is an invariant failure for the ordinary realization owner to correct. The public preparation facade still requires the installed shared retained frame and does not publish stores in ctx; the public Paged index still requires a matching actual trace/store pair. Existing retained read validation decides provisional observations against the eventual complete candidate.


## Retained document scope and derived identities

### Evidence before decision

`01_core/src/compiler/introspect/entrypoints.rs:159-172,201` returns the Locator checkpoint preceding each document's Page counter boundary; `03_infra/src/pipeline/context_stabilization.rs:1962-1994` deliberately reconstructs that boundary as `Locator::from_seed(start).next()`. A preparation-time high-water checkpoint is not such a boundary when the body's roots already carry Locations, and the first physical root can instead be a header Figure. The implementation diagnostic audit is recorded under `lab/results/set-page-header-implementation-2026-10-21/agents/index/`; this is an internal causal correction within the approved shared document-start contract, not a new language identity API.

### Contract

Public document-body preparation reserves one retained document Page-scope occurrence through the installed shared arena/frame, preserving its original allocation seed alongside its Location. Contribution and provisional source plans reserve no document boundary. Return exactly that stored document seed; its existing Locator replay reconstructs exactly the stored Page boundary. Observe the boundary before that document's source operations and apply the existing Page Set(1) action before those operations. It is an internal scope boundary, not a query element or fabricated position. The root's first physical element never substitutes for this boundary. The plan retains the arena's nominal private capability; actual preparation and source/contribution binding reject a foreign installed owner before effects. Retrying the retained frame preserves seed, Location and owner together.

Derived OutlineEntry identities are allocated only after all admitted heading prerequisites exist, through the same arena and a retained index frame installed by the existing caller. The feature owner receives a private allocator callback; legacy Locator-based entrypoints remain unchanged. Do not synthesize an independent allocation domain or infer identity from numeric/page order.


## Reservation at suppressed carrier roots

`01_core/src/entities/content/traversal.rs:436-437` deliberately preserves public bottom-up Located-root callback suppression. Consequently a public map callback at Located(SetPage) observes the wrapper but not the immediate opaque SetPage root or its deferred template fields. The physical source preparer's private reservation traversal explicitly recurses into that body root, observing its wrapper Location first. It reserves both original and visual aliases of Realized without admitting either tree; visits captured recipe floors and Context IDs; and recursively reserves all four opaque SetPage templates. Use private pre-order static enumeration with explicit carrier decisions, never alter entity traversal's callback contract or interpret reservation as semantic admission. A dormant template's Location and Context floor must remain reserved even when its SetPage is immediately inside Located.


## Conditional bibliographic reference sources

The existing bibliography owner converts known Ref roots to Cite before ordinary population (`01_core/src/compiler/introspect/bibliography.rs:17-26,48-55`), and `01_core/src/compiler/layout/references.rs:57-66` renders that same selection by delegating to Cite. Cite's selected supplement is an actual rich child; an ordinary nonbibliographic Ref supplement remains terminal. The source plan must preserve this established selection without changing original Content or eagerly admitting Ref children.

Reserve a private prospective Citation identity at an actual non-original Ref source, reusing a forced Located alias when present. Keep it auxiliary, never make ordinary Ref publicly locatable. Plan its CitationSupplement subtree with an explicit owning-Ref conditional provenance. Only bibliography keys from actually admitted Bibliography sources activate that projection and subtree; dormant SetPage/PageRun templates and inactive Ref supplements supply no keys or effects. Resolve bundle-global keys before population, preserving the existing known-key conversion. The selected Ref root populates the existing Cite snapshot/payload at its reserved auxiliary identity and does not create a Figure numeric-reference demand. The layout owner records that selected Citation's actual geometry and routes the exact conditional child. Validate active conditional coverage before Complete and reject inactive phantom admissions; Pending may source-splice a newly selected, not-yet-painted child exactly as for a newly bound running contribution. Preserve aliases, citation/back-reference order, source parents and both target boundaries; no global Ref traversal or source reevaluation.


## Deferred source anchors and physical positions

Current-pin `lab/results/set-page-header-implementation-2026-10-21/oracle-crosspage` records `note-causes-next-body-page` and `note-child-on-later-page`: query order remains H1(1,page1), FIRST(2,page1), NOTE(3,page2), H2(4,page2), SECOND(5,page2). The note's eventual position does not move its source insertion after the next page's header. This refutes ordering semantic packets by their final geometric page. Admission.page is the owning source-emission page/slot anchor; deferred subtree source events retain that anchor through frame moves. Sealed Position.page independently records the actual accepted physical paint page. The index orders source anchors and slot admissions, never assigns an admission page from coordinates or sealed geometry. Ordinary running slot keys still refer to their actual selected physical page. Preserve the measured source-relative note/header ordering and both actual page fields without geometry-derived semantic reconstruction.


## Original-view lineage

`01_core/src/compiler/introspect/walk.rs:408-413,596-624,1025-1032` preserves a subtree-wide original-walk depth independently of its root-only decision. Every unprepared Figure under an original subtree is pruned, including beneath Sequence/transparent carriers; a prepared original Figure, Table or Block root is registered without its children. The planned builder retains an independent original-lineage bit, marks genuinely pruned unprepared Figures explicitly, and allocates/adopts no source facts for those pruned subtrees. Descending ordinary children must not reset this lineage. Prospective bibliographic conversion, bibliography-key collection and actual Figure/Ref formatting demands use the visual view only, matching their established separate walkers; canonical original root snapshots remain queryable under the existing rules. A pruned original is neither an operational template nor an unknown leaf. Complete coverage excludes its explicitly pruned operations, while real visual children retain all required phases.


## Page-wide environment frontier

Measured source evidence precedes this qualification: `01_core/src/compiler/eval/markup_stream.rs:305` marks the lexical set-rule flow tail with `eval.flow.scope=true`; native partial text styling lacks this provenance. The actual unchanged `physical_page_partial_body_styles_do_not_replace_page_environment` regression fails in `lab/results/set-page-header-implementation-2026-10-21/focused-diagnostic-7/tests.log`: the selected header receives 11pt rather than the surrounding page-wide 20pt. The four source-level measurements are retained by the layout protocol owner. This is a continuous internal parity correction within the approved physical phase, not a new style/default capability.

The source plan shall classify page-wide environment frontiers from its owned source-edge ancestry. Root and transparent Source/Located/ShowRecipe/Realized visual carriers may lead to ordinary top-level Sequence parts. StyledBody may continue that frontier only for an actual lexical flow-scope marker. PageRunBody is a physical body frontier; ColumnsBody continues only for the generated page_columns wrapper. All feature/local bodies and original/operational/conditional lineages stop this route. This nominal decision does not compare StyleChain, values, contents, functions or pointers. Root environment provision remains available even when no further body frontier exists. Layout observes only this classifier and the already stored exact environment; a singleton global set-rule body is eligible without requiring a Sequence wrapper.

Acceptance preserves the existing four-case language regression (partial inline, local Block set, partial inline between root text, and full-root set), page-effective inherited20/30, explicit inner override, PageRun/page-columns and unchanged Request.matches authority. The private classifier and its source-tree tests do not authorize a public field or signature. Existing invalid-frame controls remain unchanged; validate an actual allocation owner before reserving any source identities.


### Prepared Table source coverage

The unchanged language `physical_page_cells_and_columns_preserve_source_routes` actually passes its Grid case and fails the prepared Table case with an unverified source plan; the exact evaluated source and frozen generation log are in `lab/results/set-page-header-implementation-2026-10-21/runtime-observation-1/physical_page_cells_and_columns_preserve_source_routes.log`. The raw synthetic Grid/Table phase control independently passes in `lab/results/set-page-header-implementation-2026-10-21/grid-route-observation-2/receipt.json`. These measurements distinguish raw routing from prepared TableCell/Realized transport; neither establishes a convenient exception for prepared content.

Acceptance shall include the actual evaluated caption-bearing Table source, its complete required root/post phases and unique packet admissions, preserving both Figure aliases/labels and selected caption/body descendants. Missing and duplicate phases must be attributable to the precise prepared source packet before changing the owning router. The language source, expectations and strict verification remain unchanged. A pure in-memory World test may execute the existing eval/runtime-index entrypoints to construct that actual prepared boundary; it does not add a phase or authorize the legacy renderer to reconstruct physical source membership.


The prepared isolated Table control passes in diagnostic round8 while the unchanged full-source Table program still refuses source verification; the latter includes a captured SetPage header and a retained body Context. The special template-aware occurrence finishing branch is therefore part of the source-bound acceptance boundary. Preserve every required body packet/phase exactly once in the first Pending physical transcript even before the requested header has been resolved; operational template packets remain dormant. Diagnose this actual complete evaluated source rather than guessing from an isolated prepared Table or weakening whole-document verification.


## Transform source descendant preservation

Source measurement: `01_core/src/compiler/introspect/walk.rs:1301` descends Transform.body; `01_core/src/compiler/layout/transform.rs:48,526` already obtains the declared TransformBody cursor for actual paint, keeping measurements unplanned. The source-plan edge vocabulary declares that edge, but the current builder still places Transform in its terminal complement. These are precise mechanical source/transport observations, not evidence that a transform changes source membership. The new source-plan contract already requires preserving actual source descendants through these wrappers; omitting this entry is an implementation gap, not a new public capability or phase.

A Transform must own its exact TransformBody child address and preserve the existing source parent, styles, aliases and descendant semantic facts. The source-tree change does not alter any transform operation, geometry, default, measurement or cache. Acceptance includes an identity transform containing a real Heading: the Heading remains one queryable source member with the same body, and actual physical paint covers its required phases once and supports incremental construction. Preserve the ordinary unplanned transform behavior and all existing source-plan controls. An observed difference with vanilla in transform language behavior would require independent bilateral evidence, rather than treating Rust traversal mechanics as parity by themselves.


## OutlineEntry original snapshot and actual body

Bilateral source-bound measurements distinguish the cases before changing this policy. `lab/results/set-page-header-implementation-2026-10-21/route-probes11/receipt.json` and `lab/results/set-page-header-implementation-2026-10-21/route-probes-baseline/receipt.json` record prepared body-show Metadata/Figure programs accepted by both the verified baseline and pinned oracle while the new channel rejects their source plan. Actual-outline Metadata/Figure programs separately fail new-channel stabilization. Raw body-show and default-entry acceptance/diagnostic gaps also exist at baseline and remain separate debts. Global metadata observations are meaningful; empty `<probe>` queries are not additional observer proof.

An OutlineEntry reflected in an own-show `Realized.original` retains its exact element/borrowed-body template and root identity. That immutable original is not an actual second child emission: a raw descendant in that stored template must not acquire an allocation or a required semantic child phase solely because the original is reflected. The actual realized replacement remains an independent source contribution with its real allocated identities. A canonical or actual visual OutlineEntry may still emit its selected borrowed body and retains its typed body route; this original-only distinction does not make that language content terminal. No structural equality, fabricated Location, alias equivalence or fake leaf is authorized.

Acceptance includes a real-arena original Entry snapshot with a raw Metadata template and an independently located actual replacement. Allocation-free provisional indexing must preserve the Entry/root template, admit the actual Metadata once and never attempt to allocate a stored original child. The unchanged prepared body-show language programs must compile again; the existing borrowed-child count difference with the pinned oracle is not certified away. Actual-outline stabilization requires an independent causal diagnosis, not an extra solver or this original-only test as a proxy.


## Actual Outline title versus the legacy fallback slot

Source evidence before decision: `01_core/src/compiler/introspect/page_running.rs:556`
reserves an auxiliary title for every Outline, and its post-child operation at
line1803 adds that slot to Heading membership. The baseline
`01_core/src/compiler/introspect/walk.rs:1433` used that compatibility projection
because its published index preceded Outline realization. The new physical index
instead includes actual realized children. `01_core/src/compiler/stdlib/foundations/query.rs:69`
returns bare Location when a compatibility slot has no source snapshot; the actual
infra diagnostic13 observes this as field access on Location. This is mechanical
source evidence, not a ratified new language object.

An Outline without a realized body may retain its existing fallback title slot.
Once its actual realized body contains its title Heading, that actual Heading is
the sole title occurrence; no extra compatibility title slot may be allocated or
added to Heading membership. Preserve its actual snapshot, own Location and
outlined=false source style, and preserve the separate source Heading. No global
Heading filtering, query suppression, borrowed-body removal or fake Location is
allowed. Acceptance constructs one realized title plus one source Heading in a
real retained arena and requires exactly two Heading locations with complete
actual snapshots, not three memberships or an unreflectable auxiliary.


## Actual Outline entries versus compatibility synthesis

Source measurement before decision: the public source-plan facade at
`01_core/src/compiler/introspect/page_running.rs:2502` invokes
`01_core/src/compiler/introspect/outline_entries.rs:16` after admitting actual
source packets. That compatibility algorithm iterates all retained Outline
records without distinguishing an already realized body; it can add a second
entry for an actual entry already emitted by that body. The legacy ordinary
published index precedes the realization phase, while this candidate includes its
actual products. These mechanical paths must not define duplicate language
objects.

A realized Outline's admitted actual entries are authoritative. Compatibility
synthesis applies only when the actual realized body is absent, and preserves
all existing ordering/depth/prefix/fill/temporal-location rules for that case.
Do not discard actual Entry bodies, suppress selectors, deduplicate unrelated
locations by content or merge genuine repeated entries. Acceptance uses the real
public target-aware index facade under a retained arena/Engine: an explicitly
untitled realized Outline containing one actual Entry and a separate source
Heading has one Entry snapshot, not an additional synthetic compatibility Entry.


## Plan-local payload ownership of transparent carriers

The nominal-ancestry diagnostic at
`lab/results/set-page-header-implementation-2026-10-21/outline-path-17/receipt.json`
rejects canonical Source wrappers around actual Outline and Heading roots before
SourceBody descent. The source at
`01_core/src/compiler/introspect/extract_payload.rs:20` intentionally unwraps both
Source and Located for its generic consumers. Consulting that transparent helper
as if it classified a plan packet's own payload assigns semantic ownership to a
carrier and can reject a valid Source(Located(actual)) during allocation-free
provisional indexing. A Located carrier can also publish the child too early,
preventing the real Heading packet's own begin/post-child projections.

Use a private plan-local own-payload decision that excludes Source and Located
carriers while retaining the generic extractor unchanged. Apply that decision
consistently to builder allocation, semantic parent ownership, required phases,
semantic-root lookup and root application. Keep the Located node's real alias
Location, allocator reservation and forced child identity; Source keeps its span
and transparent typed child. The actual child alone publishes payload, query
snapshot and Heading projections. A raw unlocated actual child still rejects
allocation-free indexing; no Source allocation, subtree suppression, fabricated
Location or generic transparency change is allowed. Bibliography's existing
closed actual-Ref-only projection remains unchanged.

Acceptance wraps a real Located Heading and its real Located Metadata child in
Source, then allocation-free indexing must retain exactly those two identities,
one Heading/TOC entry, the actual child snapshot and Metadata parent relation.
Unlocated Source-wrapped Heading/Metadata still reject. Preserve a sourced Located
known bibliography Ref and selected rich supplement without a carrier-owned Cite.


## Private source-builder structure and explicit allocation decisions

The actual strict Clippy receipt
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/validation-remainder/clippy.log`
reports `too_many_arguments` on `PlanBuilder::build` and `type_complexity` on
its six-field child tuple. The complete SARIF at
`lab/results/set-page-deferred-corrections-2026-10-21/candidate-2/lint-candidate/after.sarif.gz`
reports V17 on the `None` allocation arm combining non-template role and
allocation permission. Exact before source/owner texts, full SHA-256 hashes,
HEAD, UTC and dirty-tree stat are retained in
`lab/results/set-page-report-blockers-2026-10-21/agents/index/before.json.gz`.
These are mechanical maintenance findings, not evidence of a language defect.

Represent the inherited node-building inputs and pending child descriptions
with distinct private nominal carriers and named fields. Keep every source edge,
child order, environment, source parent, label, forced Location, role and span
unchanged. Preserve the exhaustive static Content partition and the existing
original/conditional lineage save-and-restore boundaries. No private carrier
authorizes a public API or a new identity domain.

Make the location decision explicit: a forced Location wins; an unforced
operational template remains unallocated; every other unforced source requires
allocation permission and otherwise returns the existing invalid-plan error.
Invoke the existing allocator exactly once only on the permitted branch. Do not
hide a conjunction in a helper, suppress diagnostics, widen authority, skip
coverage or change a phase. This is ADR-0127 continuous internal flow.

Acceptance retains all existing source-plan and physical-page controls with
unchanged expectations, and requires the actual strict Clippy/V17 findings to
clear. Root integration validates full tests, forward/reverse lineage, Hash,
strict rustdoc and the complete unsuppressed Tekt stream under the existing
resource limits. No new language-parity or performance claim follows.

The source-index consumer may access the same transcript facts through a
compiler-private boxed `PageRunningTraceData` carrier owned by the layout
protocol. This representation change keeps `PageRunningTrace` public and opaque
with exactly the same public result and field types. Read the original nominal
source authority, completeness, requests, admissions, positions and page stores
from that carrier without changing their order, ownership, clone semantics or
verification. Do not expose a public Deref or accept a transcript by numeric,
structural or allocation-address similarity in place of shared source authority.


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

The existing conditional-source completion and Label target search also inspect
the source plan (`compiler/introspect/page_running.rs:1538` and `:1806`). They must
not reactivate a privately excluded body: apply the validated activity selection
to completion expansions and conditional-effect requirements, preserving every
active conditional obligation. Resolve a Label's payload target only through
Root packets already observed in this candidate; transparent no-fact wrappers
may still be traversed. Track actual source-plan authority and packet membership,
never Location equality, source-value equality or painted-frame contents. This
consumes the private proof owned by
`00_nucleo/prompts/core/compiler/layout/page_running_requests.md`; it does not own
proof construction or layout dispatch. Source-only provisional/HTML behavior
remains unchanged, and final positions continue to project only selected events.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

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

The existing source builder initializes the new private numbering input absent.
SourcePlan fields, document boundary, source parents, root IDs and allocation
authority remain identical. The input is wrapper-local layout data, never a new
source packet, source plan field, fabricated position or first-element boundary.

Require functional RED→GREEN, original integrated controls, complete technical
gates and independent PDF/SVG/word/page/0.001pt validation with source provenance.
Finite tests do not certify general parity, geometry, performance or JPEG.

## One membership per actual Entry Location

Measured physical4 identity query reports three entries; the source-bound
outline-entry-alias-before receipt confirms all three carry the SAME Location.
Construction::apply already records a payload root only when absent from
intr.elements, but OutlineEntry's post-child special registration appends its
location unconditionally. Retain every raw phase/admission/completeness check
and original/visual snapshot; register OutlineEntry kind membership only once
for the same actual Location. Distinct Locations remain distinct even with
identical Content. This is a semantic membership correction after complete raw
validation, never suppression of raw events, content dedup or a fake entry.
The original source/template and actual-body contracts above remain mandatory.

Functional RED receipt: entry-text-weight-red.json/log and source archive retain
exact inputs/HEAD/dirty list/UTC and --no-fail-fast execution. Core records one
actual suffix at81.391pt instead of the regional right edge; four actual Entry
query tests and one real static-font emitter fail in infra. The hidden Entry
control and all prior readiness tests remain green. Preserve those logs rather
than treating later tests as historical RED.


## Measured Equation membership and original snapshots (2026-10-29)

### Measurement before decision and diagnosis

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/equation-kind-extra9/receipt.json`
records native nested block counter(1,) versus product(2,), and native one Equation
versus three product memberships. Fresh immutable product10, SHA
681cea51ff5325b36fe1845f822c059ef8971a19b56f70bac4b921d8887937eb,
is bound to full source/build/UTC/HEAD/dirty stat by math-private-build10.json
and its source archive. `equation-membership-label-before11/receipt.json`
measures native missing-label errors for inner references while product compiles;
Flow Box and outer references succeed in both. Native nested styled/metadata
cases have one Equation, product two.
`equation-membership-original-before11/receipt.json` independently measures
identity/show-set direct Metadata=M once in both; nested variants add an unwanted
product Equation. Hiding or replacing the outer Equation yields metadata=[]
natively but [M] in product, proving admission of stored original children.
Pinned oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17
and binary SHA22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34
are checked by these receipts. Source, argv, UTC and exact state are retained.

The existing generic contracts omitted Math-specific root eligibility and did
not include Equation in the bounded original-only shallow policy. This is an L0
coverage gap materialized in code, not merely hash drift. General semantic promises
do not substitute for an explicit productive rule. Amend L0 first. Acceptance
concerns language queries, counters, fields and errors, not Rust traversal identity.
Before texts/hashes/HEAD/dirty stat/UTC are archived in membership-before11.json.gz.

### Internal decision and preserved approvals

Consume equation-membership and the measured external-flow-body nucleus. Keep
all four readiness approvals exactly as ratified: no public surface change or
phase move. This is ADR-0127 continuous language parity within the existing index
construction boundary; compatibility there means binary/API. Private inherited
Math provenance is not Figure-kind inference, evaluation or callback execution.
No public field, trait/signature, arena, solver, default, retry or budget change.

Preserve canonical raw body fields, original/visual identity and active source
parents. Exclude only the nested Equation semantic root and its own label/counter;
retain every actual descendant fact and all raw source phases/authority checks.
Stored original Equation body is a snapshot, not an actual second child source;
its fields stay intact. Root-only original and actual visual policies apply only
to Equation in addition to the already documented Table/Block/Figure cases.
Do not generalize to arbitrary originals, filter queries after population,
deduplicate by value/text/hash, clear snapshots or suppress unknown packets.

### Acceptance and remaining scope

Functional RED then GREEN in legacy and typed-plan owners: nested root/counter,
inline versus actual Flow boundaries, untouched canonical body, ignored inner
label with no unreferencable fallback, preserved outer/Flow labels, hidden
original Metadata absent and actual visual Metadata once at its actual Location.
Retain strict missing/duplicate/foreign raw-source rejection and opaque math.
Native-label diagnostic remains missing label; do not silently retarget it.
All old tests/expectations stay. V15/V26 before V5/reverse seals; full warning-
denying lint, all12 technical gates, actual fresh release, broad PDF/SVG/0.001pt
and independent paint preservation, historical1720 and both original JPEG
campaigns remain required for final qualification. No tolerance/flag/recipe hacks.
Nested glyph painting and deferred Context own-show are separate remaining debts;
this index correction does not certify full math or product parity.

### Exclusive typed source-plan transport

Retain inherited Math in compiler-private PlanBuildContext/PlanNode data, derived
from actual typed parent boundaries through the shared exhaustive decision.
Initialize document/contribution roots in Flow. This is provenance propagation,
not source evaluation or semantic Figure-kind inference during preparation.
Keep private root Locations, allocation policy and required actual raw phases
unchanged. Only semantic payload application, semantic-parent selection and
label-target lookup exclude a Math-nested Equation. An OriginalOnly Equation
has no planned body child, while its full raw Content remains stored unchanged;
its independent visual Equation still owns the actual EquationBody edge.
Labels of nested canonical Equation roots contribute no association or fallback;
track the exact typed transparent target, not adjacent flattened tags.


## Issue 8954 — approved integration

### Evidence before implementation

`lab/results/construction/diagnostics/issue-8954-2026-10-31/implementation-before.json.gz`
retains this exact owner/source, UTC, HEAD and dirty diffstat/status; measurements.json.gz
contains the fresh combined native probes and actual glyph baselines. The original
case is rejected, while distinct levels reorder 0,A,B,1 into A,B,0,1. Ordered
replacement/repeat controls establish group-level language behavior independently
of Cell kind or upstream panic mechanics. Human ratification is approval.json
in that directory, patch SHA 18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20.

### Exclusive amendment

Unprepared items=Some is the exclusive raw Table child stream, routed by existing indexed TableRawChild edges. Resolved tables still select row-major resolved cells only. Do not visit parallel legacy fields or create new Location/source facts. Preserve caption, source parents, guards and all existing aliases.

L0 and restricted reseal precede RED/code. Preserve prior ratifications, purity and
static dispatch. Validate original source order, adjacent/replaced/nonrepeating
groups, effects and no hidden admission, spans/conflicts/padding, finite region
progress, legacy controls, native PDF/SVG/HTML, proportional suites/build and full
forward/reverse lineage/lint. All compilation/tests use only the exclusive RAM
target /dev/shm/typst-crystalline-8954-target; no fallback to disk. Rich interactions
remain bounded; no global parity or PDF/UA claim. Additional public/phase changes
require a new gate; the approved fields are not inferred from diagnostics.

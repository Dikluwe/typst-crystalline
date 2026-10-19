# Figure counter design: demand without a shared cache

Status: concrete bounded counter contract for approval and implementation, not active L0. No productive source or L0 was edited by this audit. The final combined proposal includes the carrier/supplement/preview owners reviewed separately. The decisions below fix the counter scope: Paged implicit consumers only, all-target explicit reads, an additive typed-target bundle facade, and an explicit break in the obsolete runtime string-index projection.

## Evidence before decision

The six bilateral cases in `counter-probes.json` were executed at the recorded UTC/HEAD, with exact fixture, runner, binary hashes and argv. The oracle is commit bd5ebec285505889e8ff1d4f68804ab147a699f3, tree cac836c07e4dec2a89219370428292393a3216b5, binary SHA-256 9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9. Product bytes are identified in that receipt; this audit did not rebuild them. They are runtime observations, not inferred intent.

| Case | Oracle | Current product |
|---|---|---|
| Uncaptioned then captioned image-kind Figure | typed counter 2; global Figure 2 | 0; 1 |
| Uncaptioned Figure plus explicit final read | 1; 1 | 0; 0 |
| Numbering:none then captioned Figure | 1; 1 | 0; 1 |
| Show-hidden raw-kind Figure then visible image-kind Figure | raw 1; image 1; global 2 | 0; 0; 1 |
| Uncaptioned, Set(7), caption b, Func(*2), caption c | b=8; c=17; final=17 | 7; 14; 14 |
| Uncaptioned labelled Figure referenced by @f | successful; counters 1; 1 | successful; counters 0; 0 |

Pinned upstream `model/figure.rs:442–447` advances whenever numbering is present, independent of caption. Earlier proposal text making caption part of automatic count eligibility is refuted. The older panic controls still establish absence of demand for no-caption/numbering:none/show-hidden alone; they do not prove absence of automatic action.

Current productive evidence:

- `compiler/introspect/populate.rs:37,92–127`: one canonical automatic action, but eligibility currently includes caption; auxiliary string namespace snapshots and labels diverge from filtered reads.
- `compiler/introspect/from_tags/counter_updates.rs:13–79`: complete demanded fold, language key equality, query-selected automatic Locations, recorded causal order, Context::none callbacks.
- `compiler/stdlib/counter/filtered_reads.rs:78–107`: each explicit demand folds then records CounterFold; replay uses the same operations.
- `compiler/eval/context_reads.rs:247–276`: replay creates fresh context and discarded sink.
- `compiler/layout/mod.rs:423,1232–1245,1375`: a caption already has current_location. An ordinal progress map is unnecessary.
- `compiler/layout/figure.rs:36–49`: increments progress even without caption and treats counter history event index as Figure index.
- `compiler/introspect/walk.rs:530–564`: Realized original and visual are both indexed, so scanning Figure tags for caption demand incorrectly demands hidden originals.
- `infra/pipeline/context_stabilization.rs:572–599,782`: candidate runtime warnings are local phase_warnings; rejected attempts discard them; accepted phase publishes them. Producer warnings remain separate retained contributions.

## Chosen design

No shared cache, no generation identity hash, no callback-once flag and no new L3 transaction abstraction. Explicit filtered reads and validation replays keep executing their own complete fold in their own transaction. A candidate-specific runtime finalizer computes only the histories required by actual retained Figure caption/reference consumers. Every successful history derives from the same canonical action log and selector evaluator; sharing a result object is unnecessary for equality of values/errors.

Automatic actions and demand have separate predicates:

1. Every indexed numbered Figure contributes exactly one automatic Step, including a captionless Figure and an original retained for query after show removes its visual body. Numbering:none contributes none. Func numbering is present for counting, even while its formatting remains a separately stated debt if outside the combined package.
2. Only an actually present visual numbered caption or ordinary numeric Figure reference creates implicit demand. Opaque callbacks are not executed by discovery; a no-caption Figure alone, an eliminated visual, numbering:none, Page reference, or bibliography reference creates no Figure counter demand.
3. An explicit counter read independently creates demand under the existing reader contract, including hidden or captionless occurrences selected by its query.

A single successful finalization replaces the complete private Figure snapshot list; never append earlier-generation entries. If any demanded fold fails, publish no partial snapshot and propagate the original SourceDiagnostic vector. A read-only Introspector or layouter never executes a callback.

## Exact internal data and APIs

In `entities/introspector.rs`, add crate-private data (no compiler dependency):

```rust
#[derive(Debug, Clone)]
pub(crate) struct FigureCounterSnapshot {
    pub(crate) key: CounterKey,
    pub(crate) history: Vec<(Location, Vec<usize>)>,
}
// in TagIntrospector:
pub(crate) figure_counter_snapshots: Vec<FigureCounterSnapshot>,
```

Empty/default starts empty; Clone preserves a complete immutable candidate view. No observed state is keyed by Rust Func equality. The owning compiler reads the Vec by counter_keys_eq. No new Introspector method or public data field is needed for this counter design.

A private owner `compiler/introspect/figure_counters.rs` (canonical prompt `00_nucleo/prompts/core/compiler/introspect/figure_counters.md`) owns:

```rust
pub(crate) fn figure_counter_key(kind: &FigureKind) -> CounterKey;
pub(super) fn prepare_figure_counters(
    content: &Content,
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()>;
```

The first constructs the native Figure identity's existing Selector::Element with ordered `kind` filter Value::Func or Value::Str. Native identity comes from the registered Figure constructor, never a name comparison. Element(table) differs from Name("table"). No new CounterKey variant and no write into user counter("figure:table").

The second gathers actual demands, deduplicates semantic keys in first-demand order via counter_keys_eq, calls the unchanged complete resolve_filtered_counter once per demanded key in this transaction, and installs all histories only after success. Before publication, validate every demanded Location: it exists in the same candidate, resolves to the demanded typed Figure identity, is numbered, has its canonical automatic action, and has a present prefix entry in the staged history at its causal ordinal. A present empty Vec is valid counter state, including Set([]), and is not an absent prefix or snapshot failure. Preserve it in the snapshot and borrowed counter_values_at/counter_final_values reads; flat_counter_at may naturally project None from an empty Vec without classifying that state as missing. Validation checks entry presence, not a scalar projection. Failure returns `figure counter snapshot could not be verified` at that demand's span before rendering; never silently borrow another Location or fabricate a number. Scopes are local; Engine sink, target/features and source span are those of this finalizer's transaction. It does not call filtered_history because finalizer preparation is not an explicit contextual read; it does not fabricate CounterFold observations in a producer.

Read methods counter_values_at, counter_final_values, flat_counter_at and formatted_counter_at consult a matching prepared Figure history before the legacy registry, using recorded causal ordinals and retaining empty state distinctly from absence. The existing global Figure counter still has one automatic update per eligible occurrence; auxiliary typed snapshots add no actions. Arbitrary explicit filtered readers continue to fold independently, so earlier manual Func results cannot be mistaken for a prepared registry snapshot.

## Exact additive bundle facade and compatibility

Add this public facade in compiler/introspect/entrypoints, reexported by compiler/introspect:

```rust
pub fn introspect_bundle_documents_with_runtime_targets_and_starts(
    bodies: &[Content],
    targets: &[crate::compiler::eval::EvalTarget],
    features: crate::entities::compiler_features::Features,
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
) -> SourceResult<(TagIntrospector, Vec<LocatorSeed>)>;
```

Require targets.len() == bodies.len() before any walk/callback. Mismatch returns a SourceDiagnostic error `bundle body target metadata does not match body count` at Span::detached; no default guesses, partial result or effects. Empty/empty is valid. The complete global walk and existing unrelated postprocessors keep their behavior. The new Figure finalizer gathers demands only from Paged bodies, using a local observer with target Paged and supplied features; source evaluation is not repeated. The snapshot's histories describe only that Paged demand context. Explicit reads in HTML bodies still use their own recorded Html/Features contexts and independent fold/replay.

Preserve `introspect_bundle_documents_with_runtime_and_starts` as an all-Paged compatibility wrapper using ctx.features and a targets vector of matching length. No existing public signature changes. The L3 bundle helper converts its real html_targets metadata into EvalTarget values and passes invocation Features; both context_stabilization callers pass existing metadata, never inspect syntax or infer a body target from its contents. This additive public facade is approval-gated. New facade tests cover empty input, length mismatch before effects, all-Paged wrapper equivalence, and mixed-body global counter actions with only Paged implicit demand.

## Actual demand traversal, not indexed-original traversal

Use a separate private owner `compiler/introspect/figure_demands.rs` and canonical prompt. Its helper returns ordered `FigureCounterDemand` records from borrowed Content plus current StyleChain and complete candidate introspector. This is an exhaustive active-visual visitor, distinct from semantic Figure-kind inference.

Its exact crate-private data/helper are:

```rust
pub(super) struct FigureCounterDemand {
    pub(super) key: CounterKey,
    pub(super) location: Location,
    pub(super) span: Span,
}
pub(super) fn collect_figure_counter_demands(
    content: &Content,
    intr: &TagIntrospector,
    chain: &StyleChain,
) -> SourceResult<Vec<FigureCounterDemand>>;
```



- Source descends with its span, retaining an existing child span when more specific. Styled pushes effective styles. Located transports identity without allocating; ShowRecipe descends into body; Realized visits visual only, never original.
- Figure consults inherited figure.numbering. Caption Some plus enabled numbering requests its typed key. Descend into body and existing caption without invoking any callback.
- RefForm::Normal first respects bibliography precedence, then label resolution and target Figure numbering eligibility; request that typed key only when a numeric Figure reference will be used. RefForm::Page does not. Missing/non-Figure/unnumbered references retain their own error path and do not demand an unrelated key.
- All other visual children follow their owner-declared active traversal, including resolved table cells, static running content and static HTML island content when that target actually renders them. Metadata/Value payloads and closure bodies are opaque. Dynamic or future Content alternatives require explicit owner review, never `_ => {}`.
- Existing visit_content_static is evidence for active children but cannot be used unchanged: it hides Source and Located before the callback and provides no inherited style frame. Implement this focused visitor with exhaustive static dispatch and explicit boundary tests; do not weaken the generic visitor or reuse semantic field inference.

The collector needs no second Locator or counter history index. Its private result is `FigureCounterDemand { key: CounterKey, location: Location, span: Span }`. For a prepared visual Figure, the Location is transported by its actual Located root through Source/Styled/ShowRecipe wrappers; that Location applies to this root only and is cleared before child recursion. A nested Figure cannot borrow its parent's Location. For Ref, resolve the target Label through the complete introspector and retain that target Location. Unprepared synthetic Figure data (resolved_kind None) does not create a productive implicit demand. A prepared numbered caption without a transported Location is an error `figure counter snapshot could not be verified` at its Source span, never guessed by pointer/hash/index.

## Location-based caption and reference reads

At entry to layout/figure, capture current_location before laying out body or caption. Its return type remains (); finalizer validation establishes the productive availability invariant before layout. For an external caller bypassing preparation, a missing prepared Location/history appends `figure counter snapshot could not be verified` to layouter.layout_errors (nearest retained Source span, detached only if unavailable) and returns without a fabricated caption prefix. Read the typed key's prefix with flat_counter_at(key, that Location), not `value_at_index`. Manual updates therefore cannot consume an occurrence ordinal, and a hidden original cannot shift another visible caption. Retain private figure_progress and its checkpoint copy/restore exclusively for synthetic unprepared FigureElem data (resolved_kind None). In that branch alone, preserve its old string-kind ordinal/index-plus-one fallback and existing numbering-enabled increment behavior, including no-caption input. Prepared productive Figure never increments or reads this map. A synthetic local test therefore retains historical 1,2 behavior, while production numbers come only from current_location and the validated typed snapshot.

References already resolve label to Location. Store typed key for all numbered Figure labels, including captionless Figures; use the same prepared history at that Location. A pure walk must not prematurely synthesize final Figure reference text from an incomplete Func history. Numeric read precedence remains unchanged; supplementary text comes from the separately approved resolved Figure supplement, including explicit empty. Custom Name("table") never gains Element(table)'s default translation.

Keep public `figure_number_at_index(&self, kind: &str, idx: usize)` and its implementation unchanged, with no productive call from Figure layout. Existing synthetic registry-backed behavior remains available. Approval explicitly accepts its runtime semantic compatibility break: productive snapshots no longer contain auxiliary automatic `figure:<kind>` string-counter writes, so this legacy method no longer reports automatic Figure numbering from a productive snapshot. It can still report an explicitly populated/user string history. Its string parameter cannot distinguish Element(table) from Name("table"). Add no compatibility shadow state and no typed replacement method; callers migrate to typed key plus Location through existing reads. This limitation and migration are part of the ADR-0127 compatibility gate, not hidden as an internal refactor.

## Finalization and sink ownership

- Both runtime functions in compiler/introspect/entrypoints invoke prepare_figure_counters after the complete walk, synthesized outline entries and parent/index construction, before consumers render Figure references. Bundle processes complete ordered body content against the one global action log; do not finalize each partial body snapshot separately.
- The legacy compiler/introspect/fixpoint runtime also invokes it against its complete iteration. Its finalizer must run in an iteration-local Sink/Engine clone, publishing that sink only for the returned successful iteration. Do not change the public five-iteration budget, stopping criterion or unrelated callback semantics. A failed demanded fold propagates original errors. This prevents adding discarded-iteration warnings to the caller's sink.
- Ordinary paged production calls through infra/pipeline/paged::introspect_with_runtime_for_target and infra/pipeline/runtime::with_introspection_runtime, which already own a fresh Sink. Preserve target/features and return warnings with the candidate.
- Bundle production calls through infra/pipeline/bundle::introspect_bundle_with_runtime_for_pipeline, which owns a fresh Sink. Add target/features parameters to this L3-private helper and migrate both calls in context_stabilization (candidate and final bundle construction). Pass actual per-body targets and invocation Features into the additive L1 facade below. Global actions/query membership include all bodies; implicit demand discovery visits only Paged bodies. Every implicit fold therefore uses Paged plus the supplied Features. No key snapshot is shared across different demand targets.
- Stabilization uses these helpers for each candidate. Keep phase_warnings local and publish only the returned candidate's phase warnings; retain original producer contribution rules. Validation replay still uses its local discarded Sink and cannot consume any retained result/effect cache because none exists.
- Explicit filtered_history uses the executing Engine sink; context replay constructs a fresh sink. Neither reads the finalizer's snapshot as a bypass for callback diagnostics.
- Standalone HTML Figure caption/reference rendering is explicitly excluded from this bounded counter package and remains existing debt. Kind/reflection/explicit filtered reads are still acceptance-tested under HTML. The finalizer skips implicit demand when observer.target is Html; mixed bundles visit only Paged demand bodies. html.rs:103–121 remains unchanged: no eager all-postprocessor invocation or new HTML formatting claim.

Successful separate retained demands may recompute callbacks. That is permitted mechanics. Their diagnostics follow each retained transaction's existing publication rule; no callback count is an acceptance oracle. Within a Sink the existing span/message dedup remains unchanged. No cache can swallow a warning from a later retained computation after an earlier validation-only computation.

## Required RED and refutation cases

1. The six frozen bilateral cases above; actual rendered caption/ref numbers additionally require b=8 and c=17, not only explicit query results.
2. Keep no-caption, numbering:none and show-hidden panic controls; then add an explicit final read to each to distinguish automatic eligibility from demand.
3. One raw figure reused under image/table style frames; typed table and string "table" plus user counter("figure:table") remain three independent identities.
4. Hidden numbered raw then visible raw caption: caption reads 2 by its Location, even though the hidden original caused no earlier caption demand.
5. A callback failure after the caption's Location must still fail the whole demanded history; prefixes are projections after complete validation.
6. Validation-only warning computation followed by retained demand must publish that retained warning. A replaced producer's warning must disappear. Two retained demands must follow existing Sink semantics without a shared published flag. Check exact original spans/traces/message and source order.
7. Change manual action and where(kind) membership between candidates while preserving final scalar values: replay must still inspect full history. No result keyed merely by kind/hash/Location may bypass it.
8. Empty Set state is retained as Some(&[]) in prepared borrowed reads, including a complete demanded history ending in Set([]). It must not cause snapshot validation failure or be rewritten to [0]; flat projection may return None. Test a captionless automatic Figure followed by Set([]), explicit final() returns (), and a later numbered Figure steps it to (1,). Also test hierarchical Step, aliases, first Label resolution, Page references and bibliography precedence; no extra automatic action, no event-index substitution.
9. Unnumbered labelled Figure after a numbered one must not inherit its prefix as a valid reference; numbering eligibility, not existence of earlier history, governs referenceability.
10. Direct pure synthetic layout tests with resolved_kind None retain historical 1,2 fallback and checkpoint restoration. Prepared Figure with missing Location/snapshot fails before rendering; the layout consumer also pushes a SourceDiagnostic to layouter.layout_errors and emits no fabricated prefix if that invariant is violated by an external caller. It does not panic or quietly use synthetic progress. Missing runtime callbacks never fabricate production numbering.
11. Disposable enum extension fails the new demand partition and inference partition; keep every linter note rather than annotating it away.

## Owner amendments

The carrier proposal owns FigureKind and resolved supplementation. Counter-specific amendments belong one-to-one to entities/introspector (private snapshot), entities/element_payload and entities/elements/figure (numbering-only eligibility), compiler/introspect/populate (one action/typed labels/no user-string alias), compiler/introspect/walk (effective gate), new figure_counters and figure_demands (fold orchestration and actual demand), compiler/introspect/{mod hub,entrypoints,fixpoint,queries,labelled}, compiler/layout/{figure,references,mod,checkpoint,measurement}. Existing compiler/stdlib/counter/filtered_reads and compiler/eval/context_reads are preserved algorithms and regression boundaries, not a new cache owner. CounterRegistry needs no new result cache or public API.

L3 runtime/paged/bundle/context_stabilization need their own prompt amendments only for actual integration/target transport/retention changes. Do not put fold/key/selector logic in L3. The complete manifest is generated separately and pins all inspected source/L0 inputs.

## Exact owner amendment checklist

| Owning source (all have canonical 1:1 L0) | Required amendment |
|---|---|
| entities/elements/figure | to_payload initializes automatic eligibility independently of caption; typed kind/supplement carrier changes remain in carrier proposal. |
| entities/element_payload | redefine Figure.is_counted as effective numbering-present, including captionless; kind becomes typed per carrier proposal. Caption text presence remains separate. |
| entities/introspector | declare crate-private FigureCounterSnapshot and snapshot Vec; empty/clone lifetime; preserve public trait, including legacy string method with explicit productive-snapshot compatibility break. |
| compiler/introspect/populate | one canonical automatic event for each numbered Figure; retain global Figure aggregation; remove automatic user-string writes; typed key for every numbered Figure label, independent of caption. |
| compiler/introspect/walk | effective numbering gate overrides placeholder consistently; preserve Source/Located/Realized originals and one action per Location; do not make demand from mere Tag presence. |
| compiler/introspect/figure_counters (new) | exact key constructor/finalizer APIs above; canonical fold, complete demanded history, validation, atomic candidate publication, no memoization. |
| compiler/introspect/figure_demands (new) | exhaustive active-visual/StyleChain/Source-span/Located-root demand collector; reference precedence, no execution, no query-original demand. |
| compiler/introspect hub | register both private modules; reexport additive bundle-target facade only; no business logic. |
| compiler/introspect/entrypoints | runtime finalization after complete indexes; additive bundle facade, strict metadata length, compatibility all-Paged wrapper, Paged-only implicit demand with supplied Features. |
| compiler/introspect/fixpoint | same private finalizer once per complete iteration; isolate its sink, publish returned-iteration effects only; preserve all existing signatures/cap/convergence/other callback ordering. |
| compiler/introspect/queries | common prepared history lookup by semantic key; prefix by causal order; all four counter read projections agree; old string-index implementation retained with documented break. |
| compiler/introspect/labelled | do not capture incomplete Figure Func number during pure walk; keep typed labelled identity so runtime numeric path wins; no orphaned legacy placeholder that suppresses errors. |
| compiler/layout/figure | productive current_location+typed key prefix; no synthetic ordinal usage for prepared data; exact missing-snapshot error above; existing pattern/supplement formatting scope stays explicit. |
| compiler/layout/references | typed key and target Location use same prepared prefix; preserve bibliography/Page/explicit supplement precedence and unnumbered-reference rejection. |
| compiler/layout/mod and checkpoint | preserve figure_progress solely as explicitly synthetic state and its rollback; no new productive progress map. |
| compiler/layout/measurement | retain rollback/effect tests; add assertion prepared Figure does not consume synthetic progress. |
| infra/pipeline/bundle | private runtime helper accepts actual per-body target list and Features; builds fresh sink, calls additive facade, returns candidate effects; no fold/key logic. |
| infra/pipeline/context_stabilization | both bundle helper calls pass existing html_targets and Features; preserve five-attempt and phase_warnings/producer sink retention without cross-generation cache. |

Existing filtered_reads/context_reads/counter_updates/Sink algorithms and L3 runtime/paged helpers are inspected regression boundaries, not authorized edits merely because they appear in the reviewed manifest. HTML Figure rendering is explicitly outside scope; no html.rs change is authorized by this counter design. New collector/compiler helper crate-private visibility follows the existing introspect child-owner pattern; it does not move rendering into entities or introduce reverse imports. Whole design still requires the combined ADR-0127 approval before new public facade/carrier code.

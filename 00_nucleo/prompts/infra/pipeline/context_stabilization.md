# L0 — infra/pipeline/context_stabilization
Hash do Código: 65f44341



**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/context_stabilization.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compilation-metric-cache.toml sha256:389926b89934d625412cbbb22bad324ea6579533d79aa6b2dcf92fa5877095e4
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/physical-number-outline-eligibility.toml sha256:40b07110378aa324ef3e1b534bd6a7d948128d1cdbb318ed0e84ed79e211c5ac
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Current evidence

- `03_infra/src/pipeline/context_stabilization.rs:42` — `Session`.
- `03_infra/src/pipeline/context_stabilization.rs:651` — `prepare`.
- `03_infra/src/pipeline/context_stabilization.rs:690` — `prepare_html`.
- `03_infra/src/pipeline/context_stabilization.rs:748` — `prepare_bundle`.
- `03_infra/src/pipeline/context_stabilization.rs:897` — `bundle_page_views`.
- `03_infra/src/pipeline/context_stabilization.rs:1043` — `strip_executed_html_markers`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/context_stabilization.rs`.

## Owned contract

- Own only the private per-compilation session: causal discovery, producer contributions, attempts, sinks, retained observations, replay and terminal decisions. Use the existing evaluation/introspection/layout facades; never duplicate selector matching, counter/state folds or Value equality in L3.
- Selection requires an actually reached observation, not AST inspection, variable naming, error substrings or the mere presence of a selector. A failure before demand retains ordinary legacy treatment. Nonselected producers retain their ordinary contribution/error during additional attempts.
- Preserve producer identity, capture, Location, input style chain, target, features and Engine resources per attempt. Replace a producer/closure/chain by invalidating its descendants; numeric context id or Debug/hash of a closure cannot certify reuse. Replay uses captured input styles, not the chain left by execution.
- For selected filtered-counter generations, discard discovery execution/sink and start with empty observational views I0 while retaining origin Locations. Discovery is not an extra stabilization attempt; selection stays latched even if a later branch makes no read.
- Attempt A_k reads I_(k-1), builds current content/document and a complete I_k (runtime introspection, positions and PageStore), then validates retained actual reads against I_k. Reexecute only invalidated selected contributions and descendants; discover real newly produced blocks recursively. Reconstruct from original content/current contributions without accumulating updates or markers. Failed bodies keep only their marker, not stale successful output.
- Reuse only complete valid records with unchanged causal inputs. Equal final values, equal page counts or a lone selection flag do not prove stability. All retained contributions must describe the same candidate generation.
- Share the existing five-attempt budget with paginated invalidations. On validated convergence return the candidate or its pending original error. For proven linguistic nonconvergence without final error, return D5 calculated from I4 and diagnostic history I0 through I5; do not execute A6, substitute replay from I5 or return D4.
- Publish only warnings of retained final executions, once and in causal order; discard validation/replaced/discovery sinks. Preserve original error spans/traces and precedence. Do not reorder by HashMap or infer error classes from text.
- Inability to prove required stability is not linguistic nonconvergence. At the terminal budget, return Error `contextual stability could not be verified`, anchored at `source.root().span()`, without fabricated trace/hints or provisional export. Proven ambiguous identity/topology may terminate at discovery. Original pending errors take precedence; prior global warnings remain.
- HTML preparation preserves causal Html target/features and strips executed context markers only in the terminal serialization view. Residual contexts, unobservably paged dependencies and closure errors fail closed, never become Empty or successful output.
- Standalone HTML position stabilization is selected only by observed LocationPosition, not filtered-counter selection. Recursively discover provisional descendants; discard selected discovery contributions, retain unselected ancestors. Seed with the original HTML introspector and empty HtmlPosition store, not an empty global introspector that would erase query/state.
- Latch the HTML phase after the first observed position read. Every later generation must build and compare its full store, even when producers disappear; nonempty-to-empty requires a further empty-to-empty confirmation. Reintrospect marker-plus-result content, measure only intrinsic islands and validate both full store and observations under the same five-attempt budget.
- Bundle preparation reconstructs/reintrospects all ordered bodies against one global candidate with causal seeds, target/style/features/resources per body. Keep Page boundaries per body in pure and runtime snapshots; local Page reduction only initializes physical pagination and is not a query/validation snapshot.
- Paged bodies contribute positions and PageStore; HTML bodies contribute separate intrinsic HtmlPosition stores without page offsets. Validate the complete set before terminal HTML projection or any export; never promote a body settled under an earlier generation.
- Without selected bundle producers, realize the final paged view once before terminal HTML projection to supply PageStore. With selection, reuse the validated store without a second realization. HTML consumes no physical pages.
- Keep CounterUpdate nodes in candidate snapshots; remove executed HTML updates only in terminal projection. Preserve SpaceFromNewline origin until causal L1 normalization; only an all-whitespace terminal Sequence of Space/SpaceFromNewline collapses to Empty.
- Layout/callback/numbering and intrinsic-position helpers keep their own owners. Do not add an observation ledger, boundary hook or instrumented default.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Causal discovery, identity and single evaluation

Causal ordering recognizes ContextBlock at a Located root: visual traversal's suppression of a duplicate transported-root callback must not lose its producer. Legacy fallback likewise recognizes prepared ContextBlock inside Located, preserving one evaluation per id before selection requires another session. After validated HTML expansion, strip executed contextual/counter-update markers even inside Located; never silently drop an unexecuted producer.

Session::content at context_stabilization.rs:285–292 substitutes even with zero roots. Existing exhaustive discovery supplies this condition, never source syntax/introspector inference. With no roots return an origin clone preserving Arcs, without a fabricated map; otherwise keep reconstruction and every validation/contribution/error/warning/normalization rule. No new selection gate or cross-generation contribution sharing. Require carrier-preservation RED→GREEN and real-producer revalidation under continuous flow.

## Page snapshot evidence and ratified boundary

`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, the uncommitted tree and exact binaries. Historical vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`. `single` prints B1/0 F0 versus B1/1 F1; two pages total/final 0 versus2; initial update7 leaves total7 versus8; an intermediate update changes the earlier page (7/7 and8/7 versus1/7 and7/7); final step prints final1 versus2. Bundle prints1/7 in the first PDF versus1/1, and final7 in the second versus8. Mixed HTML returns1 versus7. `paged.rs:360–365,840–913` and `context_stabilization.rs:969–1010` use the last update as every page/body start; `compiler/stdlib/counter.rs:490–513` and `counter/display.rs:96–110` read the causal global final. Historical upstream `lab/typst-original/crates/typst-library/src/introspection/counter.rs:246–254,730–773,934–963` combines document scope, updates and physical advances. This is language parity, without requiring upstream memoization mechanics.

Apply the pinned page-counter-snapshot nucleus. The human ratified `lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json`, SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`: its ADR-0127 gate is satisfied, superseding the earlier pending gate and historical final scope exclusion only for this package. Preserve signatures, cycles and budget; do not guess page/total before the snapshot is known. Subsequent internal clarifications use continuous flow, L0 first and RED→GREEN.

Supply the pure fold with occurrences per body and the candidate contribution's pages/positions. Prepare Page snapshots after physical positions, then reconstruct and validate under the same budget. Never use the last update as every page's start, reevaluate the entire source, borrow another document's caches or move I/O into L1. Numbering/PageStore and marginals consume their relevant realized numbering; an export-local projection must not contaminate the global snapshot. Preserve producer error/warning retention. Other owners require their own current L0; this owner does not authorize counter handlers or render logic.

Use the private L3 Engine for the ratified facade. Register bundle partition in walk by explicit occurrence membership; checkpoints are layout-only. Compare both Page views for generation stability. An observed HTML Page replay may select without changing has_filtered_counter_reads or public EvalContext. A bundle marginal's logical total comes from a final-snapshot occurrence positioned by its own document; when none exists use that document's last logical number, never an artificial Locator boundary or another body's occurrence. Include parent_locations relationships in position enumeration.

## Metrics lifetime: evidence, scope and acceptance

`lab/results/construction/diagnostics/contexto-cache-fontes-2026-10-05/before-state.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state and baseline binary `5bc45f4ec0972c6c46450c0cab3479e2ae2ef41c286c424201c1b7fc07823eb1`. GDB `count-baseline.json.gz` measured selected_distinct2243 queries/965 shaping computations after misses/two wrapper entries, selected_repeated8 misses, ordinary_control321 misses. `context_stabilization.rs:548` calls the wrapper without metrics; `paged.rs:656–661` creates fresh metrics when absent; `font_metrics.rs:71–96` creates Arc caches per instance and Clone shares them. `grid-cache-share-ab-2026-10-03.md:17–23` explicitly restricted the earlier scope to the ordinary path, with fresh contextual/bundle wrappers. The measured extension is not a correction of an assumed accidental restriction; the older fresh wrapper's origin remains unaudited.

The selected paged session may pass its existing metrics handle to layout. Share only text/size/style/font metrics for the same World and compilation. Every attempt keeps its own content, introspector, regions, callbacks, transcripts, sinks and budget. No cache of Content, frames or contextual execution, no retention across compilations/edits, and no source-key/font algorithm copied into pipeline. HTML/bundle and other callers retain their resources. This internal ADR-0127 continuous-flow change preserves public contracts, target, mode and phase; retain it only with demonstrated work reduction, A/B measurements and unchanged language/geometry/diagnostics.

`lab/results/construction/diagnostics/contexto-cache-reconciliacao-2026-10-05/before-state.json.gz` preserves the same HEAD/uncommitted state and baseline `17201a15e36b06bb3a9ac8adba0a98c61b3dc925c4ce1f118a39d26bce26cf05`. `count-baseline.json.gz` isolates fix_line_positions_with_metrics: selected_distinct321 queries/321 computations (643 total), ordinary_control320/zero. `context_stabilization.rs:14–17,660–663` drops the Session handle; `paged.rs:337–345,563–572` leaves None and reconciles with fresh metrics. The canonical owner `00_nucleo/prompts/infra/shaper/reconciliation.md` already permits an internal borrowed-metrics helper; equations and public ABI are unchanged.

Transport a clone with the accepted complete selected paged candidate. Prepared/prepare are pipeline-private; their lifetime expresses existing World/Source borrows. Only the existing settled branch transports metrics, including the accepted terminal result under proven nonconvergence. Preserve acceptance, errors, contributions, sinks and budgets. Unsettled results, HTML and bundles transport no handle. End lifetime in the paged coordinator. No extra execution/layout, frame/snapshot cache or cross-compilation retention. Output/position/diagnostic changes or no proven work/time reduction refute the change.

The transported handle ends in paged coordination. Its later bidi borrowing is governed by `00_nucleo/prompts/infra/pipeline/paged.md`; it supersedes only the temporary reconciliation-only restriction, not the no-HTML/bundle/unsettled scope.

## Recursively produced paged descendants

`lab/results/construction/diagnostics/query-compilada-2026-10-06/green-cli-focused.log` records query_compiled_context_errors_preserve_origin_trace_and_deprecation incorrectly succeeding for #context [#context unknown]. initial-source-state freezes the uncommitted state; the interrupted debug rebuild initial-materialization.log is not evidence. Suite RED precedes repair. discover at context_stabilization.rs:257–269 realizes Html only, leaving child ContextBlocks unexecuted when no observation selects a fixpoint; historical pinned vanilla errors at <stdin>:1:19.

Paged discovery recursively realizes actual produced descendants through existing realize after root execution, unless identity is impossible. An invalid=false root is not rerun. Visit only actually produced bodies; preserve guards/styles/capture/Source/span/trace/causal allocator and selection by actual child observations. Keep legacy-id treatment and five attempts; do not fabricate a fixpoint for producers without reads or reevaluate the whole program. RED→GREEN nested context and integrated validation are required; no API/phase/instrumentation/warning-policy change.

## Bare mathematical insertion

`lab/results/construction/diagnostics/math-bare-2026-10-06/{before,additional,refuters}.json.gz` and `lab/results/construction/diagnostics/math-bare-medicao-2026-10-06.md` preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state/UTC/diff/sources/binaries at historical upstream `9f2b6e8715237cb086899a42873660fe744622e8`. Bare frac/mat/sqrt/lr/op has one Equation upstream and zero locally; math.bold([x])/ordinary text zero in both, nested/explicit one in both, reuse two upstream. block-style/local-block/number-style establish insertion scope; show/context generation exposes the same gap. Explicit Equation preserves fraction render; upstream it.body reaches `maximum show rule depth exceeded`, it is valid; later show affects only the second insertion. Refuters distinguish raw frac/original label from explicit Equation.

Upstream typst-realize/src/lib.rs:245–265, 302–334 converts Mathy before show outside math; local equation.rs:680–689 projects bare math as text, realize/mod.rs:155–199 prepares show/identity without math normalization, traversal.rs:14–108 is bottom-up into math, rules/apply.rs:564–607 handles existing locatables only. These are language semantics/morphology/render, not vtable/buffer/Rust equality/PDF byte requirements.

The human ratified `lab/history/construction/records/00_nucleo/materialization/math-bare-2026-10-06.md:3` and math-bare-aprovacao-2026-10-06.json, manifest SHA `149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6`. The earlier candidate gate is satisfied only for this package. Require scoped bilateral RED→GREEN/revalidation; this supersedes prior phase restrictions only at named boundaries, without universal parity, math-label or other debt closure.

After apply_func returns Content and while the causal Engine lives, call L1 realize_math_insertions before storing, discovering children or reindexing. Pass existing ctx/styles/rules/guards/Source/sink. No L3 Mathy recognition, normalization, matching or counter fold. Preserve effects once per valid execution, observations, allocators, warning/error retention, five attempts and targets. New math contributions receive L1 identities; do not reuse producer id as Equation Location or replace insertion styles with final styles. Reused accepted contributions are not realized again; a new execution replaces the earlier result under existing policy. Public signatures remain.

## Table insertion evidence, approval and remaining scope

Historical evidence in `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted state, UTC, status/diffstat, sources/binaries, argv/results and upstream pin `9f2b6e8715237cb086899a42873660fe744622e8`; oracle SHA `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, Tekt SHA `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, :1500–1530 rejects conflicts in order, :1665–1703 creates empty cells, :1945–1988 derives breakable. Tekt grid_placement.rs:91–112 prioritizes explicit cells and :262–270 knows only GridCell; realize/mod.rs:160–200 visits recipes bottom-up; table.rs:127–152 dumps wrappers without attributes. `order` measures zero cell effects when the table is replaced and zero heading effects when hidden by the cell; `aliases` measures LOCAL a, OUTER LOCAL a, NEW a. These are language observations, not mechanical/byte/Rust equality requirements. Unknown fields, hidden effects or wrong order refute the hypothesis; cache/pagination equivalence is not inferred.

The exact insertion package is ratified: `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3` records the owner's `comfirmo`; manifest SHA `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. This satisfies its gate for five public fields, two public carriers, one facade and the logical phase before Cell show. Isolated-cell defaults and fresh show-cell-reconstruct generation are separately ratified in `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, owner `confirmo`, manifest SHA `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. Preserve their exact scope; cell-set/style defaults remain incomplete. General Table defaults/fields (table-root-equality), rich functional styles/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA remain uncertified. Do not change legacy extra casts from the earlier grid package, syntax, flags or the global nonlocatable policy.

Acceptance remains RED→GREEN, suites/build/lint, restricted lineage maintenance, and D08A/B2, D05/D06/D07/query collections. Judge language, effects and real geometry; PDF bytes and whitespace normalization cannot close render acceptance.

After Content production in its causal Engine, delegate realize_table_insertions and current-owner math realization before storing/discovering/indexing. Preserve Table/Cell capture without L3 type/property classification. Accepted/repeated contributions do not rerun recipes. Preserve budget/reads/sink/observations/allocators.

## Input snapshot ownership and execution retention

`lab/results/construction/diagnostics/memoria-fases3-2026-10-08/{profiles,profiles-growth}.json.gz` with full state.json.gz records context300 peak≈196MiB versus plain300≈46MiB, prepare≈62.5→138.7MiB. In context-snapshots-memoria4/before.json.gz, context_stabilization.rs:196,244 clones input into mutable ctx and again into every immutable Execution record; TagIntrospector owns cloned collections as well as Arcs. Sharing the second view per concrete generation is a private-copy-cost inference, refuted by isolation/replay failure or no paired saving; RSS alone proves no leak.

Keep ctx.introspector separately owned/mutable throughout execution, including closure/show/realization mutations. Execution.input may be private Arc<TagIntrospector>, shared only by producers/descendants consuming the same concrete generation input. Discovery creates its own snapshot; every attempt creates another snapshot of historical input. Preserve history Vec, budget and L1 diagnostic facades; no equality/hash/page-count/id deduplication. Replaced/retired records retain original input Arc, never mutated by the next generation. HTML creates its own view before Page replacement for replay. Do not share mutable ctx, remove histories/retired records or anticipate contribution reuse. Public signatures/phase/defaults remain. RED→GREEN verifies physical sharing within one input, mutable/new-generation/retired isolation; address identity is a cost witness, not language acceptance. Run full context/infra, PDF/HTML/error/replay/budget controls, build/lint and paired memory/time.

After equivalent flow, context300 measured≈95MiB locally versus31MiB upstream; phase RSS40.7→80.1MiB over300 execute calls, ≈130KiB/producer. Removing retired had median94892KiB versus94826KiB and was reverted. The memory diagnostic's item3 preserves commands/binaries/state. context_stabilization.rs:196 clones EvalContext introspector; after apply_func and insertion realization, productive Execution.ctx uses are validation/replay and target/features/errors. context_reads.rs:232–265 clones a candidate into a fresh context per replayed read; no later access uses Execution.ctx.introspector. This private-retention inference is refuted by output/replay changes or no measured gain.

Only after evaluation and before storing Execution, release ctx.introspector. Keep recorded observations/input Arc/result/sink/Location/features/replay parameters. Retired causal records remain, with only disposable introspection view released. No change to public EvalContext, I0–I5, five attempts, order, phase or defaults. Require release RED→GREEN, context/PDF/error equivalence and measured memory reduction.

`lab/results/construction/diagnostics/clippy-retencao-2026-10-16/before.json.gz` preserves HEAD/uncommitted tree/UTC/diffstat/sources/L0/global hashes. clippy-aridade-final-2026-10-16 lists17 warnings including unread parent/final_styles and Vec<Box<Execution>>. Full local inventory finds no pin/self-reference/raw pointer/address test; input/Func/StyleChain retain independent Arc identity. This is Rust mechanics, no new parity/RSS benefit.

Remove only stored Producer.parent; keep Session.add(parent), which chooses target through the parent producer. Rename Execution._retained_styles while moving identical styles at the same final point for the same duration/drop. The unread chain intentionally retains values/closures, not replay input; do not empty/remove/free it early. Use retired: Vec<Execution>, moving *previous/*old/*discovery from current's Box in all three transitions. current remains Option<Box<Execution>>. Keep every record/order/input Arc/ctx/observation/result/sink/Location through session drop; only remove retired records' outer allocation. Never introduce container-address identity, reexecute closures to rebuild history or erase generations.

A real Session.execute retention test uses Value::Version Arc/Weak lifetime: after removing the other owning chain, retain the value in current, retired and after real Vec growth, then release on session drop. Keep identical input identity and replay/generation tests. Arc counts are mechanical retention evidence, not language parity. Keep target/capture/HTML/bundle/PDF controls; no syntax-mirror container test. Continuous flow is limited to private representation. Require dead_code/vec_box RED and all three selected spans GREEN, no new allow/expect/threshold; subset_font belongs to its own separate owner. Build, Clippy all-targets/expectations, full workspace, Rustdoc/fmt/strict reverse lineage, no new Tekt warning, 12GiB/swap0/jobs1. No memory/time claim without paired measurement.

## Empty ordering proof and private result structure

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/cpu-style-current.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, release `14d023d36fb765eb041f24f440eaad3bc5d67183325b611e4fd98502bd26815e`, UTC/state/fixture/commands. ordered_blocks has 76 local-set-1600 and25 no-fill-1600 samples over12 compiles each. ordered-blocks-empty-red-gate.json.gz uses empty-reference-fixed release `6765fa1113676347dd00007bec1ba1f4e35bf49337d01d0e033d662091c462c4`: predefined≤523000 glibc requests fails at527107 local versus402236 control. External load invalidates that gate's isolated time as benefit evidence. context_stabilization.rs:74–99 collects by_id then reconstructs Content to order/detect duplicates.

If exhaustive by_id is empty, ordered_blocks may immediately return Some(Vec::new()) without Content reconstruction, only because collection covers every visual carrier the ordering walk sees. A carrier seen only by the second walk refutes the proof. Any producer keeps full ordering/duplicate-ID detection/None legacy fallback. Test nested empty table, repeated id, distinct blocks and Realized with producer only in original. No public L3 contract/phase/budget/selection/warning/error change. Retain only with RED→GREEN allocation gate, equivalence and paired time gain.

context_stabilization.rs:926 (C096), `lab/results/construction/diagnostics/type-complexity-2026-10-15/before.json.gz` and red-clippy.log preserve source/owner hashes/HEAD/tree/UTC/diffstat and lint RED. Internal prepare_bundle success uses pipeline-private PreparedBundle fields bodies/introspector/document_starts, moving identical Vec/TagIntrospector/LocatorSeed in the same validated order/generation. SourceResult/warnings, errors/early paths/budget/replay/snapshots/external APIs remain unchanged. This is Rust representation, no AI benefit/functional bug/new parity. An alias requires proven identical expansion; incompatibility/new public type needs another gate. Nominal type_complexity exceptions cover only the three justified batch items, superseding generic bans only there, with no module/impl/crate suppression, raised threshold or suppression for other repair cases. Continuous flow, L0 first/restricted lineage. Keep existing expectations; selected lint/denied unfulfilled_lint_expectations RED→GREEN, full build/Clippy/workspace/fmt/Rustdoc/lineage. No tests mirroring aliases/field names.

## Heading setup and occurrence arena

`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/{final-freeze,target-controls}.json.gz` records HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/UTC/diffstat/source/argv, upstream `bd5ebec285505889e8ff1d4f68804ab147a699f3`. heading-raw-constructor-func passes baseline/upstream but fails candidate because transport Styled changes func; query sees a temporary causal wrapper and HTML ignores callback panic. walk.rs:930–945/paged.rs:419–427/html.rs:101–106 use pure setup. These are language values/errors, not upstream algorithm requirements. Ratified `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md` SHA `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c` satisfies the Heading-only gate.

The nonbundle candidate runtime branch passes self.target/self.features to the private introspection helper instead of Paged/empty defaults. Preserve bundle behavior, selection/five attempts/producer order/snapshots/metrics cache. No L3 Heading callback/fold logic. Require ratified H1–H8/bilateral RED; five State helpers are separately authorized, no additional public fields/contracts. Ordinary Styled/other callbacks retain boundaries. Validate workspace/doctests/build/Clippy/Rustdoc/fmt/V1/V5/V15/V26/reverse/no V16 increase, 12GiB/High8GiB/swap0/jobs1, sequential heavy commands; RAM gain requires measurement.

`lab/results/construction/diagnostics/context-paridade-2026-10-18/before.json.gz` preserves HEAD `b7e7c83223e5daa67bbdf124c36b5b7f93200731`, source/UTC/diffstat/argv/binary SHA; baseline-build-resources.json contains the new recipe. Upstream `bd5ebec285505889e8ff1d4f68804ab147a699f3`: two equations yield EQ EQ Q1/Q2, Heading prefix H EQ Q0/Q1, query it.location00/01. context_stabilization.rs:257 resets EvalContext, eval/mod.rs:438 Locator, realize/mod.rs:41 allocates without a common domain; introspect/walk.rs:375–390 deduplicates by Location. This language-identity collision hypothesis is refuted by count/alias/callback/replay failure. Upstream typst-realize/src/lib.rs:353–418 prepares identity before recipes, without requiring copied representation/vtables/algorithm.

The owner approved `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18/review.md`, SHA `500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388`, approval-receipt.json alongside. ADR-0127§2.1 is satisfied only for its additive API, no phase/unrelated API change. Session owns the L1 facade arena; each Producer owns its exclusive emitted frame. Initialize from real origin; no producers may use None, avoiding redundant traversal. New Producer gets a fresh frame; reexecution retains its frame; discarded descendant slots are never recycled. Wrap the existing apply_func→realize_math_insertions→realize_table_insertions operation in arena.evaluate without order/extra-execution changes. L3 never chooses/manipulates numeric Locations. A new legacy Session reconstructs arena/frame deterministically from the same origin/causal order, never from prior results. Keep retention/selection/errors/budgets/styles/targets/effects/callbacks.

workspace-rest-occurrences-final.log, validation-occurrences-final and occurrences-final-sources.json.gz preserve existing execution_retains_style_values_through_retirement_and_vector_growth failure: Session::add:235–242 assumes arena, while test:1485–1506 creates empty then adds. Initialize arena lazily in add using session origin/get_or_insert_with; Session::new may always start None. Zero producers retain no arena; do not change that test, selection or phase. This private correction stays within the approved API; recorded RED requires GREEN/full revalidation.


## Approved Figure/Grid occurrence counter correction

### Evidence before decision

Baseline HEAD `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`; the reviewed current-pin measurements and exact source anchors are retained in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, `agents/counter/design.md` and `agents/counter/counter-probes.json`. In that evidence, captionless/numbered Figures count; manual Set/Func actions make caption ordinal indexing incorrect; typed element kinds and custom names require distinct keys. These are language values/errors, not Rust or rendering-byte equality. The human explicitly approved the complete Figure/Grid contracts, phases and compatibility changes in the current implementation session. This approval satisfies ADR-0127 for this exact scope. Historical conflicting caption-only or productive string-projection clauses are superseded only as described here; unrelated contracts remain.

### Exclusive responsibility

Both bundle runtime helper call sites pass existing actual html_targets and invocation Features. Preserve five attempts, phase_warnings candidate locality and accepted-only publication, producer contribution rules and validation replay isolated sinks. No cross-generation result/effect cache or new transaction API.

### Acceptance and limits

L0 precedes functional RED and implementation. Require independent fixed expectations for complete demanded histories, captionless/hidden/None demand controls, manual actions before/after requested Locations, empty Set state, typed aliases/homonyms/custom names, actual Paged caption/reference text, both-target explicit reads, bundle metadata rejection before effects and candidate diagnostic retention. No callback count as parity oracle. Preserve the independent filtered-read/replay/Sink algorithms and HTML implicit-rendering debt. Require exhaustive real-enum extension probes at new owned decisions, all twelve established gates, full differential lint, canonical owner paths, V1/V5/V15/V26 and reverse lineage with scoped resealing. No lint suppression, contract weakening, new defaults, memoization or public changes beyond the reviewed approval. Record HEAD/dirty file list, UTC, argv and executable identity for measurements.


## Figure retained diagnostics and explicit HTML read integration regressions

The frozen `candidate-target-controls-2/receipt.json` under the current Figure implementation evidence binds the actual product/source state: warning-demanded and warning-explicit-plus-caption each publish the same sourced Hebrew-zero warning twice; html-explicit-filtered-read terminates with `contextual stability could not be verified`. These are required language diagnostics/explicit reads in the approved Figure package, not callback-count claims. Add end-to-end regression controls to this owning file before the integration correction. Require one retained warning for the exact sourced update, no warning without a demand, independent recompilations each retaining their own sourced warning, and successful HTML explicit Figure selector read without implicit paged caption demand. This regression paragraph authorizes tests, not blanket removal of existing HTML fail-closed contracts or a new public classifier. Productive corrections must first specify their exact transaction/admission boundaries in the corresponding owner.


### Approved explicit filtered reads under HTML

The already approved Figure package requires explicit typed filtered reads under both targets. The orchestrator classified the frozen HTML failure as the existing blanket filtered-counter veto, not a new producer channel or phase: `prepare_html` rejects selected_filtered_counter and `Session::stabilize` repeats that rejection despite already accepting Html with module=None. Supersede the earlier position-only/blanket-veto clause only for actual observed filtered-counter demands using this existing shared protocol. Select and latch actual filtered reads alongside existing position/page observations; retain Html target/features, the original HTML introspector seed, shared five-attempt budget, full recorded reads/replay and candidate-local diagnostics. With module=None never enter Paged layout; implicit Figure finalization still skips Html, and no Figure/type matching logic belongs in L3. Preserve ordinary unselected producers and errors, page/position-specific comparisons and terminal marker stripping. Require HTML Figure typed read, non-Figure filtered update/read, and unconsumed paged-caption panic controls. No new public helper, phase, classifier, syntax test or weakening of fail-closed observation proof is authorized.


### Accepted candidate publication Sink

`integration-tests-8` observed both intended warning regressions and the HTML explicit-read failure. The pinned `agents/counters/warning-origin-oracle/receipt.json` under Figure implementation evidence measures identical Hebrew-zero messages at distinct source lines1 and2, both retained. Preserve each independently executed retained producer/finalizer transaction and its original Sink; replay, discovery and replaced transactions remain discarded. At accepted candidate publication, pass all retained producer contributions in existing causal order followed by the retained phase contribution through one fresh existing TypstSink using its existing warn policy. This restores existing span/message Sink semantics at the publication boundary; it is not a new dedup predicate or a global/cache publication flag. Different source spans/messages remain distinct, first diagnostic metadata and order remain intact, and independent compilations own independent publication sinks. Preserve pending errors/nonconvergence diagnostics and all existing transaction execution. The ordinary paged generation buffer has its own exclusive owner.


### Terminal failed demanded transaction

Frozen exact-source controls in `agents/counters/warning-error-oracle/{receipt,selected-receipt,candidate-selected-receipt}.json` under Figure implementation evidence show that the pinned oracle retains the Hebrew-zero warning emitted before panic(AFTER_WARNING). The isolating source selects an unrelated Heading filtered read while a Figure caption demands the failing Figure fold. Verified frozen candidate-two SHA12265c0a05442725b28274a3d11f89ee8c93578d4dc308bcd224178249e0396a returns the original panic but zero warnings, whereas pinned oracle returns that panic plus one sourced warning. This is an actual diagnostic-loss RED, distinct from earlier warning duplication.

At the terminal runtime-introspection Err boundary, publish retained current producer diagnostics and that failed current runtime transaction's returned phase warnings through the same existing fresh candidate Sink. Preserve self.fail's pending-source-error precedence, no retry/convergence/budget changes, and all warning metadata/source order. Discarded iterations and validation replay still do not publish. Do not infer successful result from warning presence or alter unrelated fail-closed proof boundaries. Regression covers implicit ordinary failure, selected implicit failure and explicit Figure read failure, always retaining the original sourced panic plus prior warning.


## Approved emitted Figure reference supplement phase

The human approved `reference-phase-approval.json` under the Figure implementation evidence on 2026-10-20, binding frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2 and its separate no-child refinement. ADR-0127 items1 and3 are satisfied for precisely those three public transport functions and the described contextual Ref supplement contribution. This owner coordinates actual generated producers with the existing Session; it owns no selector matching or Figure preparation.

First restore the exact archived explicit/callback INNER_PREPARATION sources as real Paged pipeline regressions. An eval-only assertion is no longer the correct phase after this approval; preserve the archive and expected language panic, and prove it at the actual retained contribution boundary. Generated reference producers use the same immutable candidate, shared occurrence arena, exclusive retained frame, five-attempt budget and diagnostic publication as existing producers. Select their separate private admission bit through the exact approved L1 classifier when they actually execute; ordinary Query/filtered-read getters retain their contracts. Collect/order/substitute recognized prepared-reference actual children through the approved borrowed/map facades while generic Ref entity traversal stays terminal. Source, lexical styles/rules, original Ref morphology, target/features, aliases and stable nested producer IDs remain causal inputs; do not use a new arena, fake counter observation, hash identity, result cache or semantic Context query element. Complete walk/action indexing and demanded histories follow actual expanded content, and every retained error/warning remains in its current transaction. L0-first regression/proof gates precede productive edits.


### Actual generated-reference call ancestry

Source measurement before decision: Session::realize at `03_infra/src/pipeline/context_stabilization.rs:440` recursively executes newly emitted child producers before the attempt loop; duplicate-ID detection does not bound a self-reference producing fresh IDs. The existing language call-depth owner provides Route::increase at `01_core/src/entities/world_types.rs:348`, MAX_CALL_DEPTH80 at :372 and check_call_depth at :419. The orchestrator classified reuse of that existing protocol as within the approved cycle/depth requirement, not a new limit or phase.

Retain a private reference_depth on each Producer, inherited from its actual parent plus one only for an exactly classified generated reference producer. Ordinary unrelated producers preserve depth0; ordinary descendants preserve their real generated-reference ancestry without adding fictitious calls. Execution and validation replay build the existing Route with that ancestry through Route::increase. The existing Route::within implementation at `01_core/src/entities/world_types.rs:439–465` accepts an exhausted root segment with outer=None; therefore incrementing a root alone does not represent nested calls and cannot bound this ancestry. The frozen reference-tests-2 integration run observed a host stack overflow in the linear/branching cycle control, while the unchanged current-product CLI inputs and hashes are recorded in reference-build-2/receipt.json and the adjacent frozen replay receipts. Construct a real child segment with Route::extend(base.track()) for positive reference depth and increase it depth-minus-one times; preserve the original root route unchanged for ordinary depth0 producers. Execution and validation replay must use the same actual child-segment protocol, including the existing sourced terminal depth-error publication and sibling-discovery stop. The L1 native reference owner invokes the existing check_call_depth and anchors its ordinary detached error at the captured Ref source. Do not impose a second arbitrary depth cap, compare language targets in L3, cache cycles or count discarded frames as active ancestors. Require real self/mutual cycle termination and a finite-chain positive control, preserving errors and diagnostic transactions.

When actual generated-reference ancestry exceeds the existing Route call-depth limit, retain that producer's original depth error and stop further DFS discovery through the existing terminal/impossible path. The decision is the Route protocol plus exact producer identity, never error text or an arbitrary count. Missing-label/numbering/other provisional errors retain normal invalidation/retry and must not globally abort sibling discovery. Branching recursive-reference controls require bounded termination without enumerating exponential siblings.

Rust integration controls additionally require a labeled raw Ref query entry at its real execution Location, no generated Context query membership (ordinary source Context remains), explicit/callback sourced preparation panic, HTML implicit nested-panic exclusion, linear/branching depth failure and a finite nested-reference positive chain. These are the approved language/target/termination obligations, not callback-count equivalence.


### Emitted Figure context topology

Source measurement before decision: pipeline.rs:402–500 collects through generic bottom-up map_content without an explicit Figure arm; that mapper visits resolved_supplement even when caption=None. The frozen reference-build-2 raw-context-two-references replay records contextual instability from a template Context ID absent from canonical context_block_locations; the caption-plus-references control succeeds. Only emitted Figure children are producers: body, then resolved supplement when caption exists and numbering is default/Str, then caption. A raw unused selected template is not an execution root or a substitution site. Preserve source and semantic snapshots, normal ancestor styles and exact prepared-Ref facade boundaries.

Use explicit actual Figure descent in collection and substitution, including ordinary visual wrappers, and preserve Realized.original without traversal as the existing L1 mapper already does. Ordering must mark duplicate IDs only for collected actual members; raw template metadata does not poison uniqueness, while two actual occurrences of the same ID still fail. Do not add phantom index entries, relax active topology or change generic L1 mapping/public contracts. Add a direct unused-template control, a rendered-caption positive control and distinct-actual duplicate preservation before implementation; real language repeated-reference acceptance remains mandatory.

The separate nested-grid-dormant-template-receipt.json under agents/counters/reference-extra-controls records an actual generic Grid wrapper: pinned oracle succeeds without executing the dormant context panic; frozen reference-build-2 product returns contextual stability failure. Add that exact actual Paged regression. B's Figure owner must prune dormant templates from the visual occurrence while retaining the raw original snapshot, making generic traversal safe for all containers; this owner's explicit wrapper branches are guards, not a replacement for P1037's exhaustive generic fallback or a claim of complete wrapper enumeration.


### Closed actual-child ordering delegation

Measurement before decision: sealed lint-final-3 reports a new V16 at context_stabilization.rs:129, the wildcard false arm selecting generic ordering rather than explicit children. Total error count19 masked this addition because the original pipeline callback V16 was independently removed by its closed complement. The complete baseline diagnostic review records both changes; do not compensate a new error with an old removal.

Replace only that new wildcard with the closed Content complement to the explicitly borrowed actual-child cases, using entities/content.rs:233–1309. False here selects the existing generic context traversal and actual-member duplicate checks; it does not classify a delegated container or primitive as an empty leaf. All delegated current variants retain their existing semantics and generic recursion. Future Content additions must fail Rust exhaustiveness until explicitly assigned to borrowed descent or generic delegation. No suppression, parser escape, neutral new alternative or unrelated baseline collector repair.


### Captionless Block actual pipeline regressions

The measured existing hidden-reference-parent rendered mismatch predates the Ref extension; the orchestrator classified its narrow captionless Paged default Block product correction as continuous parity in the same recipe phase. Add actual source controls before implementation under figure_captionless_default_block_: prune raw body Context/nested Figure panic through ordinary Block None replacement; prove Block body is actual Strong plus Parbreak rather than a Figure wrapper; preserve one typed automatic action, canonical label/query Location and position/page capture; retain reference output; and ensure an own Figure replacement does not gain an artificial Block. No pipeline business logic, public API, HTML implicit path or caption-number calculation changes belong to these tests. Default breakability reflection/show-set and full caption synthesis remain separate debts; do not turn a narrow GREEN into complete parity.

The first captionless RED observed two genuine causes (Block callback never reached; hidden body panic executes) and two confounded preservation controls (underscore text shaping and nonbreaking-space reference text). Preserve original evidence. Use plain-word fixed markers, normalize Unicode whitespace before exact reference number-token comparison, and retain all typed Location/history/page assertions. Add a normally reached Context+nested Figure body with original label positions and both typed histories, proving the actual product is not skipped by alias deduplication. These source-control refinements do not authorize implementation before the orchestrator's second RED run.

The second RED's reached-body control also asserted an unpromised final counter-history cache. Source evidence queries.rs:305–314 reads an already demanded Figure history or registry fallback; figure_counters.rs:55–81 materializes only actual caption/reference demands. Explicit filtered .final reads independently fold and record their real dependency, without publishing a shared snapshot cache. Therefore captionless metadata-only controls assert exact actual <count> Metadata Values [1] or [[1],[1]], and explicitly require absent undemanded intr.counter_final_values history. Retain all canonical Figure/label/Context/position assertions. The separate actual Ref control retains Some([1]) because that Ref is an actual demand. Never manufacture a cache or weaken language values to repair this fixture. Root's RED3 precedes implementation.

Atomic Strong, Empty, source-bound Empty, immediate multi-child Sequence and strong(empty) default Block morphology are fixed by the two pinned oracle receipt sets. Add actual callback assertions for one-child Parbreak Sequence, flat six-child text/Strong Sequence, and retained Strong(empty), not only the original Strong atomic case. Source wrappers remain transparent to language fields. These are fixed independent language expectations, not an implementation mirror.

Add fixed actual language controls for Block None child Heading count0 and identity/labeled-identity child count1, bound by ordinary-block-hidden-heading and ordinary-block-labelled receipts. Preserve the separate open Block-label membership gap; do not assert that a missing label projection is correct. Require Heading/Page reference layout errors at their measured lexical Source columns, retaining exact messages/hints and all earlier actual Context/typed Figure controls.


## Closed producer recognition and transparent Source roots

Measurement before decision: clean baseline4382891f0d5f0839669597cb5c90a98f24cfb8bd context_stabilization.rs:263–269,390–396 recognizes a bare ContextBlock or the immediate ContextBlock body of Located. entities/content.rs:233–1309 contains120 alternatives. pipeline.rs:429–430 overwrites duplicate map entries, while Session::new:381–407 keeps first encounter order and retrieves each Arc/chain from the completed map. The immutable source catalogue, actual enum-growth diagnostics and two actual Source-root failures are retained in lab/results/v16-corrections-2026-10-19/batch-g/{structural-baseline-2,focused-source-red}/.

The Source failures expose Located(Source(ContextBlock)) below generic Underline/MathFrac: collection includes the producer, but L1's suppressed Located-body root callback propagates through Source, and immediate-body recognition loses it. Direct ordered Located→Source descent and Source→Located work. The Source nucleus permits no orientation restriction. Current ordinary/Ref constructor paths traced generally produce Source(Located(bareContextBlock)); frontend generation of the reverse orientation remains unproven. This is an existing domain-contract correction, not a measured frontend parity improvement.

At each of the two recognition sites, retain bare ContextBlock recognition and consume only transparent Source layers at a Located producer root before its ContextBlock/nonproducer decision. Retain the exact producer Arc, Location, origin and effective chain. Both outer and inner decisions must enumerate the complete closed Content complement. A Source alternative in the inner nonproducer catalogue is unreachable after Source-root consumption; it must not bypass that consumption. Do not peel Styled/Realized, change generic mapping or add another producer-admission channel. None means no producer at that callback, never an empty subtree.

Actual duplicate IDs still make ordered topology impossible. The unselected legacy fallback retains first-ID order plus last-collected Arc/chain, one initial execution per ID, and its existing error/warning policy. It gains no retry or retained-cycle budget. No equality, hashing, Debug fingerprint or pointer-based producer deduplication is permitted. Preserve captured ShowRecipe snapshots, inner Styled overrides, Figure actual-child order, hidden-original exclusion, occurrence frames, reference Route ancestry, descendant invalidation, five attempts and retained diagnostics.

Acceptance includes generic bare/Located/source ordering in both orientations, duplicate rejection, first-order/last-payload+chain and one legacy initial execution, current generated Ref plus ordinary Context capture, and all forty approved Ref controls. Each original recognition decision requires attributable enum-growth evidence. Any new wrapper admission or public/phase/default/compatibility change requires separate classification; the bounded Source correction and nominal closure are continuous internal flow under ADR-0127.


## Approved physical running-content candidate integration

Evidence before this decision is the immutable source-qualified current-pin
SetPage baseline and ambient20/30 measurements under
lab/results/set-page-header-2026-10-21. Header-only/repeated/footer/context/style
and duplicate-label failures are language observations; the twelve existing gates
passing is structural preservation only. Actual seven semantic REDs and one inert
template control are recorded in
lab/results/set-page-header-implementation-2026-10-21/initial-red/receipt.json.
Human approval of review SHA256
cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039
is recorded in that implementation directory's approval.json. ADR-0127 items1/3
are satisfied for exactly the frozen five L1 facades/types and their phase/order.
This section supersedes earlier preserve-phase/default boilerplate only for the
approved physical channel. It adds no public L3 API, source reevaluation, new
solver/budget, generic observation/equality rule, or unrelated behavior change.

Use the approved owned Content/source plan and immutable request/resolution/trace
facades as opaque L1 transports. L3 coordinates the actual World, Engine, retained
arena frames, existing Context Session and candidate diagnostic sinks; no selector,
Figure-kind, semantic child-edge, temporal action, style-rebase or render business
logic moves to L3. The pure layout renderer executes no language closures. The
initial canonical body identity comes from the existing reached realization and
its protected operational templates; preparation publishes no invented body index
or PageStore. Only a matching actual trace/store pair builds the complete Paged
source-derived index. Global Bundle input uses actual ordered target metadata,
features and document seeds; HTML has no physical requests.

Keep the Session alive through physical discovery, including initially unselected
body producers. Actual Pending requests select the approved physical contribution
channel, without fabricating filtered/position reads or running unselected source
producers a second time. Raw operational SetPage/PageRun templates are excluded
from body discovery/substitution; realize only requested actual insertions in one
retained frame per actual request lineage/key. Inspecting a dormant template for
allocator reservation is not execution, query/counter admission or a new producer.
Register actual generated Context descendants in the same Session, preserve their
source/style/target/features, and rebind expanded Content through the approved L1
function under the shared frame. No synthetic operational Located wrapper/Context
query element or page-derived numeric Location is allowed.

Reuse a body plan only with unchanged actual retained contribution versions and
immutable source provenance. Content Eq/hash/Debug, equal counters/page counts or
freshly reconstructed closure equality are not evidence. New source generations
invalidate prior plan/request authority; pure layout reflow of the same retained
plan keeps its authority. Candidate-local stores/transcripts must be complete and
agree with current geometry/effective page environment; no partial candidate may
reach shaping/export. Maintain the existing five contextual/page attempts and
separate eight math/par.line request attempts, no hidden sixth execution or second
context solver. Discovery/replay/replaced sinks remain discarded; publish only the
retained final transactions through existing Sink policy, preserving source error
precedence and independently sourced identical warnings.

Own only this consumer's integration. Its existing private helpers and a private
descendant page_running module may coordinate the protocol; each new productive
file has its own canonical owner. Other owner's clauses do not authorize bodies
here. Preserve legacy facade contracts, non-running paths, PageRun lexical restore,
page snapshots and numbering, source parents/original morphology, HTML semantics,
Bundle validation/resource/write policy and all unrelated F/G debts. Require actual
focused GREEN and frozen CLI/counter/query/style/error/alias/dormant/template and
rollback controls, followed by full twelve-gate validation, canonical paths,
strict forward/reverse lineage, ownership/Nucleus pins/DAG and complete diagnostic
comparison. No suppression, test relaxation, opaque catchall or convenience global
hash reseal is authorized.

### Actual physical-channel selection and retained ordinary Context reads

Static evidence: the current Session selection originally targeted filtered
counter/reference/HTML reads; the validation loop skips unselected producers.
An initial source Context can fail its query-cardinality assertion before reaching
a later filtered counter read (initial physical-page RED programs at
`lab/results/set-page-header-implementation-2026-10-21/initial-red/tests.log`).
Once L1 issues actual running requests, retaining that query failure forever
would prevent the new complete candidate from correcting it. This is internal
selection/transaction orchestration, within the already approved physical phase.

Actual Pending running requests select retained active body producers for the
existing recorded-read validation, without creating any fake filtered read and
without discarding/reexecuting discovery transactions merely to select them.
A retained producer with unchanged actual reads/location remains retained; a
query-only producer with changed reads is invalidated for the next existing
attempt. Physical and body descendants use the same Session/arena/history.
Preparation of the approved physical route retains requested features/full-error
options from discovery; do not evaluate the selected route with an unrelated
legacy feature set. Preserve the five-attempt limit and original error precedence.


### Outline phase read transactions in the existing candidate

Source measurement and the omitted-phase evidence are owned by
`00_nucleo/prompts/infra/pipeline/context_stabilization/page_running.md`, with the
unchanged L1 algorithm specified in
`00_nucleo/prompts/core/compiler/eval/rules/outline.md`. A physical candidate may
introduce real Context descendants and execute Outline own-show reads before
planning. Validating only ordinary producer nodes would omit those actual phase
reads and could accept stale output.

Compose the private physical child's active Outline transaction read proof into
this owner's existing valid/closed decision. Require every actual first/focal run's reflexive proof against
its recorded input and validation against the same complete candidate used
for all ordinary producers. Preserve captured per-read styles/rules/guards/file,
source errors, original precedence and the current five-attempt history. The child
retains the transaction and diagnostics; generated actual Context descendants are
normal Session producers included in its existing topology/order. No synthetic
queryable producer, new Session, budget, public API or observation rule is allowed.
A failed proof invalidates the same next outer candidate; an opaque/unproven
reflexive result must not become a cache authorization. Keep the ordinary
nonphysical route and its diagnostics unchanged.

## Approved ordinary Repeat physical candidate eligibility

Measurement before decision: the source-bound paired source receipt `lab/results/nested-fractions-repeat-2026-10-21/prototype-2/zero-source-phase-pair/measurement.json.gz` records no-header STATE1/COUNTER1/LABELS1 versus oracle0/0/0 and the empty-header stability rejection with immutable source/binary/state/UTC. The human approved exact review SHA-256 `2609d1a9231b1b8a665f2d8284b7343366a1bc44b0471fdd6008b0f180a52bb2` with “Aprovar a integração física de repeat”; `lab/results/nested-fractions-repeat-2026-10-21/phase-approval.json` records that authorization.

For actual source-owned Repeat contributions in paginated preparation, retain the existing Session through physical discovery and derive the semantic candidate from the matching actual source plan/trace/store even without a running marginal. Reuse the existing facade/channel; no public signature or new solver. Layout-only reflow preserves source authority; actual accepted contribution replacement invalidates/rebinds it under existing rules. Do not select by an unexecuted template, fabricated observation or source reevaluation. A reached contextual Repeat contribution joins the same authority. Keep five attempts and independent callback budgets, sinks/errors/warnings, target separation and all prior noneligible paths.

Require causal integrated RED→GREEN for actual no-header and empty-header sources, positive/zero/nested effects and produced Repeat, with HTML/Bundle/legacy controls and full immutable corpus/lineage validation. This approved section supersedes prior preserve-phase/default restrictions only for the named Repeat eligibility extension.

The corrected real-source pipeline RED is recorded in
`lab/results/nested-fractions-repeat-2026-10-21/real-repeat-pipeline-phase-corrected-red/receipt.json.gz`:
with the declared DejaVu fixture installed, no-header returns effects1/1/1
instead of0/0/0 and empty-header rejects contextual stability. This source has
its reporting Context OUTSIDE the zero-copy Repeat. Preserve that active
Context and its strict location/read checks; do not call it an inactive body
producer or infer a missing-location exemption. Source-qualified diagnostics
must establish any further stability cause before changing that decision.


## Preserve physical position projection and actual transaction selection

Measurement before decision: `phase-coverage-diagnostic/test.log` under
`lab/results/nested-fractions-repeat-2026-10-21` reports for each actual paired
source that the strict physical source index carries one active position,
whereas the document carries four raw positions. Both unconditional parent
position injections replace the former with the latter. Existing pagination
coverage then remains false through all five attempts. The empty-header source
retains its outside Context Location and strict read proof after attempt one.
The no-header source reports its actual outside producer selected=false.
The exact code state, UTC and diagnostic hashes precede this decision in
`agents/acceptance/zero-source-phase-proposal/before-position-selection-correction.json`
under the same evidence root.

These measurements classify language source-membership/state/query results as
parity; internal raw-position transport and selection are causal mechanics.
They do not prove all remaining contextual stability causes closed. In
particular, no-header candidate Context Location differs from discovery
topology and may expose another failure under unchanged identity validation.

### Exclusive internal correction

When the existing physical facade supplies a matching source-derived candidate,
retain that candidate's already projected position store through page snapshot
and numbering realization. Raw document positions include computational source
preparation that contributes no actual Repeat copy; they cannot overwrite the
validated semantic projection. Keep raw document geometry unchanged and preserve
the nonphysical route's existing document-position injection. Retain strict
coverage, page stability, source/arena/trace/store validation and read replay.

Every actual physical candidate, including a completed no-request Repeat body,
selects retained active Context transactions for the same recorded-read
validation already used by Pending running discovery. Selecting the transaction
does not discard or execute it. Existing reflexive/current replay and exact
Location equality alone decide invalidation; the next existing attempt may
reexecute only invalidated transactions. No synthetic observation, missing
Location exemption, new budget, source reevaluation or public API is authorized.

The actual paired pipeline tests remain the causal acceptance gate. A remaining
strict identity failure requires a separate measured source-transport correction;
never turn the present Location mismatch into accepted equivalence.


## Reexecute a retained producer at its actual physical input Location

Measurement before decision: the corrected source-qualified phase receipt
`lab/results/nested-fractions-repeat-2026-10-21/prototype-3/approved-phase-controls-corrected/receipt.json.gz`
ran 2026-10-22T10:38:58.955518+00:00 through
2026-10-22T10:39:18.604479+00:00 with unchanged frozen inputs. The actual
no-header source returns position x=0pt/y=0pt while pinned oracle returns
x=158.33pt/y=16.36pt; both retain page1 and effects0/0/0. The otherwise identical
empty-header source returns the actual oracle position. These are language
Location observations; its separate geometry mismatch remains a separate debt.
The earlier source-qualified trace mapped no-header Context0 to actual physical
Location8 while discovery used its legacy topology. `realize` reconstructs that
legacy topology for every invalidated execution instead of transporting the
actual prior physical input's established Context Location. The existing terminal
policy can return D5 with stable reads, so final-state success alone did not
refute this position failure. Exact pre-change state and receipt hashes are in
`agents/acceptance/zero-source-phase-proposal/before-physical-location-tests.json`
under the same evidence root.

Stage actual-source position and private retained-transaction proof regressions
RED before implementation. For a retained current producer in the selected
physical route, use its Location already established by the actual current input
context map when present. New actual producers without an input occurrence
continue to obtain their real provisional topology Location. This transport does
not allocate a Location, infer producer continuity from an integer, replace a
closure or alter execution identity. The actual retained Producer/occurrence
frame remains its owner; a later source-generation change remains subject to the
strict current candidate Location equality and recorded-read replay. Keep the
nonphysical route's topology policy, both unconditional replay transactions,
error precedence, missing-location failure and five-attempt terminal policy.
No missing/mismatching Location exemption, body reevaluation, additional pass,
API, or changed diagnostic policy is authorized.

Acceptance requires both actual paired source position observations, original
effects0/0/0 and source markers, plus exact final execution/candidate Location
agreement with reflexive and current read validity. A newly generated producer
still requires actual source discovery; dormant templates stay dormant. Existing
header, changed-generation, Bundle and callback-budget controls remain required.


### Actual retained Location transport RED

`lab/results/nested-fractions-repeat-2026-10-21/physical-location-red/test.log`
records the staged actual-source RED: private final physical return maps its
Context to Location8 while its retained execution remains Location3; the actual
no-header pipeline reports POSX0pt/POSY0pt. The empty-header position source and
both original effects0/0/0 controls pass.
`agents/acceptance/zero-source-phase-proposal/before-physical-location-transport.json`
binds exact code state, dirty-file list, UTC and RED log hash before this change.
This runtime proof confirms the transport cause and refutes final-state success
as evidence of matching execution identity.

Limit actual-input Location transport to an already retained current Execution
of the same active Producer in the enabled physical route with actual Paged
target. Its exact producer object/occurrence frame remains retained. New nodes
without a current Execution, absent input occurrences and HTML target producers
keep existing real provisional topology lookup. Do not synthesize, allocate,
compare producer payloads, accept a present mismatch or transfer an old integer
id's execution into a replacement Producer. Existing current candidate equality
and unconditional reflexive/current observation replay remain strict after the
transport; the five-attempt terminal policy is unchanged.


## Local Repeat allocation observation and source control — 2026-10-22

`lab/results/remaining-parity-2026-10-22/baseline-nine/measurement.json.gz`
records source-effects-local-repeat with exact HEAD/dirty state, immutable
source/font, source-bound product and pinned oracle provenance. Its prepared
source allocation remains RED: the local Repeat has 23 X visuals instead of
6, while the outside Context is active. The eval-only dump in
`eval-content-before-fixed/receipt.json.gz` retains the initial Context node
and cannot prove the final layout certificate's refusal cause.

Keep an owner-local source-derived control that evaluates the exact bounded
state/counter/label Repeat program using only its injected portable font,
then calls the existing contextual preparation and inspects its actual final
Content plus settled document. This is evidence before selecting a width
correction, not permission to treat unknown Context output as zero or execute
closures during preflight. Preserve strict source authority, actual Context
reads, callback budgets, phase/target boundaries and warnings. Diagnostic
printing may accompany a local failing investigation but must not remain in
final productive or test output; archive that raw measurement in the lab.
No public API or new preparation phase follows from this observer.


The retained final prepared-content observation reveals only the positively
known inline classes after Context substitution, while the settled document
still paints 23 X text leaves and TAIL at x52.0751953125pt. Preserve this
actual count and location with its source-bound stage-2 receipt; it does not
prove state duplication. Before changing arithmetic, record the actual
injected provider's source-token widths and the existing ordered sum/cursor
calculation. A floating-point refusal is only a hypothesis until those
measurements and a meaningful private allocation RED reproduce it. The
withdrawn earlier Python rounding model remains refuted historical evidence.


The actual-source control must assert the pinned oracle's five X visuals and
STATE1/COUNTER1/LABELS1 together, rather than stopping at successful preparation.
Preserve all font/source/preparation preconditions. This is a genuine source
RED if the unchanged source still emits 23 X after the statement-whitespace
repair; derive its cause from the separately printed actual width budget,
never by special-casing these source names or expected counts in production.


### Refuted source-count and rounding hypotheses

The stage-3 source observer's expected six X was a counting error, not an
upstream requirement. Fresh `baseline-nine/measurement.json.gz` shows five
native X glyphs: BEFOREX plus four standalone X. Preserve the erroneous
six-count receipt historically; the control now requires exactly five with
STATE1/COUNTER1/LABELS1, without changing the source.

`red3-infra-local-repeat/test.log` records actual injected fixed width
226.95556640625pt in a224pt region, free -2.95556640625pt and final cursor
232pt exactly. This refutes floating-point overshoot. Native breaks before
LABELS1: first-line fixed occupancy174.947265625pt leaves a49.052734375pt
Box, which produces five actual X copies with gap2pt. Full source tree
substitution and callback/authority checks remain untouched. The allocation
owner must choose complete known word boundaries before regional shares;
unknown output cannot be assigned zero and no arithmetic epsilon is allowed.


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

## Approved readiness physical eligibility — plain numbering and heading Outline

### Measurement and human decision before code

readiness-fixes-2026-10-28/{number2-build.json,number2-build-source.json.gz,
number2-extras/comparison.json,article-route-before/receipt.json,
outline-route-debugger/receipt.json} bind exact binary/source/fixture hashes,
HEAD212aed0449150dd296ceea13fc2be665cc5db01a dirty stat and UTC. Product
92c6346f8562438618e2cd7ba57166423316ee26fc08aa597ac357c482859fd7;
oracle64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17,
executable22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The terminal17pt/DejaVu page loses lexical style after restoration; the selected
Ref source bypasses ordinary Outline realization and paints an incorrectly
numbered title without fill. Actual debugger function entries/piles distinguish
both routes and full compiles, rather than inferring them from appearance.
Readiness review documents exact source anchors and independent remaining debts.
These are language/style/source observations, not upstream Rust-mechanics parity.

The human directly approved “Aprovar a ampliação proposta” in this chat;
physical-number-outline-approval.json records authority, UTC and frozen review
SHA-2569d52d041904a94221cae76e4d9d394afd3a758be31e0e4b8845c6839a0dcdc87
with its owner/receipt manifest. ADR-0127 phase gate is satisfied ONLY for that
reviewed extension. This section supersedes earlier preserve-phase/noneligible
clauses only for its named heading-Outline/plain-number eligibility; prior
Repeat approval is not used as authorization. Existing APIs and other phases
remain unchanged. The approval does not certify tests, rendering or readiness.

### Exclusive implementation and acceptance

Initialize the same existing physical-channel latch for the approved reached members, and apply its widened reached predicate through the existing post-discovery/post-replacement hook. Use the current actual Bundle paginated-target partition; retain standalone HTML's target barrier. Keep the existing Session/arena, retained producers, Outline read proof, five/eight budgets, diagnostic/error/source policy and current terminal result. Own source-derived pipeline REDs for terminal17pt/bold, unary/total/top/bottom numbers, actual Outline with/without Ref and one canonical query result; preserve hidden/dormant/None/Content and previous physical tests. No syntax/name/error-text heuristic, public facade or new solver.

Stage real current-API REDs before code. Revalidate the approved exact source
variants, source words/pages/0.001pt and separate PDF/SVG, actual styles/query/
Location/effects once, causal errors/warnings, aliases/dormant/hidden producers,
None/Content, all old marginals/Repeat/rollback/notes/callback/Bundle/HTML gates.
Retain strict authority, coverage, Location and reflexive/current observation
proof; no extra attempt, normalization, suppression or weakened expectation.
V15/V26 before resealing, forward/reverse lineage, twelve technical gates,
full original historic corpus and JPEG time/RSS preservation remain mandatory.
Unresolved numbered-fill, justification, explicit-footer/bold-PDF and all other
readiness debts keep their separate receipts and acceptance; no general claim.

## Readiness: preserve total for a source with no final-map location

Measurement before decision: readiness-fixes-2026-10-28/physical-candidate-source.json.gz,
physical-candidate-execution.json and physical-candidate-physical-preservation.log
bind exact frozen source/L0, HEAD212aed0449150dd296ceea13fc2be665cc5db01a dirty
stat/UTC and commands. The unchanged p1160 PageRun source now paints X1/0,X2/0,III
instead of X1/3,X2/3,III. All35 readiness controls passed before the full L3
suite found this distinct regression. Source counter_updates.rs:249-385 stores
final values per actual location; a document without a corresponding location
can therefore have an empty final map while its realized logical page vector
is present. This parent's logical_final fallback is0, whereas the existing
ordinary paged coordinator already falls back to the actual last logical page.

Within this existing numbering realization only, when the final-map projection
is absent use the existing current document's last realized logical page number,
or0 only if that vector is empty. Preserve a PRESENT zero final exactly, all
counter actions, Source PageRun restoration, arity, source once, warnings and
five/eight budgets. No synthetic location, final map entry, source replay or
public contract/phase. The unchanged p1160 language expectation is a real RED;
require it GREEN and the full page/zero/update/Bundle/Func/shape suites. This
continuous internal fallback correction does not close the separate Bundle
input-view API or OutlineEntry original-source failures.

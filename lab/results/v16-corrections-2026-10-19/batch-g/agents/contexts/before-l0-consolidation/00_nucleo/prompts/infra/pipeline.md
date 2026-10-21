# L0 — infra/pipeline
Hash do Código: 31cd3c66

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline.rs`

## Current evidence

- `03_infra/src/pipeline.rs:174` — `eval_expression_with_document_features`.
- `03_infra/src/pipeline.rs:244` — `eval_to_module_with_sink_target_features`.
- `03_infra/src/pipeline.rs:355` — `collect_context_blocks_into`.
- `03_infra/src/pipeline.rs:409` — `substitute_context_blocks`.
- `03_infra/src/pipeline.rs:438` — `Timings`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline.rs`.

## Owned contract

- Own the evaluation adapters, comemo setup (`Routines`, `Traced`, `Sink`, `Route`), causal context walkers, public expansion facades and `Timings` serialization. Return raw diagnostics; never format them or write output.
- Isolated expression evaluation must not manufacture markup content. Document-context evaluation compiles through the paged owner, evaluates against its final introspector, and appends expression warnings after compilation warnings without exporting.
- Keep compatibility defaults (`full_error=false`, empty features, Paged evaluation) and explicit target/features unchanged. Reject assets outside Bundle with `assets are only supported in the bundle target` and hint `try enabling the bundle target`.
- Discover context blocks through the exhaustive L1 content traversal, accumulating the effective ancestor `StyleChain`. Substitute through the same topology; delegate attempt coordination to its child rather than implement a second stabilization algorithm.
- Preserve public import paths through explicit reexports. Private helper access stays within this module family. Target compilation, resources and stabilization belong to the child owners listed below; this prompt does not own their bodies.
- Timing fields describe existing production phases, including shaping and subsetting. Do not add boundary callbacks, an observation ledger, extra replay or an instrumented default.

## Child owners

- `00_nucleo/prompts/infra/pipeline/bundle.md` owns only `03_infra/src/pipeline/bundle.rs`.
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md` owns only `03_infra/src/pipeline/context_stabilization.rs`.
- `00_nucleo/prompts/infra/pipeline/html.md` owns only `03_infra/src/pipeline/html.rs`.
- `00_nucleo/prompts/infra/pipeline/paged.md` owns only `03_infra/src/pipeline/paged.rs`.
- `00_nucleo/prompts/infra/pipeline/pdf.md` owns only `03_infra/src/pipeline/pdf.rs`.
- `00_nucleo/prompts/infra/pipeline/raster.md` owns only `03_infra/src/pipeline/raster.rs`.
- `00_nucleo/prompts/infra/pipeline/resources.md` owns only `03_infra/src/pipeline/resources.rs`.
- `00_nucleo/prompts/infra/pipeline/svg.md` owns only `03_infra/src/pipeline/svg.rs`.
- `00_nucleo/prompts/infra/pipeline/tests.md` owns only `03_infra/src/pipeline/tests.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Borrowed visual context collection

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/hashes/commands/UTC.1600 local-set cells cause4304 contextual-collection allocation samples versus952 block-no-set; TableElem::map_content9835 versus3430. Skipping only empty second ordering in red/green-gate reduced1394597→1365741 requests(2.07%), below5% gate, and was reverted. pipeline.rs:403–460 uses map_content for observation; traversal.rs:25–45/Table/Cell rebuild even then. Borrowed descent saving copies is a mechanical inference, refutable by allocation test, not a language property.

Collect Sequence/Table/TableCell by reference with exactly map_content's visual projection/order. Sequence in order; resolved Table resolved cells then caption, ignoring raw children/header/footer; raw Table children/header/footer/caption; TableCell standalone visual, else resolved visual, else raw body. Recurse through same collector so Styled/ShowRecipe/Located/Realized/Source retain scopes/projection; every other variant keeps exhaustive L1 map_content fallback, never a partial unknown-as-empty L3 table. No substitution/order/duplicate-id/phase/public signature/map_content semantic change. Require≥5% fewer allocations at1600 cells RED→GREEN, raw/prepared Table/Cell/style/carrier/order/context tests, equivalent output and existing contexts; revert if projection differs or gate fails.

## Evaluation, realization and causal source identities

Timings.eval_ms covers single evaluation plus recipe realization through the compatible facade, not two program executions. Context collection explicitly traverses carrier roots and Realized visual only; Located producer survives suppression of duplicate body-root callback. Bottom-up traversal reenters carriers with causal chain/current visual contexts. Substitution keeps original marker Located before resolved result, preserving here/location/Position; never replace a locatable root by Sequence inside that same Located.

Once the contextual session uses explicit metrics, rg finds layout_with_realized_math_callbacks only at paged.rs:592 and unused pipeline.rs:90 import. Remove only private unused wrapper/import; preserve explicit-metrics/bundle-seed helpers/public reexports/signatures/defaults; continuous flow.

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json` preserves commands/source/UTC/HEAD/dirty tree/binaries and historical9f2b6e8715237cb086899a42873660fe744622e8. Separate document error/warning and expression errors show document source lost/expression mislabeled. eval/mod.rs:609–615 uses world.main for source/base; wiring/eval.rs:66–95 repeats id as primary; cache.rs:58–78 registers/recycles ids; pinned typst-cli/eval.rs:159–179/world.rs:176–190 separates expression/stdin/absent input. Integer equality/global interner are mechanics; shown origin/source/position/relative reads are language. Owner `confirmo`2026-10-06 ratified `lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`.

The approved public facades are:


```rust
pub fn eval_source_with_sink_features(
    world: &dyn World, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
pub fn eval_source_with_document_features(
    world: &dyn World, document: &Source, expression: &Source, features: Features,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

The first delegates to Source-based L1 with empty introspector. The second compiles using exactly the paged owner, passes its final introspector to Source-based eval; document failure returns raw sources without expression evaluation, success appends expression warnings without losing spans/traces. No program reexecution/error-location scan/index rebuild/phase-inference bool. Keep string facade APIs/compatibility; no private paged helper becomes public for wiring convenience.

query-residuos-2026-10-06/before.json.gz records HEAD/tree/UTC/pin/binaries/source/commands/stdout/stderr/status. query-stdin-callback wrongly succeeds instead of error/trace; query-context-success loses generated metadata, nested-error disappears, query-selector-quotes rejects early with P206C; where-level rejects where upstream returns[2]. document-error-invalid-selector establishes compilation precedence; valid-document-invalid/text-selector retrieval errors carry no deprecation. eval-context-reference/eval-where-reference already match through paged compilation. Upstream typst-cli/query.rs:34–66 compiles before retrieve, :86–105 evaluates selector in default Scope then LocatableSelector; query_helpers.rs:488–522 only uses Module while pipeline.rs:182–214 already returns final introspector.

Human `confirmo`2026-10-06 ratified compiled query in query-compilada-aprovacao-2026-10-06.json. Compile complete paged document before selector without exporting, with single ordinary program execution and existing realization/context protocol. No detached expansion/pre-context index/parallel stabilization. Explicit enum distinguishes compilation/retrieval failure, never message/span/string inference. Approved facade:

```rust
pub fn compile_query_context(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<typst_core::entities::introspector::TagIntrospector>, Vec<SourceDiagnostic>);
```
With local Timings call existing compile_to_paged_document_full_error_and_features_with_introspector(full_error=false,Features::default()), project its final introspector and drop document without export. Preserve Source/raw compilation warnings. No second eval/new flag/observer/persistent cache/other-caller pipeline change; algorithms stay with children.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` preserves HEAD/tree/UTC/source/argv/binaries at historical9f2b6e8715237cb086899a42873660fe744622e8. Owner `confirmo`2026-10-07 ratified `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`. Existing transparent ShowRecipe.body delegation also accepts Source.body, without separately executing origin/publishing carrier/assigning Location; Source never observable. Preserve this unit's classification/preparation/serialization, exclusive ownership/static dispatch/no I/O/new signature/trait/flag/phase. C2/C/macro tests/collections/build/lineage remain required.

## Private imports, named results and runtime module

pipeline.rs:39,85,101,127,148 and `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz` preserve source/L0/spans/compiler output/HEAD/tree/UTC/status/diffstat and productive/test/comment audit. Keep compile_to_paged_document_full_error, collect_bundle_entries, require_ua1_image_alt, resolve_pdf_pages_and_tags, first_font_from_doc, resolve_font only cfg(test) for descendants. Remove only unused instantiate_variable_font import, not implementation/instancing path. Other required public/child imports stay. No API/default/phase/formula/acceptance/suppression change; all-target/infra names+suite/build/workspace Clippy/Rustdoc/lineage, no import/comment mirror tests.

pipeline.rs:163,212 and `lab/results/construction/diagnostics/type-complexity-2026-10-15/{before.json.gz,red-clippy.log}` preserve source/owner/hashes/HEAD/tree/UTC/diffstat and lint RED. Only adapt private PagedCompilation consumers: query projects named introspector, eval_source_with_document_features moves it. Preserve public signatures/single compile/Source identity/spans/warning append/final-snapshot eval, no extra compile/export. This Rust representation proves no AI benefit/bug/new parity. Aliases need identical expansion; new/incompatible public type another gate. Nominal type_complexity exceptions only three justified batch items, no module/impl/crate suppression/raised threshold/other repairs. Continuous flow/L0 first/scoped lineage, unchanged controls/denied expectations/selected RED→GREEN/full build/Clippy/workspace/fmt/Rustdoc/lineage; no alias/field-name tests.

Heading evidence/ratified review is `lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`, SHA `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`, supported by lote-a1-callbacks-reflexao-seletores-2026-10-17 final-freeze/target-controls at HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/uncommitted/current bd5ebec285505889e8ff1d4f68804ab147a699f3. Transport Styled changes func, query sees causal wrappers, HTML ignores callback panic; walk.rs:930–945/paged.rs:419–427/html.rs:101–106 prove setup boundaries. Declare only mod runtime for causal Engine/ctx composition, no hub Heading semantics/export/API change. Require H1–H8/bilateral RED; only Heading facade approved, five State helpers separate, no other fields; Styled/callback boundaries stay. Workspace/doctests/build/Clippy/Rustdoc/fmt/V1/V5/V15/V26/reverse/no V16 increase, 12GiB/High8GiB/swap0/jobs1/sequential heavy work; RAM gain measured only.


## Approved reference contribution integration

Measured existing boundary before decision: `03_infra/src/pipeline.rs:402 and :503` retains the old generic transport or legacy assertion behavior. Source inspection establishes ownership; the independently frozen actual L3 INNER_PREPARATION RED establishes the language omission.

The human approved frozen proposal SHA1578cbea7378b739a7c1b0809d3437c0165c2de6d288e40b2db75c9ce7e38ff2, recorded in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/reference-phase-approval.json`, including the separate no-child clarification. Actual L3 explicit/callback INNER_PREPARATION regressions failed on the unchanged pre-implementation source (`reference-phase-red-frozen`), authorizing RED-to-GREEN for this exact phase. Existing unrelated obligations remain.

Collect and substitute only actual selected children of recognized prepared reference carriers through the approved pure borrowed/map facades. Keep generic Ref maps terminal. The marker-only Styled transport contributes no inherited style; exterior lexical styles, Source/Located/Realized visual and existing ShowRecipe captures remain authoritative. Nested generated/source contexts remain in causal order and substitutions rebuild only the selected child, preserving token, Ref morphology, original snapshots and numeric placeholder. Never inspect user marker strings or reimplement selector/Ref business logic in L3.

No additional public contract beyond the exact three approved transport functions, new target inference, second arena, shared result/effect cache, fake observation, weakened diagnostic or unrelated generic visitor change is authorized. Preserve five-attempt stabilization, canonical paths and scoped lineage; validate reached and hidden effects, forward/repeated references, aliases, missing/unnumbered/non-Figure/bibliography/page controls and mixed target behavior.


### Emitted Figure context topology

Source measurement before decision: pipeline.rs:402–500 collects through generic bottom-up map_content without an explicit Figure arm; that mapper visits resolved_supplement even when caption=None. The frozen reference-build-2 raw-context-two-references replay records contextual instability from a template Context ID absent from canonical context_block_locations; the caption-plus-references control succeeds. Only emitted Figure children are producers: body, then resolved supplement when caption exists and numbering is default/Str, then caption. A raw unused selected template is not an execution root or a substitution site. Preserve source and semantic snapshots, normal ancestor styles and exact prepared-Ref facade boundaries.

Use explicit actual Figure descent in collection and substitution, including ordinary visual wrappers, and preserve Realized.original without traversal as the existing L1 mapper already does. Ordering must mark duplicate IDs only for collected actual members; raw template metadata does not poison uniqueness, while two actual occurrences of the same ID still fail. Do not add phantom index entries, relax active topology or change generic L1 mapping/public contracts. Add a direct unused-template control, a rendered-caption positive control and distinct-actual duplicate preservation before implementation; real language repeated-reference acceptance remains mandatory.

Explicit Figure/body-wrapper branches are bounded actual-view guards, not a complete container catalogue. The feature owner keeps raw selected templates in Realized.original and prunes dormant resolved supplement from the visual Figure before generic traversal. Preserve the exhaustive fallback and prove an unlisted generic Grid wrapper with the frozen nested-grid-dormant-template source; do not claim a common-wrapper list alone closes generic traversal.


### Closed substitution decision catalogue

Measurement before decision: the unsealed reference-three lint records two newly introduced V16 sites at pipeline.rs:666 and :695. The pre-existing collector wildcard at :536 remains the separately tracked baseline G obligation and is not repaired by this scope. The new top-down substitution decision delegates every current Content variant not explicitly reconstructed to the existing exhaustive generic L1 map; Located with a ContextBlock body is deliberately delegated to retain its existing marker semantics. The generic node callback replaces only actual ContextBlock or Located(ContextBlock) nodes; every other current Content variant keeps its already-mapped node unchanged. This is ordinary generic descendant delegation, not treating new content as semantically empty.

Spell both complements against the closed Content catalogue at entities/content.rs:233–1309, preserving tuple/struct/unit forms. Each future variant must fail Rust exhaustiveness at these new decisions until classified. No wildcard, allow/expect, parser rewrite, fake leaf, new public facade or repair of the unrelated baseline collector decision is authorized. Existing actual Grid/dormant-template, emitted caption ordering and duplicate-member regressions remain required.

Source transport correction: the frozen reference-four compile records E0624 at pipeline.rs:573 because Content::with_source_span is crate-private L1. Preserve the same existing Source carrier by cloning its public SourceContent fields and recursively replacing only body, retaining span exactly. Do not widen any method visibility or add a public API; this is representation-preserving L3 transport through the already approved public entity.


## Batch G: closed collector callback decisions

Measurement before decision: at baseline 4382891f0d5f0839669597cb5c90a98f24cfb8bd, pipeline.rs:415–539 distinguishes borrowed carrier descent from the generic L1 map callback at509–536. The complete Content catalogue is entities/content.rs:233–1309 (120 current alternatives); traversal.rs:411–438,667–673 makes Source transparent, maps Realized visual only and suppresses a Located body's root callback. Exact source/owner bytes and command state are archived under lab/results/v16-corrections-2026-10-19/batch-g/agents/contexts/before-l0-tests/. These are local mechanical measurements, not new upstream language claims.

V16-099 must name the complete complement to Located, Realized, ShowRecipe, ContextBlock and Styled in that callback. Its115 other node alternatives perform no *local callback action*; the enclosing exhaustive L1 map traversal still visits their actual descendants. They must not become terminal subtrees. Preserve immediate Located producer recognition, Source transport, Realized visual-only collection, the approved prepared-reference child accessor, Figure emitted-child order and dormant-template exclusion. L3 must not decode private reference markers or infer Figure kinds. Styled pushes its delta; ShowRecipe replaces the inherited chain with its captured snapshot. Bottom-up later carrier reentry intentionally overwrites provisional descendant map entries with the correct scope.

A repeated numeric producer ID overwrites its map entry: last collected Arc and chain win. This is not equality/hash/Debug/pointer deduplication. The session owner separately preserves first-ID encounter order on its legacy fallback. A claim that both order and payload always come from the first occurrence is refuted by current source and the existing later-visual-occurrence regression; preserve behavior rather than adopt that imprecise historical ledger wording.

Before replacing the productive complement, add batch_g_ inline characterization controls for generic math/decoration containers, bare and immediate Located producers with Source transport, captured ShowRecipe/Styled chain override, hidden original exclusion, last-map payload/chain, substitution through the same topology, and actual current generated Ref producers. Root-controlled enum-growth compile RED must attribute missing decisions to this site; ordinary controls preserve existing behavior. ADR-0127 class is continuous internal exhaustiveness: no public contract, producer admission, default, phase, compatibility or Nucleus change. Keep all forty approved Ref acceptance cases, diagnostic transactions and known-style/no-Location counter boundaries binding.


The subsequent Located/Source audit refutes any implication that only Source outside Located is a supported producer orientation. Generic collector descent already reaches Located(Source(ContextBlock)); the session owner's two recognition sites must honor existing Source transparency there as well, after independent batch_g_source_root_ RED. No collector traversal or default/phase/public contract changes follow from that correction. Actual frontend reverse-orientation generation remains unproven and must not be claimed as measured parity.

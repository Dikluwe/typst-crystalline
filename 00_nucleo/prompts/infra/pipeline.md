# L0 — infra/pipeline
Hash do Código: 34878872


**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/export/svg-output-files.toml sha256:c113eedd71c8ccb4bedb949e5765dadcc14cc8435a5a5485d6e2ff14eb02a570
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb


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


## Closed context collector decisions

Measurement before decision: clean baseline4382891f0d5f0839669597cb5c90a98f24cfb8bd pipeline.rs:415–539 separates borrowed carrier descent from the generic L1 map callback at509–536. entities/content.rs:233–1309 contains120 alternatives. traversal.rs:411–438,667–673 makes Source transparent, maps Realized visual only and suppresses a Located body's root callback. The immutable source catalogue and real enum-growth diagnostics are retained in lab/results/v16-corrections-2026-10-19/batch-g/structural-baseline-2/. These are local mechanism observations, not new upstream parity measurements.

The generic collector callback must explicitly enumerate the115 alternatives other than Located, Realized, ShowRecipe, ContextBlock and Styled as requiring no local callback action. The enclosing exhaustive L1 map continues to traverse their actual descendants: local no-action never establishes an empty subtree. Preserve generic containers, immediate Located recognition, Source transport, visual-only Realized collection, the approved prepared-reference child accessor, Figure emitted-child order and dormant-template exclusion. L3 must not decode private reference markers or infer Figure kinds.

Styled pushes its delta; ShowRecipe uses its captured chain. Bottom-up carrier reentry overwrites provisional descendant entries with their correct scope. Repeated numeric IDs retain the last collected Arc and chain. The session separately retains first-ID encounter order on its legacy fallback: first occurrence does not determine the final payload or chain. No equality, closure hash, Debug fingerprint or pointer-based producer deduplication is permitted. Both Source/Located orientations remain transparent under the existing Source contract; Source inside Located does not justify an empty producer decision.

Acceptance preserves generic math/decoration descendants, bare and Located/source producers, captured and inner style overrides, hidden-original exclusion, last-map payload/chain, substitution topology and current generated Ref producers. Structural enum-growth evidence must identify this closed decision independently. Existing forty Ref controls, retained diagnostics and known-style/no-Location counter boundaries remain binding. Runtime execution of one grouped branch does not certify every alternative. The scope is continuous internal exhaustiveness and existing Source-contract preservation under ADR-0127, with no API, default, phase, compatibility or Nucleus change.


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

## Ratified complete SVG facade re-export (2026-10-22)

### Evidence and ratification

`lab/results/remaining-parity-2026-10-22/svg-outputs-before/receipt.json.gz`
records exact UTC, HEAD/uncommitted diffstat, fixed executable/build/source archive,
font bytes and actual CLI results for the unchanged two-page finite source and
a separately pinned12-page source. `qualification.json` preserves strict1e-8 SVG
geometry, glyph/paint and complete destination-name comparisons: all10cases RED,
including selected page2 whose filename alone had concealed first-page content.
The human approved the exact additive proposal SHA-256
`d2a1ddfd71d9f741253b00b20c23084aae474e4c9cb979d70f3daa37b40d943d`;
`lab/results/remaining-parity-2026-10-22/svg-pages-approval.json` records the gate.
This is the existing paged compile/export phase, without source reconstruction,
new flag, second evaluation or PNG/bundle behavior change.

Re-export only the approved `svg::compile_to_svg_output_files` capability. Its
selection, template, export and compatibility implementation belongs exclusively
to the SVG owner; keep this facade decision-free and the import topology intact.


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

## PDF warning carrier import

Reexport the approved `PdfExport` type into the PDF child module through this
module's existing private import surface. Public pipeline signatures remain
unchanged; `pipeline/pdf.md` owns destructuring and warning integration. This
root adds no publication, filtering, phase, option or fallback behavior.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

## Approved issue 8954 — exclusive carrier reader migration

The human-approved schema is recorded in approval.json, patch SHA
18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20
under lab/results/construction/diagnostics/issue-8954-2026-10-31/.
carrier-audit-before.json.gz retains exact owner/source/UTC/dirty state. Manual
Table readers currently inspect only legacy children/header/footer: with the
new ordered stream this incorrectly certifies empty/context-free raw subtrees.
In this owner's existing traversal/certificate use Complete visual cells, else
authoritative Some(items), else legacy children/header/footer, and caption once.
No parallel visitation, new allocation, context evaluation, phase or public API.
Retain existing policies and negative certificates for all other carriers.

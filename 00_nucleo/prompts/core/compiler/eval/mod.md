# L0 — core/compiler/eval/mod
Hash do Código: a1a8f0c7


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:727a433beef7710d70729176c3b5a5f851b7e98be320c7594b86e5b269d8d49d
- 00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml sha256:983b118684c0ddab1fbb3c619744957c2e1f93af7708b82c97f665e47410fbf3
- 00_nucleo/prompts/_nuclei/expression-source-identity.toml sha256:a54eea6def2bac71520060ddf084fc434983bb1ca20eb59dde942a0778a130cb
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/show-occurrence.toml sha256:fcd0b9d9148567685d6ad281d549bdcaf1008d151f9de8afc320f76e05ca371c

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/mod.rs:81` — `pub fn realize_outlines`.
- `01_core/src/compiler/eval/mod.rs:92` — `pub fn repr_stroke_value`.
- `01_core/src/compiler/eval/mod.rs:100` — `pub fn repr_value_for_serialization`.
- `01_core/src/compiler/eval/mod.rs:110` — `pub fn pdf_standards_conflict_diagnostic`.
- `01_core/src/compiler/eval/mod.rs:252` — `enum ContextReadRequest`.
- `01_core/src/compiler/eval/mod.rs:331` — `struct ContextRead {`.
- `01_core/src/compiler/eval/mod.rs:345` — `struct ContextReads {`.
- `01_core/src/compiler/eval/mod.rs:738` — `pub struct EvalContext {`.
- `01_core/src/compiler/eval/mod.rs:905` — `pub enum EvalTarget {`.
- `01_core/src/compiler/eval/mod.rs:915` — `pub fn eval`.
- `01_core/src/compiler/eval/mod.rs:928` — `pub fn eval_expression`.
- `01_core/src/compiler/eval/mod.rs:940` — `struct ExpressionWorld<'a> {`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/value.rs:210–214`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:343-362`.
- `lab/typst-original/crates/typst-library/src/lib.rs:374`.
- `lab/typst-original/crates/typst-library/src/lib.rs:397-401`.
- `layout/grid/mod.rs:422-438,574-677,767-768`.
- `model/table.rs:289-305,494-613,732-733`.
- `typst-bundle/src/lib.rs:67-76`.
- `typst-eval/src/call.rs:638`.
- `typst-eval/src/code.rs:148`.
- `typst-library/src/lib.rs:430`.
- `typst-realize/src/lib.rs:1261-1325`.

## Contract

Own EvalContext storage, public evaluation entrypoints, base-scope construction and one document evaluation. Context-read capture/replay, markup-stream coordination and the exhaustive Expr dispatcher belong to `00_nucleo/prompts/core/compiler/eval/context_reads.md`, `00_nucleo/prompts/core/compiler/eval/markup_stream.md` and `00_nucleo/prompts/core/compiler/eval/expression_dispatch.md` respectively. Retain explicit Engine/Scopes/context state and the pinned core and
feature-gate invariants. No child behavior becomes hub-owned merely because
the hub calls or reexports it.

## Entrypoints and scope

Keep eval, eval_expression, eval_expression_with_features,
eval_expression_with_features_and_introspector, eval_with_full_error,
eval_with_full_error_and_target and eval_with_full_error_target_and_features with
their existing signatures, return values, sink behavior and defaults. Existing
default target is Paged; full_error defaults false and Features default empty.
Target Paged/Html/Bundle is explicit and does not enable a feature. Eval receives
resolved configuration, never reads environment or imports infra.

Acquire World inputs once for the single document evaluation. Build a fresh read-only Library base containing the composed stdlib,
predefined colors, text and supplied element-registry constructors. Local scopes
remain mutable/shadowable. The lexical binding std holds an independent clone of
the unshadowed stdlib in a Module publicly named global; do not create a global
binding called global. Ordinary local std bindings can shadow it normally.
Private make_stdlib wrappers delegate composition to
00_nucleo/prompts/core/compiler/stdlib/registry/mod.md and its family owners. Global
registration, namespace membership, color aliases, native PDF/HTML/asset identity
and compatibility math globals remain normative there, not copied into this hub.

The legacy string expression facades build a numbered parse_code Source from
world.main() and the complete expression; the Source-based facade below preserves
the caller-supplied identity. The private per-invocation World overlay returns that
identical Source for its id and delegates every other World service/id unchanged,
including defaults, inputs, plugin/path/package/font/date services. Do not mutate
the caller's World or recreate its resources. The injected-introspector sibling
clones the supplied snapshot before evaluation; the ordinary feature-aware
entrypoint delegates with an empty snapshot and preserves its default behavior.
Keep parser diagnostics, scopes, route, sink, limits and feature transport.

A produced Func with detached private origin may acquire the producer-expression
span. Never overwrite a known closure/alias/With/factory/contextual origin, change
other Values, execute a function or make origin part of public equality. A Span
without its Source is insufficient for a resolvable call trace.

## Document coordination and dispatch

Propagate syntax errors with hints; markup empty-stars/underscores warnings remain
nonfatal sink diagnostics. Preserve the existing empty-source warning literal.
Run one fresh-scope/context document evaluation, capturing show recipes without executing them. Keep its scope, bibliography styles and metadata. After common code executes, call the pure realization facade; retain the resulting provenance graph for visual layout and semantic introspection. Preserve diagnostics, warning hints, page-column wrapping and forbidden FlowEvent handling. The previous requirement for two fresh evaluations is revoked by the confirmed realization gate.

Expr dispatch remains static and exhaustive. Preserve literal/unit construction
(Percent -> Ratio), lexical lookup, arrays/dictionaries/spreads, blocks, contextual
content and first-error order. Code blocks accumulate through operators::join.
Boolean And/Or evaluate the left operand first, type-check as currently required,
and do not evaluate a short-circuited right operand. Assignment, calls, closures,
control flow, imports, markup and math delegate to their owners below. Retain the
first flow event until the responsible loop/function consumer handles it.

Markup captures SpaceFromNewline only when the whitespace token contains newline,
otherwise Space. Comments/trivia do not erase adjacent origin; Parbreak/Linebreak
remain distinct. Defer eager per-leaf textual show to the rules owner with the
causal lexical style/rule snapshot, so later declarations do not retroactively
affect earlier groups. A ContentBlock held as a value retains raw whitespace
until insertion; local textual-show matching preserves unmatched separators.
Normalization and realized-group matching are specified only by
00_nucleo/prompts/core/compiler/eval/rules/textual.md. Under the ratified smartquote contract, markup preserves SmartQuote and known fields; target render resolves glyphs through the shared pure helper with explicit properties above the active style chain. This narrowly supersedes the earlier eval-time Text projection; HTML remains a separate target and never invokes paged layout.

## Contextual observation coordination

Keep a private empty-per-evaluation log of the operations actually observed,
independent of introspector state. Store the effective request/arguments, span,
current file, location, in-context state, target/features, immutable styles/rules/
guards and complete result or error. Do not reconstruct requests from AST,
diagnostic strings, equal Values or a final global stack. Preserve query/locate/
here, state/counter (including label resolution and legacy requests), Location
and outline helper requests as their typed existing operations.

A counter read whose selector tree contains Element selects the block before
location/label/callback resolution, including failing reads. Traverse Where,
And/Or, Within and both Before/After branches without flattening them. Once
selected, every already-observed introspective entry participates in validation.
has_filtered_counter_reads remains a pure getter. The doc-hidden public
has_location_position_reads getter reports actual LocationPosition requests even
if their result is None/error; page, page-numbering/query/state/counter do not
select that demand. L3 owns the retry/fixpoint choice, not this getter.

context_reads_valid_for replays only recorded requests against the candidate
TagIntrospector with captured causal context and current supplied Engine resources,
delegating operation semantics to their owners. Do not reexecute the block body,
callbacks never demanded or append new observations while replaying. The private
relation comes exclusively from
00_nucleo/prompts/core/compiler/eval/observation/mod.md: Same certifies preservation;
Different/Unproven, success/error transitions or changed full diagnostics invalidate
reuse. Mechanism failures propagate SourceDiagnostic; a stable observed error
does not turn the body into success.

context_nonconvergence_diagnostics uses the orchestrator's real ordered candidate
history and final-attempt requests projected onto it; it does not claim that the
body ran those requests in every round or invent details for opaque/alien reads.
OutlineEntryPage and OutlineEntryIndented remain distinct causal reads, not
counter/state substitutes; indented carries the actual entry/prefix/inner/gap and
private parent styles, while absent parent fails before registration.

## Delegation boundary

- 00_nucleo/prompts/core/compiler/eval/bindings/mod.md owns the reexport hub and its children.
- 00_nucleo/prompts/core/compiler/eval/call_dispatch.md owns argument/call routing/traces;
  00_nucleo/prompts/core/compiler/eval/closures.md owns closure capture/application.
- 00_nucleo/prompts/core/compiler/eval/control_flow.md owns loops/conditionals;
  00_nucleo/prompts/core/compiler/eval/flow.md owns event representation/forbidden text.
- 00_nucleo/prompts/core/compiler/eval/operators/mod.md delegates pure operator semantics.
- 00_nucleo/prompts/core/compiler/eval/modules.md owns imports/includes;
  00_nucleo/prompts/core/compiler/eval/markup.md owns specialized markup adapters;
  00_nucleo/prompts/core/compiler/eval/math.md owns mathematical syntax adapters.
- 00_nucleo/prompts/core/compiler/eval/rules.md owns set/show/realization, with
  00_nucleo/prompts/core/compiler/eval/selector_matching.md and
  00_nucleo/prompts/core/compiler/eval/show_rule_termination.md as independent owners.
- 00_nucleo/prompts/core/compiler/eval/font_dict.md,
  00_nucleo/prompts/core/compiler/eval/bibliography.md,
  00_nucleo/prompts/core/compiler/eval/table.md and
  00_nucleo/prompts/core/compiler/eval/cast.md
  own their exclusive helpers (their full canonical paths are in the owner map).
- 00_nucleo/prompts/core/compiler/eval/repr.md owns morphology formatting;
  00_nucleo/prompts/core/compiler/eval/observation/mod.md and its children own internal
  observation comparison, distinct from both public equality and public repr.
- 00_nucleo/prompts/core/compiler/eval/tests.md and the independent test prompts own
  their exclusive test consumers, not this productive module.

The pure public repr_stroke_value and repr_value_for_serialization facades delegate
to repr without Debug, I/O or deciding structured serialization classes. Keep their
signatures and caller-owned classification. Public realize_outlines and
pdf_standards_conflict_diagnostic facades delegate to rules; they do not transfer
rule or PDF-schema ownership to this hub.

Preserve public contracts, defaults, compatibility and phases. Any ambiguous
change affecting these requires ADR-0127 direction before implementation.

## Single evaluation, recipe evidence and realized index

The measured paragraph-precedence refinement is retained by `00_nucleo/prompts/core/compiler/eval/rules.md`: actual #set origin uses private eval.flow.scope, never style-value inference, and whole-flow Styled scopes factor before paragraph formation. This hub only emits the marker in the lexical fold; rule/preparation logic stays with its owner.

Evaluate common document code once with internal defer_show_rules, preserving scope/causal metadata. Realization uses an independent EvalContext with realizing_show_rules and per-document Locator; historical capture flags do not authorize rerunning source. Capture lexical styles/guards/file without applying recipes in common execution. Existing eval facade retains compatibility and calls pure realization after execution, also available for L3 coordination. Initialize every realization/capture state per EvalContext, no globals. Sibling compiler/realize accesses recipe application through a crate-internal boundary.

Internal pass result transports actual has_deferred_show_rules/has_recipes capture evidence together with content/scope/metadata; do not infer it from source syntax or element heuristics. Without recipes or the separately approved bare-math boundary below, pass common content directly to page-column wrapping, preserving carriers/compatibility without rebuilding an equivalent tree or rerunning eval. Realized Module.content/introspection_content retains provenance tree; introspection sees original and product without cardinality-based choice. Forward complete realization diagnostics/hints/traces through existing Sink/dedup.

mod.rs:888–896 discarded RealizedDocument.introspector and published only visual. Under the confirmed canonical `00_nucleo/prompts/core/entities/module.md` gate retain both: realized visual already has realization facade's wrap_page_columns, recipe-free path uses existing helper. Publish Content carriers before Module internal index setter. Module owner controls validity/invalidation/lifetime; no extra eval/synthetic recipe-free index. Test evaluated Module with show originals/generated plus query/order equivalence to fresh reconstruction.

apply.rs:110–136 protects only RuleId during callback; EvalContext:285–405 lacks occurrence pairs. show-recursao-auditoria-2026-10-06 measures four wrongly accepted infinite generations; apply owner received human confirmation. Store/init private Location/RuleId map for in-progress expansion/completed product and depth 0; apply owns algorithm under pinned show-occurrence. Depth follows active transformations only. full_error keeps initially empty ancestor-form vector to classify depth failure without morphological termination. Apply advances next_rule_id above captured snapshots before local recipes. No public field/method change.

## Source-based expression identity and empty-markup warnings

`lab/results/construction/diagnostics/diagnostic-paths-2026-10-06/identity-next-audit.json` preserves commands/sources/UTC/HEAD/dirty tree/binaries at historical9f2b6e8715237cb086899a42873660fe744622e8. Document error/warning loses source while expression incorrectly inherits document name; eval/mod.rs:609–615 and wiring/eval.rs:66–95 use world.main id, cache.rs:58–78 registers/recycles, upstream typst-cli/eval.rs:159–179/world.rs:176–190 distinguishes expression/stdin/absent input. Source/position/relative reads are language; equal integers/global interner are mechanics. Owner `confirmo`2026-10-06 ratified `lab/history/construction/records/00_nucleo/materialization/origem-fontes-expressao-aprovacao-2026-10-06.json`.

Approved pure facade:

```rust
pub fn eval_source_with_features_and_introspector(
    world: &dyn World,
    source: &Source,
    features: Features,
    introspector: &TagIntrospector,
) -> (SourceResult<Value>, Vec<SourceDiagnostic>);
```

Caller supplies already numbered parse_code Source. Use its tree/id for syntax/spans without reparsing/replacing/allocating id. Private overlay intercepts source.id only; initial current_file/Route remain world.main, other IDs/services delegate. Preserve setup/scopes/features/cloned introspector/Sink. Legacy string facades retain signatures/compatibility, construct legacy Source with world.main and delegate; they do not promise separate identity. Contextual callers/CLI needing distinct sources use new entry. No World/FileId/Source/realization-layout phase change.

empty-markup-warning-2026-10-06/before.json preserves HEAD/tree/UTC/binaries/source/bilateral **, __ /Unicode-prefix/adjacent/nonempty/open-delimiter controls. Historical9f2b6e871 marks both delimiters versus one local character. parse/markup.rs:143–177 inserts warn_at before wrap; diagnostics.rs:99–102 uses empty text; mod.rs:765–778 publishes its empty span. Upstream typst-syntax/parser.rs:145–169 attaches warning to wrapped construct. Language diagnostic range matters, not CST representation.

When publishing only NoTextWithinStars/NoTextWithinUnderscores, find empty Error node by span identity in numbered tree; use direct Strong/Emph parent span only if corresponding kind/text length2. No message inference, offset guess or Source/CST mutation. Missing relation keeps original span. Preserve severity/hints/order/count/other errors/defaults. Internal continuous parity, no public field/API/trait/phase/unsafe.

## Ratified bare-math realization selection

math-bare-2026-10-06/before.json.gz frac-query0versus1 and valid-red.log prove index missing Equation; mod.rs:908–925 realizes only has_recipes. Human confirmation is `lab/history/construction/records/00_nucleo/materialization/math-bare-2026-10-06.md:3`/math-bare-aprovacao-2026-10-06.json, manifest 149d6535b442a9275aa6612647ad79eac460351808ea1858175d09d1cf07e1f6. After single eval choose existing realize_document when has_recipes or private bare_math::needs_preparation detects structurally external insertion, stopping at Equation/Located. Otherwise retain direct path. Same raw values/scope/target/features/Sink/index, no source reevaluation/new public contract. This is the narrow ratified exception to recipe-free bypass, not a general second realization.

## Test declarations and private evaluated result

cfg-test-ci-2026-10-13/before.json.gz/refinement-before.json.gz preserves HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty tree/source/L0/UTC/status/diffstat. Redundant inner:5/parent attrs at972, 975, 978, 981, 985, 989 cover math_limits_override_observation_tests, math_root_evaluation_tests, math_frac_evaluation_tests, math_accent_observation_tests, math_cases_evaluation_tests, math_matrix_evaluation_tests. Removing inner cfg preserves Rust tests but pinned architectural linter fails to inherit parent cfg and reports V2. Remove only external cfg(test) from these declarations; keep paths/names/visibility/path attrs and each file's internal #![cfg(test)] excluding production imports/helpers. Test consumers/L0 unchanged, no dependency/logic/API/default/phase; same test list/core suite/normal build/Clippy/architecture.

mod.rs:958/helpers-testes-ci-2026-10-13/before.json.gz preserve source/L0/reference audit/HEAD/tree/UTC/status/diffstat. Gate only internal make_stdlib cfg(test), same signature/body for existing tests; make_stdlib_with_features stays productive. Call audit precedes classification, not performance/new parity. Keep productive routes/test bodies/names/APIs/defaults/phase; all-target workspace build/check/core suite/test inventory/Clippy/lineage, never delete expectations.

mod.rs:811(C006), type-complexity-2026-10-15/before.json.gz/red-clippy.log preserve source/L0/hashes/HEAD/tree/UTC/diffstat/RED. Private run_pass result becomes EvaluatedPass{value,scope,bibliography_styles,document_info,has_recipes}; construct/consume by name, identical single execution/flow errors/Sinks/metadata/realization/public facades. Rust representation proves no AI benefit/functional bug/new parity. Aliases require identical expansion; new/incompatible public type another gate. Narrow nominal type_complexity exceptions only three justified batch items, no module/impl/crate suppression/raised threshold/other repairs. Keep functional expectations; selected RED→GREEN/denied expectations/full build/Clippy/workspace/fmt/Rustdoc/lineage/scoped resealing, no alias/name-mirror tests.

## Ratified Default and exact compatibility exceptions

mod.rs:397 pub new(), field-reassign-2026-10-15/clippy-inventory.json and clippy-default-2026-10-16 preserve source/L0/hashes/new_without_default. new initializes Paged, empty features, full_error=false, apply_show_rules=true, loop budget1_000_000, fresh private state. Field-derived Default is wrong for budget/recipes. Owner approved manifest 11f3e138f5ba006d04ed4dd1e829e27c004778e691f4a1df63127b1644146d03 on2026-10-16, receipt `lab/history/construction/records/00_nucleo/materialization/clippy-default-2026-10-16/approval-receipt.json`. Earlier pending gate satisfied only for three explicit impls/D1–D5, not arbitrary API work.

Add only impl std::default::Default for EvalContext, fn default()->Self{return Self::new()}. Preserve new/visibility/fields/values/all methods/semantics, no other trait/mode/flag/shared global/I/O/phase. Existing callers need no migration; new callers may use <EvalContext as Default>::default()/T:Default. Future removal breaks this additive contract. Bound-based tests must fail for missing impl before code/pass after delegation; verify fresh values/context-session isolation/resource laziness/controls, workspace/all-target Clippy/build/Rustdoc/fmt/scoped lineage, no changed defaults/new_without_default suppression. EvalContext tests belong exclusively to `00_nucleo/prompts/core/compiler/eval/tests.md`, no parallel productive-module suite.

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/{before.json.gz,classification.json}` preserves checked source/prior L0/signatures/hashes/HEAD/tree/status/diffstat/UTC2026-10-15T16:39:18.873477+00:00. eval_with_full_error:682(C003,8), and_target:709(C004,9), target_and_features:734(C005,10), threshold7. registry/full_error/config/World/Source/Sink are explicit inputs; _routines/_traced/_route forward through compatibility wrappers but are unread in complete body, retained for contracted compatibility rather than proven algorithmic necessity. Target adds no feature; never replace signature merely for threshold.

clippy-locais-2026-10-16/clippy-inventory.json records 79 warnings; clippy-excecoes-2026-10-16/before.json.gz preserves source/spans/reasons/signatures/hashes/HEAD/tree/UTC/diffstat. Only C003/C004/C005 nominal expectations materialized, superseding audit-only pending status; other correction cases including conditional C047/C066 gain no annotation. Preserve signature/body/calls/cfg/order/other attrs/semantics; exact function-local literals:

- `#[expect(clippy::too_many_arguments, reason="C003: Fachada pública de compatibilidade: registry e full_error são entradas explícitas; _routines/_traced/_route são transportados ao próximo wrapper, mas não consumidos na entrada completa atual")]`
- `#[expect(clippy::too_many_arguments, reason="C004: Fachada pública que acrescenta target à anterior sem ativar features")]`
- `#[expect(clippy::too_many_arguments, reason="C005: Entrada pública completa de avaliação: World/Source/registry/sink e configuração são utilizados; _routines/_traced/_route não são lidos pelo corpo atual")]`
- `#[expect(clippy::too_many_arguments, reason="ID: motivo prescrito")]`

Use only named function/lint;  no module/impl/crate/global application, threshold
increase or other lint. This narrowly supersedes generic suppression bans only
for named item/lint. If no longer over threshold remove/review the expectation;
never silence unfulfilled_lint_expectations. Conditional exceptions requiring
dead-argument removal still require that correction; pending correction cases
(including historical C047/C066) gain no annotation here.



Continuous ADR-0127 changes only lint metadata, no public contract/executable body/
cfg/default/phase/dependency/architecture. L0 first/scoped resealing. Removing added
attributes and lineage header must recover prior rustfmt-normalized source.
Measure real RED, then all-target inventory: historical batch exactly 46 arities
plus five test warnings satisfied by 49 expectations, no new warnings and
unfulfilled_lint_expectations as error; its 11 correction arities remain visible.
Keep owner functional/call controls without fixture/tolerance relaxation; private
signature corrections migrate callers/L0 before code and new public/compatibility
needs another ADR-0127 gate. Validate build/fmt/strict/reverse lineage/Tekt no new
errors/warnings, existing Counter/coverage tests under 12 GiB, zero swap, one job.
Prior full suite is evidence for identical executable code, not a fresh run;
no lint-attribute mirror tests or new performance comparison for metadata.

## Explicit guards and closed origin decisions

T026 mod.rs:937,tekt-guards-2026-10-16/before.json.gz preserves source/L0/SHA/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/status/diffstat/hash-reconfirmed28guards/21owners. Join Content arms, choose has_recipes || bare_math::needs_preparation inside body, retaining short circuit/single realization/warning order/error-introspector propagation/direct wrap fallback. No phase/extra eval. Pinned CI demands split/moved guards, not proof of better AI reading/functional defect/upstream intent. Internal continuous flow, no new suppression/threshold/private-body change; real selected RED→GREEN/existing controls/full validation.

tekt-v16-introspect-origins-2026-10-16/before.json.gz preserves full HEAD/tree/UTC/status/diffstat/source/hashes:Value39/Content120, mod.rs:956 latent T027 None, layout/realized.rs:14–45 implicit T158/T160, :77–130 if-let false, extract_payload.rs:18–98 fails Source normalization while locatable.rs:21 sees body. Source nucleus requires transparent single occurrence; hypothesis Source(Heading) None/Located(Source(Heading)) loses preparation proof must have RED first. map_content:79 already transparent; no new vanilla parity/intent or unaudited OutlineEntry closure.

Final document Value match keeps sole Content body and explicitly enumerates other 38 variants→None, without touching single eval/has_recipes-bare_math/warnings/realization/index/wrapping. Existing single_evaluation_ tests stay separately owned, no parallel suite. Same-consequence nominal group may keep justified V19 informational note, not duplicate bodies/global suppression/all-alternative execution claim. Source producers/nonproducers/Located-Source/no duplicate collection/fallback/predicate tests are co-located with their actual owners, RED transparency before code/existing controls before-after. Disposable real-workspace Value/Content sentinel variants must yield E0004 at six selected sites after, with absent prior barriers/other errors/source retained. Scoped pairs only; full build/workspace/strict Clippy expectations/fmt/Rustdoc/V1/V5/V15/V26/reverse/SARIF removing only scoped V16 and allowing only prescribed equal V19, 12GiB/swap0/jobs1/sequential heavy work. General objective/OutlineEntry remains open.

## Trusted modules and causal feature gates

html-feature-identity-2026-10-17/before.json.gz preserves HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/UTC/status/diffstat/full source/L0/hashes; 2123 prior hashes checked. before-probe.json.gz records fixtures/argv/outputs/binaries, historical9f2b6e8715237cb086899a42873660fe744622e8 oracle7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. fields.rs:39–52 gates format.html but not std; identity.rs:84–101 authenticates names/bindings. Proper-import probes show disabled std.html direct/alias/closure/reexport accepted and ordinary global.typ(type/format/document/asset)/format.typ(svg/png/pdf/html) wrongly gated. Invalid let m=import probe returns none and is archived candidate-probe-syntax, no evidence credit. Upstream module.rs:138–154/code.rs:347–368 reads binding guard/field span; language semantics/diagnostics, not required BindingGuard/addresses. field_access.md:61–62 already forbids binding-comparison std recognition.

At both productive std constructions(expression/Source and document), use Module::stdlib(stdlib.clone()), preserving scopes/global names/phases/features/entrypoints/defaults; never mark final document Module std. Pinned builtin-module-identity governs private marker; Module public opaque API gains no external fields/methods, compare signatures. Internal parity continuous flow. RED→GREEN direct/alias/closure/import/homonym gates across eight profiles with exact message/hints/span/causal precedence; disabled cases gain no active MATCH/Preserved. Keep prior cross-file std equality divergence, no Module Eq/global parity change. Full previous disabled-path matrix+active controls, no publish/commit/release; workspace/build/strict Clippy/Rustdoc/fmt/V1/V5/V15/V26/reverse/global differential no new warning, error; only six owner seals after graph validation, 12GiB/High10GiB/swap0/jobs1/sequential.

candidate-1/validation-green.log passes four approved controls then fails: std.html(panic(...)) executes panic before gate. before-set-probe.json.gz freezes same HEAD/tree/UTC/hashes/source/bilateral outputs: called std.html/format.html/std.asset/format.bundle evaluate args too early; set std.html/std.format.html/alias erases target error/ignores arguments; missing variable/field set wrongly succeeds, condition false short-circuits; active alias HTML set ignores panic locally versus upstream executes it. collection_methods.rs:36 evaluates arguments before module authentication, set.rs:44 uses ok(), :326 literal-syntax HTML identification.

Keep hub responsibility; fields owner supplies eval-internal check_module_field_feature(&Value,&str,Span,Features)->SourceResult<()> before lookup. No duplicated diagnostics/rules by other owners. Same nucleus/public fields/API/defaults/fallbacks, no suppression; L0 first/eight-profile matrix+bilateral argument/set effects with/without features; rerun full suite after new source freeze, never reuse changed-source gates.

## Syntax validation and inherited import context

import-syntax-2026-10-17/before.json.gz preserves same HEAD/tree/UTC/status/diffstat/full source/L0/hashes, 2125 prior closure hashes checked; before-probe/before-context-probe records fixtures/argv/outputs/eight profiles/real Bundle/binaries at historical pin/oracle above. modules.rs:108/324 enters typed markup without root validation; mod.rs:649/778 validates main entries/warnings. Reserved/mixed/nested panic101 at ast/mod.rs:94 while upstream returns actual-source syntax error; missing string/expression may succeed, prefix panic runs before syntax error, module warnings disappear. Upstream lib.rs:71–82,128–137 validates errors_and_warnings before eval/import.rs:36–47 traces files. Acceptance is no crash/incorrect effects and causal span/hint/trace, not upstream mechanical intent.

modules.rs:104–106 inherits full_error only: active Html/A11yExtras/Bundle imports wrongly report disabled feature, Bundle module document fails in Paged context. Valid inferred .html/auto bundle exports child.html upstream/fails locally; earlier format: format.html attempt is archived context-format-function-attempt, its separate contract divergence proves no target issue.

Declare only mod syntax_diagnostics; delegate document/expression validation. Document uses tracked Sink.warn_diagnostic; expression engine.sink.warn_diagnostic in existing block. Preserve validation-before-execution/empty-source warning/parser/Source/scopes/features/target/single eval/input registration/public signatures-fields-traits. Remove only now-unused LinkedNode/SyntaxErrorKind imports, retain child-needed SyntaxNode/SyntaxKind; no duplicate normalization/markup parser for expressions. Helper/closure static/private, pinned source-validation shared policy, no Rust API/default/flag/SourceDiagnostic/Tracepoint/World/Sink/eval-layout change.

RED first; no productive catch, tests may catch_unwind solely for readable prior-crash failure. GREEN requires syntax errors before effects, nonfatal warnings/stable dedup, imported source/nested traces, in-memory World packages, RootedPath include/all feature profiles/Bundle document. Keep main/expression/valid-file/frozen corpus, disabled cases gain no active feature credit. Preserve full stderr/HTML without byte normalization masking preexisting markup/diagnostic differences. Parser/include morphology/document.format/unsupported import categories remain separate if unmeasured/unfixed. Graph/V26 before scoped seals; workspace/build/Clippy-denied warnings+expectations/strict Rustdoc/fmt/V1/V5/V15/V26/reverse/SARIF no new diagnostics; extraction may move justified uniform three-error V19 note to new owner, retain exhaustive match.12GiB/High10GiB/swap0/jobs1/sequential, no commit/push/merge/release/suppression.

## Ratified State read and Sequence depth

lote-a1-callbacks-reflexao-seletores-2026-10-17/before/final-freeze/state-counterproof.json.gz preserves HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/tree/UTC/status/diffstat/source/current bd5ebec285505889e8ff1d4f68804ab147a699f3. Same-key init1/10 after+1 gives2/11; unread callback never executes, reused update executes per occurrence. Upstream state.rs:226–245,419–455,477–520 starts from reader init/document order, not Location integer; old Func-as-Set/eager pass loses semantics. Withdrawn prior proposal is not active. Owner approved `lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`, SHA 6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4, only five signatures/read phase. This hub changes no public field/API/entrypoint. Its existing contextual replay test migrates to approved state_get using existing Engine; candidate difference comes from registered Set action, not global init replacement. Keep captured Location/context versus later environment, no duplicate observations.

State acceptance: L0 first/functional RED/linguistic GREEN against pin+Tekt extension controls, no other public expansion; build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/no V16 increase, 12GiB/High8GiB/swap0/jobs1/sequential. Consumption needs measurement, no representation-based global benefit.

sequence-constructor-2026-10-17/before/expanded/controls.json.gz, candidate.log preserve same HEAD/tree/UTC/diffstat/source/binaries/currentpin:eight initial RED, five candidate pass, show/recursion still fail. markup_stream.rs:240–260 flattens before selection, realize/mod.rs:170–210 children before recipe, apply.rs:140–170 sends nonlocatables to morphology fixpoint; upstream accepts alias/rejects fresh equal-children Sequence. Owner approved `lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`, manifest 46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327. ADR-0127§2.3 covers new boundary before children/Table/math, no public API. Existing private occurrence_show_depth also bounds marked Sequence expansion with causal increment/restoration on success/error, initial0/public fields/Route limit unchanged, no global. Ratified S1–S20:RED first/bilateral GREEN/regressions, no weaker expectations for mechanical limits; valid 1:1 owners/nuclei before seals, build/workspace/doctests separately/strict Clippy-Rustdoc/fmt/reverse/SARIF no new errors-warnings, 12GiB/High8GiB/swap0/jobs1/sequential, no publication/release.

## Ratified lexical textual realization and occurrence slots

textual-realization-2026-10-18/{before, scope, refuters}.json.gz and before-sources.json.gz preserve HEAD b1b486f3d42431f8ea360bc5712637ee1e3d4f6c/tree/UTC/status/diffstat/source/L0/argv/binary hashes/current bd5ebec285505889e8ff1d4f68804ab147a699f3, no candidate code at measurement. matching.rs:118–150 reverse-regex then declaration literals, :194–207 reexposes products; textual.rs:102–123 captures full snapshot; realize/mod.rs:177–217 bottom-up; apply.rs:620–648/matching.rs:218–225 function calls without child capture; expression_dispatch.rs:125–129 already separates insertion_rules/local state.

Measured local/oracle:identical literals C/B, tie literal-regex R/L, overlap X/YB, first effect LATER/FIRST, inverse chain BBBB/BB, separate product AB/X, product+sibling X/AB, Strong-hidden child panic HIDDEN/Q, old-global captured then new-global L/R but captured-local L/L, CJK local 水\n果 gives水果/X. Pinned selector.rs:110–125 unifies escaped literal/regex, typst-realize/lib.rs:1328–1405 chooses earliest/revokes in product. These language/effect observations do not authorize Rust/buffer/byte equality; causal scope/root-first hypothesis can be refuted by acceptance.json/equality/fields/style/query/error controls.

Human ratified manifest 47e714dfbb813eae9f2be20b8f57de5c197cda13db1871c8bb3da49a08430302/pinned textual-recipe-frontier. This narrowly supersedes provisional ContentBlock show execution and pending-recipe bottom-up visits, not ordinary expression eval order. No program reevaluation, early State/Counter, morphology-derived render/public Rust contract. EvalContext holds only private compilation-local lexical-output frames distinguishing inherited baseline from actually emitted-tail rules; let/definition eval alone emits no value. Restore frames/flags on every return, retain new/Default/public facades/single ordinary eval, no public field.

Candidate 2 passes63/64 but T36 turns isolated A-emitted space into S unlike oracle; T53 requires S when same product between X/X. Guards alone do not determine text support. Private left/right support bools accompany product realization; matcher derives from prefix/suffix intervals+current external support, restores on success/error and includes only supported weak edges. A spaces-only product is eligible with support both sides; explicit whitespace Text stays eligible independently. No public/global/layout/permanently captured support: neighbors depend on current insertion.

Require acceptance families/R2-S/R3/single-eval RED→GREEN, PDF/HTML separately; full build/workspace/doctests/strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/SARIF no new errors-warnings, 12GiB/High8GiB/swap0/jobs1/sequential, no commit/push/release/suppression or unmeasured RAM benefit.

context-paridade-2026-10-18/before.json.gz/baseline-build-resources.json preserves HEAD b7e7c83223e5daa67bbdf124c36b5b7f93200731/source/UTC/diffstat/argv/binaries/current bd5ebec285505889e8ff1d4f68804ab147a699f3. Two equations EQ EQ Q1/Q2, Heading prefix H EQ Q0/Q1, query it.location00/01; fresh per-producer EvalContext/Locator/realize allocator and walk.rs:375–390 Location dedup suggest collision, refutable by count/alias/callback/replay. Upstream realize/lib.rs:353–418 prepares identity before recipes, no required vtable/representation/algorithm. Owner approved `lab/history/construction/records/00_nucleo/materialization/context-occurrences-2026-10-18/review.md`, SHA 500e01cf0754a166deb7f493ff734f040ef38e0374cbf5aa0ba879a37d19f388, approval-receipt.json alongside: specified additive API only, no unrelated phase/API.

Store crate-private Option<OccurrenceSlots>, default None, beside existing Locator. `00_nucleo/prompts/core/compiler/realize/context_occurrences.md` owns it and installs/removes per execution; EvalContext stores only. No public field/current_location/Context id/observation-policy change.

## Owned test module declarations

Separately owned: math_limits_override_observation_tests, math_root_evaluation_tests, math_frac_evaluation_tests, math_accent_observation_tests, math_cases_evaluation_tests, math_matrix_evaluation_tests. Declare/delegate bodies without absorbing their owners.

## Approved Figure occurrence realization selection

The 2026-10-19 integrated target controls preserve exact dirty source and binary
provenance under `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/`.
`mod.rs:919–940` only selected realization for captured recipes or bare math.
Plain Figure output therefore bypassed the approved occurrence preparation:
explicit table kind rendered the old Figure prefix and typed counter reads missed
its unprepared kind. Show-driven tests happened to enter the correct boundary.
This is measured language behavior and a missing call path, not a reason to infer
kind again during layout or publish a constructor-time synthetic recipe.

After single common evaluation, also select existing realize_document when the
feature-owned pure `figure::needs_preparation` predicate finds a reached Figure
in actual output. Keep has_recipes/bare-math short-circuit behavior, all scopes,
metadata, diagnostics and Module index publication. Unused let-bound Figures do
not force realization or execute supplement callbacks. Figure-free recipe-free
content retains the existing direct wrap_page_columns path and absent realization
index. This bounded route implements the already approved Figure occurrence
phase; it supersedes old text excluding every exception beyond bare math, without
adding public signatures, source reevaluation or layout-time inference.

## Ratified Paged reference supplement admission

After human approval of the exact Ref phase extension SHA1578cbea...7e38ff2,
include actual normal Ref output in the existing realization gate through the
feature-owned borrowed needs_preparation predicate, only when target is Paged.
It performs no query/callback/preparation and never admits unused variables.
HTML implicit Ref rendering remains explicitly outside this phase. Preserve the
single run_pass/realization call, warnings, returned content/index and public APIs.

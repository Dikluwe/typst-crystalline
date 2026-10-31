# L0 — compiler/eval/call_dispatch/invoke
Hash do Código: 1ec60315


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/math/selected-symbol-accent-call.toml sha256:c6ef1be7fbad9f9513c172b144b3dbe314ba24a6d4c8c09a26a9bca6c6f8b48b
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/call_dispatch/invoke.rs`. At clean HEAD
`1492976302212e007282875aef066f84a3cf13aa`, call.rs:638–827 contains extracted
behavior. SHA/time/initial state in
`lab/results/construction/diagnostics/atomizacao-call-core-2026-09-30-before.json.gz`.

## Exclusive invocation and causal error origin

Resolve retained callee/evaluate args/execute closed Func-Type-Plugin-Content
routes with existing spans/traces, subject to noncallable rejection below.
Preserve parent call_dispatch/call owners, public signature/default/phase/
compatibility. Static L1. Keep L1/call_dispatch tests/crystalline-lint with valid
ownership/hashes; extraction implies no speedup.

controls-first.json measures `maximum show rule depth exceeded` without causal
source: invoke.rs:145 intercepts after apply_func trace, losing construction span.
Propagated intercept_content failures preserve known spans; anchor only detached
to actual call.span. Reconstruct no origin from message or overwrite internal
source. Signature/phase/apply_func trace stay.

show-clone-body in after.json.gz has correct error/hints but no insertion span;
invoke.rs:145–155 anchors eager errors only. Capturing ShowRecipe with external
Mathy insertion stores call.span in private Styles eval.math.origin via
Span::into_raw bits as Value::Int. No recipe means no raw constructor/value change;
context lives only in recipe. Realization restores Span::from_raw and anchors
detached only, retaining known spans/real Source/hint-trace order. Restore capture
scope before every return/error. No message-derived span/text-recreated Source or
public/phase change beyond approved mathematical-insertion gate D06.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive raw Table-call results

Constructor/closure result Content::Table/TableCell alone is no insertion. Keep
raw value, without baking global recipe into let-stored value. Other results keep
interception. Specific argument spans/traces stay. Table/Cell insertion belongs
to markup/join/emitted product, never function name.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



### Structural group refinement

{after-second,refuters-second}.json.gz in table-cell-realizacao-d08b3-2026-10-07
retains pin/binaries/HEAD/state/sources/UTC/argv/results. after-second/header-fields
shows table.header/footer as Cell with repeated recipes. invoke intercepts groups;
native_table extracts only direct variants, table_grid.rs:85–100. TableHeader/
TableFooter remain raw at construction for Table owner extraction/plan realization.
Change no other native/Grid policy. Bilateral effects once/geometry, no whitespace
normalization or ratified-contract change.

## Single-evaluation evidence and boundary

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz`
records UTC/status/git diff HEAD --stat/sources/L0/config/hashes. Exactly2124
preceding hashes were checked before baseline. before-probe.json.gz and
before-math-probe.json.gz retain bilateral binaries/argv/fixtures/output.
Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
calc.abs/array.len/str.from-unicode measure receiver3/args2 versus1/1;
module.measure/layout/within/before/after/func/has/body reach receiver4;
mutating fallback args3 versus1. Missing methods execute argument panic before
causal error. math.mat adds twice; prior array(1,) becomes(1,1,1) versus(1,1),
while empty array masks defect with truncate. call.rs:70–118 loses receiver;
collection_methods.rs:43 evaluates args before recognizing operation; contextual/
content/selector reevaluate AST on fallthrough; method_dispatch.rs:247 discards
args; invoke.rs:66–120 compensates by name. call_dispatch L0 already requires
receiver/args once. pinned-source.json.gz pins upstream call.rs/methods.rs:
interest is language order/effect/diagnostic, not copying Vm/storage/Rust equality.

First fixture declared reserved or as function: invalid, archived in
invalid-fixture-attempt with no call-effect credit. It revealed panic101 on
invalid import, still open for parsing/import owner's causal correction. Valid
baseline replaces only fixture.

### Exclusive acquired-argument invocation

Add only internal resolved_args: Option<Args>. Use Args transported by mutating
fallback or evaluate once normally. Remove name-based is_mat_call, isolate_mat_args,
isolated Scopes copies and array_lengths/prefix/truncate repairs: legitimate
user effects cannot be deduplicated by content. Preserve call metadata/spans,
closed match/callbacks/With/traces/Content-ShowRecipe/existing mathematical
insertion contracts.

### Acceptance and limits

Consume pinned eval/call-once nucleus. Internal mechanics/parity ADR-0127
continuous flow keeps pub fn/pub use/public structs; new parameters/returns/
helpers remain crate/eval-visible, no public trait/field/default/phase. Supersede
only conflicting historical internal-signature/name-compensation restrictions;
keep Tekt extensions.

Co-located functional tests before code: receiver/args effects, mutating fallback,
error before args, where/with/selector/collections preserved, matrix without
truncating legitimate effect. Confirm real RED then GREEN. Compare valid bilateral
cases/controls, every collection method's hits/misses, errors/spans/occurrences/
Value complements. Residual diagnostic debt is not equality. Unhandled dispatch
must retain receiver without running args callback. Count observed effects, not
PDF bytes/Rust Eq/microbenchmark.

Ownership/V26 before restricted batch-owner reseal. Require build/full workspace,
all-targets Clippy -D warnings/-D unfulfilled_lint_expectations, strict Rustdoc,
fmt/V1/V5/V15/V26/reverse/differential SARIF without new error. Uniform V19 stays
informational with alternative matrix/behavior relation; no suppression/config
change. Heavy commands sequential under 12 GiB/High 10 GiB/swap0/jobs1. No publication/
commit/merge/release or global-objective closure claim.

### Noncallable callee precedes argument effects

before-probe control/30 measures m.data(panic(...)) with integer data: Tekt panics,
pin returns `expected function, found integer` first. Private prepare_call_args
runs only in Func/Type match arms. Transport resolved_args or evaluate AST once;
return Args and already-computed private HTML spans. Change no Func definition/
fields or callable type. Other37 nominal Value variants reject with
`expected function, found <vanilla_type_name>` at callee span without preparing
args. Exhaustive final match removes old wildcard; uniform V19 means pre-effect
rejection, not every-alternative execution proof. Add Value category controls/
keep prior misses-builtins. Unmeasured stdlib-shadowing hints remain separate
diagnostic work.

V19 acceptance matrices
call_once_unhandled_keeps_every_value_and_does_not_evaluate_arguments and
call_once_callable_classification_covers_all_value_variants_before_arguments
construct39 distinct Value discriminants (36 through language, Path/Location/
LocatedContent via existing RootedPath/Locator/IntrospectedContent pure constructors).
First checks unknown-name Unhandled, transferred payload equality, original Vec
pointer and args callback panicking if called. Second checks37 pre-args rejections
and Func/Type paths with real argument panic. Internal transport/classification
tests certify neither upstream cast capacity for every category nor all schemas'
parity.

### Measured flat-complement refinement

candidate-4/validation-architecture.log and differential-review.json measure new
V20 from target/other @ (variant group), depth3. Capture is unnecessary: bind
mutable place as target before match and reference it in complement, or reference
original callee with ignored payload. Keep explicit flat groups/same members/
returns, no V20/V19 suppression or duplicate classification. Preserve39-alternative
matrices/effects/original pointer.

## Sequence realization evidence and approval

before/expanded/controls.json.gz and candidate.log in
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17` fix HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree,
UTC/diffstat/sources/binaries and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Eight initial REDs; candidate
passed five, with show/recursion unresolved. markup_stream.rs:240–260 flattens
before selecting Sequence; realize/mod.rs:170–210 visits children before recipe;
apply.rs:140–170 delegates nonlocatable elements to fixed point. Vanilla accepts
alias and rejects a new Sequence with the same children. These are language
effects/scope/values/diagnostics.

Owner approved
`lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`,
manifest `46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327`,
on 2026-10-17. ADR-0127 §2.3 gate covers realization order of this boundary
before children/Table/math, not a new public contract.


### Exclusive Sequence-call insertion boundary

With active Sequence rule, raw Sequence/Empty call results keep identity/Source
from call.span and proceed to insertion frontier without premature generic recipe
on let-stored return. Other results/no-Sequence-rule paths keep current interception.
Never execute recipe from fields/func reads or function name.

Require ratified S1–S20, RED before code/bilateral GREEN/regressions, no lowered
expectations for mechanical limits. Valid1:1/pinned graph before reseal; separate
build/workspace/doctests/strict Clippy-Rustdoc/fmt/direct-reverse lineage/SARIF
without new error-warning.12GiB/High 8 GiB/swap0/jobs1/heavy tasks sequential.
No publication/release.

## Ref construction source before deferred interception

Actual pinned-oracle/product8 evidence in
`agents/entities/reference-acceptance/show-origin-selected-product-8/receipt.json`
under the Figure implementation diagnostics measures a label-selected callback
constructing `ref(<newmissing>)`: identical missing-label error, oracle source2:2
versus detached product origin. Identity/input alias controls preserve their
prior source after the preceding bounded fixes.

Measured invoke.rs:45–78 intercepts the returned raw native Ref before eval_expr
can attach first Source. Under deferred show rules interception stores raw body
in ShowRecipe; eval_expr later wraps only that recipe. recipes.rs:606–609 restores
captured styles before raw body realization, so the external origin cannot prove
its construction site. native_ref is intentionally a source-free constructor;
actual FuncCallNode::span is already available at this owning invocation boundary.
This is observable source correction under ADR-0107/0127 continuous flow, not a
new callback/phase/default/API or generic Args-span policy.

Before interception, preserve actual call.span in existing Source for a returned
Ref source root only. Existing first Source wins, including aliases; no output
span override after callbacks. NonRef roots stay unchanged and synthetic
constructor calls outside this lexical invocation remain detached. Keep general
argument transport, tracing, interception priority, recipe capture/restoration,
Type/noncallable paths and all existing namespaces unchanged. Actual new-Ref
source and distinct alias controls must pass; tests also preserve detached
synthetic defaults, first origin and unrelated Content.


## Measured Repeat layout-diagnostic origin transport — 2026-10-22

### Evidence before decision

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/repeat-after/measurement.json.gz`
preserves uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`,
UTC/source/L0 hashes and exact dirty-file stat, candidate build receipt and
ratified oracle identity. Measurement SHA-256 is
`2d3884c52e80f20d32c72d14113d04f6b229ff94953f92b5f7cf9324c9673bd5`;
candidate binary SHA-256 is
`380224b96e7d12e15f7d8ee02ad2200d96664eb14f0bbe879fbd3eb265bd96d7`.
The oracle retains commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
Its unbounded-region rows run at UTC
`2026-10-22T01:44:19.907521+00:00` (product) and
`2026-10-22T01:44:21.283382+00:00` (oracle). The immutable source is
`agents/repeat/fixtures/unbounded-region.typ` under the same result root.
Both reject with `repeat with no size restrictions`; candidate stderr has no
source location, while oracle has its actual line4, column1 and complete call
underline. Full stderr is retained, not normalized away. `prototype-1/parity-matrix.json`
classifies this observable mismatch RED_DIAGNOSTIC under its unchanged policy.
The selected source, commands and all provenance are also retained in
`agents/repeat/origin-transport/diagnostic-measurement-excerpt.json.gz`.

At this consumer's lines45–78 invocation intercepts raw constructor Content
before deferred recipe capture. Existing private `reference_call_origin` at
lines199–205 retains actual FuncCallNode::span only for Ref. The expression
owner's eval_expr later adds Content origin for content/code blocks, deferred
show-rule contexts and Ref, so a plain Repeat constructor has no diagnostic
origin. Layout's existing Source carrier already installs current_source_span
and the Repeat layout error already uses it; no formatter change or derived
source reconstruction is required. `Content::with_source_span` preserves a
known first origin and leaves a detached synthetic constructor unchanged.

### Scoped origin contract and classification

Before interception/capture, extend the existing private retained-origin match
from Ref to the exact two source roots Ref and Repeat. Keep actual call.span in
the existing SourceContent diagnostic carrier. Existing first Source wins,
including aliases; no function-name classification, rewritten body, guessed
column, message-derived span, callback replay, generic constructor policy or
all-Content wrapping. Synthetic Repeat construction outside lexical invocation
remains detached. All other roots stay unchanged. This narrowly supersedes
the prior NonRef-unchanged clause only for Repeat.

This is continuous vanilla-parity diagnostic transport under ADR-0107,
ADR-0108 and ADR-0127: no public entity/trait/signature/default/compatibility
change and no eval-layout phase movement. Source origin is existing metadata,
not a new source occurrence or recipe execution point. Preserve callee/args
once, traces, callbacks, show-interception order and recipe capture/restoration.

### Acceptance

Add a co-located test before implementation: parse an actual Source with a
Repeat call, resolve the real AST call span back to its independently declared
byte range, transport exactly that span, and preserve Repeat payload/defaults.
Keep first-origin alias, synthetic-detached and unrelated-root controls.
Confirm a genuine detached-origin assertion RED, then GREEN. Recompile and
rerun the immutable unbounded fixture against the pinned oracle; require exact
source location/underline and existing diagnostic policy. A call AST range
alone does not certify the CLI column; actual new candidate stderr must decide
that. No column offset constant, formatter adjustment or weakened comparison
is authorized by this owner. Existing Ref origin/alias and invocation suites,
full tests, Hash/rustdoc/Clippy and both lineage directions remain required.

## Measured refutation and final causal HTML boundary

`lab/results/remaining-parity-2026-10-22/agents/html-length/empty-warning-before/receipt.json`
SHA-256 `8bd8aac399399978418fd9a5c82cdbfa07bf93c6fe9779401901b55cc489a4d6` records UTC
`2026-10-22T12:42:21.170883+00:00`–`2026-10-22T12:42:21.559949+00:00`, exact HEAD/dirty stat, sources, argv,
stdout/stderr and the cached baseline's own verified source archive/build, oracle
and font pins. The cached binary is not claimed as current uncommitted source.
Empty page/empty nested page do not warn or error. Returned ContentBlock content
and nested initial Bundle content warn rather than error; the construction-marker
proposal is refuted and must not be materialized. Direct standalone Html ContentBlock
insertion and semantic HtmlFrame/Block reject page configuration. Both Heading/Page
warning orders are measured; missing standalone Heading warnings cannot justify
late sorting. This supersedes only the refuted construction/Document-consumption
clauses, preserved as diagnostic history in that task's LAB records.

Extend the existing bounded call-origin adapter to actual Heading content roots,
preserving their first real call span before recipe capture. Other constructors
retain their current treatment. Do not reevaluate arguments/recipes or invent
text-based spans. This is internal diagnostic provenance, no public API/phase.

Meaningful before-code RED and final bilateral exact source/message/hint/order/body
checks remain required. Internal ADR-0127 continuous flow; no new public contract,
default or phase. Preserve full unsuppressed lint/test/path/hash/lineage gates.


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


## Selected-symbol accent callability — readiness 2026-10-29

### Measurement before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/accent-callability-findings6.json`
binds actual release6 source/archive, HEAD 212aed0449150dd296ceea13fc2be665cc5db01a,
uncommitted exact dirty-file stat, UTC and pinned oracle64c5e8c0. The original
14-case receipt's single-letter a is notation in BOTH engines; only the corrected
long-name before6b cohort proves local alias callability. Bare grave/arrow render
symbol+parentheses in Tekt, native accents; qualified and Code Symbol calls reject
Func in Tekt. Long local symbols lose accents/options/arity; beta/string notation
and noncallable Code rejection remain controls. accent-effects-before6/receipt.json
measures one seen.push effect in valid math/native Code controls, with Code Tekt
blocked at callee. No source/font/PDF-byte equality is language acceptance.

Pinned typst-library/src/foundations/symbol.rs:133-137 dispatches selected Symbol
through its accent function; math/accent.rs:212-290 owns strict aliases and the
single-base wrapper. Existing Func::native/with and call_dispatch's With merger
already transport bound arguments, final call span and occurrence order.
This is an internal semantic/morphological parity repair under ADR-0127 continuous
flow, not a public API/default/phase change.

### Exclusive amendment and acceptance

Before the existing closed Func/Type/noncallable match, normalize only a recognized raw Value::Symbol callee through the accent helper with actual call.span. Existing Func branch owns Args preparation once, apply/With, tracing, source retention and interception. Unknown/nonaccent Symbols retain pre-argument rejection; all other Value variants remain unchanged. This supersedes the earlier all-Symbol noncallable clause only for strictly recognized accent Symbols. The existing every-Value control's nonaccent Symbol remains rejected. Add no new argument evaluator, result interception or function-name heuristic.

L0 precedes actual functional RED, then implementation/GREEN; preserve old tests,
source-bound bilateral PDF/SVG/error/effect controls, full core/workspace, build,
strict lint/Clippy/rustdoc/fmt and both lineage directions. Validate V15/V26 before
resealing. No completion/performance/JPEG qualification from a finite call fix.

# L0 — compiler/eval/call_dispatch/invoke
Hash do Código: c05d9dac


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
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

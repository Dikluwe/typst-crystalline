# L0 — compiler/eval/call_dispatch
Hash do Código: fea57bf4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/call_dispatch.rs`
**Internal atomization:** this facade preserves tests/interface. Individual L0
owners under `00_nucleo/prompts/core/compiler/eval/call_dispatch/` own spans,
arguments, application, type fields, value methods, traces/plugins, call resolution
and Location methods. Clauses below remain their normative boundary criteria.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Internal contextual selector validation linkage

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`, this hub reexports
`selector_before_after` from its value_methods child. Current-pin probes and
source receipts in `lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/`
show that contextual selector boundaries require locatability, separately from
logical selector construction. Add an eval-visible direct reexport of the
child-owned `validate_contextual_selector` so both adapter routes share the same
validation. Keep the algorithm in the child and all public signatures unchanged.
The child must reject textual Regex boundaries recursively without changing the
different contract of the generic native-element locatability validator. L0
sealing precedes linkage; consumer diagnostic and once-only-effect tests provide
RED→GREEN, followed by workspace/path/lineage validation. No new phase or product
mode is introduced by this internal ADR-0127 continuous-flow correction.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/call_dispatch.rs:69` — `struct FloatIsNanCallSpans {`.
- `01_core/src/compiler/eval/call_dispatch.rs:124` — `enum P1293MathNative {`.
- `01_core/src/compiler/eval/call_dispatch.rs:147` — `struct P1293NamedSpan {`.
- `01_core/src/compiler/eval/call_dispatch.rs:154` — `struct P1293MathCallSpans {`.
- `01_core/src/compiler/eval/call_dispatch.rs:300` — `enum P1293HtmlNative {`.
- `01_core/src/compiler/eval/call_dispatch.rs:332` — `struct P1293HtmlCallSpans {`.
- `01_core/src/compiler/eval/call_dispatch.rs:399` — `fn eval_args`.
- `01_core/src/compiler/eval/call_dispatch.rs:465` — `fn transport_native_call_span`.
- `01_core/src/compiler/eval/call_dispatch.rs:503` — `fn p1339_native_call_span`.
- `01_core/src/compiler/eval/call_dispatch.rs:535` — `pub fn apply_func`.
- `01_core/src/compiler/eval/call_dispatch.rs:604` — `fn p1284_type_field`.
- `01_core/src/compiler/eval/call_dispatch.rs:722` — `fn take_p1339_self`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `crates/typst-library/src/foundations/func.rs:390-397`.
- `crates/typst-library/src/foundations/func.rs:412-422`.
- `crates/typst-library/src/foundations/selector.rs:177-178`.
- `crates/typst-library/src/foundations/selector.rs:188-189`.
- `crates/typst-library/src/foundations/selector.rs:275-285`.
- `crates/typst-library/src/introspection/counter.rs:383,456,490,506`.
- `crates/typst-library/src/introspection/state.rs:274`.
- `crates/typst-library/src/layout/layout.rs:65`.
- `crates/typst-library/src/layout/length.rs:140-158`.
- `crates/typst-library/src/layout/measure.rs:47`.
- `foundations/args.rs:160-173`.
- `foundations/args.rs:320-340`.
- `foundations/array.rs:164-169,1176-1179`.
- `foundations/func.rs:372-374`.
- `foundations/func.rs:430-450`.
- `foundations/mod.rs:140–156`.
- `lab/typst-original/crates/typst-eval/src/call.rs:412-460`.
- `lab/typst-original/crates/typst-eval/src/call.rs:56-78,458-460`.
- `layout/angle.rs:142-151`.
- `typst-eval/src/call.rs:155-158,167-174`.
- `typst-eval/src/call.rs:28,56,64,78`.
- `typst-eval/src/call.rs:56–78`.
- `typst-library/src/foundations/args.rs:160-173`.
- `typst-library/src/foundations/args.rs:160–173`.
- `typst-library/src/foundations/func.rs:372-374`.
- `visualize/gradient.rs:545-716`.
- `visualize/gradient.rs:719-874`.

## Contract

Own apply_func, eval_args, eval_func_call and private call adapters. Preserve
their existing public/internal signatures, explicit Scopes/EvalContext/Engine,
closed FuncRepr and Value::Type dispatch, syntax interception priorities and
causal traces. Resolve target/receiver and argument expressions exactly once.
Generic Closure delegates to closures; Element calls its constructor; Plugin
delegates through the existing host with Bytes positional arguments and no named;
Native/NativeWithEngine use their real typed pointers; With recursively applies
the inner function with prearguments. Do not invent a new callable representation.

eval_args evaluates occurrences in AST order. Positional/named retain both full
argument and expression-value spans. Spread None is empty; Array/Dict use the
spread origin and their container order; Args spreads retain original occurrences,
including duplicate named values and cross-source origins. Other spread values
emit `cannot spread <long type>` (not literal-spread 'into' wording). Build via
Args::from_occurrences; do not rewrite carrier views directly or legalize invalid
literal duplicate syntax. Synthetic receivers retain detached origins; receivers
copied from actual arguments keep their original occurrence.

With preserves all pre/new occurrences in order, oldest bindings first and final
call last, with the final aggregate span. Last named determines the view but earlier
occurrences remain available to downstream validation. This is not Args+Args join,
whose collision-removal algorithm belongs to operators/join. function.with and
bound f.with share the same constructor: consume strict Func self, keep all remaining
prearguments, do not validate f's signature, convert Type to Func or execute f.
Static arguments evaluate before self cast; bound invalid receiver fails before
arguments. Resolve and cache the target for fallback rather than evaluating it again.

## Delegated methods and constructors

Typed interceptors route collection/Args methods to stdlib collections, mutable
methods through the existing lvalue path, color/state/counter/version and selector
adapters through bindings/value_methods, and content/snapshot methods through
bindings/field_access. A LocatedContent receiver carries its entire snapshot and
Location; this facade does not strip it to raw Content or inspect its fields.
Unrecognized names retain ordinary field/callee fallback and diagnostic precedence.

Keep integer signum/bit-not/bit-and/bit-or/bit-xor/bit-lshift/bit-rshift/to-bytes
instance routing (not from-bytes), float is-nan/is-infinite/signum/to-bytes and the
measured rejection of a Float receiver for from-bytes, and the existing gradient
instance catalog including sharp/repeat. Delegate once to the same native as the
static route, with the causal receiver first; no duplicate numeric formula.
Version.at delegates to its stdlib adapter without a second index parser.

Angle deg/rad share a strict Angle self parser and delegate only to Angle::to_deg/
to_rad, returning Float. Missing self uses the whole call; named-only self emits
``the argument `self` is positional`` at that occurrence with the removal hint;
wrong self uses its value span (`expected angle, found <type>`); leftovers use
the first offending full occurrence. Static function.with uses corresponding
function self diagnostics. Length.to-absolute/state/location preserve their
context gate; wrappers do not bypass context or mutate an external binding.

Bound/static where uses the single semantic value_methods helper. Cache the
target through native function, Type::Function or ordinary-field fallback; do not
promote dict/module homonyms. Static arguments evaluate before self cast; bound
receiver resolution comes first. before/after converts base and boundary through
the existing Selector conversion, accepts inclusive Bool default true, preserves
missing/extra/unknown/type diagnostics and constructs the tree without query.

OutlineEntry body/prefix/page/inner/indented bound calls intercept only actual raw
or located entry content, not arbitrary Content. Prepend the once-evaluated receiver
and delegate validation/composition to the qualified native owner. body returns
the outlinable element body and rejects arguments; non-outlinable errors remain
explicit. Contextual helpers receive current Engine/scopes. HTML native lookup
uses the actual callable html namespace, not Module-only access. SpotColorant tint
intercepts bound calls only and delegates receiver/spans to the same color parser
as the static form; isolated field access remains invalid.

Type constructors retain the closed delegated Decimal/Duration/Regex/Selector/
Stroke/Tiling/Version/Label routes and other already-supported cases without
normalization or validation duplication. Type::Arguments returns the full evaluated
Args, accepting named as payload, not unknown constructor keywords; language len
counts positional and named, without altering the Rust Args::len contract.
Type::Array routes only a first-positional Bytes receiver to native_array_bytes;
other/missing/named-only inputs retain the legacy no-constructor error at the
callee span. Evaluate all arguments before selecting this restricted route.
Other nonconstructible types retain `type {name} does not have a constructor`.

## Diagnostic transport

trace_call adds the causal Call trace only when a resolvable call range does not
contain the primary error. Keep message, hints, original span, name and stack order.
Recognized early Args.filter/map returns use this same containment rule; do not
create a fictitious Func, re-evaluate a callback or add traces for isolated fields.

Transport whole-call aggregate span before native application only for the closed
resolved identities: json/toml/yaml encode, panic, CSV, calc_abs, static where,
Angle conversions, function.with and the existing Float signum/from-bytes/to-bytes
and Version.at wrappers. Traverse With by real pointers, never name/spelling,
message, arity or argument values. Preserve individual occurrences/preargs.
Generic math routes reuse this helper; synthetic apply_func without AST retains
its received span. Encoder/loading/calc/native owners keep casts and validation.

Keep the existing private AST anchor selectors for collections, float.is-nan,
math.attach/binom/mono/script and residual HTML button/col/iframe/select/template/
video/wbr. Capture full call, positional and full/value named spans before consumption;
after ordered evaluation adjust only the selected diagnostic anchor. Missing
uses whole call, offending values use value/positional span and unknown named uses
full occurrence; script.cramped uses its value span. HTML void/second-body uses
the offending positional, cross-tag/data-*/body named uses full named occurrence.
Spread or unsafe structural correspondence retains the aggregate fallback. This
metadata does not redefine Args or the native schema. Existing legacy selectors
are not generalized by the typed identity transport.

## Introspective projections

Location page/position/page-numbering use the current target and introspector.
Paged and Bundle retain their existing safe defaults and ordered paginated dict.
Html page is always None; Html position uses only html_position_of, returning
None or ordered x/y dict with no page. Do not infer a frame island by feature,
content, coordinates or intrinsic page. page-numbering returns raw Pattern->Str,
Func->Func or absence->None, without executing a callback or reading a formatted
view. Register the effective operation/result once, including defaults/errors,
and replay the same projection: x/y-only changes invalidate position, not page.

Measure interception recognizes native_measure by pointer, calls the internal
geometry helper and returns width/height/baseline in that order. Do not calculate
baseline or inspect Content here. Preserve contextual gate, shadowing, evaluation,
validation and indirect fallback; the public measure_content_real signature stays.

## Test-only observation

Under cfg(p1339_observation), preserve real Func/Args producer and dispatch records.
An active typed occurrence records the actual With wrapper, inner target and known
edge, then normal recursion; Closure records the same target before delegation.
Body observation uses its actual SyntaxNode, never an invented function. IDs are
local typed ledger identities, not stable pointer or repr oracles. Do not execute
anything additionally, alter origins/flow, or add a normal-build API/branch.


## Ratified Source trace visibility boundary

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz records HEAD/
working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized
operations have corresponding transparency; Source cannot leak as observable
element. Human `confirmo`,2026-10-07, final approval record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history.

Expose trace module only within compiler for pure reuse of late Table trace;
no external signature/API. Exclusive owner/static dispatch/no I/O/new signature/
trait/flag/eval-layout phase. Require C2/C/macro tests/recollections/build/lineage
before certification.

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

### Exclusive facade refinement

Update only internal deferred-collection-dispatcher import. Keep public exports,
span types/modules/tests; no new callee/global AST.

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

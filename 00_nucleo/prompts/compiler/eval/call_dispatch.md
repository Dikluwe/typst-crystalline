# L0 — compiler/eval/call_dispatch
Hash do Código: 30f9c85c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/call_dispatch.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

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

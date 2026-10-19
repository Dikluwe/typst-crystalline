# L0 — compiler/eval/call_dispatch/collection_methods
Hash do Código: a86e8294

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

## Ownership and extraction evidence

Layer L1. Sole productive consumer: `01_core/src/compiler/eval/call_dispatch/collection_methods.rs`. Original behavior in `01_core/src/compiler/eval/call_dispatch/call.rs:265-315` at HEAD `51a027f99`, clean tree, 2026-10-01 00:44:23 UTC. This is internal extraction, not fresh bilateral parity or performance evidence.

Duplication provenance is also in
`lab/results/construction/diagnostics/arrays-dicts-item6-2026-10-09.md` and
`00_nucleo/prompts/core/compiler/eval/call_dispatch/instance_methods.md`.

## Current exclusive contract

Own collection interception in existing order. Transfer cached receiver once,
check feature gate before args, preserve argument spans/traces/fallthrough and
public/default/phase/compatibility/L1 purity. Replace eager route with
try_collection_method: take receiver from cache once, gate, build spans and pass
static FnOnce deferred-args closure to stdlib dispatcher. Handled keeps Args
map/filter trace; Unhandled returns owned Value to cache. Remove redundant late
With branch: call owner handles this form first. Duplicate no catalogue; never
decide by public function name.

Preserve L1/call_dispatch tests, cargo build and crystalline-lint with valid
ownership/hashes. Extraction implies no performance gain.

## Feature-gate causal continuation

candidate-1/validation-green.log records four passing controls and real argument
test failure: std.html(panic(...)) panics before gate. before-set-probe.json.gz
retains UTC/hashes/sources/bilateral outputs at same HEAD/uncommitted tree:
called std.html/format.html/std.asset/format.bundle evaluate args before gate;
set std.html/std.format.html/alias hides target error/ignores args. Missing
set variable/field wrongly accepted; false condition short-circuits. Active-HTML
set alias ignores panic only in Tekt. Language order/diagnostic/effect.
collection_methods.rs:36 evaluates args before module authentication;
set.rs:44 uses ok() resolving target, :326 authenticates HTML by literal syntax.

Invoke check_module_field_feature on resolved cached receiver before collecting/
evaluating args. Never reevaluate receiver/change collection dispatch/other
fallbacks. L0 precedes correction; internal/parity continuous ADR-0127, pinned
builtin-module-identity unchanged, no public field/fallback/suppression change.
Keep controls/eight-profile matrix; add bilateral effect/set checks with/without
feature. Full suite reruns after new source freeze; changed source cannot reuse
old gates.

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

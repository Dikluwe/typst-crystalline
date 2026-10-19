# L0 — compiler/eval/bindings/method_dispatch
Hash do Código: 0a7ed794

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/method_dispatch.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/method_dispatch.rs:29` — `fn is_accessor_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:35` — `fn expect_positional`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:52` — `fn finish_args`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:74` — `fn has_readonly_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:91` — `fn call_method_access`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:202` — `fn is_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:208` — `fn is_dict_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:222` — `fn try_eval_mutating_method`.
- `01_core/src/compiler/eval/bindings/method_dispatch.rs:269` — `fn call_method_mut`.

## Contract

Own mutable-method classification, mutation and mutable accessor routing.
Array mutation names are push/pop/insert/remove; dictionary mutation names are
insert/remove. Mutable accessors are first/last/at where the receiver supports
them. Preserve readonly classification so temporary mutation fails with the
existing diagnostic rather than becoming a mutation of a detached copy.

Evaluate arguments, resolve the mutable place, then perform the mutation in the
existing order. The ordinary unrecognized nonmutating route preserves its no-interception
behavior; the later internal MutatingCall transport below carries already
evaluated fallback target/Args without replay. Preserve array/dictionary bounds, empty-collection, missing
argument, default and unexpected-argument diagnostics and span priorities.
first/last/at return the actual mutable slot, never a cloned substitute.

Argument helpers consume via remove_positional(0)/remove_named and finish_args;
they keep Args views, occurrences, names, individual origins and aggregate span
coherent. Never retain an occurrence carrier after modifying its views directly.


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

### Exclusive mutation transport and partial supersession

Add eval-restricted enum MutatingCall { Handled(Value), Fallback { target: Value,
args: Args } }. try_eval_mutating_method returns SourceResult<MutatingCall>
with same parameters. Preserve args before access and actual writeback. Mutation
success maps Handled; Module/Func/Type/Symbol/Content fallback clones local value
once and transports Args. Reexecute neither access nor args. This supersedes
historical double-evaluation acceptance in P717 comment: new measurement proves
observable effect requiring repair.

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

### Measured flat-complement refinement

candidate-4/validation-architecture.log and differential-review.json measure new
V20 from target/other @ (variant group), depth3. Capture is unnecessary: bind
mutable place as target before match and reference it in complement, or reference
original callee with ignored payload. Keep explicit flat groups/same members/
returns, no V20/V19 suppression or duplicate classification. Preserve39-alternative
matrices/effects/original pointer.

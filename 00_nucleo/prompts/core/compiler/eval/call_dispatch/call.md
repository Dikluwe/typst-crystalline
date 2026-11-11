# L0 — compiler/eval/call_dispatch/call
Hash do Código: 17e11be5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

## Ownership and extraction evidence

Layer L1. Sole productive consumer: `01_core/src/compiler/eval/call_dispatch/call.rs`. At HEAD `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca` with clean tree, `01_core/src/compiler/eval/call_dispatch.rs:1305-2125` contains the extracted behavior. SHA-256/time/initial state are in `lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Current exclusive contract

Resolve/intercept functions/methods in existing priority. Delegate Value instance
methods after mutation and before collections; preserve once-only receiver/args.
Share local Option<Value> receiver/callee caches and Option<Args> mutating-fallback
cache. Internal resolve_receiver(access, cache, scopes, ctx, engine) and
resolve_callee(call, cache_receiver, cache_callee, scopes, ctx, engine) return
references to stored values. Reuse eval_field_access_with_target for full lookup,
including features/spans/styles/deprecations. Preserve where/with, selectors,
mutation, instance, collection, context, content, callee-error and invoke priority.
Carry cache through fallbacks without duplicated lookup; ordinary method error
precedes args. where/with stays unique initial branch. Name field_callee_error
before final field resolution; invoke receives acquired callee/Args. Co-located
tests use in-memory World/current APIs before code.

Each descendant retains its sole L0: instance_methods, selector_intercepts,
contextual_calls, collection_methods, content_methods and invoke under
00_nucleo/prompts/core/compiler/eval/call_dispatch/. Parent public signatures/
diagnostics/traces/argument semantics stay in
`00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Static L1, no I/O/mutable
globals.

Preserve L1/call_dispatch tests, cargo build and crystalline-lint with valid
ownership/hashes. Extraction implies no performance gain.

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


## Borrowed collection length: evidence and contract

Source audit of the frozen received working tree at HEAD
`0a15ccebf861e3cc0ea16d0397d55dc057664556`, retaining C06/C12/C14:
`01_core/src/compiler/eval/expression_dispatch.rs:121–161` resolves identifiers
through `Scopes::get` then clones their Value;
`01_core/src/compiler/scopes.rs:154–171` exposes the same local/captured/base
lookup as an immutable reference. `01_core/src/compiler/eval/call_dispatch/instance_methods.rs:18–22`
acquires the owned receiver before collection dispatch.
`01_core/src/compiler/stdlib/collections/array_access.rs:11–14` and
`01_core/src/compiler/stdlib/collections/dictionary.rs:97–99` return only
`Value::Int(len as i64)` for length. `01_core/src/entities/ast/expr.rs:1033–1053`
exposes syntactic argument items, including spreads. The existing owner
`00_nucleo/prompts/core/entities/value.md` retains owned Array/Dict storage and
independent mutation. This source evidence makes no new runtime measurement.

Classification follows the evidence: receiver cloning is execution mechanics
under ADR-0107; scalar length, lookup, mutation independence, argument effects,
feature gates and causal diagnostics are language behavior. Under ADR-0127
continuous flow, authorize private
`borrowed_collection_len(call: FuncCallNode<'_>, scopes: &Scopes<'_>) -> Option<Value>`
in this sole consumer, invoked at the start of `eval_func_call` before any owned
receiver acquisition. Eligible syntax is only direct `Expr::FieldAccess` with
field exactly `len`, target directly `Expr::Ident`, and no syntactic argument
items. Reject positional, named and spread items even when a spread is empty.
Exclude both `html` and `asset`, preserving their existing identifier policies.
Resolve with `Scopes::get` and recognize actual borrowed `Value::Array` or
`Value::Dict`, never names/native identity. Return `Value::Int(len as i64)`
without a size threshold. The Value match is exhaustive with explicit
noncollection alternatives and no wildcard, requiring review of future variants.

The short immutable borrow ends before evaluation, arguments, callbacks or
mutable scope operations; only the detached scalar escapes. All other syntax,
unresolved/reserved names, values and methods take the existing route, caches,
feature checks, once-only effects and diagnostics. This includes expression,
parenthesized, chained-field and module receivers and strings. A dictionary key
named `len` cannot override the builtin. Preserve `b=a` deep cloning, nested
mutation independence, lexical snapshots, defaults/callbacks, Value/Scopes and
stdlib contracts. No public API, dependency, unsafe code, representation,
sharing, pipeline phase or module split is authorized. The pinned call-once
nucleus and all existing fallback obligations remain in force.

Inference to validate: this pure read can avoid a collection clone while
preserving language behavior; a changed value, effect, diagnostic or fallback
refutes it. Co-located independent tests must cover empty/small/large Array/Dict,
outer length, scalar detachment, lexical shadow/capture, mutation independence,
receiver-once, argument-effect fallbacks, empty spreads, reserved names, method
and actual-type discrimination, and error followed by valid evaluation. Require
RED before implementation and GREEN afterward, directed helper mutations,
public-pipeline regressions and isolated allocation evidence without a universal
speedup threshold. Preserve the known outline divergence from vanilla pin
`64c5e8c02fe0394d913ead36e42ef8c1334d2107` and StyleChain presence debt
`highlight=Some(None)`; this local optimization certifies no new vanilla parity.

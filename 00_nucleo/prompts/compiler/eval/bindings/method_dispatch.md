# L0 — compiler/eval/bindings/method_dispatch
Hash do Código: 03acd060

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
existing order. The mutation interceptor returns None for an unrecognized
nonmutating call. Preserve array/dictionary bounds, empty-collection, missing
argument, default and unexpected-argument diagnostics and span priorities.
first/last/at return the actual mutable slot, never a cloned substitute.

Argument helpers consume via remove_positional(0)/remove_named and finish_args;
they keep Args views, occurrences, names, individual origins and aggregate span
coherent. Never retain an occurrence carrier after modifying its views directly.

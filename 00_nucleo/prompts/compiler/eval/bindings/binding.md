# L0 — compiler/eval/bindings/binding
Hash do Código: f5915c2b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/binding.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/binding.rs:31` — `fn eval_let`.
- `01_core/src/compiler/eval/bindings/binding.rs:78` — `fn destructure_pattern<F>`.
- `01_core/src/compiler/eval/bindings/binding.rs:119` — `fn destructure_array<F>`.
- `01_core/src/compiler/eval/bindings/binding.rs:190` — `fn destructure_dict<F>`.
- `01_core/src/compiler/eval/bindings/binding.rs:262` — `fn wrong_number_of_elements`.
- `01_core/src/compiler/eval/bindings/binding.rs:300` — `fn destructure_let`.
- `01_core/src/compiler/eval/bindings/binding.rs:332` — `fn eval_destruct_assignment`.
- `01_core/src/compiler/eval/bindings/binding.rs:361` — `fn eval_assign`.

## Contract

Own let bindings, destructuring and assignment. An omitted initializer yields
None. Normal bindings and closure bindings preserve lexical capture and the
existing definition/initialization order. Destructuring accepts normal,
placeholder, parenthesized, array and dictionary patterns, including the existing
named and single-spread forms. Preserve arity, missing-value, missing-key,
duplicate-spread and incompatible-pattern errors at their causal spans.

Evaluate each initializer/right-hand side once. Simple assignment writes the
resolved place; compound assignment delegates the operator to
00_nucleo/prompts/compiler/eval/operators.md. Destructuring assignment reuses
mutable-place lookup rather than creating new bindings. Keep scopes and the
existing evaluation/error order; do not mutate closure snapshots indirectly.

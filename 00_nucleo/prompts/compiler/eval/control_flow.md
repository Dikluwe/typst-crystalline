# L0 — compiler/eval/control_flow
Hash do Código: 11c1a60f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/control_flow.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/control_flow.rs:20` — `fn eval_conditional`.
- `01_core/src/compiler/eval/control_flow.rs:48` — `fn eval_while`.
- `01_core/src/compiler/eval/control_flow.rs:101` — `fn eval_for`.
- `01_core/src/compiler/eval/control_flow.rs:137` — `fn run_for_loop`.

## Contract

Own conditional, while and for evaluation. Conditions require boolean values;
preserve condition/type errors and expression spans. Loops use the explicit
EvalContext iteration limit, lexical scopes and pattern destructuring. Join
iteration values through 00_nucleo/prompts/compiler/eval/operators/join.md.
Consume Break and Continue at the loop that owns them and propagate Return
unchanged, including value, span and conditional flag. Preserve iterable order,
the existing accepted iterable domains, first failure and evaluation order.
Do not introduce global loop state or perform I/O.

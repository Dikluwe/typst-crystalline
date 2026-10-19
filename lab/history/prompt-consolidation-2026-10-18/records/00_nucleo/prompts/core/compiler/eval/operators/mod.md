# L0 — core/compiler/eval/operators/mod
Hash do Código: 9555d34c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/mod.rs:38` — `fn eval_binary_op`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/ops.rs:344-359`.
- `typst-eval/src/ops.rs:52-66`.

## Contract

Own the static exhaustive BinOp dispatcher and existing unary/join reexports.
Delegate Add/Sub/Mul/Div/And/Or to
00_nucleo/prompts/core/compiler/eval/operators/arithmetic.md; Eq/Neq/In/NotIn to
00_nucleo/prompts/core/compiler/eval/operators/equality.md; ordering to
00_nucleo/prompts/core/compiler/eval/operators/ordering.md. Do not duplicate leaf
algorithms or diagnostic formatting. All operator functions remain pure over
their parameters, without EvalContext, scopes, I/O or global state.

The exclusive error formatter is
00_nucleo/prompts/core/compiler/eval/operators/error_formatting.md and sequential
combination belongs to 00_nucleo/prompts/core/compiler/eval/operators/join.md.
Unary evaluation delegates to arithmetic. Unsupported combinations retain the
leaf owner's diagnostic, not a hub-specific fallback.

Boolean short-circuit and percent-literal construction belong to
00_nucleo/prompts/core/compiler/eval/mod.md. Code-block and loop accumulation call join
but do not transfer their evaluation order to the operator module. Language
equality remains distinct from Rust PartialEq and internal observation.

# L0 — compiler/eval/math/expression
Hash do Código: 978f875f

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/expression.rs`. At HEAD `e754ab364a221dbe344852178e84bd42b02c463e` with a clean tree, `01_core/src/compiler/eval/math.rs:599-1554` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contract

Evaluate the exhaustive mathematical expression match while preserving morphology and evaluation order. Delegate `Expr::FuncCall` to `00_nucleo/prompts/core/compiler/eval/math/expression_call.md` without moving the arm. Preserve signatures, language semantics and diagnostics from `00_nucleo/prompts/core/compiler/eval/math.md`; preserve pipeline phase, defaults and compatibility. Extraction remains internal to L1, without I/O or mutable global state.

## Acceptance

The L1 suite and math tests must pass; `crystalline-lint .` must validate ownership and hashes. Do not infer performance gains.

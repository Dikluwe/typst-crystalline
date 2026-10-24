# L0 — compiler/eval/math/expression
Hash do Código: 9745e8e2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/expression.rs`. At HEAD `e754ab364a221dbe344852178e84bd42b02c463e` with a clean tree, `01_core/src/compiler/eval/math.rs:599-1554` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contract

Evaluate the exhaustive mathematical expression match while preserving morphology and evaluation order. Delegate `Expr::FuncCall` to `00_nucleo/prompts/core/compiler/eval/math/expression_call.md` without moving the arm. Preserve signatures, language semantics and diagnostics from `00_nucleo/prompts/core/compiler/eval/math.md`; preserve pipeline phase, defaults and compatibility. Extraction remains internal to L1, without I/O or mutable global state.

## Acceptance

The L1 suite and math tests must pass; `crystalline-lint .` must validate ownership and hashes. Do not infer performance gains.


## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Produce native Symbol for symbol lookup, Greek names, field access, single graphemes and shorthand. Numeric/quoted MathText remains its own carrier. Retain deprecation and scope shadowing behavior.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.

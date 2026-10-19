# L0 — stdlib/calc/numeric
Hash do Código: cfcdc6fc

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/calc/numeric.rs`. At HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` with a clean tree, `01_core/src/compiler/stdlib/calc.rs:1550-1570` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. This decision only separates algorithms by behavior.

## Contract

Share Int/Float conversion to f64 and the Float result guard used by these families. Preserve type messages and the NaN/infinity distinction; do not implicitly apply the guard to other functions.

Parity details, types, diagnostics and bindings remain in `00_nucleo/prompts/core/compiler/stdlib/calc.md`. Extraction preserves public signatures, module functions, phase, results and error policy. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and calc tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains from this separation.

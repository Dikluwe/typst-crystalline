# L0 — stdlib/shapes/polygon
Hash do Código: 8c5790ff

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/polygon.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:633-888` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Construct polygon and polygon.regular as Path with points and analytical bbox, vertex count, fill/stroke, defaults and closure. Preserve the regular formula and degenerate cases.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.

# L0 — stdlib/color/palette
Hash do Código: 6e48f880

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/color/palette.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/color.rs:141-245` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-color-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Maintain the 18 predefined colors and extra global bindings, preserving Luma/sRGB spaces, values and order. The color map remains in the parent.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/color.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and color tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.

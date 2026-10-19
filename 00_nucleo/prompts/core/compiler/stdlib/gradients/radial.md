# L0 — compiler/stdlib/gradients/radial
Hash do Código: 3a275b96

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/gradients/radial.rs`. At HEAD `b2705f23fe5bfb53e623fd3b65f8b4ea50815560` with a clean tree, `01_core/src/compiler/stdlib/gradients.rs:803-951` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-gradients-stdlib-2026-09-30-before.json.gz`.

## Contract

Construct a radial gradient with center, radius and focal. Preserve signatures, defaults, diagnostics and semantics from `00_nucleo/prompts/core/compiler/stdlib/gradients.md`. Extraction remains internal to L1 with static dispatch and without I/O or mutable global state.

## Acceptance

Preserve the L1 suite and gradient tests; `crystalline-lint .` must validate ownership and hashes. Do not infer performance gains.

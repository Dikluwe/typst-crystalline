# L0 — stdlib/html/attr_values
Hash do Código: decc16ad

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/html/attr_values.rs`. At HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b` with a clean tree, `01_core/src/compiler/stdlib/html.rs:1693-2033` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-html-2026-09-30-before.json.gz`.

## Contract

Own helper coercions for enums, lists, dates, durations, sizes and images, and their diagnostics. Preserve public signatures, defaults, diagnostic classes, element morphology and validation order specified in `00_nucleo/prompts/core/compiler/stdlib/html.md`. This extraction only separates responsibility within L1; dispatch remains static, without I/O or mutable global state.

## Acceptance

Preserve `html` tests, the L1 suite and valid `crystalline-lint .` results with correct ownership and hashes. Do not infer performance gains from separation.

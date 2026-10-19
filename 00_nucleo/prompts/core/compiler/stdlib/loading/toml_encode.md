# L0 — stdlib/loading/toml_encode
Hash do Código: 42f0970a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/toml_encode.rs`. At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, encoders were in `01_core/src/compiler/stdlib/loading.rs:40-646`; complete state in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. Separation is internal and is not a performance measurement.

## Contract

Convert Value to a TOML tree with a top-level dictionary, omit None in maps and reject None in arrays; preserve value classes, pretty/compact output, multiline quoting and causal diagnostics.

The detailed specification and historical evidence are in `00_nucleo/prompts/core/compiler/stdlib/loading.md`. A present snapshot is authoritative under the pinned nucleus; do not reconstruct from the current style.

## Acceptance

Preserve output and existing diagnostics, build, and pass the L1 suite and `crystalline-lint .`.

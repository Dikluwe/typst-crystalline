# L0 — stdlib/loading/json_encode
Hash do Código: aefaa6a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/json_encode.rs`. At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, encoders were in `01_core/src/compiler/stdlib/loading.rs:40-646`; complete state in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. Separation is internal and is not a performance measurement.

## Contract

Convert Value to a JSON tree with Content/LocatedContent projected through public fields; emit pretty or compact according to Args, preserving numeric exponents and strings without normalizing string content. Diagnostics preserve the value span.

The detailed specification and historical evidence are in `00_nucleo/prompts/core/compiler/stdlib/loading.md`. A present snapshot is authoritative under the pinned nucleus; do not reconstruct from the current style.

## Acceptance

Preserve output and existing diagnostics, build, and pass the L1 suite and `crystalline-lint .`.

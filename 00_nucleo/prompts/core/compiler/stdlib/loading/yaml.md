# L0 — stdlib/loading/yaml
Hash do Código: 7183d19f

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/yaml.rs`.
At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, `01_core/src/compiler/stdlib/loading.rs:687-741` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. This is internal separation under ADR-0109/0127/0129, without a new public contract or measured performance claim.

## Contract

Decode UTF-8 YAML through saphyr. One document produces one Value; multiple documents produce an ordered Array; scalars, sequences and maps preserve the existing projection. Invalid UTF-8 and parser input produce the contracted diagnostics.

Detailed rules and historical behavior evidence remain in `00_nucleo/prompts/core/compiler/stdlib/loading.md`; this unit inherits them while preserving parity and pipeline phases. The consumer operates in L1, receives data/World through arguments and introduces no direct I/O or mutable global state.

## Acceptance

Preserve language results and existing diagnostics. Build and pass the L1 suite and `crystalline-lint .` with valid ownership and hashes.

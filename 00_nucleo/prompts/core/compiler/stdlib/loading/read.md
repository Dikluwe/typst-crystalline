# L0 — stdlib/loading/read
Hash do Código: 1b460c8f

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/read.rs`.
At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, `01_core/src/compiler/stdlib/loading.rs:1155-1296` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. This is internal separation under ADR-0109/0127/0129, without a new public contract or measured performance claim.

## Contract

Validate and resolve the source for read and the five loaders. Path/Str use injected World; Bytes enter loaders directly. read distinguishes utf8 and none encoding and reports the first invalid UTF-8 byte position. Preserve spans, type names, messages and validation order; no direct filesystem access in L1.

Detailed rules and historical behavior evidence remain in `00_nucleo/prompts/core/compiler/stdlib/loading.md`; this unit inherits them while preserving parity and pipeline phases. The consumer operates in L1, receives data/World through arguments and introduces no direct I/O or mutable global state.

## Acceptance

Preserve language results and existing diagnostics. Build and pass the L1 suite and `crystalline-lint .` with valid ownership and hashes.

## Paths package

File access errors of `read` and the data loaders are reported as the world gives them, without a function prefix.

Approved on 2026-11-26 (“Aprovo A–C”): `00_nucleo/diagnostics/paths-package-2026-11-26.pt-BR.md`.

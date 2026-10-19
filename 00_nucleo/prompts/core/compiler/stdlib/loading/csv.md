# L0 — stdlib/loading/csv
Hash do Código: c368d28e

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/loading/csv.rs`.
At HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` with a clean tree, `01_core/src/compiler/stdlib/loading.rs:939-1154 e 1335-1451` contains the corresponding behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. This is internal separation under ADR-0109/0127/0129, without a new public contract or measured performance claim.

## Contract

Decode CSV into an Array of rows or header-keyed Dictionary, preserving delimiter, row-type, causal Args validation, spans and parser position. The native accepts its source and uses injected World only through the existing read contract; Bytes triggers no read.

Detailed rules and historical behavior evidence remain in `00_nucleo/prompts/core/compiler/stdlib/loading.md`; this unit inherits them while preserving parity and pipeline phases. The consumer operates in L1, receives data/World through arguments and introduces no direct I/O or mutable global state.

## Acceptance

Preserve language results and existing diagnostics. Build and pass the L1 suite and `crystalline-lint .` with valid ownership and hashes.

## Test-only legacy wrapper boundary

### Evidence before decision

compiler/stdlib/loading/csv.rs:19,25 contains the historical Clippy inventory
items. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
preserves sources/owners/references/UTC/status/diffstat. Call-site and productive
path research precedes classification; no new parity/performance evidence.

### Contract and acceptance

Compile internal decode_csv only under `#[cfg(test)]`, preserving signature/body for existing tests. The csv module is private and its reexport is already cfg(test). Private CsvErrorContext::Pure and its match arm are also cfg(test), as only this wrapper constructs it. Bytes/File and native CSV stay productive with intact spans/positions.

Preserve productive routes, existing test names/bodies, external APIs, defaults
and phase. This internal correction follows ADR-0127 continuous flow. Revalidate
all workspace-target build/check, core suite, test inventory, Clippy and lineage;
do not remove expectations to pass the gate.

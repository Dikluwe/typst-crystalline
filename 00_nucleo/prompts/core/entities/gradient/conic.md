# L0 — entities/gradient/conic
Hash do Código: 58929f63

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/gradient/conic.rs`

## Evidence before specification

At clean commit `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229`,
`01_core/src/entities/gradient.rs:796-911` contained the extracted behavior.
Source SHA-256, timestamp, and initial state are recorded in
`lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.

## Contract

Define Conic and its sampling methods. Preserve public contracts, defaults, sampling semantics, and the parity
criteria in `00_nucleo/prompts/core/entities/gradient.md`. Remain in L1,
without I/O or mutable global state.

## Acceptance

Gradient tests and the L1 suite must pass; `crystalline-lint .` must validate
ownership and hashes. Do not infer a performance improvement.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/gradient/conic.md`. Historical observations are not a new parity certification.

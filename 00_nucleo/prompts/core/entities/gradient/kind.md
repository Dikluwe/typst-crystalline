# L0 — entities/gradient/kind
Hash do Código: d27fbebf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/gradient/kind.rs`

## Evidence before specification

At clean commit `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229`,
`01_core/src/entities/gradient.rs:912-1074` contained the extracted behavior.
Source SHA-256, timestamp, and initial state are recorded in
`lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.

## Contract

Define the closed Gradient enum and constructors, accessors, and delegations shared by its variants. Preserve public contracts, defaults, sampling semantics, and the parity
criteria in `00_nucleo/prompts/core/entities/gradient.md`. Remain in L1,
without I/O or mutable global state.

## Acceptance

Gradient tests and the L1 suite must pass; `crystalline-lint .` must validate
ownership and hashes. Do not infer a performance improvement.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/gradient/kind.md`. Historical observations are not a new parity certification.

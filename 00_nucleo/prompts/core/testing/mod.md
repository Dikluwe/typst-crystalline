# L0 — Test verification module
Hash do Código: 295132e5

**Layer:** L1, test-only
**Exclusive consumer:** `01_core/src/testing/mod.rs`

## Contract

Declare only `pub(crate) mod math_oracle`. Contain no formulas, I/O, productive
logic or public reexports. Formula ownership belongs to
`00_nucleo/prompts/core/testing/math_oracle.md`.

## Acceptance

The oracle is available to internal crate tests and absent from production
builds. Do not duplicate oracle responsibilities in this owner.

# L0 — entities/content/equality
Hash do Código: c18f9f0d

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/equality.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own `PartialEq` and `Eq` for the closed `Content` algebra. Preserve all existing
distinctions and intentional equivalences, including
`Space == SpaceFromNewline`. Rust hashing is owned independently by
`entities/content/rust-hash.md`; language morphology is owned separately.

## Acceptance

Existing equality and representation tests pass; matches remain static and the
module contains simultaneous verification.

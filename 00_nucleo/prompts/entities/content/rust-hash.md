# L0 — entities/content/rust-hash
Hash do Código: fba6070e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/rust_hash.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the Rust `Hash` implementation for `Content`. Hashing must remain compatible
with `PartialEq`, including intentional equivalence between `Space` and
`SpaceFromNewline`, while preserving causal fields for specialized carriers.
This is mechanical Rust identity, not language morphology.

## Acceptance

Existing hash and cache tests pass; equal values produce equal hashes and the
implementation remains pure and statically dispatched.

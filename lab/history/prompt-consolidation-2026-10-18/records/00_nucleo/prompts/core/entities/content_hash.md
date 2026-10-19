# L0 — entities/content_hash
Hash do Código: 4e5ebc2f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content_hash.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own pub fn hash_content(content: &Content) -> u128 and its private seeded
hash helper. Remain pure L1 with no I/O, environment reads or global state.
Serialize through Content Debug, hash that string with DefaultHasher and seeds
0/1, then combine high and low u64 halves into u128. Introduce no external
dependency. Determinism is scoped to the same Rust/stdlib/compiler version;
Debug formatting changes may change the hash. Hash equality does not prove
content equality, and zero is a valid hash.

Equal Content values, clones and structurally equal sequences must have equal
hashes. Space and SpaceFromNewline have the same public hash, including nested
sequences; this hash is not a proof of causal origin. Float decimal Debug
serialization may collapse distinct bit patterns. Do not implement Content
Debug, Rust Hash, observational comparison, introspection or tag consumers.
Tests verify repeated determinism, representative distinctions and equal
hashes for equal content and the two space origins.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

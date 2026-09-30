# L0 — entities/content/traversal
Hash do Código: 252a3e45

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own bottom-up content transformation and text transformation through
`map_content` and `map_text`. Traversal remains exhaustive, static,
order-preserving, and pure. Variant-specific recursion delegates to the owning
element. Replacement nodes are not recursively reprocessed unless the
existing contract already requires it.

## Acceptance

Existing traversal tests pass unchanged and the module includes local
tests.

# L0 — entities/content/identity
Hash do Código: 12678c35

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/identity.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Provide the stable diagnostic name and bounded structural debug representation
of every closed `Content` variant through `Content::elem_name` and
`Debug for Content`.

- The implementation is a nominal, exhaustive, statically dispatched match
  over `Content`.
- It must not use a wildcard arm, dynamic dispatch, a registry, or a property
  map.
- Returned diagnostic names must remain stable.
- `Space` and `SpaceFromNewline` both return `"space"`, because their origin
  distinction is causal and not part of the public element identity.
- The method remains public with the signature
  `pub fn elem_name(&self) -> &'static str`.
- Adding a `Content` variant must force this owner to classify its diagnostic
  name at compile time.
- Debug formatting remains depth-bounded and delegates to an exhaustive static
  match. Existing output strings and payload formatting must not change.

## Acceptance

- Existing callers compile without changes.
- Existing identity and field-access tests remain green.
- `cargo test -p typst-core --lib` passes.
- `crystalline-lint .` reports no ownership, lineage, or prompt-drift
  violation after hashes are resealed.

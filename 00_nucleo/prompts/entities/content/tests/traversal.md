# L0 — entities/content/tests/traversal
Hash do Código: fba8f1b0

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/traversal.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for bottom-up `map_content`, recursive `map_text`, terminal preservation, and payload reconstruction. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

# L0 — entities/content/tests/representation
Hash do Código: 8afa531e

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/representation.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for construction, closed-variant representation, structural equality, cloning, defaults, and payload preservation. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

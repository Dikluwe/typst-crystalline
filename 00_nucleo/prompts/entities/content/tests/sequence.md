# L0 — entities/content/tests/sequence
Hash do Código: 24e68dd6

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/sequence.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for sequence normalization, ordering, cloning, equality, and nested sequence behavior. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

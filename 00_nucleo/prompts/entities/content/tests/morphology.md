# L0 — entities/content/tests/morphology
Hash do Código: 82eb78ce

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/morphology.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own regression coverage for language morphology identity and distinctions that must not collapse into Rust representation mechanics. Tests observe public or crate-visible
behavior and must not prescribe private implementation structure, dynamic
dispatch, or reverse layer dependencies.

## Acceptance

Cases retain independent assertions and fixtures, execute as
part of the `Content` test suite, and introduce no productive dependency.

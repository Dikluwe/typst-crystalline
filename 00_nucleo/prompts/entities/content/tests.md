# L0 — entities/content/tests
Hash do Código: de69f6e2

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/content/tests/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the composition of the behavior-focused `Content` test modules:
representation, sequence, traversal, morphology, and projections. Production behavior remains outside this test-composition owner.

The module must not contain test logic, helpers shared with production, or a
second implementation of any `Content` behavior.

## Acceptance

All five test modules execute through the standard core test suite and remain
independently readable. Test organization does not alter production behavior.

# L0 — compiler/stdlib/tests/support
Hash do Código: 26d5c505

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/support.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own shared Args, CounterKey, NullWorld, synthetic context/Engine/FileId fixtures and test helpers. All remain private to the test module tree; World performs no real I/O. Do not implement namespace behavior or create expectations from candidate output.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

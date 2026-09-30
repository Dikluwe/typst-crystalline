# L0 — compiler/stdlib/registry/tests
Hash do Código: 8756d111

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/tests.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own registry-surface controls: ordered bindings across feature combinations,
namespaces, format projections, PDF/bundle gates, inputs and fresh scopes.
Use independent fixed expectations, not candidate-generated expectations.
Declare this test module once under registry. Do not implement registrations,
argument validation or namespace behavior.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

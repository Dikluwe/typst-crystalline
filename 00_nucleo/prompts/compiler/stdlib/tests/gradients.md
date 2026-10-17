# L0 — compiler/stdlib/tests/gradients
Hash do Código: f609b428

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/gradients.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the gradients regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

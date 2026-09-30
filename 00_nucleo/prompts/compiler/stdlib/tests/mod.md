# L0 — compiler/stdlib/tests/mod
Hash do Código: c526a531

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Declare support before test families to provide null_ctx, import namespace symbols and compose test-only modules. Do not contain assertions, test algorithms or production logic.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

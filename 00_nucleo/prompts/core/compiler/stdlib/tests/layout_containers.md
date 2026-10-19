# L0 — compiler/stdlib/tests/layout_containers
Hash do Código: f8c3dd70

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/layout_containers.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own only the layout_containers regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Test quality maintenance

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/testes-ci-2026-10-13/before.json.gz`
preserves sources/hashes/UTC/status/diffstat. Input Clippy inventory is
matematica-ci-2026-10-13/clippy-inventory.json, retained in that batch's
evidence.json.gz. Under ADR-0108 this measurement precedes decision.
The inventory reports equality assertions against Bool literals and unused
pattern bindings. Observed fields include clip, breakable, sticky and overhang.

### Contract and acceptance

Express booleans with assert!/assert!(!...), preserving polarity/message;
rename unused bindings without removing the expression, execution or pattern.
Preserve every input/default/negative control. No productive-code change or new
lint exception. This internal test repair follows ADR-0127 continuous flow.
Quality RED is Clippy -D warnings; rerun cases and validate lineage. Tests without
a prior functional failure must not acquire an artificial functional RED→GREEN
claim.

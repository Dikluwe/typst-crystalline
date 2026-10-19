# L0 — compiler/stdlib/tests/support
Hash do Código: 024fecdf

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/support.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Evidence before specification

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz` and classification.json preserve dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`,2026-10-15T16:39:18.873477+00:00, sources/L0/signatures/hashes/status/diff HEAD --stat and reconfirmed spans/owner clauses. C121 make_bib_dict_full at 228 has 8 arguments; C122 make_bib_dict_full_p159g at 272 has 10, threshold 7. Structural warning is neither functional failure nor parity/cognitive-benefit measurement.

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/clippy-inventory.json` has 79 warnings; clippy-excecoes-2026-10-16/before.json.gz reconfirms source/spans/reasons/signatures/hashes/HEAD/dirty tree/UTC/stat before nominal materialization. Later authorization materializes only specified expectations, superseding original audit-only pending boundary for these two items.

## Exclusive test contract


Own shared Args, CounterKey, NullWorld, synthetic context/Engine/FileId fixtures and test helpers. All remain private to the test module tree; World performs no real I/O. Do not implement namespace behavior or create expectations from candidate output.



C121 is one test-only bibliography dictionary from eight supplied fields; direct function exception preserves independent fixtures. C122 retains ten explicit literal test fields; no builder solely to hide arity. Preserve signature/order/semantics/body/calls/cfg/unrelated attributes. Exact direct item expectations:

```rust
#[expect(clippy::too_many_arguments, reason="C121: Factory cfg(test) constrói um único dicionário bibliográfico com os oito campos fornecidos, sem política de produção")]
#[expect(clippy::too_many_arguments, reason="C122: Factory cfg(test) da extensão bibliográfica mantém dez campos explícitos e valores literais dos testes")]
```

Only these nominal lint/items override generic suppression bans. Never annotate module/impl/crate/global config, raise threshold, silence another lint or unfulfilled_lint_expectations. If arity drops, remove/review expectation. Conditional correction entries including C047/C066 stay visible/unannotated. Private-signature corrections elsewhere migrate callers/their L0 first; public/compatibility need separate concrete ADR-0127 approval.

## Acceptance and limits

Internal continuous-flow lint metadata only, no executable/cfg/public/default/phase/dependency/architecture change. Measure structural RED then all-target inventory: exactly 46 arity+5 test warnings satisfied by 49 expectations globally, no new warning, unfulfilled as error,11 correction arities remain visible. Do not attribute global totals to these two functions alone. Removing new attributes/lineage header must recover prior rustfmt-normalized source. Preserve language fixtures/tolerances/calls/functional controls and private no-I/O support. Revalidate build/fmt/strict-reverse lineage/Tekt no new errors/warnings/Counter and coverage tests under 12GiB/swap 0/jobs 1. Earlier complete suite remains evidence of unchanged executable code, no attribute-mirroring tests/performance comparison required. Ownership/nuclei/lineage stay valid; no unrelated consumer authorized.

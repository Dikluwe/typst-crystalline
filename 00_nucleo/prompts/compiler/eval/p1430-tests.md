# L0 — compiler/eval/p1430-tests
Hash do Código: 7fdb614b

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/p1430_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/p1430_tests.rs:16` — `struct TestWorld {`.
- `01_core/src/compiler/eval/p1430_tests.rs:56` — `fn id`.
- `01_core/src/compiler/eval/p1430_tests.rs:60` — `fn evaluate`.
- `01_core/src/compiler/eval/p1430_tests.rs:65` — `fn find_cases`.
- `01_core/src/compiler/eval/p1430_tests.rs:78` — `fn evaluated_cases`.
- `01_core/src/compiler/eval/p1430_tests.rs:87` — `fn direct_result`.
- `01_core/src/compiler/eval/p1430_tests.rs:100` — `fn direct_math_content`.
- `01_core/src/compiler/eval/p1430_tests.rs:109` — `fn p1430_delimitadores_malformados_sao_rejeitados`.
- `01_core/src/compiler/eval/p1430_tests.rs:133` — `fn math_body`.
- `01_core/src/compiler/eval/p1430_tests.rs:143` — `fn p1430_bare_converge_e_preserva_alignpoint_no_carrier`.
- `01_core/src/compiler/eval/p1430_tests.rs:155` — `fn p1430_ordem_de_avaliacao_propaga_primeiro_erro`.
- `01_core/src/compiler/eval/p1430_tests.rs:188` — `fn p1430_avaliacao_opaca_falha_fechado`.

## Contract

Own independent cases evaluation tests using real sources. Require bare/qualified
convergence, MathAlignPoint retained in the child, common invalid-input validation
and no cascaded fields fabricated in carrier/repr. Positional/named occurrences
evaluate once in source order; first failure ends construction without ignored,
reordered or fallback errors. Explicit named values win without manufacturing
presence for inherited ones. Preserve spread and error transport through the
common constructor boundary. Unsupported parser, ambiguous carrier, unattributed
diagnostic, unobservable order/counter, timeout or infrastructure failure is
Unknown and blocks acceptance, never presumed success.

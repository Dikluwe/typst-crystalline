# L0 — core/compiler/eval/math_frac_evaluation_tests
Hash do Código: c9c8c421

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_frac_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:17` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:58` — `fn evaluate`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:63` — `fn find_frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:76` — `fn evaluated_frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:85` — `fn frac`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:97` — `fn p1427_set_style_valida_e_respeita_precedencia`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:115` — `fn p1427_frac_bare_e_slash_convergem_no_carrier`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:142` — `fn p1427_slash_associa_esquerda_e_preserva_parenteses_causais`.
- `01_core/src/compiler/eval/math_frac_evaluation_tests.rs:172` — `fn p1427_observation_relation_compara_todos_os_campos`.

## Contract

Own independent fraction evaluation tests from the frozen contract, not productive
implementation. Evaluate real sources for explicit/cascaded/default vertical
precedence and invalid set-rule failures, bare/qualified/slash convergence,
left-associative a/b/c and causal deparenthesization. Only slash supplies exactly
one origin bit per removed exterior pair; bare and qualified calls invent none.
Compare complete optional style, both bits and line in both observation directions;
any isolated difference is Different. Do not infer success from render bytes,
consult a candidate for expected results or declare overall parity from this suite.

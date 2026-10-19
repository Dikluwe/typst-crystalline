# L0 — core/compiler/eval/math_root_evaluation_tests
Hash do Código: c1007a91

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_root_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:16` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:57` — `fn evaluate`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:62` — `fn find_root`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:75` — `fn evaluated_root`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:87` — `fn p1426_sintaxe_bare_preserva_vazio_none_e_conteudo`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:106` — `fn p1426_root_bare_exige_slots_e_preserva_presenca`.
- `01_core/src/compiler/eval/math_root_evaluation_tests.rs:119` — `fn p1426_observation_relation_observa_presenca_do_indice`.

## Contract

Own only tests of bare-root syntax and its observation relation. Evaluate real
sqrt(x) as None/not-explicit; root(,x) as Some(Empty)/explicit; present index as
Some(_)/explicit; evaluated none as None/explicit. root(x) remains a missing
radicand error, not a sqrt alias. Complete identical carriers compare Same;
carriers differing only in index_explicit compare Different in both directions,
never Unproven. Syntax belongs to 00_nucleo/prompts/core/compiler/eval/math.md and
comparison to 00_nucleo/prompts/core/compiler/eval/observation/math.md. Ambiguous
observations are Unknown and fail closed; auxiliary test structure is mechanics.

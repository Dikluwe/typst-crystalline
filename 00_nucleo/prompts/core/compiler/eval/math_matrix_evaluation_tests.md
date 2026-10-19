# L0 — core/compiler/eval/math_matrix_evaluation_tests
Hash do Código: c82d1718

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:16` — `struct TestWorld {`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:53` — `fn id`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:56` — `fn evaluate`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:60` — `fn find_mat`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:72` — `fn evaluated_mat`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:80` — `fn direct`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:94` — `fn p1431_bare_e_qualificado_convergem_sem_pre_resolver_alignpoint`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:117` — `fn p1431_bare_rows_arrays_padding_e_invalidos_usam_construtor_comum`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:150` — `fn p1431_avaliacao_ocorre_uma_vez_e_na_ordem_causal`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:208` — `fn p1431_cascata_nao_fabrica_presenca_e_explicito_ganha_por_eixo`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:244` — `fn p1431_set_rejeita_named_e_tipos_invalidos`.
- `01_core/src/compiler/eval/math_matrix_evaluation_tests.rs:271` — `fn p1431_observacao_de_ordem_opaca_falha_fechado`.

## Contract

Own independent mat evaluation tests from real sources, requiring bare/qualified
convergence for scalar/array rows, complete named values and rejections. Every
positional/named occurrence evaluates once in causal source order. A successful
mutable-array witness records one label per occurrence; a separate arguments
spread fixture proves expansion/order but is not used as an execution counter.
First error ends construction; cascade cannot manufacture presence/repr fields,
and explicit values win per axis. Unobservable parser/order/counter yields
UNKNOWN_EVALUATION_OBSERVATION and fails closed.

Set-rule negatives cover unknown, rows:, invalid delim/align/augment/gap/row-gap/
column-gap and duplicates, including mandatory #set math.mat(column-gap: 1).
Bare-route negatives cover mixed rows and invalid/unknown/duplicate named values;
no rejection may rely solely on testing the qualified route. This consumer owns
tests only, not the constructor or style schema.

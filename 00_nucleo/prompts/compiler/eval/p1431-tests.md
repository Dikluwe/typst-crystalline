# Testes independentes P1431 — avaliação de `math.mat`

## Dono test-only

Consumer único: `01_core/src/compiler/eval/p1431_tests.rs`.

## Obrigação

Avaliar fontes reais e exigir convergência bare/qualificada no mesmo carrier,
inclusive rows escalares e arrays, named completos e rejeições. Cada ocorrência
posicional/named é avaliada uma vez em ordem causal; o primeiro erro termina a
construção. Cascata não fabrica presença nem aparece no `repr`, e explícitos
ganham por eixo. Parser, ordem ou contador não observável classifica
`UNKNOWN_EVALUATION_OBSERVATION` e falha fechado.


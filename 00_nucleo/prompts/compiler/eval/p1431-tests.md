# Testes independentes P1431 — avaliação de `math.mat`

## Dono test-only

Consumer único: `01_core/src/compiler/eval/p1431_tests.rs`.

## Obrigação

Avaliar fontes reais e exigir convergência bare/qualificada no mesmo carrier,
inclusive rows escalares e arrays, named completos e rejeições. Cada ocorrência
posicional/named é avaliada uma vez em ordem causal; P20 usa uma chamada que
termina com sucesso e cujo array mutável registra exatamente uma etiqueta por
ocorrência, na ordem de fonte. Um fixture separado usa `arguments(..spread)`
e prova expansão e ordem sem servir como contador. O primeiro erro termina a
construção. Cascata não fabrica presença nem aparece no `repr`, e explícitos
ganham por eixo. Parser, ordem ou contador não observável classifica
`UNKNOWN_EVALUATION_OBSERVATION` e falha fechado.

N15 cobre no set rule: unknown, `rows:`, tipos inválidos de `delim`, `align`,
`augment`, `gap`, `row-gap` e `column-gap`, além de duplicados. Em particular,
`#set math.mat(column-gap: 1)` é testemunha causal obrigatória contra remoção
da validação desse eixo.

N14 repete pela rota bare mixed rows, todos os tipos inválidos, unknown e
duplicados dos seis named; nenhuma rejeição pode depender da rota qualificada.

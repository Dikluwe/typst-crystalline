# Registo — contradição de P583 resolvida, dois testes coexistentes

**Data:** 2026-07-05

## O que aconteceu

Houve duas versões do relatório de P583, com o mesmo commit citado, e conclusões opostas sobre a existência do teste `p581_cobertura_de_escape_e_shorthand_em_layout`.

P585 resolveu isto com prova directa: o teste existe, em `01_core/src/rules/layout/tests.rs:3488`, corre e passa (confirmado com o caminho completo do módulo, `rules::layout::tests::tests_set_rule_integration::p581_cobertura_de_escape_e_shorthand_em_layout`).

## Consequência

P584 foi executado com base na versão errada de P583 (a que dizia que o teste não existia). Como resultado, criou um segundo teste, `p584_escape_shorthand_linebreak_em_markup_preservados`, em `01_core/src/rules/eval/tests.rs`, cobrindo conteúdo parecido ao do teste original.

## Decisão

Nenhuma acção necessária. Dois testes a cobrir a mesma área não é um problema — é redundância inofensiva, não um bug. Não vale a pena gastar um passo a remover um dos dois.

O que ficou de valor real do P584, independentemente do engano na razão que o motivou: o documento `lab/parity/corpus/markup/escape-shorthand-linebreak.typ`, novo no corpus. P583 já tinha confirmado, por busca directa, que nenhum dos 90 ficheiros do corpus cobria escape ou shorthand antes disto. Esse vazio ficou fechado.

## Nota para o histórico

Este é o segundo caso, depois do bug de `#for` (P538h), em que uma verificação automática (`cargo test --workspace`) não apanhou um problema porque o problema não era de comportamento errado — era de cobertura em falta. Testes a passar não significam sempre que está tudo coberto.

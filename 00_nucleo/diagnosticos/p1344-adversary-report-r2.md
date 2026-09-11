# P1344 — relatório adversarial independente R2

## Veredito

`ADVERSARIAL_R2_REPEATED_CAUSES_BUDGET_STOP_BLOCKS_PRESEAL`.

Regime: **executado sem atestacao de isolamento**. O candidato produtivo não
foi lido, executado ou alterado. Contrato, oráculos, L0, passo e todos os
artefatos R1 permaneceram intocados. O corpus full de 168 casos não foi
executado: `full_corpus_runs = 0`.

## Resultado

Os 23 ataques protegidos R1 foram reexecutados focalmente e os 23 resultaram
`Violated`; portanto a revisão R2 corrigiu exatamente os vetores conhecidos.
Foram acrescentados 17 ataques R2, dos quais 14 sobreviveram. Resultado
agregado: 26/40 negativos rejeitados, 14 sobreviventes, mutation score `0.65`.
As 14 mutações de fonte novas foram parser-valid segundo `rustfmt`.

As defesas de owner-helper exato, chave JSON duplicada e authority root
rejeitaram os três controles adversariais correspondentes.

## Causas públicas e stop

- `OWNER_HELPER_PRODUCTION_IS_TOKEN_PRESENCE_NOT_CLOSED_GRAMMAR`: zero
  sobreviventes R2; fechada neste recorte.
- `INHERITED_AND_FUNC_PRODUCTIONS_NOT_CLOSED`: treze sobreviventes R2. Ainda
  são aceitos efeitos por contagem de identificadores, signatures `Eval`
  opacas, storage por alias, mutadores fora de `append`, writers mortos ou
  condicionais, fachada sem def-use fechado e observação sob `if false`.
- `UNKNOWN_LAUNDERED_WITHOUT_WITNESS`: um sobrevivente R2. Um objeto com os
  sete valores canônicos autoatesta os predicados de fonte/runtime e recebe
  `Unknown` sem que o chamador os tenha executado.

As duas últimas causas repetem classes públicas que já consumiram sua única
revisão focal R2. Isso aciona a condição de parada do corpus e o budget de
rendimento decrescente do passo. Este adversário não escolhe nem escreve a
correção: registra `BLOCKER_REQUIRES_PROTOCOL_OR_CONTRACT_REVIEW`.

O pré-selo permanece proibido, e não há autorização para uma R3 local. O único
full continua preservado para uma cadeia futura que primeiro resolva o blocker
por decisão arquitetural/protocolar independente.

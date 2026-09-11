# P1344 — relatório adversarial independente R1

## Veredito

`ADVERSARIAL_SURVIVORS_BLOCK_PRESEAL`.

Regime: **executado sem atestacao de isolamento**. O workspace é compartilhado;
a segregação aplicada é de papel, entradas pinadas, ordem e allowlist de
escrita. Não li candidato produtivo, não corrigi contrato/oráculo, não executei
o corpus full de 139 casos e não emiti selo.

## Resultado medido

Em `2026-09-11T00:03:19-03:00`, sobre HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, executei 23 ataques focais
válidos. As 19 mutações de fonte foram aceitas por `rustfmt`; quatro ataques
de autoridade/classificação não requeriam parser Rust.

- negativos corretamente rejeitados: 5/23;
- sobreviventes: 18/23;
- mutation score adversarial: `0.21739130434782608`;
- execuções do corpus completo: `0`.

O runner reproduz cada árvore exclusivamente em `/dev/shm/p1344-adversary-*`.
O JSON detalhado registra todos os IDs, resultados e reason codes.

## Classes de falha

1. `OWNER_HELPER_PRODUCTION_IS_TOKEN_PRESENCE_NOT_CLOSED_GRAMMAR` — oito
   ataques mostram que os helpers de `Content` e `CounterUpdate` aceitam
   presença solta dos nomes das variantes, assinatura sem `self`, sem `match`,
   sem encaminhar carrier, semântica invertida e até execução do callback.
   Owner físico e nomes dos métodos são verificados, mas a produção/effect
   rule prometida pelo contrato não é.
2. `INHERITED_AND_FUNC_PRODUCTIONS_NOT_CLOSED` — nove ataques mostram que a
   composição não reexecuta a gramática P1343 integral: helper `Func`
   não-terminante ou com storage alternativo, conjunto `EvalContext`
   substituído, argumento lateral em hook direto, método destrutivo extra no
   ledger, projeção mutável, segundo storage, troca do `World` na fachada e
   `loop` num expression wrapper foram todos classificados `Preserved`.
3. `UNKNOWN_LAUNDERED_WITHOUT_WITNESS` — o caso opaco não transporta payload
   dict nem predicados runtime não-payload. `judge_local` devolve `Unknown`
   unicamente pela identidade `P1344-P02`, após prova apenas de fonte.

As defesas de marker lexical e autoridade funcionaram nos controles: callee
apenas em comentário, owner apenas em string, chave JSON duplicada por escape,
corpus alternativo e runtime sintético foram `Violated`.

## Consequência para a cadeia

O contrato exige score `1.0`; o passo manda parar diante de sobrevivente. Uma
autoridade distinta precisa revisar focalmente o oráculo contra as três causas
públicas antes do pré-selo. Este adversário não escolhe nem escreve a correção.

O pré-verificador **não deve** consumir o único full enquanto os 18
sobreviventes não forem eliminados focalmente por autoridade distinta.

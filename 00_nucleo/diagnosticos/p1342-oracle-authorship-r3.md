# P1342 — revisão focal final R3 dos oráculos

Regime: **executado sem atestacao de isolamento**. Papel: autor independente
de oráculos, sem leitura de candidato futuro P1342. Veredito:
**ORACLE_REVISED_NOT_VERIFIED_NOT_SEALED**.

R1 e R2 permanecem intactos. Esta revisão final da classe responde somente aos
nove sobreviventes X05/X06/X09/X11/X15/X16/X17/X18/X22 encontrados pelo
adversário R2. Não amplia lifecycle/profile P1340, NT01–NT06, retenção,
descarte, invalidação, política terminal, equivalência geral nem aceitação de
candidato.

## Fecho focal

O checker R3 mantém integralmente as validações composite do R2 e antepõe à
fronteira de opacidade quatro verificações fechadas:

1. Cada `receipt_tail` runtime é reconciliado um-para-um por
   `(origin_cell, hook)` com um recibo de hook. Hooks sem evento raw, inclusive
   H00/transport, recebem recibo externo fechado; hooks semânticos carregam o
   último recibo de append raw da mesma célula/hook. O recibo liga challenge,
   célula, hook, contagem, tail raw e digest raw por SHA-256.
2. Cada challenge resolve um objeto issuer de schema fechado. O recibo liga
   modo, challenge, commitment, nonce externo e predecessor; os nonces são
   distintos e os três issuers formam a cadeia normal → repeat → reverse.
3. A ordem causal base é completa, não apenas parcial. Discovery usa seis
   eventos; selected usa quinze, desde `attempt-open` até `attempt-close`.
   Troca ou inserção permanece `Violated` mesmo após reconstruir raw, projeção
   e recibos.
4. `run` e `ledger` permanecem iguais aos endpoints de criação em todos os
   eventos da célula. `callback_with` criado é o mesmo do dispatch e da aresta
   With; `func_callback` criado é o mesmo da aresta, dispatch interno e
   entradas/saídas do body.

Essas verificações precedem a delegação ao R2 e, portanto, precedem `Unknown`.
O payload deliberadamente opaco só pode resultar `Unknown` depois de issuer,
tails, ordem, run/ledger e endpoints Func passarem. A evidência source/runtime
é entrada externa fechada e pinada; o DTO não autodeclara cobertura.

## Corpus e calibração

O corpus fechado tem exatamente 45 casos: um positivo inspectable, um positivo
deliberadamente opaco e 43 negativos válidos. A01–A21 e X01–X22 foram todos
reproduzidos como `Violated`; nenhum vetor foi declarado inválido.

Em `normal`, `repeat` e `reverse`, P01 foi `Preserved`, P02 foi `Unknown` e os
43 negativos foram `Violated`. A concordância foi total, score local `1.0` e
lista de sobreviventes vazia. Esta foi a primeira execução do R3; nenhuma
revisão adicional foi usada. A condição final de parada não disparou.

Esse resultado é calibração autoral contra os vetores fechados, não teste de
candidato, pré-selo ou certificado. O próximo poder de decisão permanece com
um adversário/verificador independente que forneça evidência real pinada.

## Proveniência

Execução em `2026-09-10T21:56:21Z`, HEAD
`2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado.
O estado amplo já continha alterações alheias; esta autoria criou somente
novos artefatos `p1342-oracle-*-r3.*`. O recibo JSON registra hashes dos inputs,
outputs e estado observado.

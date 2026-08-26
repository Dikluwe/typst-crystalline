# P1219 — fechamento da aritmética Decimal↔Int

## Veredito

**DECIMAL ↔ INT ARITHMETIC GREEN — APPLY-BINARY READJUDICATED.** As oito
direções de `+`, `-`, `*` e `/` fecharam e `eval-apply-binary` foi promovido
de `parcial` para `declarada-fechada`, com alegação limitada à semântica
pública dos operadores, não à mecânica Rust.

## Medição antes da decisão

Baseline HEAD `3f0a2638fffd8cddc0fde6e83058f69dedc7d838`, working tree não
commitado, congelado em 2026-08-26T16:15:20-03:00. Vanilla ratificado SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Os 22 oráculos foram derivados de `foundations/ops.rs:102-111,183-192,
227-236,301-310` antes do candidato.

O RED focado foi 0/2, com `cannot add decimal and integer`. Depois da mudança,
duas rodadas deram 22/22 focais, 58/58 controles P1216/P1217 e 16/16 unários
P1218 exatos. Oito compilações deram 4/4 positivos e 4/4 erros com mensagem e
região iguais.

Uma primeira sonda de array obteve 21/22 porque o CLI cristalino não serializa
array de Decimal em JSON. A entrada foi invalidada e ressellada como
`repr(array)`, que observa o operador sem confundir o serializer; então ambas
as rodadas fecharam 22/22.

## Implementação e ataques

Cada Int é convertido exatamente por `Decimal::from_i64`; não há passagem por
Float. As operações mistas usam `checked_add/sub/mul/div`, preservam direção e
traduzem falha para `value is too large`. O gate existente cobre Int zero e
Decimal zero com escala.

Os 16 mutantes válidos foram rejeitados: `mutation_score = 16/16 = 1.0`.

## Readjudicação DSM

`p1219-binop-readjudicacao.tsv` contém 19/19 variantes, owners atuais, probes
e nenhum `remaining_gap`. Assign e assigns compostos permanecem no evaluator
de atribuição por desenho. `decimal-int-arithmetic` está RESOLVED; o próximo
cluster aberto é `svg-morphology`.

Esta materialização usou o protocolo completo numa única sessão e foi
**EXECUTADA SEM ATESTAÇÃO DE ISOLAMENTO**.

## Gates finais

Passaram os testes focados P1216–P1219, `cargo test --workspace`, builds
workspace/release, `crystalline-lint .`, fmt e `git diff --check`. A lente DSM
produziu o mesmo SHA-256 nas duas rodadas:
`1fe99432417381468747263b09a1424b09efbb966d60648e0edcf29413a69f92`.
Hashes e vereditos estão no certificado `p1219-tekt-certificado.tsv`.

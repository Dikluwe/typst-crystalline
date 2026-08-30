# P1268 — selar contrato, oráculos e ataques do envelope corrigido

**Estado:** PLANEADO
**Predecessor:** P1267
**Regime:** protocolo Tekt completo; preseal antes de candidato

## Objetivo

Transformar a readjudicação P1267 num contrato discriminatório selado, sem
adaptar oráculos à implementação cristalina vigente.

## Autoridades e ordem

1. O autor da obrigação congela baseline `a51e02804`, população 96, contrato
   v2, observáveis e política `Unknown`.
2. O autor do contrato não lê patch candidato nem resultados privados de uma
   futura correção.
3. O autor dos oráculos executa primeiro somente o vanilla e deriva sources,
   budgets, máscaras, landmarks e limites de custo por intervalo.
4. O adversário cria mutantes contra cada cláusula corrigida: alterar stop
   original, mover apenas stop adaptativo dentro/fora do budget, perder
   descontinuidade, truncar um intervalo, expansão não limitada, apagar stroke
   degenerado e pintar fill sem área.
5. O verificador sela apenas se todos os mutantes válidos forem rejeitados e os
   positivos forem preservados; `Unknown` só vale para opacidade declarada.

## Gates

- `mutation_score=1.0` com zero mutante necessário classificado `Unknown`;
- repetição e ordem inversa semanticamente idênticas;
- nenhum budget derivado da saída candidata;
- hashes do manifesto, contrato, baseline, corpus, oráculos e ataques;
- capacidades por papel registradas, sem alegação de isolamento se a
  infraestrutura não o provar.

## Trava

Produto, L0 e testes produtivos permanecem inalterados. Mudança em qualquer
entrada selada invalida o preseal e reinicia a primeira fase afetada.

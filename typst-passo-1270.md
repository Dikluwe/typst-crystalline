# P1270 — decompor causalmente as divergências numéricas remanescentes

**Estado:** PLANEADO
**Predecessor:** P1269, somente se houver falhas numéricas
**Regime:** diagnóstico causal; nenhuma implementação

## Objetivo

Encontrar a primeira divergência causal de cada cluster numérico, em vez de
ajustar a implementação ou os budgets por tentativa.

## População inicial

Usar somente as falhas certificadas pelo P1269. A lista histórica do P1266
(S03, S05, S06, S09, S11, S12, S16, S20 e S22 em pares aplicáveis) é pista,
não população normativa até ser reproduzida pelo contrato v2.

## Método

1. Comparar vanilla e cristalino em checkpoints: domínio local, sampling no
   espaço nativo, conversão sRGB, clamp, premultiplicação, decisão de
   bissecção, offset serializado e curva reparsada.
2. Para cada primeira divergência, registrar `file:line`, valores de entrada e
   saída, precisão, ramo e efeito nos quatro budgets.
3. Classificar língua versus mecânica e intenção versus comportamento conforme
   ADR-0107/0108; declarar a inferência e o contraexemplo que a refutaria.
4. Executar contrafactuais de uma variável por vez, sem editar a baseline nem
   alargar budgets.
5. Agrupar apenas fixtures com a mesma primeira causa demonstrada. Causas
   distintas tornam-se materializações distintas, ainda que o sintoma seja o
   mesmo p95.

## Saídas

- tabela fixture→primeira causa→owner;
- witness mínimo e mutante inverso por cluster;
- decisão `CONTRACT-ERROR`, `PRODUCT-DIVERGENCE` ou `UNKNOWN`;
- proposta L0 por owner somente para clusters `PRODUCT-DIVERGENCE`;
- ordem de materialização que evite uma correção mascarar outra.

## Gate

Sem causa reproduzível não há autorização para código. `Unknown` permanece
bloqueante e não pode ser convertido em tolerância adicional.

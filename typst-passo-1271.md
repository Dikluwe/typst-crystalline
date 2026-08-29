# P1271 — materializar as correções numéricas causalmente provadas

**Estado:** PLANEADO E CONDICIONAL
**Predecessor:** certificado causal P1270
**Regime:** L0-first, RED→GREEN e protocolo Tekt por cluster

## Objetivo

Corrigir exclusivamente os clusters `PRODUCT-DIVERGENCE` demonstrados pelo
P1270, preservando o contrato e os oráculos selados.

## Ordem obrigatória por cluster

1. Identificar o único Prompt L0 proprietário do consumer afetado.
2. Redigir no L0 a medição `file:line`, a decisão interna, o scope-out e os
   critérios de linguagem antes de escrever código.
3. Se a correção alterar contrato público, comportamento por defeito, fase de
   pipeline ou compatibilidade, **PARAR** no gate ADR-0127. Correção interna de
   paridade segue em fluxo contínuo após o L0.
4. Ressellar a linhagem sem tocar contrato, baseline, corpus ou oráculos P1268.
5. O testador independente escreve o RED a partir do contrato selado.
6. O implementador aplica a menor correção no owner; não altera `Color`
   globalmente, PDF P274, outros espaços ou o fallback produtivo por arrasto.
7. Executar o witness, o cluster, os quatro pares, P1234/P1236/P1237/P1264 e
   mutantes focais antes de aceitar GREEN.

## Atomização

Se P1270 encontrar mais de uma primeira causa, cada causa recebe um subpasso
numerado e um resselo próprio. Este documento não autoriza uma implementação
genérica que tente fechar vários sintomas de uma vez.

## Proibições

- alargar budgets ou mover máscaras;
- quantizar antes da lente;
- copiar bytes, contagem ou passos do vanilla sem classificação de linguagem;
- alterar o cap por intervalo sem nova decisão medida;
- promover qualquer par durante a correção.

# P1267 — readjudicar a conjunção P1266 no nível da linguagem

**Estado:** PLANEADO
**Predecessor:** P1266
**Regime:** auditoria de obrigação; nenhuma escrita produtiva

## Objetivo

Corrigir o contrato de generalização antes de tentar corrigir o produto. O P1266
misturou três obrigações de linguagem com critérios mecânicos ou geometricamente
incorretos. Os recibos P1266 permanecem imutáveis; este passo produz um contrato
sucessor, nunca reescreve o passado.

## Medição antes da decisão

- `p1266-generalization-contract.tsv:5` tratou todos os offsets efetivos como
  morfologia pública, mas `svg.md:144-146` exige os offsets **originais**, ordem,
  descontinuidades, alpha e envelope; a posição literal de stops adaptativos é
  mecânica quando esses observáveis permanecem preservados.
- `p1266-generalization-contract.tsv:11` impôs `cap <=64` global. O L0
  `adaptive.md:53-67` mede e especifica máximo de 64 subdivisões **por
  intervalo**, e `adaptive.md:75-76` declara mecânica a quantidade literal de
  stops. O máximo agregado 774 do P1266 não prova sozinho regressão.
- `p1266-generalization-contract.tsv:8` exigiu paint de área zero para toda caixa
  degenerada. `typst-p1266-svg-generalization.md:43-46` mediu que S20 possui
  stroke de 8pt e portanto área pintada positiva; S21 é fill de altura zero.
- Todos os 74 casos que falharam a conjunção falharam G04; 24 também falharam
  budgets numéricos, 36 também o falso cap global e quatro também a obrigação
  de área zero.

## Tarefas

1. Classificar separadamente stops declarados pelo utilizador e stops
   adaptativos introduzidos pelo exporter.
2. Para stops declarados, conservar offset, ordem, coincidência,
   right-continuity, cor e alpha como morfologia obrigatória.
3. Para stops adaptativos, observar ordem, suporte da curva e envelope
   cromático/alpha; não exigir igualdade do carrier `f32/f64`, contagem ou
   ortografia quando não alteram a linguagem.
4. Substituir o cap global inventado por um limite derivado e publicável por
   intervalo, comprovando terminação e ausência de expansão não limitada.
5. Separar S20 e S21: stroke degenerado deve preservar a região efetivamente
   pintada; fill sem área deve permanecer sem paint positivo. Ambos exigem grafo
   fechado e geometria correta.
6. Marcar cada mudança como `CONTRACT-CORRECTION`, `PRODUCT-DIVERGENCE` ou
   `UNKNOWN`, com a evidência que refutaria a classificação.

## Saídas obrigatórias

- delta normativo P1266→P1267 por cláusula;
- contrato candidato v2 e política `Unknown` v2;
- inventário de observáveis língua/mecânica;
- decisão explícita sobre S20, G04 e G10;
- manifesto com P1266 final certificate
  `sha256:dce8aabec0d6bbf8b714d75f8cfc22e38f83539d04f1c8d034110c8dfa1067ec`.

## Gate

Nenhum resultado candidato pode ser convertido em sucesso neste passo. O
contrato v2 segue para autoria independente de oráculos e ataques no P1268.

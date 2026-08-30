# P1259 — ensaio localizado midpoint versus quartos

**Estado:** EXECUTADO — QUARTOS REJEITADOS; RED MANTIDO  
**Predecessor:** P1258

## Objetivo

Testar se probes em 1/4, 1/2 e 3/4 de cada segmento discriminam os máximos
residuais que uma política somente-midpoint pode perder. Ambas usam threshold
`0.001`, split binário localizado e profundidade máxima 6 por intervalo.

## Contrato

- samples públicos vanilla ratificado e budgets congelados;
- cor sRGB codificada premultiplicada e alpha separado;
- `color_max`, `color_p95`, `alpha_max` e `alpha_p95` obrigatórios;
- 24 fixtures e quatro pares adjudicados individualmente;
- custo publicado por fixture; nenhuma promoção ou mudança produtiva.

## Gate

Quartos só podem virar candidato se aumentarem cobertura sem violar alpha e
com custo localizado. Qualquer fixture sobrevivente mantém a política em RED.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

## Resultado executado

As duas execuções foram byte-idênticas. Incluindo cor e alpha:

- midpoint fechou 12/24 fixtures;
- quartos fechou as mesmas 12/24 fixtures;
- quartos aumentou o número de stops em todas as famílias relevantes, sem
  produzir ganho de cobertura;
- sobreviveram, em Linear/Radial, `two-wide-stroke`,
  `alpha-mid-wide-stroke` e `alpha-last-tall-fill` para Oklab e LinearRgb;
- em `alpha-mid` LinearRgb, quartos reduziram p95 e alpha, mas elevaram o
  `color_max` de aproximadamente `0.006429145` para `0.006721554`;
- os máximos de `two-wide-stroke` ficaram inalterados.

## Veredito

A mutação “midpoint → midpoint+quartos” é rejeitada: aumenta custo e não reduz
o conjunto de falhas. Não será materializada. As 18/24 violações do corpus
produtivo continuam abertas; este ensaio ideal fecha apenas 12/24 quando alpha
também é exigido.

O próximo passo deve localizar se os máximos sobreviventes coincidem com
descontinuidades/limites declarados ou com a política de gamut. Refinar mais
pontos sem essa classificação seria tuning cego.

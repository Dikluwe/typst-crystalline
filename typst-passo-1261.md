# P1261 — corrigir os quatro máximos endpoint-near

**Estado:** EXECUTADO — QUATRO ENDPOINT-NEAR FECHADOS, ZERO PARES PROMOVIDOS  
**Predecessor:** P1260  
**Owner inicial:** `00_nucleo/prompts/infra/export/gradients/adaptive.md`

## Escopo congelado

Somente os quatro `color_max` de `two-wide-stroke`, Linear/Radial ×
Oklab/LinearRgb, próximos de `t=1`, sem saturação pública. Excessos atuais:
Oklab aproximadamente `7.2239e-5`; LinearRgb aproximadamente `7.8475e-5`.

## Contrato RED

1. congelar endpoint, penúltimo stop emitido e `worst_t` de cada par;
2. medir separadamente sample exato, aproximação pré-serialização, offset
   serializado, cor/opacity serializadas e reparsed;
3. provar que o último endpoint ocorre uma vez, com ordem e cor exatas;
4. reproduzir o excesso no intervalo final sem depender da role stroke;
5. preservar o restante da curva e o cap 64.

## Ataques obrigatórios

- omitir ou duplicar o endpoint final;
- arredondar o último offset antes da decisão;
- usar `t=1` para esconder o erro em `t<1`;
- aumentar globalmente cap/stops;
- corrigir somente Linear e inferir Radial;
- promover Oklab a partir de LinearRgb ou vice-versa.

O contrato só sela com mutantes executáveis, todos rejeitados. `Unknown` não
entra no numerador.

## Implementação condicionada

Se o RED surgir dentro de `svg_adaptive_stops`, atualizar primeiro o L0
`adaptive.md`, ressellar e corrigir somente a representação/decisão do último
intervalo. Se surgir apenas em `fmt_num`/`write_gradient_stops`, o owner muda
para `infra/export/svg.md`; não editar adaptive por conveniência.

Esta é correção interna de paridade e pode seguir fluxo contínuo ADR-0127.
Qualquer mudança de comportamento público geral da conversão de cor exige
paragem.

## Fechamento

- os quatro excessos endpoint-near ficam dentro dos envelopes congelados;
- nenhum outro máximo/p95/alpha regride;
- custo adicional é limitado ao intervalo causal;
- P1237 é reexecutado, mas somente os pares que atingirem `6/6` podem ser
  promovidos.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` se não houver isolamento forte.

## Resultado executado

O RED localizou a primeira divergência em `write_svg_gradient_stops`, não em
`svg_adaptive_stops`: stops, cores e offsets pré-serialização coincidiam; o
candidato emitia o penúltimo offset como `0.984375`, enquanto o `Ratio::repr`
ratificado emitia `98.44%`. O owner efetivo mudou para
`00_nucleo/prompts/infra/export/svg.md`, atualizado e ressellado antes da
implementação.

A correção ficou limitada à representação do último intervalo adaptativo. Os
quatro `color_max` passaram: Oklab `0.008998699508545004` e LinearRgb
`0.020706225660859`, sem regressão de p95/alpha, sem mudança dos outros 20
SVGs e preservando cap 64. Sete mutantes executáveis foram rejeitados
(`mutation_score = 1.0`).

O P1237 reexecutado ficou em 1/6, 1/6, 4/6 e 4/6 no gate numérico; nenhum par
atingiu 6/6 ou foi promovido. Recibos em
`00_nucleo/diagnosticos/typst-p1261-endpoint-near.md` e
`00_nucleo/diagnosticos/p1261-manifest.tsv`.

**Veredito:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

# P1260 — classificar fronteiras dos 18 gaps SVG

**Estado:** EXECUTADO — TRÊS FRONTEIRAS CAUSAIS  
**Predecessores:** P1255, P1256 e P1259

## Objetivo

Classificar cada pior ponto das 18 fixtures produtivas violadas como stop
declarado, vizinhança de endpoint ou interior de segmento. Registrar também se
o sample público vanilla no ponto está saturado em algum canal RGB e separar
alpha máximo zero de divergência real.

Saturação pública significa somente canal `00/ff` após o observável `to-hex`;
não prova, sozinha, que a cor fonte estava fora do gamut.

## Gate

- endpoint-near pode justificar teste focal de representação do último
  intervalo, não aumento global de cap;
- interior saturado exige separar gamut/clamp;
- interior não saturado mantém sampling/interpolação como owner candidato;
- nenhuma classe promove paridade ou autoriza código sem teste RED próprio.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

## Resultado executado

As duas execuções foram byte-idênticas. Nas 18 fixtures violadas há 36 linhas
de máximo (cor e alpha); 22 métricas realmente excedem seu envelope:

- 4 `color_max` ficam perto do endpoint final, todos nos witnesses
  `two-wide-stroke`, sem saturação pública;
- 12 `color_max` ficam no interior: 8 com canal público saturado e 4 sem
  saturação;
- 6 `alpha_max` ficam no interior e sem saturação RGB;
- nenhum máximo aberto coincide com stop declarado;
- 8 alpha máximos são zero e 6 outras métricas pertencem a fixtures violadas,
  mas individualmente já estão dentro do envelope; não contam como fronteira
  aberta.

## Decisão

O problema não possui um único owner:

1. endpoint final não saturado → representar/testar o último intervalo;
2. interior com saturação → política de gamut/clamp antes de sampling;
3. interior sem saturação → sampling de cor ou alpha separado.

Isso refuta nova tentativa global de threshold, cap ou probes. O próximo passo
deve atacar primeiro as quatro fronteiras endpoint-near, o menor grupo causal,
com teste RED que preserve o endpoint exato e o intervalo anterior.

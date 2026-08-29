# P1265 — decisão do dono sobre promoção SVG

**Estado:** EXECUTADO — OPÇÃO B CONFIRMADA  
**Decisão:** ampliar o envelope antes da promoção

Em `2026-08-28T19:52:26-03:00`, o dono escolheu explicitamente a opção B do
gate ADR-0127. O fragmento P1264 continua certificado, mas não é extrapolado
para toda entrada pública Linear/Radial × Oklab/LinearRgb.

Nenhuma mudança foi feita em L0, `paint_is_svg_native`, testes produtivos ou
código. O fallback diagnóstico `gradient-color-space` permanece o
comportamento por defeito desses quatro pares.

O sucessor `typst-passo-1266.md` foi criado com:

- 24 seeds válidos por par, 96 fixtures no total;
- cardinalidades de 2 a 32 stops e distribuições variadas;
- alpha/gamut extremos, coincidência e quase coincidência;
- constructors, `sharp` e `repeat` com preservação de `anti_alias`;
- fill/stroke, transformações e caixas positivas/degeneradas;
- ângulos e geometria focal radial válidos;
- oito famílias de probes inválidos do domínio;
- contrato de 14 obrigações, 24 mutantes planejados e política `Unknown`
  explícita.

Offsets fora de ordem/limite não foram tratados como gradients válidos: o L0
vigente exige offsets explícitos completos, monotônicos e em `[0,1]`; esses
casos devem ser rejeitados pela superfície pública.

O contrato P1266 foi redigido sob `/root`, mas oráculos, adversário e
verificador permanecem deliberadamente não atribuídos. Portanto, este turno
registra a decisão e prepara o protocolo; não alega preseal, mutation score ou
atestação de isolamento do futuro corpus.

Qualquer promoção produtiva posterior requer que cada par passe o P1266
independentemente e volte a uma nova confirmação do dono pelo ADR-0127.

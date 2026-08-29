# P1261 — quatro máximos endpoint-near

**Estado:** EXECUTADO — QUATRO WITNESSES FECHADOS, ZERO PARES PROMOVIDOS  
**Regime:** protocolo Tekt completo, executado sem atestação de isolamento

## Causa medida

Os quatro witnesses já coincidiam com o vanilla ratificado em quantidade de
stops, cores, penúltimo offset pré-serialização (`0.984375`) e endpoint final.
A primeira divergência ocorria no owner SVG: o candidato emitia
`offset="0.984375"`; o vanilla passa o ratio por `repr` e emite `98.44%`,
numericamente `0.9844`. Assim, `adaptive.md` não era o owner da correção.

O L0 `infra/export/svg.md` foi atualizado antes da implementação. A produção
agora usa a representação percentual de duas casas somente no último
intervalo do ramo adaptativo. Stops nativos, `fmt_num`, cores, alpha, decisões
de subdivisão e cap 64 permanecem inalterados. O endpoint continua único,
ordenado e com cor `#0074d9`.

## RED → GREEN e envelope

O teste focal Linear/Radial × Oklab/LinearRgb falhou primeiro no offset
`0.984375` e passou depois da correção. Os quatro `color_max` mudaram:

- Oklab: `0.009071938982020467` → `0.008998699508545004`, limite
  `0.0089997`;
- LinearRgb: `0.020785701026582535` → `0.020706225660859`, limite
  `0.020707226000000002`.

Os p95 não regrediram, alpha permaneceu zero, contagens ficaram em 25/29,
e os outros 20 SVGs ficaram byte-idênticos. A decomposição exato →
pré-serialização → offset → cor/opacity → reparse está em
`p1261-endpoint-stages.tsv`.

## Ataques e P1237

Sete mutantes executáveis foram rejeitados: omitir endpoint, duplicá-lo,
arredondar antes da decisão, medir apenas `t=1`, elevar cap globalmente,
corrigir apenas Linear e promover Oklab a partir de LinearRgb. Mutation score:
`7/7 = 1.0`; `Unknown` não entrou no numerador.

O P1237 foi reexecutado sobre os 24 fixtures. O gate gráfico permanece 6/6 em
todos os pares; o gate numérico ficou em 1/6 (Linear/Oklab), 1/6
(Radial/Oklab), 4/6 (Linear/LinearRgb) e 4/6 (Radial/LinearRgb). Portanto
nenhum par foi promovido.

## Gates e limitação de atestação

Passaram `cargo fmt --check`, `git diff --check`, a suíte do owner SVG,
`cargo build` e `crystalline-lint .` com zero violações. As entradas e duas
execuções completas foram byte-idênticas.

Contrato, adversário, implementação e integração do veredito foram exercidos
sob a mesma autoridade `/root`. Os oráculos P1231 estavam congelados, mas isso
não fornece isolamento forte de capacidades. Veredito proporcional:
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**; o resultado cobre somente os quatro
máximos endpoint-near, não equivalência funcional geral.

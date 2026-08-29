# Certificado P1278 — refinamento SVG polar

## Veredito

`POLAR-REFINEMENT-PRESERVED-NOT-PROMOTED`.

Na matriz congelada de P1277, os seis pares polares melhoraram de
83/144 para 134/144 fixtures gerais preservados, sem
regressão por par. Raster fechou 144/144, focal público 336/336,
domínio inválido 56/56, determinismo
384/384 e ataques
30/30.

| Par | P1277 válido | P1278 válido | Raster P1278 |
|---|---:|---:|---:|
| linear/oklch | 14/24 | 22/24 | 24/24 |
| radial/oklch | 15/24 | 22/24 | 24/24 |
| linear/hsl | 14/24 | 23/24 | 24/24 |
| radial/hsl | 14/24 | 23/24 | 24/24 |
| linear/hsv | 13/24 | 22/24 | 24/24 |
| radial/hsv | 13/24 | 22/24 | 24/24 |

## Unknown remanescente

Dez fixtures excedem somente o budget `color_p95`; todos os demais gates
aplicáveis passam. Elas permanecem `Unknown-generalization`:

| Fixture | color_p95 | limite vanilla + 1e-6 |
|---|---:|---:|
| S06-linear-oklch | 0.003601088237497192 | 0.003593484514311737 |
| S06-radial-oklch | 0.003601088237497192 | 0.003593484514311737 |
| S11-linear-hsv | 0.003706521473974237 | 0.00370625125947493 |
| S11-radial-hsv | 0.003706521473974237 | 0.00370625125947493 |
| S16-linear-oklch | 0.002696842086348248 | 0.002693971685555013 |
| S16-radial-oklch | 0.002696842086348248 | 0.002693971685555013 |
| S16-linear-hsl | 0.0024456242185255258 | 0.0024319209080343285 |
| S16-radial-hsl | 0.0024456242185255258 | 0.0024319209080343285 |
| S16-linear-hsv | 0.0023866224636436314 | 0.0023543075642044357 |
| S16-radial-hsv | 0.0023866224636436314 | 0.0023543075642044357 |

Não houve arredondamento de veredito, aumento de budget ou promoção cruzada.
Luma permaneceu controle negativo (7/24 por par) e CMYK continuou fora do
escopo por ADR-0097.

## Fronteira produtiva

`paint_is_svg_native` não foi ampliado. Os 192/192 grupos mantiveram fallback
produtivo, portanto `productive_promotions_applied = 0`. Uma promoção futura é
mudança de comportamento por defeito e exige o gate ADR-0127.

## Proveniência

- baseline: upstream/main `a51e02804` ratificado;
- HEAD medido: `9aade8e53b93efc5a05f5bb3c5d98b67fc8296b5` com working tree não commitado;
- medição: `2026-08-29T14:08:44-03:00`;
- resultados gerais reproduzidos com SHA-256 `4172653e3198a7d9756288b52c1fe6c64caf8054ff079082235fafffe55d5d94`;
- focal reproduzido com SHA-256 `f6dabe249c0d5c3a27d4296961a37c0428f1842c1e6cb3928e5eaddae3771992`.

## Atestação

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`. Os papéis foram separados por ordem e
artefatos, mas compartilharam checkout e capacidade de leitura.

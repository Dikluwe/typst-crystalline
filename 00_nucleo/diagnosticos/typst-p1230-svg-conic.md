# P1230 — Conic SVG vetorial

**Veredito:** `ACCEPTED_WITH_CONTRACTUAL_UNKNOWN`.

O exportador SVG agora preserva Conic como `pattern` vetorial de cunhas e
subgradients nos espaços sRGB, Oklab, LinearRgb e Hsv. O grafo fecha sem URL
pendente ou rasterização e preserva centro, ângulo, sentido horário, aspect
ratio, stops coincidentes, alpha, fill, stroke e matriz local.

A primeira adjudicação foi rejeitada corretamente. Ela revelou que o paint do
stroke usava a caixa geométrica, não a caixa pintada. Após incluir metade da
espessura em cada lado, C6 passou com deltas zero. A divergência C7 vinha apenas
da tradução externa de `align`; removê-la preservando literalmente as matrizes
locais de rotate/scale produziu deltas zero, conforme a separação B02/U04.

Foram executadas 21 fixtures em ordem direta/inversa, duas repetições e raster
1x/2x/4x. As 20 combinações locais contratadas passaram. O gate adversarial
rejeitou 35/35 mutantes. CMYK continua `Unknown` por ADR-0097, e
grid/gutter/flow externos continuam `Unknown`; por isso o cluster permanece
parcial. A medição partiu do HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`
em working tree não commitada, inicialmente às `2026-08-26T21:35:42-03:00`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

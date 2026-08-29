# P1237 — readjudicação corrigida Oklab e LinearRgb

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada em `2026-08-27T22:30:51-03:00`: 59 ficheiros
alterados, 2247 inserções e 188 remoções.

**Veredito:** `NO-PROMOTION`; quatro pares permanecem `Unknown`.

## Medição antes da decisão

O universo contém exatamente 24 fixtures: seis para cada par Linear/Radial ×
Oklab/LinearRgb. Os SVGs candidatos, grafo e baseline vanilla são artefatos
congelados da campanha P1237/P1231, identificados no manifesto de entradas.
Esta execução não mede o binário produtivo atual. A recomputação usa a malha 4096,
preserva `stop-opacity` separado e calcula os limites exclusivamente como:

`color_max <= max(0.001, V_color_max) + 1e-6` e os demais observáveis
`<= V_* + 1e-6`.

A decisão consulta somente os quatro campos `V_*` publicados no manifesto. O
parser TSV carrega outras colunas em memória; não se apresenta uma constante
do runner como prova de capacidade ou isolamento.

## Resultado

Os quatro pares passam grafo em `6/6`. Numericamente, Oklab falha em `6/6`
para ambas as geometrias; LinearRgb passa `3/6` para ambas. Os witnesses
iniciais continuam, respectivamente, `base-square-fill` para Oklab e
`two-wide-stroke` para LinearRgb.

A simetria não promove uma geometria pela outra: cada par foi reduzido
separadamente. Linear/Oklab e Linear/LinearRgb são
`Unknown-native-approximation`; os dois pares Radial são `Unknown-fallback`.
Nenhuma whitelist, L0 ou implementação foi alterada.

Gate executado: duas execuções completas byte-idênticas. Os cinco ataques
anteriores eram sintéticos e foram revogados; não há mutation score nem selo
Tekt. O resultado cobre somente este corpus congelado e não declara
equivalência SVG geral nem estado do binário produtivo atual.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

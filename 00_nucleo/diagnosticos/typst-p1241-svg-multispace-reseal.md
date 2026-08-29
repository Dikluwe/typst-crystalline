# P1241 — consolidação SVG multi-space

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada, medição em `2026-08-27T22:58:19-03:00`:
59 ficheiros alterados, 2247 inserções e 188 remoções.

**Veredito:**
`ACCEPTED_CONSERVATIVE_MULTISPACE_CONSOLIDATION_WITH_EXTERNAL_V5`.

O consolidator `lab/parity/matrix/p1241_consolidate.py` vinculou por hash os
resultados saneados P1237–P1240 e o L0 SVG atual. Percursos normal e inverso
produziram artefatos byte-idênticos.

A adjudicação mantém dois pares `Preserved` — Linear/sRGB e Radial/sRGB — e
14 pares `Unknown`. Oklab e LinearRgb não satisfazem integralmente os budgets
numéricos P1237; Oklch, Luma, Hsl e Hsv têm evidência L1, mas não budget SVG
selado; CMYK continua `Unknown-ADR0097`.

P1240 é harness auxiliar retangular, sem selo ou mutation score, e não cobre
paths/wedges Conic. Portanto, Conic permanece `Unknown` nesta consolidação.
As 12 regras históricas são políticas conservadoras, não mutações executadas;
não existe mutation score P1241.

Os gates atuais são registrados sem transformar os dois V5 externos conhecidos
em sucesso. Nenhuma whitelist, mapa ou fila foi ampliada, e não há alegação de
equivalência SVG geral ou certificado global selado.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

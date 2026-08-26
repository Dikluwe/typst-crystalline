# P1221 — SVG solid shapes

Estado medido em `3f0a2638fffd8cddc0fde6e83058f69dedc7d838` com working tree não
commitado, às `2026-08-26T16:46:24-03:00`. Binário vanilla ratificado:
`7b4f40c5…`; binário cristalino reconstruído: `fe608fe2…`.

O comparador canônico passou a reconhecer o fragmento analítico focal de
retângulos e linhas, normalizando a diferença mecânica `<path>` versus
`<rect>/<line>`. Retângulo preenchido já era Preserved. A primeira divergência
real foi `stroke:` rico: `2pt + blue/red` era descartado pelos constructors
stdlib, produzindo preto 1pt ou nenhum contorno. O RED unitário foi 0/2; após
o L0 e a correção em `stdlib/shapes.rs`, foi 2/2. As três sondas públicas
(rect stroke, rect fill+stroke e line stroke) são Preserved.

A alegação fechada é somente `svg-solid-rect-line-stroke`: geometria focal,
fill sólido, ordem e stroke modelado. Elipses, círculos, rounded rects, paths
gerais, rotações e strokes com dash/cap/join não representados continuam
Unknown. Portanto `svg-morphology` permanece PARTIAL; a próxima fronteira é
o canonicalizador analítico de ellipse/rounded/path, não o exportador SVG.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

# P1222 — SVG analítico e extensão upstream

Execução iniciada em `2026-08-26T17:00:56-03:00`, HEAD
`0a0fabe05cd001802c6dfcfffb183f7651ab17d1`, working tree não commitado.
Baseline: vanilla ratificado `a51e02804`, binário SHA-256 `7b4f40c5…`.

O canonicalizador passou a suportar comandos SVG absolutos/relativos,
shorthands, transforms compostos, paths fechados cíclicos e identidade
analítica de elipse. Ellipse, circle, overlap e transform passaram de Unknown
para Preserved. A primeira divergência real foi rounded rect com stroke:
`radius: 4pt, stroke: 2pt` usava centro de raio 4pt no cristalino, crescendo o
raio externo para 5pt; o vanilla separa fill/stroke e usa centro de 3pt.

Após RED produtivo, o exportador passou a separar as pinturas, injetar metade
da espessura no raio central e preservar fechamento próprio de fill/stroke.
O corpus final ficou 6/6 Preserved em duas rodadas. A separação é também um
seam privado útil para futuras estratégias SVG, sem API nem mudança default
fora da correção de paridade.

As issues upstream #4702, #6035, #6858 e #8678 foram triadas. Nenhuma foi
implementada: duas exigem decisão pública/produto ADR-0127, uma pertence ao
pipeline import/PDF e uma requer desenho de recursos recursivos e sanitização.
Decisão: `NO EXTENSION SELECTED`.

O mapa DSM foi repartido em cardinalidades válidas e as nove correspondências
foram aplicadas sem diagnóstico. `svg-morphology` permanece PARTIAL para
corpus geral de arcos/paths, paint servers, clip/mask, imagens/links e stroke
complexo.

Os gates de Python, P1138-X-001, P1222 infra, build, fmt, diff e linter
passaram. A suíte workspace chegou a todas as suites funcionais, mas o teste
temporal preexistente `p1137_watch_dependencias_recuperacao_e_filtro` falhou
duas vezes no timeout de 20s quando executado dentro da suíte; a repetição
isolada passou em 2.01s. Isto fica registrado como instabilidade do gate, não
como evidência de regressão SVG nem como suíte integral verde.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

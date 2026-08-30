# P1237 — readjudicar Oklab e LinearRgb

**Estado:** EXECUTADO — SEM NOVAS PROMOÇÕES
**Predecessores:** P1234 e P1236 corrigidos

## Execução corrigida

Foram recomputadas as quatro combinações Linear/Radial × Oklab/LinearRgb,
com seis fixtures por combinação, sobre o corpus SVG congelado. A decisão
consulta somente `V_color_max`, `V_color_p95`, `V_alpha_max` e `V_alpha_p95`;
o parser carrega também as demais colunas do TSV, portanto não se alega “zero
leituras” de colunas `candidate_*`.

Todas as 24 fixtures preservaram grafo, variante, referência e papel. O gate
numérico permaneceu:

- Linear/Oklab: `0/6`;
- Radial/Oklab: `0/6`;
- Linear/LinearRgb: `3/6`;
- Radial/LinearRgb: `3/6`.

Logo, nenhuma promoção é válida no corpus congelado. Linear permanece aproximação nativa sem
alegação de paridade; Radial permanece fallback `Unknown`. O resultado foi
byte-idêntico em duas execuções. Os antigos 5/5 mutantes eram linhas
sintéticas produzidas pelo runner e foram revogados; não se alega mutation
score nem comportamento do binário produtivo atual.

Não houve mudança de whitelist, L0 ou código produtivo.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

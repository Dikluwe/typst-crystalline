# Critério dos controles extras

Antes de comparar o candidato, a medição inicial mostrou divergências de paginação
em region2em70 e de hanging em MLA. Não usar contagem de diferenças como proxy
para a qualidade: ela pode esconder um erro maior num número menor de tokens.

Para cada palavra correspondente, comparar atribuição de página e coordenadas
com o mesmo oracle. Não pode surgir erro de página maior, nem erro maior em
qualquer coordenada quando a atribuição de página não melhora. Se a página foi
corrigida, registrar qualquer resíduo geométrico separadamente. O desvio da
contagem de páginas também não pode aumentar. Os72 genéricos requerem igualdade
completa (0.001pt numérico); bibliografia aceita só título/primeiro corpo.

Esta regra foi escrita antes de coletar o candidato extra e não relaxa uma
falha observada. Diferenças anteriores continuam registradas, sem fechamento
universal de métricas, hanging ou sticky.

## Falha observada e distinção causal posterior

A primeira auditoria interrompeu em MLA/Testing. O corpo inteiro foi transladado
-5.4pt até sua origem correta, sem alteração de linha, palavra, estilo, página
ou coordenada relativa interna. Testing já estava uma linha antes do oracle por
hanging não implementado; por isso sua distância absoluta y piorou enquanto a
origem melhorou. Evidência em mla-residual-classification.json, fontes congeladas
e first-audit.py/log. A falha original foi preservada, não descartada.

Os dois controles MLA continuam falhas de paridade e não certificam hanging.
Para o reparo de origem, exigimos primeiro corpo coincidente com oracle e todos
os tokens até OUTSIDE com translação vertical única, preservando coordenadas
relativas, fronteiras e páginas. Não inferir melhora global de distância por
palavra a partir disso. Os14 extras restantes mantêm o critério mais estrito;
os72 genéricos continuam exigindo igualdade completa. Não houve mudança de
código produtivo para satisfazer esta classificação.

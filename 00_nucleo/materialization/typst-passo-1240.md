# P1240 — tornar o raster SVG local e confiável

**Estado:** FECHADO — HARNESS AUXILIAR RETANGULAR ACEITE  
**Predecessor:** P1239  
**Saída:** gate raster que não confunde layout externo com paint local.

## Objetivo

Materializar uma normalização reproduzível a partir do grafo SVG e geometria
declarada: bbox pintada, stroke, transform, máscara completa e mesmas coordenadas
locais. IDs, ordem de defs e translação externa não são observáveis; crop por
cor/erro, erosão, resize e alinhamento oportunista são proibidos.

## Execução

Provar primeiro que Linear/Radial sRGB e pattern controles produzem resultado
estável em 1x/2x/4x e ordem direta/inversa. Injetar mutantes de máscara e
transform para demonstrar discriminação. Manter o comparador em `lab/parity` ou
diagnósticos, nunca em L1/L2 produtivo.

## Gate

O raster só volta a ser gate de promoção quando os controles passam e todos os
mutantes de masking são rejeitados. Até lá ele é evidência auxiliar `Unknown`.

## Resultado

O script não foi executado. Sobreviveram mutantes de composição de transforms
ancestrais, cobertura geométrica de stroke/mask e access audit. Score 7/10.
Raster permanece auxiliar `Unknown`.

## Retomada e fechamento

Foi materializado um harness local em `lab/parity/matrix/p1240_raster.py`.
Transforms ancestrais são compostos e stroke/mask entram na geometria. O
recibo declara os SVGs e o renderer usados, sem alegar auditoria dos acessos
reais do processo. Linear, Radial e pattern passaram em 1x/2x/4x; percursos
1→2→4 e 4→2→1 produziram saída canônica byte-idêntica.

Os dez ataques históricos não possuem mutações executáveis e recibos duráveis;
foram revogados, sem mutation score ou selo Tekt. O harness é evidência
auxiliar do fragmento retangular registrado; path/wedges Conic e construções
não suportadas continuam `Unknown`. Nenhuma promoção SVG decorre deste passo.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.

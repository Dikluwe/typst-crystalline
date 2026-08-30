# P1242 — inventário pós-resselo das lacunas SVG

**Estado:** EXECUTADO — INVENTÁRIO DIAGNÓSTICO FECHADO (TILING PRIMEIRO)  
**Predecessor:** P1241  
**Saída:** fila fechada, ordenada e hash-pinned das lacunas SVG restantes.

Consumir o veredito real P1241, reler o mapa DSM e medir no vanilla ratificado
as superfícies ainda `Unknown`. Separar paint, tiling, clip, links, imagens,
glifos e diferenças externas. Registrar `file:line`, witness mínimo, owner L0,
classe ADR-0107 e se a correção exige gate ADR-0127.

Não alterar produção. Produzir matriz `impacto × independência × risco` e escolher
o primeiro item que possa fechar paridade sem contrato público. Se nenhum existir,
os passos seguintes executam apenas diagnóstico/L0 e encerram com stop explícito.

## Resultado

Ordem medida: (1) tiling modelável, owner SVG existente; (2) clip geométrico,
campo já chega ao exporter mas é ignorado; (3) imagens visíveis; (4) paint
multi-space, conservando os 14 pares `Unknown`; (5) links internos; e (6)
glifos matemáticos diretos. P1243 é o primeiro item executável sem mudança
pública presumida, após reparo do seu próprio pré-selo.

## Reexecução após o saneamento P1241

P1240 é harness raster auxiliar, sem mutation score ou selo Tekt. P1241
terminou como consolidação diagnóstica conservadora: dos 16 pares multi-space,
dois sRGB são `Preserved` e 14 continuam `Unknown`.

O inventário fica fechado com tiling modelável em primeiro lugar por impacto
alto, independência alta e risco médio, sem mudança pública presumida no
subconjunto color+size. P1243 v3 selou `34/34`, porém o universo adicional
SVG provado é vazio. Assim, P1244 deve fechar sem código, salvo nova evidência
bilateral; o selo negativo não autoriza produção.

A matriz foi atualizada com o estado real de P1243, P1246, P1247, P1248 e
P1249, preservando os gates ADR-0127 de links internos e glifos. O runner
reproduzível foi executado duas vezes com saída byte-idêntica. Nenhum código
produtivo, whitelist ou Prompt L0 foi alterado.

Receipt: `00_nucleo/diagnosticos/p1242-dependency-receipt.tsv`.

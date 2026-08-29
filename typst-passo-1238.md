# P1238 — corrigir espaços polares Oklch, Hsl e Hsv

**Estado:** FECHADO — POLAR L1 PRESERVED 6/6; SVG NÃO PROMOVIDO  
**Predecessor:** P1237  
**Saída:** costura e direção de hue provadas nos seis pares polares.

## Objetivo

Medir rota curta/longa, wrap 0°/360°, hue indefinido em baixa cromaticidade e
alpha para Linear/Radial × Oklch/Hsl/Hsv. Fixtures devem usar construtores
públicos aceitos nos dois lados; equivalentes RGB precisam demonstrar delta
vanilla zero antes de substituir uma fixture inválida.

## Gate

Separar erro da entidade L1, adaptive L3 e evaluator do SVG. Atualizar L0 antes
de código e parar sob ADR-0127 se a correção mudar contrato/default. Cada par é
promovido isoladamente; analogia entre Hsl, Hsv e Oklch é proibida.

## Resultado

O antigo stop de dependência foi retirado: esta campanha mede alpha diretamente
e não depende de um selo ou mutation score do P1236. A campanha pública mediu
36 fixtures e sete posições nos seis pares, totalizando 504
observações e 252 comparações. Linear/Radial × Oklch/Hsl/Hsv preservaram
`42/42` amostras por par, com zero `Unknown`. Os antigos 11 ataques eram
predicados autocontidos, não mutações executadas, e foram revogados.

O veredito cobre somente `gradient.sample(t).components(alpha:true)`: prova
costura, direção curta, baixa cromaticidade e alpha na fronteira L1. Não promove
adaptive L3, raster ou evaluator SVG. Ordem normal e completamente inversa
produziram artefatos byte-idênticos. Nenhum L0 ou código produtivo mudou.
Recibo diagnóstico em `00_nucleo/diagnosticos/p1238-tekt-certificado.tsv`.

# Testes independentes P1430 — layout de `math.cases`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/math/layout/p1430_tests.rs`.

## Obrigação

A suíte observa `FrameItem` atribuível com métricas determinísticas. Verifica
um ramo por child, split tardio de `MathAlignPoint`, estilo de denominador,
delimitador esquerdo/direito conforme `reverse`, omissão de lado `None`, gap
relativo resolvido contra o estilo exterior e precedência explícito > cascata
> default sem alteração do carrier.

Os controles P919/P923/P945/P1042/P1132 preservam alinhamento natural, centro
único no eixo, ausência de padding redundante, baseline relativa e extents do
delimitador. A aceitação usa relações geométricas e morfologia de frame, nunca
bytes serializados.

## Política de opacidade

Fonte, carrier, StyleChain, glyph, frame, métrica ou geometria não atribuível é
`Unknown`; falha fechado e não mata mutante.


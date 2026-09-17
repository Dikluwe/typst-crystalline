# Testes independentes P1431 — layout de `math.mat`

## Dono test-only

Consumer único: `01_core/src/compiler/math/layout/p1431_tests.rs`.

## Obrigação

Observar frames com métricas determinísticas. Verificar alinhamento horizontal,
split tardio de `MathAlignPoint`, gaps relativos por eixo, todos os offsets de
hline/vline inclusive bordas e negativos, stroke auto/custom, delimitadores,
estilo denominador e extents. Offsets fora da dimensão são diagnóstico, não
clamp/ignore. Preservar controles P825/P912/P919/P923/P945/P1042 e wrapper
legado. Frame, fonte, stroke ou geometria não atribuível é `Unknown` e bloqueia.

P26 inclui quantidades desiguais de `MathAlignPoint` e classes mistas. A
fixture assimétrica `(1 & 2), (33 & 4)` observa as duas equações do alinhamento
alternado: a subcoluna esquerda alinha à direita, portanto
`x(1) + width(1) == x(33) + width(33)`; a subcoluna direita alinha à esquerda,
portanto `x(2) == x(4)`. As larguras são derivadas independentemente por
`FixedMetrics::text_width` no tamanho observado do item, nunca inferidas das
posições produzidas pela grelha. Centralizar subcolunas deve falhar a primeira
equação. P27 compara a mesma matriz em gaps default, 1em e 150% nos dois eixos. P29 observa
espessura, cor e cap: auto é 0.05em Square e custom/fold conserva os três
componentes. P30 inclui delimitadores opcionais esquerdo e direito em grid
alto. P31 percorre Display, Text/Inline, Script e ScriptScript, mantendo a
descida de denominador. P34 observa Square automático, cap customizado e Butt
legado no `FrameItem::Line` final.

M23 exige offsets duplicados e fora de ordem, preservando quantidade e ordem
das coordenadas, e calcula o midpoint interior a partir das métricas independentes
das células; bordas 0/n coincidem com os limites completos da caixa. M25 usa
matrizes 2x3 e 3x2, válidas e inválidas, para hline validar rows e vline validar
columns. M26 chama diretamente `Content::math_matrix`, exige carrier `mat`
completo e compara sua geometria com a rota comum equivalente.

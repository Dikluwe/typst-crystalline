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

P26 inclui quantidades desiguais de `MathAlignPoint` e classes mistas. P27
compara a mesma matriz em gaps default, 1em e 150% nos dois eixos. P29 observa
espessura, cor e cap: auto é 0.05em Square e custom/fold conserva os três
componentes. P30 inclui delimitadores opcionais esquerdo e direito em grid
alto. P31 percorre Display, Text/Inline, Script e ScriptScript, mantendo a
descida de denominador. P34 observa Square automático, cap customizado e Butt
legado no `FrameItem::Line` final.

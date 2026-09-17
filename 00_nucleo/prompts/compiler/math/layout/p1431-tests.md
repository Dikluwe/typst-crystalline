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

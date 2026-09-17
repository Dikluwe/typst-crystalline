# Testes independentes P1429 — layout de `math.accent`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/math/layout/p1429_tests.rs`.

## Obrigação

A suíte observa a árvore de frame atribuível, nunca bytes serializados.
Acentos superiores ficam acima da base cramped e conservam as regressões de
attachment/baseline; acentos Unicode `Below` e os quatro símbolos inferiores
explícitos ficam sob base não cramped. Somente o ramo superior aplica dotless.

O tamanho efetivo resolve `Rel<Length>` contra a largura da base e usa
short-fall de `0.5em`. Métricas sintéticas com variantes identificáveis devem
discriminar 100% de 150%. `size` e `dotless` obedecem, independentemente,
explícito > cascata > default, sem fabricar presença no carrier.

## Política de opacidade

Fonte, variante, glyph, frame, cascata ou geometria não atribuível produzem
`Unknown`; `Unknown` falha fechado nos casos obrigatórios e não mata mutantes.

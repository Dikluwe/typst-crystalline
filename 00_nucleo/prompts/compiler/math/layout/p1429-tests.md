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

## Revisão R2

A reconstrução por `apply_math_default` e `apply_math_style` deve preservar
todos os campos causais. A morfologia dotless cobre `i` e `j` nas duas
polaridades. Para base de `1200du`, a métrica sintética põe a primeira
variante em `750du`: short-fall correto `0.5em` produz alvo `700du` e escolhe
essa variante; `0.4em` produz `800du` e escolhe a maior; `0.6em` produz
`600du` e conserva o glifo-base. Assim o teste discrimina diretamente o
limiar exato, não apenas a diferença genérica entre 100% e 150%.

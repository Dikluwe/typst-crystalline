# Testes independentes P1431 — carrier `MathMatrix`

## Dono test-only

Consumer único: `01_core/src/entities/elements/p1431_math_matrix_tests.rs`.

## Obrigação

Construir o carrier pela fronteira pública e discriminar rows/células, lados
opcionais do delimitador, alinhamento, offsets hline/vline e sua ordem, stroke,
gaps relativos e os cinco bits de presença. Clone, igualdade, hash, `repr`,
`plain_text`, `map_content`, `map_text` e reconstruções preservam todos os
campos. `MathAlignPoint` permanece dentro da célula até layout.

P21 compara pares que diferem isoladamente em ordem e conteúdo de células,
lado opcional do delimiter, alinhamento, eixo/ordem de offsets, componente de
stroke inclusive cap, eixo/componente de gap e cada bit de presença. Tanto
igualdade quanto hash devem discriminar cada par.

Zero ou múltiplos carriers, cascata não separável ou extração ilegível são
`Unknown`; nunca contam como sucesso nem como mutante morto.

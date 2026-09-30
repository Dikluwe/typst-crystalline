# Testes independentes P1430 — carrier `MathCases`
Hash do Código: 260f900f

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/entities/elements/p1430_math_cases_tests.rs`.

## Obrigação

A suíte constrói `MathCases` somente pela fronteira pública congelada e
discrimina children e sua ordem, os dois lados opcionais do delimitador,
`reverse`, `Rel<Length>` e cada bit de `MathCasesExplicit`. Ausência e
explicitude igual ao default não podem colapsar em igualdade, hash ou `repr`.

Clone, `plain_text`, `map_content`, `map_text` e reconstrução observável devem
preservar todos os campos. `map_content` transforma children em ordem;
`map_text` é terminal. Um `MathAlignPoint` permanece dentro do child até o
consumer de layout, nunca vira row/célula no carrier público.

## Política de opacidade

Binding ausente, zero ou múltiplos carriers atribuíveis, identidade divergente,
extração ambígua ou execução ilegível são `Unknown`; não concedem aceitação nem
crédito de mutação.

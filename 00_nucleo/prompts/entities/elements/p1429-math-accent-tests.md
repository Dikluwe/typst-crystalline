# Testes independentes P1429 — carrier `MathAccent`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/entities/elements/p1429_math_accent_tests.rs`.

## Obrigação

A suíte constrói carriers pela fronteira pública congelada e discrimina
separadamente base, acento normalizado, `size`, `dotless` e os dois bits de
presença. Igualdade, hash, clone, `plain_text`, `map_content`, `map_text` e
`repr` devem preservar a causalidade completa; somente a base é filho
recursivo de `map_content`, e transformação terminal não pode alterar o
carrier estrutural.

Valores iguais com presença diferente continuam diferentes. `repr` mostra
`size` e `dotless` apenas quando explicitamente fornecidos, na ordem pública,
e nunca imprime cascata efetiva ou orientação derivada.

## Política de opacidade

Zero ou múltiplos `MathAccent` atribuíveis, extração ambígua ou identidade
protegida divergente são `Unknown`, sem crédito de aceitação ou mutação.


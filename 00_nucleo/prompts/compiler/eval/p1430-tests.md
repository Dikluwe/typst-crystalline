# Testes independentes P1430 — avaliação e convergência de `math.cases`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/eval/p1430_tests.rs`.

## Obrigação

A suíte avalia fontes reais e exige convergência bare/qualificada, preservação
de `MathAlignPoint` no child, validação comum de inválidos e ausência de campos
cascaded no carrier/`repr`. Argumentos posicionais e named são avaliados uma
vez e em ordem de ocorrência; o primeiro erro encerra a construção e não pode
ser reordenado, ignorado ou substituído por fallback.

Named explícito prevalece sobre cascata sem esta fabricar explicitude. Spreads
e falhas são propagados pela fronteira comum; opacidade do contador/ordem é
`Unknown`, nunca sucesso presumido.

## Política de opacidade

Parser sem suporte, carrier ambíguo, diagnóstico sem atribuição, ordem não
observável, timeout ou falha de infraestrutura são `Unknown` e bloqueiam.


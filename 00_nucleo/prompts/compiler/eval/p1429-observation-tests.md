# Testes independentes P1429 — relação de observação de `MathAccent`

## Dono produtivo

Este L0 de teste possui exatamente um consumer:
`01_core/src/compiler/eval/p1429_observation_tests.rs`.

## Obrigação

A relação interna de observação deve devolver `Same` para identidade e
clone. Pares que diferem isoladamente em acento normalizado, tamanho,
dotless, presença de tamanho ou presença de dotless devolvem `Different`
simetricamente. O teste constrói os carriers pela fronteira pública, sem
reimplementar o predicado observado.

Ausência de binding, carrier ambíguo ou execução opaca é `Unknown` no
protocolo e não pode ser reinterpretada como `Same`.

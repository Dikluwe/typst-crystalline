# Testes públicos P1430 — `math.cases`
Hash do Código: 5b27ed65

## Owner

`01_core/src/compiler/stdlib/structural/p1430_tests.rs`.

## Contrato

Cobrir binding e identidade; zero, um e muitos children; casts; delimitadores;
`reverse`; gaps; named e duplicados; explicitude; `repr`; cascata; convergência
bare/qualificada; igualdade, hash, clone, `plain_text`, `map_content` e
`map_text`. Os testes observam API pública e carrier `Content::MathCases`, sem
harness de veredito, budget, ordem artificial ou crédito de mutação.

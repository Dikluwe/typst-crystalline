# Testes P1426 — reconstrução e inércia geométrica de presença

## Autoridade

Prompt test-only derivado do contrato congelado P1426. Legitima exclusivamente
`01_core/src/compiler/math/layout/p1426_tests.rs` e o seu wiring `#[cfg(test)]`.

## Obrigações

1. verificar que `apply_math_default` e `apply_math_style` preservam
   `index_explicit` nos três estados causais enquanto transformam os filhos
   como antes;
2. comparar carriers com índice/radicando idênticos e bit oposto, exigindo a
   mesma projeção de layout, texto, classe, baseline e vazio;
3. manter `sqrt` como controle `None/false`, com morfologia
   `root(radicand: ...)` e geometria de raiz quadrada estabelecida.

O bit é opaco ao algoritmo de layout. O teste não fixa bytes de render nem uma
estrutura Rust além do necessário para observar o contrato.


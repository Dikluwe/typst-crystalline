# Testes P1426 — sintaxe bare e relação observacional de raiz

## Autoridade

Prompt test-only derivado do contrato congelado P1426. Legitima exclusivamente
`01_core/src/compiler/eval/p1426_tests.rs` e o seu wiring `#[cfg(test)]`.

## Obrigações

1. avaliar sintaxe bare real e distinguir `sqrt(x)` (`None/false`),
   `root(, x)` (`Some(Empty)/true`), índice presente (`Some(_)/true`) e índice
   avaliado como `none` (`None/true`);
2. manter `root(x)` como erro de radicando ausente, nunca como alias de `sqrt`;
3. exercer `ObservationEq` em ambos os sentidos: carriers completos idênticos
   são `Same`, e carriers que diferem apenas por `index_explicit` são
   `Different`, nunca `Unproven`.

Asserções são de semântica/sintaxe/morfologia pública. Estrutura auxiliar de
teste é mecânica. Qualquer observação ambígua é `Unknown` fail-closed.


# Testes P1426 — integração pública bilateral de `math.root`

## Autoridade

Prompt test-only derivado do contrato congelado P1426. Legitima exclusivamente
`03_infra/tests/p1426_math_root.rs`; não legitima alterações no pipeline L3.

## Obrigações

1. compilar programas reais e provar convergência morfológica de
   `root(3, x)` com `math.root([3], [x])`;
2. preservar separadamente ausência de índice em `sqrt`, `none` explícito e
   índice vazio;
3. reproduzir os diagnósticos públicos medidos N01–N08;
4. manter `math.sqrt` e sintaxe `sqrt` como controles `None/false` sem
   `index: none` explícito, preservando a morfologia própria dos filhos de
   cada rota: conteúdo qualificado `[x]`, identificador math bare `x`.

Convergência bilateral significa o mesmo estado causal e a mesma semântica
de raiz quadrada; não autoriza igualar textualmente representações distintas
do radicando que já existiam antes de P1426.

Cada execução usa mundo temporário isolado por caminho. Erro de identidade,
timeout, crash, OOM, saída truncada ou extração ambígua é `Unknown` e bloqueia.

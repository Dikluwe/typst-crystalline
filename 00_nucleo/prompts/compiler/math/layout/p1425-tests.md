# Prompt L0 — testes A/B de preservação/layout `math.limits` P1425
Hash do Código: cccc33c1

**Camada:** L1 (testes)  
**Ficheiro alvo exclusivo:** `01_core/src/compiler/math/layout/p1425_tests.rs`  
**Owner produtivo observado:** `compiler/math/layout/_comum.md`

## Obrigação

Este consumer exclusivamente de teste congela a preservação P1425 sem
legitimar código produtivo. Deve provar:

- `apply_math_default` recursa no body e preserva exatamente `limits`,
  `inline` e `inline_explicit`, inclusive a morfologia posterior;
- alterar apenas `inline_explicit` nunca altera items nem geometria;
- omitido e `inline:true` mantêm o empilhamento vigente em inline e display;
- `inline:false` usa scripts laterais em inline e empilha em display;
- `scripts` permanece lateral mesmo em display.

Asserções com posições observam somente a relação empilhado/lateral já
estabelecida. Bytes de render, algoritmo, helper e representação Rust não são
aceitação.

## Limite

A cobertura limita-se ao fragmento P1425 e não depende de artifacts externos.

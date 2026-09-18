# Prompt L0 — testes independentes de `math.overbracket` P1440

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1440_tests.rs`

## Entradas congeladas

Passo P1440 aprovado no commit
`edf968e779c283d72d3343f6825a8834c2218118`, baseline
`fbd4f07dabc0e526d61b0fe14b1abe673fcf5032` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `overbracket` e a forma bare converge no
  mesmo `Content::MathOverbracket`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts
  inválidos falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem
  a mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `overbracket(body: ..., annotation?: ...)`, preserva nesting e
  nunca expõe `xover`, `MathOverbrace` ou `MathUnderover` como identidade;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23B4 acima do body e uma annotation superior
  externa, reutilizando o owner de `MathUnderover` sem alterar sua geometria.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23B4/posição, perdem
annotation, deixam de herdar classe ou mantêm a rota bare histórica. Todos
devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

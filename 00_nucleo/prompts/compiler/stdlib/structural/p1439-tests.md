# Prompt L0 — testes independentes de `math.overbrace` P1439

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1439_tests.rs`

## Entradas congeladas

Passo P1439 aprovado no commit
`1476196c1b327cf9fdc18f54685fe51cc1eb249e`, baseline
`243fa2784675bd1eb3710cc4fb03b2b6f4e24b92` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `overbrace` e a forma bare converge no mesmo
  `Content::MathOverbrace`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem a
  mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `overbrace(body: ..., annotation?: ...)`, preserva nesting e nunca
  expõe `xover`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23DE acima do body e uma annotation superior externa,
  reutilizando o owner de `MathUnderover` sem alterar sua geometria madura.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23DE/posição, perdem
annotation, deixam de herdar classe ou mantêm a rota bare histórica. Todos
devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

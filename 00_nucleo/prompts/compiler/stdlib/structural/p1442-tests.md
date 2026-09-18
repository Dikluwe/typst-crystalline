# Prompt L0 — testes independentes de `math.overparen` P1442

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1442_tests.rs`

## Entradas congeladas

Passo P1442 aprovado no commit
`84a990d65d002a6eb08c05ae2e2e2c5c23c3d489`, baseline
`2eb37dc53d32cb9e73f942cd63a02cfc7af2bd66` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `overparen` e a forma bare converge no
  mesmo `Content::MathOverparen`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts
  inválidos falham com os diagnostics medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem
  a mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `overparen(body: ..., annotation?: ...)`, preserva nesting e nunca
  expõe o carrier genérico como identidade;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23DC acima do body e uma annotation superior
  externa, reutilizando o owner de `MathUnderover`.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23DC/posição, perdem
annotation, deixam de herdar classe ou mantêm a rota bare histórica. Todos
devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

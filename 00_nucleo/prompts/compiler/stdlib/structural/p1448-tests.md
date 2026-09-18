# Prompt L0 — testes independentes de `math.underbrace` P1448

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1448_tests.rs`

## Entradas congeladas

Passo P1448 aprovado no commit
`1483f9f2ceb6b1574581d15ad725a50ab71a3bf3`, baseline
`661c9b021be4b543d4a5f5770709c94e8445b14e` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `underbrace` e a forma bare converge no mesmo
  `Content::MathUnderbrace`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem a
  mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `underbrace(body: ..., annotation?: ...)`, preserva nesting e nunca
  expõe `xunder`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23DF abaixo do body e uma annotation inferior externa,
  reutilizando o owner de `MathUnderover` sem alterar sua geometria madura.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23DF/posição, perdem
annotation, deixam de herdar classe ou mantêm a rota bare histórica. Todos
devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

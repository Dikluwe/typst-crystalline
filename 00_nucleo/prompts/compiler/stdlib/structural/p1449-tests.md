# Prompt L0 — testes independentes de `math.underbracket` P1449

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1449_tests.rs`

## Entradas congeladas

Passo P1449 aprovado no commit
`5c7fbb632`, baseline `3d9a0e63abc1e235d04128a5937d7313ff92819c`
e fonte vanilla pinada `586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `underbracket` e a forma bare converge no
  mesmo `Content::MathUnderbracket`;
- body é positional obrigatório convertível em conteúdo; annotation é o único
  segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`; valor e presença
  sobrevivem a igualdade, fields e reconstruções;
- `repr` é `underbracket(body: ..., annotation?: ...)`, preserva nesting e
  nunca expõe `xunder`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23B5 abaixo do body e uma annotation inferior
  externa, reutilizando o owner de `MathUnderover`.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23B5/posição, perdem
annotation ou classe e mantêm a rota bare histórica. Todos devem ser
rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

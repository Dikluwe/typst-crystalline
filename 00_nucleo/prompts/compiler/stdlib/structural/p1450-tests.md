# Prompt L0 — testes independentes de `math.underparen` P1450

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1450_tests.rs`

## Entradas congeladas

Passo P1450 aprovado no commit
`db93ffc77`, baseline `e3e0cced85ea675faa58a9a4c5facd57e5ac903a`
e fonte vanilla pinada `586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `underparen` e a forma bare converge no
  mesmo `Content::MathUnderparen`;
- body é positional obrigatório convertível em conteúdo; annotation é o único
  segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`; valor e presença
  sobrevivem a igualdade, fields e reconstruções;
- `repr` é `underparen(body: ..., annotation?: ...)`, preserva nesting e
  nunca expõe `xunder`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23DD abaixo do body e uma annotation inferior
  externa, reutilizando o owner de `MathUnderover`.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23DD/posição, perdem
annotation ou classe e mantêm a rota bare histórica. Todos devem ser
rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

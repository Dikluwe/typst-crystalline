# Prompt L0 — testes independentes de `math.overshell` P1443

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1443_tests.rs`

## Entradas congeladas

Passo P1443 aprovado no commit
`d616cace141bf64f02137ed002dadce019aff0fa`, baseline
`13f6a76a5a9392644920346f15043323f012805b` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

## Obrigações observáveis

- o módulo `math` contém a função `overshell` e a forma bare converge no
  mesmo `Content::MathOvershell`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts
  inválidos falham com os diagnostics medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem
  a mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `overshell(body: ..., annotation?: ...)`, preserva nesting e nunca
  expõe o carrier genérico como identidade;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23E0 acima do body e uma annotation superior
  externa, reutilizando o owner de `MathUnderover`.

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/presença,
aceitam assinatura inválida, alteram `repr`, trocam U+23E0/posição, perdem
annotation, deixam de herdar classe ou mantêm a rota bare histórica. Todos
devem ser rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

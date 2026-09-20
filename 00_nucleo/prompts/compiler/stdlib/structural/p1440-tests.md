# Prompt L0 — testes independentes de `math.overbracket` P1440
Hash do Código: 3eabd6e6

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1440_tests.rs`

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

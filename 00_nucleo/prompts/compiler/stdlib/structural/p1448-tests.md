# Prompt L0 — testes independentes de `math.underbrace` P1448
Hash do Código: 6c2ad4c0

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1448_tests.rs`

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

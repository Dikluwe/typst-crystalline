# Prompt L0 — testes independentes de `math.underbracket` P1449
Hash do Código: d314524e

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1449_tests.rs`

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

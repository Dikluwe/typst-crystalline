# Prompt L0 — testes independentes de `math.underparen` P1450
Hash do Código: c9f9fa02

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1450_tests.rs`

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

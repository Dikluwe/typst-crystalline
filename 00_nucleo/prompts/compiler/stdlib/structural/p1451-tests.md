# Prompt L0 — testes independentes de `math.undershell` P1451
Hash do Código: 0c2aa884

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1451_tests.rs`

## Obrigações observáveis

- o módulo `math` contém a função `undershell` e a forma bare converge no
  mesmo `Content::MathUndershell`;
- body é positional obrigatório convertível em conteúdo; annotation é o único
  segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`; valor e presença
  sobrevivem a igualdade, fields e reconstruções;
- `repr` é `undershell(body: ..., annotation?: ...)`, preserva nesting e
  nunca expõe `xunder`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23E1 abaixo do body e uma annotation inferior
  externa, reutilizando o owner de `MathUnderover`.

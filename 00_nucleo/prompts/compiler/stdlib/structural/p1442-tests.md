# Prompt L0 — testes independentes de `math.overparen` P1442
Hash do Código: 44114d4c

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1442_tests.rs`

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

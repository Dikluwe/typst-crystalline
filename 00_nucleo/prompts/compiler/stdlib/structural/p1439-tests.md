# Prompt L0 — testes independentes de `math.overbrace` P1439
Hash do Código: 97d057a7

**Camada**: L1 test-only  
**Alvo**: `01_core/src/compiler/stdlib/structural/p1439_tests.rs`

## Obrigações observáveis

- o módulo `math` contém a função `overbrace` e a forma bare converge no mesmo
  `Content::MathOverbrace`;
- body é um positional obrigatório convertível em conteúdo; annotation é o
  único segundo positional opcional; named, ausência, excesso e casts inválidos
  falham com os diagnósticos medidos;
- omissão de annotation diverge de `annotation: none`, embora ambas realizem a
  mesma geometria; valor e presença sobrevivem a igualdade e reconstruções;
- `repr` é `overbrace(body: ..., annotation?: ...)`, preserva nesting e nunca
  expõe `xover`/`MathUnderover` como identidade pública;
- o carrier desce estruturalmente em body/annotation, é não-locatable e herda
  classes esquerda/direita do body;
- a redução interna usa U+23DE acima do body e uma annotation superior externa,
  reutilizando o owner de `MathUnderover` sem alterar sua geometria madura.

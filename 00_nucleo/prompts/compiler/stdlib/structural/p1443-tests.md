# Prompt L0 — testes independentes de `math.overshell` P1443
Hash do Código: cfe276fb

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1443_tests.rs`

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

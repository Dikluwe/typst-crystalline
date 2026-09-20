# Prompt L0 — testes independentes de `math.overline` P1441
Hash do Código: 3604d0ff

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1441_tests.rs`

## Obrigações observáveis

- o módulo `math` contém a função `overline` e a forma bare converge no mesmo
  `Content::MathOverline`, distinto da decoração textual `Content::Overline`;
- body é exatamente um positional obrigatório convertível em conteúdo;
  named, ausência, excesso e casts inválidos falham com os diagnósticos
  medidos;
- `repr` é `overline(body: ...)`, preserva nesting e nunca projeta a forma
  textual `overline[...]`;
- o carrier desce estruturalmente no body, é não-locatable e herda classes
  esquerda/direita do body;
- o layout força cramped somente no body, usa constantes MATH `overbar_*`,
  põe a regra acima, conserva largura lógica e não a ajusta por correção
  itálica.

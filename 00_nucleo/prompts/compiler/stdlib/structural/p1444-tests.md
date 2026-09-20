# Prompt L0 — testes independentes de `math.primes` P1444
Hash do Código: 65f33e05

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1444_tests.rs`

## Obrigações observáveis

- o módulo `math` contém a função `primes`, com exatamente um inteiro
  posicional não-negativo; zero é válido e named, ausência, excesso e tipos
  incompatíveis falham fechados com as mensagens primárias medidas;
- `Content::MathPrimes` preserva count, igualdade, hash, `repr`, `plain_text`,
  `.count`, ausência de filhos, reconstruções e identidade nominal;
- sintaxe por apóstrofos produz o mesmo carrier; hat/underscore ficam em
  `t`/`b`, primes diretamente anexados ficam em `tr`, e nesting segue a AST;
- classe esquerda/direita é `Normal`, o carrier é não-locatable e terminal;
- counts 1–4 selecionam U+2032/U+2033/U+2034/U+2057; zero e counts acima de
  quatro usam U+2032 com largura `(count + 1) / 2` e offsets de meia largura.

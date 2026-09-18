# Prompt L0 — testes independentes de `math.primes` P1444

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1444_tests.rs`

## Entradas congeladas

Passo P1444 aprovado no commit
`d28d22d2e5db92339b63e8d805bbc9b75f53622d`, baseline
`e041b66d91efcdb7c42585aee87b39527eef4fa5` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

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

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade/count,
aceitam assinatura inválida, alteram `repr`, trocam glyph/receita, fundem ou
invertem slots, tornam o carrier locatable ou mudam a classe. Todos devem ser
rejeitados; `mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

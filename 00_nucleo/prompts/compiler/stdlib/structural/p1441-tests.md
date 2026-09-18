# Prompt L0 — testes independentes de `math.overline` P1441

**Camada**: L1 test-only
**Alvo**: `01_core/src/compiler/stdlib/structural/p1441_tests.rs`

## Entradas congeladas

Passo P1441 aprovado no commit
`1d2dbe984c5c47edcfcfed7f92bc1bc59ba1fa6e`, baseline
`3f4633b9267ccf89fa565601bbded58c259bf696` e fonte vanilla pinada
`586e1bd43fae6c9a973218163d3165c53ab8d16d`.

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

## Política e gate discriminatório

Falha de build alheia, timeout, identidade ambígua ou observação opaca é
`Unknown` e bloqueia selo. Mutantes válidos removem binding/identidade,
aceitam assinatura inválida, alteram `repr`, mantêm alias textual, omitem
cramped, usam `underbar_*`, põem a regra abaixo, ajustam largura por correção
itálica ou perdem classe herdada. Todos devem ser rejeitados;
`mutation_score = 1.0`.

## Regime

`executed_without_isolation_attestation`: os papéis compartilham workspace; a
ordem causal, os hashes e os recibos são preservados sem alegar isolamento.

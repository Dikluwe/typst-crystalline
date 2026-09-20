# Prompt L0 — integração de `format.svg`
Hash do Código: 37d50b05

**Camada:** L4
**Consumer exclusivo:** `04_wiring/tests/p1462_format_svg.rs`

## Contrato

Executar o binário cristalino e verificar comportamento público de
`format.svg.pretty`:

- default compacto;
- set rule `true`/`false` e regra vazia não destrutiva;
- precedência `caller explícito > documento > false`;
- rejeição de regra em container, named/tipo inválido e construção direta;
- XML válido e morfologia idêntica entre serialização compacta e pretty;
- neutralidade em PNG, PDF, HTML e `html.frame`;
- sucessão P1465: sem feature, `format.html` produz o gate público de `html`;
  com feature, sua superfície pertence exclusivamente ao teste P1465 e não é
  revalidada como parte de SVG. `format.pdf` não é sibling futuro.

O teste usa temporários isolados e timeout finito. Não valida hashes de SVG,
catálogos de mutantes, ordem de agentes ou artefatos administrativos.

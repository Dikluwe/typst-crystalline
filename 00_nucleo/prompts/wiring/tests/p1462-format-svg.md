# Prompt L0 — integração de `format.svg`
Hash do Código: 8333c9bf

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
- ausência de `format.pdf` e `format.html`.

O teste usa temporários isolados e timeout finito. Não valida hashes de SVG,
catálogos de mutantes, ordem de agentes ou artefatos administrativos.

# Archived pre-sanitation L0 — 00_nucleo/prompts/wiring/tests/p1462-format-svg.md

This is historical evidence, not a materializable prompt or an approval.
Original SHA-256: `8f35408f79d997be83bd0a39837f7f0c914f062679e0dba3ed63340cd5397d6b`.
The original text below is preserved without rewriting; dates, line references,
working-tree inventories, proposals and old parity claims retain their original scope.
Dead or unpinned references are archival only and cannot prove current behavior.

---

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

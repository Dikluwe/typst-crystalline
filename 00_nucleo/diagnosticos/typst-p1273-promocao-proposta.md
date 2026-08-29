# P1273 — proposta L0 de promoção SVG

**Estado:** GATE ADR-0127 CONFIRMADO; ROTA P1274
**Baseline commitado:** `9f0bed3633f3245cf27609751a3743968d5956b0`
**Atestação:** autoridade única `/root`; sem alegação de isolamento

## Resultado

O owner `00_nucleo/prompts/infra/export/svg.md` e o consumer
`03_infra/src/export/svg.rs` mantêm cardinalidade 1:1; V15/V26 terminou sem
violações. A medição produtiva encontrou o gate em
`03_infra/src/export/svg.rs:302`: Linear e Radial admitem somente sRGB. O writer
adaptativo para Oklab/LinearRgb já existe em `03_infra/src/export/svg.rs:542`,
mas fica inacessível porque o fallback é decidido antes, em
`03_infra/src/export/svg.rs:233`.

Foi acrescentada ao L0, em `00_nucleo/prompts/infra/export/svg.md:230`, uma
proposta que promove exclusivamente Linear/Oklab, Radial/Oklab,
Linear/LinearRgb e Radial/LinearRgb. Hsv, Oklch, Hsl, Luma, CMYK e qualquer par
não certificado permanecem `Unknown` com `gradient-color-space`. Conic e Tiling
não mudam.

A autoridade probatória é o certificado P1272 de 96 fixtures — 24 por par — e
o seu manifesto final. A proposta repete expressamente que esse fragmento não
prova equivalência SVG geral e não autoriza novos espaços, budgets ou conversão
automática de `Unknown`.

## Hashes da proposta pré-gate

- L0 anterior, SHA-256 integral:
  `41c5d785d9e4ecdb184fd5e6344dd7422d874e826e3bdeac2ddb2d6617530d29`;
- L0 proposto, SHA-256 integral:
  `209dbb2ac14be0f05e5e8bc9c651bbb6e7e1fdcd69533d6b2bd40a348be95478`;
- hash canônico proposto pelo linter: `f742c368`;
- hash ainda selado no consumer: `69409a7d`.

A V5 única (`f742c368` versus `69409a7d`) era deliberada e provou que o
consumer não foi ressellado antes da decisão. Não houve mudança em código ou
teste produtivo. O plano RED→GREEN está em `p1273-red-green-plan.tsv`.

## Confirmação do dono

O dono confirmou explicitamente a proposta em 2026-08-29. A redação aprovada
tem SHA-256 integral
`59d55a816fd2102db6286ac89febe55fed7b5c76ab5bf57323b6a12b420dec84` e
hash canônico `636c0c06`. O consumer continua no baseline com
`@prompt-hash 69409a7d`; portanto, a única V5 agora esperada é `636c0c06`
versus `69409a7d`. Esse estado fecha o gate documental sem antecipar o código.

P1273 para aqui. A implementação, os testes produtivos e o resselo pertencem a
P1274. A confirmação não amplia a promoção além dos quatro pares nomeados.

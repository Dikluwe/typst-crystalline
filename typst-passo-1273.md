# P1273 — redigir o L0 de promoção e parar no gate ADR-0127

**Estado:** EXECUTADO — GATE ADR-0127 CONFIRMADO; ROTA P1274
**Predecessor:** P1272 com pelo menos um par `Generalization-Preserved`
**Regime:** mudança de comportamento por defeito; paragem obrigatória

## Objetivo

Converter a evidência limitada do P1272 numa proposta explícita de promoção,
sem alterar código produtivo antes da decisão do dono.

## Tarefas

1. Medir novamente o owner `infra/export/svg` e confirmar a relação L0↔consumer.
2. Redigir no L0 SVG quais pares certificados deixam o fallback
   `gradient-color-space` e passam ao servidor SVG nativo.
3. Manter expressamente Hsv, Oklch, Hsl, Luma, CMYK e qualquer par não
   certificado como `Unknown` com fallback.
4. Vincular o certificado e manifesto P1272, o envelope exato e os limites da
   alegação; não declarar equivalência SVG geral.
5. Publicar o diff L0, hash proposto e plano RED→GREEN.

## Trava ADR-0127

**PARAR.** A remoção do fallback muda o caminho por defeito do produto. Código,
hash do consumer e testes produtivos só podem mudar após confirmação explícita
do dono para os pares nomeados.

**Gate fechado:** o dono confirmou explicitamente a proposta em 2026-08-29;
a materialização posterior limita-se aos quatro pares nomeados no L0.

## Resultado pré-gate de 2026-08-29

- baseline commitado: `9f0bed3633f3245cf27609751a3743968d5956b0`;
- relação owner/consumer confirmada por V15/V26 sem violações;
- proposta acrescentada a `00_nucleo/prompts/infra/export/svg.md:230`;
- SHA-256 integral proposto do L0:
  `209dbb2ac14be0f05e5e8bc9c651bbb6e7e1fdcd69533d6b2bd40a348be95478`;
- hash L0 canônico proposto pelo linter: `f742c368`;
- redação aprovada, SHA-256 integral:
  `59d55a816fd2102db6286ac89febe55fed7b5c76ab5bf57323b6a12b420dec84`;
- hash L0 canônico aprovado: `636c0c06`;
- consumer permanece com `@prompt-hash 69409a7d`, produzindo exatamente uma
  V5 esperada (`636c0c06` versus `69409a7d`) até P1274;
- código e testes produtivos não foram alterados.

Recibos: `00_nucleo/diagnosticos/p1273-l0-proposal.tsv`,
`00_nucleo/diagnosticos/p1273-red-green-plan.tsv` e
`00_nucleo/diagnosticos/typst-p1273-promocao-proposta.md`.

O código produtivo, os testes e o resselo pertencem a P1274 e não integram o
fecho documental deste passo.

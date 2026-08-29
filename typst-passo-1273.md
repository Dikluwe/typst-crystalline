# P1273 — redigir o L0 de promoção e parar no gate ADR-0127

**Estado:** PLANEADO E CONDICIONAL
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

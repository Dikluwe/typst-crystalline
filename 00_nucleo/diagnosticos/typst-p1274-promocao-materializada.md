# P1274 — recibo de implementação da promoção SVG

**Estado:** EXECUTADO
**Baseline commitado:** `b989c95a5ceaf7300904594b690f70f9d986f4e0`
**Regime:** materialização sobre decisão P1273 e contrato P1272 congelado
**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO

## Resultado de implementação

Linear/Oklab, Radial/Oklab, Linear/LinearRgb e Radial/LinearRgb deixaram o
fallback `gradient-color-space` e agora usam os servidores SVG Linear/Radial
com stops adaptativos. A única mudança produtiva além do resselo foi o
predicado proprietário; Hsv, Oklch, Hsl, Luma, CMYK e qualquer par não
certificado continuam fora da rota nativa.

O RED focal produziu as quatro falhas de referência nativa esperadas e manteve
o teste dos sete pares não certificados em PASS. Após a alteração do predicado,
o GREEN fechou 5/5. Três mutantes reversíveis — promoção de Hsv, retirada de
Oklab e bypass de `svg_adaptive_stops` — foram rejeitados 3/3.

O envelope congelado fechou 96/96 fixtures e 24/24 por par, 384/384 métricas,
1.224/1.224 intervalos, 28/28 entradas inválidas rejeitadas, 192/192 recibos
determinísticos e 24/24 ataques P1268. A fronteira produtiva observou 96/96
servidores nativos, zero fallback e manteve 10/10 casos opacos fora do sucesso.

## Linhagem e gates

- L0 final SHA-256:
  `6a8c1ad3858426f8a43ee8e172e5eb7bfca33dc1689d86e2eae0e1df7760c6d8`;
- consumer final SHA-256:
  `3ce7bd9ada1f32002e687e829e6f083f27decb446d6db8c364a25eaa7bf25e30`;
- hash L0 canônico no consumer: `636c0c06`;
- `Hash do Código` no L0: `dc4da342`;
- candidato testado:
  `522c0bde4dec1a8056a5aa5c2875af349b5e6fc249b2b8e054e0504294f776c8`;
- execução efêmera: `/tmp/p1274-envelope.q1OLuu`.

Workspace tests/build, formato, sintaxe dos runners, `diff --check` e
V1/V5/V15/V26 terminaram sem violações. O lint completo saiu com código zero e
manteve os avisos consultivos preexistentes do repositório.

## Integridade e limite

Os artefatos P1272 pinados conservaram os hashes aprovados: certificado
`d07786a8...` e manifesto `0c711546...`. Nenhuma saída histórica foi usada ou
reescrita; P1274 publicou somente artefatos `p1274-*`.

Contrato, implementação e verificação compartilharam `/root`, filesystem e
contexto. Portanto, não se alega segregação atestada nem se emite aqui um
veredito independente final. Este recibo prova apenas a execução da decisão
P1273 no envelope de 96 fixtures; não prova equivalência SVG geral.

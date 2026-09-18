# Relatório P1452 — recertificação integral das 66 rotas

## Veredito

**INCONCLUSIVE para o fecho integral; 65/66 rotas em MATCH.** Não houve
`Unknown`, instabilidade de ordem ou falha de recurso. A única diferença foi
`html.frame`, estável nas três ordens e na repetição integral independente.

Este veredito não altera produto, L0, fixtures nem expected. A campanha mede o
payload público P1420 byte a byte e preserva a política fail-closed.

## Proveniência

- baseline causal: commit `10798bc8907fd27c102c9ca522adfa54d0dded7b`,
  tree `14625f6aa023410c42a1ee6a4b93d32d53a352c7`;
- vanilla ratificado: SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino release: SHA-256
  `4c55affaa6486683165d91af507b03f99e0041f18ce5a4fc062cbc6e2b2d05de`;
- resultado primário: SHA-256
  `769cd0feba8293a7bc030c1b511e48b9338aef30b80bafd1c0b42ffd1040b6f4`;
- repetição integral: SHA-256
  `45b298daf9b3b29c7c0327182b999f3717eb3f5eafdb260042b103c86becdb38`;
- focal: SHA-256
  `2c089a1ca7c383272b9b17f95e18434bb25bcf66c9e709dca58d9d32badf86ca`.

## Resultado reproduzível

O focal `html.area`/`math.abs` passou com duas rotas `MATCH`. Cada campanha
integral classificou 66 rotas: 65 `MATCH` e uma `DIFFERENCE`. As 65 formas
qualificadas convergentes incluem todas as 22 rotas math. As alternativas
mediram 58 `MATCH` e 8 `DIFFERENCE`, mas não concedem nem retiram crédito quando
a forma qualificada já é `MATCH`.

Cada execução integral produziu 924 recibos de probes de rota e 72 recibos de
controles, totalizando 996 processos. Não houve timeout, truncamento nem swap;
o pico observado foi 39.596 KiB de RSS. Os vetores direct, direct-repeat e
reverse foram idênticos, e a repetição independente reproduziu o mesmo vetor.

O gate discriminatório preservou 8/8 positivos, rejeitou 27/27 negativos e
classificou 9/9 opacos como `Unknown`: score `1.0`, zero promoção implícita.

## Witness `html.frame`

Vanilla e cristalino terminam com exit 0, o mesmo warning público e SVG inline
com `width="5.39pt"`, `height="7.238pt"`, `viewBox="0 0 5.39 7.238"` e texto
visível `x`. A projeção P1420, porém, compara a árvore SVG inteira: o vanilla
converte o glyph para `use/defs/path`; o cristalino usa `<text>`. Também diferem
detalhes de style e namespace.

O L0 P1424 exige identidade da linguagem — binding, assinatura, SVG inline,
geometria, pintura, texto e nesting — e exclui igualdade de paths, IDs e bytes.
Assim, o witness não prova defeito de produto, mas prova que a projeção genérica
P1420 é estrita demais para certificar esta rota. Reclassificá-la depois da
observação violaria o preseal.

## Próximo passo permitido

P1453 deve medir e congelar uma projeção pública específica de `html.frame`
antes de executar o candidato. Ela precisa distinguir mudanças de raiz,
dimensão, viewBox, display, texto, pintura ou nesting, mas ignorar outlines e IDs
privados do renderer. Até esse novo contrato passar por ataques e repetição,
P1452 permanece `INCONCLUSIVE` e o denominador não é declarado integralmente
fechado.

## Limite da alegação

Execução sem atestação de isolamento. O relatório cobre somente as 66 rotas e
os observáveis congelados; não afirma equivalência geral de HTML, SVG, math ou
Typst.

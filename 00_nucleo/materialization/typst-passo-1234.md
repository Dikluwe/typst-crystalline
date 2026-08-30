# P1234 — auditar precisão, quantização e serialização SVG

**Estado:** EXECUTADO — DIAGNÓSTICO SANEADO; CAUSALIDADE DESCONHECIDA
**Predecessor:** P1233 corrigido
**Saída:** nenhuma divergência pública reproduzida; todos os owners permanecem `Unknown`.

## Objetivo

Medir separadamente B01–B07 sem transformar uma fronteira interna inacessível
em sucesso implícito e sem alterar produção.

## Execução corrigida

O probe antigo foi descartado porque fundia B02/B03, calculava distância RGBA
não premultiplicada e invocava `typst eval` incorretamente. O novo runner
`lab/parity/matrix/p1234_precision.py` usa a superfície pública
`Gradient.sample(...).components().map(repr)` para B01/B02 e os SVGs congelados
do P1231 para B05–B07. B03/B04 ficam `Unknown` quando não observáveis.

Foram executados 30 probes sobre 15 fixtures e 630 linhas de fronteira com o
binário cristalino atual pinado. As duas execuções foram byte-idênticas e não
reproduziram as 12 divergências Oklab históricas: todos os 30 probes coincidem
na superfície pública B01/B02. B03/B04 permanecem opacas; B05–B07 são apenas
observações do output já quantizado/serializado. Logo há zero causas L1, zero
causas L3 e 30 causalidades `Unknown`. Os antigos ataques sintéticos foram
rebaixados a checks de execução; nenhum mutation score é alegado.

## Gate

Diagnóstico somente. O resultado não muda L0 nem código produtivo e não
autoriza correção Oklab, Linear RGB ou HSV. Qualquer atribuição de owner exige
sonda legitimada das fronteiras B03/B04 e ligação quantitativa com o erro P1233.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

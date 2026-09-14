# P1373 — relatório da altura relativa de `block`

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** `block(height: 100%)` deixou de perder a
percentagem em eval. O valor atravessa a entidade como `Rel<Length>`, resolve
uma vez contra os 220pt úteis da região e a linha posterior avança antes de
ser emitida fora da página.

A matriz final obteve `MATCH` nos dois perfis (`a11y-extras` e `html+a11y`).
Vanila e cristalino produziram `ProbeAfter`; nos casos lone e controle,
`After` ficou na página de índice 1 em `yMin=7,404`.

## RED, descoberta e GREEN

Os três testes iniciais falharam antes da implementação. Depois do primeiro
GREEN local, a matriz bilateral fechou em `Unknown/TEXT_MISMATCH`: o PDF
cristalino continha `Probe` na primeira página e uma segunda página vazia.
A inspeção única mostrou que a linha `After` era drenada fora dos bounds antes
da transição. O L0 de cursor foi atualizado, um teste novo reproduziu o erro
(`ProbeAfter` retido na primeira página) e então a progressão pré-emissão foi
implementada. O filtro final P1373 passou 4/4; os controles absolutos passaram
2/2; o runner passou 1/1.

## Memória e limites

Cargo foi serial. Check usou 3 GiB; teste e release usaram 4 GiB. O primeiro
link a 3 GiB abortou por `memory allocation` sob RLIMIT, sem OOM do host. Uma
tentativa posterior com teto de arquivo de 256 MiB foi recusada pelo linker,
pois o binário de teste mede 308.865.296 bytes; a repetição incremental usou
512 MiB. O release final mede 49.076.160 bytes.

A matriz executou 8 jobs sequenciais com 2 GiB, 16 MiB por arquivo, 1 MiB por
canal e 45 s de timeout/CPU. O maior job levou 316,153 ms, o maior PDF teve
6.244 bytes e o maior canal 1.138 bytes. Não houve OOM, timeout, `SIGKILL` ou
execução concorrente. Uma checagem auxiliar foi inicialmente lançada com
teto virtual transcrito como 1 TiB; ela foi repetida corretamente com 1 GiB e
só a repetição limitada conta como evidência.

## Proveniência e gates

Medição em 2026-09-13, commit
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado com P1356–P1372. A listagem integral e `git diff HEAD --stat` foram
observados no mesmo turno; não se reivindica atribuição isolada do diff.

Vanila pinado: `eb60986b…1468f`; cristalino final: `0b318c3f…4e15`;
`pdftotext`: `0fb98ea1…565e7`. `cargo check`, formatação, JSON, diff e
`crystalline-lint .` retornaram zero. O linter terminou sem drift nem violação
bloqueante, embora reporte warnings/infos preexistentes.

## Limite do veredito

O protocolo completo foi aplicado sem atestação de isolamento. O selo cobre
somente S07 vertical nos dois perfis medidos. Width relativa e altura
fracionária permanecem no P1374; morfologia de tags PDF e paridade geral não
são alegadas.

# P1362 — preflight dos corpos de status do `watch`

## Proveniência

- instante: `2026-09-13T07:57:19-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado:
  - `/tmp/p1362-preflight-diff-stat.txt`, 1546 bytes, SHA-256
    `f2713146010dccaa6d7eda08f59224bb6d4b35808d74543b8a118d4938d0d1fd`;
  - `/tmp/p1362-preflight-status.txt`, 3884 bytes, SHA-256
    `c7368bfce2f16ce4f7bb3ed28cd1d1e0edf6100d18b867aef973b86bd1ee5b4c`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## Medição antes da decisão

Em `lab/typst-original/crates/typst-cli/src/watch.rs:82-163`, o estado do
observador possui quatro mensagens: `compiling ...`, `compiled successfully in
{duration}`, `compiled with warnings in {duration}` e `compiled with errors`.
O output é `StandardStream::stderr` em `terminal.rs:67-84`. A fonte mede com
`Instant` dentro de `compile_once` (`compile.rs:287-331`); warning só distingue
sucesso total de sucesso parcial. Erro não mostra duração.

No cristalino, `04_wiring/src/main.rs:105-151` executa o ciclo sem mensagem de
estado. `run_compile_observed` devolve apenas exit e dependências; embora já
possua `warnings`, não transporta o bit `has_warnings`. P1361 materializou em
L2 o formatter puro de duração, mas explicitamente deixou sua ligação ao ciclo
para passo próprio.

Classificação ADR-0107: os quatro corpos e sua seleção são morfologia pública;
timestamp, cor, fullscreen, limpeza de terminal e posição exata perante cada
diagnóstico são eixos não cobertos neste recorte. Inferência: transportar um
booleano de warnings e formatar corpos em L2 fecha este fragmento sem alterar
pipeline ou observação de ficheiros. Refutariam a hipótese a necessidade de
mudar L3, iniciar `watch` no teste ou inferir warnings a partir de texto.

## Segurança

Nenhum `watch`/`w` será iniciado. Testes usarão apenas durações e outcomes
literais no mesmo processo. Cargo: um job, um teste exato por comando, timeout
máximo 120 s e streams redirecionados. Não executar suíte de wiring, testes
externos de watch, corpus ou filtro amplo.

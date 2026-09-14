# P1376 — relatório de altura fracionária em colunas contínuas

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** `#columns(2)` e
`#set page(columns: 2)` iniciados na fronteira superior agora distribuem 1fr e
2fr como o vanila: uma página, `One=7,404`, `Two=82,724670` e
`After=220,166`.

O gate abre somente quando `column_mode` e `page_columns: Some` comprovam o
fluxo contínuo. A base passa a ser o fundo útil menos o cursor no início da
Sequence. O modo segmentado continua bloqueado. A fórmula é O(1), sem
alocação, recursão ou novo estado.

## Descoberta no caso deslocado

Com `Intro` antes do contentor, a correção leva `After` corretamente a
`220,166`, demonstrando que a base cursor-relativa está ativa. A composição
completa não é `MATCH`: `One` permanece em `21,792` contra `27,842` no vanila,
e `Two` em `97,142` contra `96,349990`. A causa observada é a diferença de
baseline/espaçamento de entrada do próprio contentor, anterior ao P1376. O
caso fica `Unknown`; não se calibrou a fórmula fracionária para esconder outro
bug.

## ADR-0124

As quatro superfícies foram medidas. `grid` e `box` ainda não distribuem a
fração; `place` tem origem/overlay divergente; colunas segmentadas preservam o
comportamento anterior e continuam comprimidas. Nenhuma delas recebeu crédito.
O próximo passo mais coeso é corrigir a entrada vertical de `columns` antes de
abrir o modo segmentado; grid e box permanecem candidatos independentes.

## RED, GREEN e memória

O RED terminou em 14,7 s com o helper ausente. GREEN: 1/1 P1376, 2/2 P1375 e
4/4 P1374. Check e links foram seriais sob 3 GiB e 4 GiB; símbolos de debug
foram desligados nos links. Matrizes rodaram uma por vez sob 2 GiB, arquivo
16 MiB e timeout 45 s. Binário de testes: 87.210.280 bytes; cristalino:
98.166.960 bytes; maior PDF cristalino final: 6.061 bytes. Não houve OOM,
timeout ou SIGKILL; pico RSS não foi medido.

Medição em 2026-09-13T16:27:29-03:00, HEAD `8ceea20f…5248`, working tree não
commitado e agregado: 45 ficheiros no diff, 3.269 inserções, 336 remoções e
256 entradas de status. Protocolo completo executado sem atestação de
isolamento; mutantes são declarativos.

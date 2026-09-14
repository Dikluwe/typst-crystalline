# P1365 — preflight da fronteira temporal do `watch`

## Proveniência

- medido em `2026-09-13T09:30:14-03:00`;
- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado: 121 linhas e 5.342 bytes em
  `/tmp/p1365-preflight-status.txt`, SHA-256
  `505a2ae1dac210a2eeb941c83ebe940128d02b692ce2c22e7ca881ddd528f194`;
- diff stat: 26 linhas e 1.682 bytes em
  `/tmp/p1365-preflight-diff-stat.txt`, SHA-256
  `e5c057dab0e6217cc69b1ccf57c576a006e4fe2b44db3acc9519a3a890219827`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- fonte vanila `compile.rs`: SHA-256
  `a943a87637dd1f85a95f39c29175225c378cbf6d7a0e8d30645eb5cca7470703`;
- candidato ausente em `04_wiring/src/main.rs`: SHA-256
  `b229863975b1cd1be08e90861e4df20bbbc761389d65e5ce6f681f256f35d3cd`.

## Medição anterior à decisão

No vanila, `typst-cli/src/compile.rs:292-297` captura `Instant::now()` antes de
chamar `Status::Compiling.print`, e só depois inicia compilação/export. Assim,
a duração humana inclui preparação, formação e emissão do status inicial.

No cristalino, `04_wiring/src/main.rs:202-204` forma e emite o bloco
`Compiling` antes de capturar `Instant::now()`. Essa diferença altera o valor
textual potencialmente apresentado em `compiled ... in DURATION`.

## Decisão

P1365 introduz um helper privado de ordenação que recebe duas closures: captura
primeiro o marcador temporal e somente depois forma o bloco inicial. Um wrapper
produtivo fornece `Instant::now` e `watch_block_now(Compiling)`. `run_watch`
recebe o par, emite o bloco e usa o marcador já capturado para `elapsed()`.

O teste substitui as duas closures por sentinelas determinísticas e prova a
ordem `capture` antes de `format`; não consulta relógio, não mede duração real e
não executa watcher. O contrato observa a fronteira sem exigir a mesma função,
tipo ou macro do vanila.

Cor, fullscreen, ordem do status final perante diagnósticos, flush e mecânica
de polling permanecem fora.


# P1367 — preflight do flush dos status de `watch`

## Proveniência

- medido em `2026-09-13T11:20:16-03:00`;
- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado: 141 linhas e 6.304 bytes em
  `/tmp/p1367-preflight-status.txt`, SHA-256
  `2b2fada8b12dc542dff20998403345d143430f8b2756281f406c50eb90abf524`;
- diff stat: 26 linhas e 1.678 bytes em
  `/tmp/p1367-preflight-diff-stat.txt`, SHA-256
  `475d5ded1374fe9f1b17644d777db3ef871c56e647e389d0492735ceea2fdde8`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- fonte vanila `watch.rs`: SHA-256
  `3d5ee88a01513902055881b8cd4245540327ef33a4d10a6788bcabce17cdb585`;
- fonte vanila `terminal.rs`: SHA-256
  `0fbde0a28d5f7842d2c0b692b6710d422784f8df1b61ee7c77d74bf40d2f04a5`;
- baseline `04_wiring/src/main.rs`: SHA-256
  `36eac05d1eb208b21dbcb7e936cab328c4e1f262e15123d6f73f4daa74462e3d`.

## Medição anterior à decisão

O vanila `watch.rs:97-109` forma o status e chama `out.flush()` antes de
retornar. `terminal.rs:51-58` encaminha esse flush ao stream de stderr. Isso
torna `Compiling` observável antes que uma compilação longa prossiga.

O cristalino P1366 usa `eprint!` em `main.rs:257,261`, sem uma fronteira
explícita que o contrato consiga observar. A string contém LF, mas depender de
buffering implícito não prova que o status foi descarregado antes do próximo
estágio.

## Decisão

P1367 introduz em L4 `write_watch_block`, que recebe `std::io::Write`, executa
`write_all(block.as_bytes())` e, apenas se a escrita tiver sucesso, `flush()`.
O wrapper produtivo bloqueia stderr, chama o helper e trata falha com a mesma
semântica de pânico já documentada pelas macros `eprint!`.

Os dois call sites de status P1366 usam o wrapper. Um writer falso em memória
prova bytes, ordem e cardinalidade sem abrir processo, terminal ou watcher.
Cor, fullscreen, detecção TTY e conteúdo dos diagnósticos ficam fora.


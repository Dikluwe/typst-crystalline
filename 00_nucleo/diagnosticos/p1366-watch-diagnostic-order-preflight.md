# P1366 — preflight da ordem entre status e diagnósticos

## Proveniência

- medido em `2026-09-13T10:51:52-03:00`;
- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado: 131 linhas e 5.822 bytes em
  `/tmp/p1366-preflight-status.txt`, SHA-256
  `e961d9ba2f78e0eb2e063ea1d16abb6689619ff4938399843f560cf8c6fea7b5`;
- diff stat: 26 linhas e 1.685 bytes em
  `/tmp/p1366-preflight-diff-stat.txt`, SHA-256
  `319be077e5d798f026317155884d6fc1f45da7e8823fd36818b5e37787c658b1`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- `lab/typst-original/crates/typst-cli/src/compile.rs`: SHA-256
  `a943a87637dd1f85a95f39c29175225c378cbf6d7a0e8d30645eb5cca7470703`;
- baseline `04_wiring/src/main.rs`: SHA-256
  `fa78bb9f1b91c7b16bb5d564b5c7038b8c8780f0e5b9063b2a6558c5a14206f5`.

## Medição anterior à decisão

No vanila, `compile.rs:307-334` emite `Success`/`PartialSuccess`/`Error` antes
de `print_diagnostics`, tanto no resultado positivo quanto no negativo.

No cristalino, `run_compile_observed` drena warnings em `main.rs:612` e errors
em `main.rs:630`, retorna, e somente então `run_watch` forma e emite o status
terminal em `main.rs:234-236`. O bloco vazio P1364 acaba depois dos
diagnósticos, invertendo a morfologia observável do stderr.

## Decisão

P1366 injeta em `run_compile_observed` uma callback privada de status e usa um
helper causal `report_in_order(status, diagnostics)`: o status é chamado uma
vez antes do bloco que apresenta warnings/errors. `run_watch` fornece a
callback que deriva `WatchStatus` e emite `watch_block_now`; `run_compile`
fornece callback vazia, preservando ausência de status fora de `watch`.

O resultado e a escrita do artefato são resolvidos antes do status terminal.
Falhas preliminares e de escrita chamam status de erro antes da mensagem de
erro. Warnings continuam antes de errors entre si. O teste usa somente duas
closures em memória e não compila documento nem executa watcher.

Cor, fullscreen, flush explícito e mecânica de polling continuam fora.


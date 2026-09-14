# P1364 — preflight do espaçamento do status `watch`

## Proveniência

- medido em `2026-09-13T08:56:31-03:00`;
- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado: 111 linhas e 4.869 bytes em
  `/tmp/p1364-preflight-status.txt`, SHA-256
  `5d77596ef3e3373db9a71668f23bb115163e9148f47c21e040ed1f379252e931`;
- diff stat do workspace: 26 linhas e 1.677 bytes em
  `/tmp/p1364-preflight-diff-stat.txt`, SHA-256
  `db9959234d9c576fed09e435c848702c67182c3a05342be9c1131be2d1e8b973`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- `lab/typst-original/crates/typst-cli/src/watch.rs`: SHA-256
  `3d5ee88a01513902055881b8cd4245540327ef33a4d10a6788bcabce17cdb585`;
- `04_wiring/src/main.rs`: SHA-256
  `beaafca23a97e86774e8f6d81d2c756575ca0e210a8ea79cb41a2650487d0290`.

## Medição anterior à decisão

No vanila, `watch.rs:97-109` chama `print_message`, que já escreve uma quebra
de linha. Quando o estado não é `Compiling`, `watch.rs:104-106` escreve uma
segunda quebra. Portanto, no modo não-fullscreen, os blocos observáveis são:

```text
Compiling                  -> "[HH:MM:SS] compiling ...\n"
Success/Partial/Error      -> "[HH:MM:SS] BODY\n\n"
```

No cristalino, `04_wiring/src/main.rs:174,178` usa `eprintln!` nos dois pontos,
logo todas as variantes recebem exatamente uma quebra. A diferença é
morfologia terminal da linguagem CLI; o uso de `writeln!`, `eprintln!` ou
buffer intermediário é mecânica livre.

## Decisão

P1364 acrescenta em L4 um helper privado e puro que forma o bloco a partir de
`watch_line_at`: uma quebra para `Compiling`, duas para as outras variantes.
Os dois pontos produtivos passam a emitir o bloco pronto com `eprint!`. O teste
usa `03:04:05` literal e não chama relógio, processo ou watcher.

A medição também mostra que o vanila colore o bloco, pode usar fullscreen e
emite o status final antes dos diagnósticos. Esses eixos permanecem fora deste
fragmento; combiná-los impediria atribuir a correção ao espaçamento.


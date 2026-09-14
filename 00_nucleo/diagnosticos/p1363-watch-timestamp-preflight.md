# P1363 — preflight do timestamp de status do `watch`

## Proveniência

- instante: `2026-09-13T08:28:43-03:00`;
- HEAD: `8ceea20f92dff06b6ca5a34828135c0fd3e35248`;
- working tree não commitado:
  - `/tmp/p1363-preflight-diff-stat.txt`, 1553 bytes, SHA-256
    `ad8d2edd8349c08be97e12e736024d8fb402cef14cc3ebcf24df372c8616f7b1`;
  - `/tmp/p1363-preflight-status.txt`, 4356 bytes, SHA-256
    `63c9649bab031ad0869396a43cc40fb360bf220a1a0e5f1d46bb21aa7a5fe5a4`;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## Medição antes da decisão

`lab/typst-original/crates/typst-cli/src/watch.rs:149-152` obtém hora local e
escreve `[{timestamp}] {message}` com `%H:%M:%S`. O canal é stderr conforme
`terminal.rs:67-84`. A string usa dois dígitos para hora, minuto e segundo; a
data, offset e frações não aparecem.

P1362 já emite os quatro corpos em stderr, mas sem prefixo. Em
`04_wiring/src/main.rs`, os dois `eprintln!` de `run_watch` recebem diretamente
`WatchStatus::message()`. O workspace já possui a crate `time` 0.3 com
`local-offset` em `[workspace.dependencies]`, usada por L3; L4 ainda não a
declara. L2 não deve ler relógio nem depender de timezone.

Classificação ADR-0107: `[HH:MM:SS]` e padding são morfologia pública. O instante
concreto varia e não é comparável bilateralmente; testes devem injetar
componentes literais. Inferência: um formatter puro L2 e uma leitura local L4
fecham o envelope sem executar o observador. Refutariam a hipótese necessidade
de alterar L3/polling ou um teste dependente do tempo real.

## Segurança

Nenhum processo `watch`/`w`. Apenas testes exatos in-process com `03:04:05`,
um job, timeout máximo 120 s e streams redirecionados. O valor do relógio real
não participa de expected. Nenhuma suíte externa de wiring.

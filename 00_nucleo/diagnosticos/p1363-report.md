# P1363 — relatório final do timestamp de `watch`

## Veredito

`PASS_WITH_LIMITED_ATTESTATION` — o fragmento congelado do P1363 foi
materializado e os recibos sustentam os dois GREEN exatos, build e gates
estáticos. A formulação correta continua sendo **executado sem atestação de
isolamento**, porque todas as autoridades trabalharam no mesmo workspace.

O resultado prova somente o envelope textual `[HH:MM:SS] BODY`; não prova
paridade geral de `watch`, terminal ou diagnósticos com o vanila ratificado.

## Estado verificado

- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado;
- vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- manifesto `b8060f28…`, contrato `00afeec9…`, oráculos `f48e4d38…`,
  mutantes `27916f91…`, gate `ebc82637…` e selo `d8c276c3…` preservados;
- digests normativos dos três L0 continuam exatamente os selados;
- removida a única linha autorizada `time = { workspace = true }`, o
  `04_wiring/Cargo.toml` volta byte a byte ao hash baseline `dbc8a208…`;
- o `Cargo.lock` acrescenta somente `time` às dependências de `typst-wiring`.

No instante da verificação (`2026-09-13T08:51:07-03:00`), o snapshot de
`git status --short` tinha 109 linhas e 4.777 bytes, SHA-256 `f8a8a723…`; o
snapshot de `git diff HEAD --stat` tinha 26 linhas e 1.677 bytes, SHA-256
`db995923…`. Esses números descrevem o workspace partilhado inteiro, incluindo
mudanças anteriores e alheias ao fragmento P1363.

## Comportamento observado

L2 implementa o formato exato `[{hour:02}:{minute:02}:{second:02}] {body}`.
O teste público cobre as quatro variantes com `03:04:05`, preserva os corpos,
confere um único espaço e rejeita CR/LF. O teste L4 usa apenas
`Time::from_hms(3, 4, 5)`; nenhum expected depende do relógio real.

`watch_line_at` usa exclusivamente o `Time` fornecido. `watch_line_now` tenta
hora local e escolhe UTC somente no fallback. As únicas duas chamadas
produtivas de `watch_line_now` estão em `run_watch`, ambas sob `eprintln!`; o
caminho normal de `compile` permanece sem esses status.

## Gates e contenção de memória

- teste exato L2: `1 passed`, `0 failed`, `2 filtered out`;
- teste exato L4: `1 passed`, `0 failed`, `4 filtered out`;
- build limitado de `typst-wiring`: `Finished dev profile`;
- `cargo fmt --check` e `git diff --check`: streams vazios;
- `crystalline-lint .`: concluído sem erro, com warnings/infos existentes e
  zero violations bloqueantes.

Não houve execução externa de `watch`/`w`, suíte externa de wiring ou suíte
ampla. Os dois testes foram exatos, sequenciais, in-process, com
`CARGO_BUILD_JOBS=1` e timeout de 120 s por comando. Os 22 recibos P1363 somam
437.158 bytes; o maior tem 277.268 bytes. Ambos ficam muito abaixo do teto
total de 16.777.216 bytes e do teto de 4 MiB por stream. Não há evidência de
timeout ou OOM.

## Scope-outs e limites da atestação

Continuam fora: cor/fullscreen/cursor; data, offset, frações e locale; ordem
perante diagnósticos, buffering e flush; toda a mecânica de polling, staging,
publish e espera; instante real capturado; entradas de tempo inválidas; e
execução externa de `watch`.

O verificador não reexecutou cargo, testes, binário, `watch`, linter ou gates:
leu somente código, artefatos selados e recibos. O workspace compartilhado não
permite provar capacidades por isolamento do sistema operacional e o
implementador pôde ver os testes depois de escrever sua implementação. Isso
limita a força da atestação, mas não altera a ordem causal registrada — testes
foram congelados antes da implementação — nem o resultado do fragmento
observável certificado.

O certificado reproduzível completo está em
`00_nucleo/diagnosticos/p1363-verification.json`.

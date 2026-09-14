# P1365 — relatório final da fronteira inicial da duração

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** A implementação satisfaz o fragmento
congelado: o ciclo captura um único marcador monotônico antes de formar e
emitir `Compiling`, e o status terminal usa `elapsed()` desse mesmo marcador.
O resultado não atesta isolamento técnico nem paridade geral do `watch` com o
vanila.

Regime: protocolo completo, **executado sem atestação de isolamento**. O
verificador fez apenas inspeção de artefatos e recibos; não executou Cargo,
teste, build, binário, `watch`, `w`, lint, gate ou rede, e não alterou nenhum
artefato verificado.

## Cadeia selada

As entradas raw continuam idênticas ao selo:

- manifesto `0c140646…16cbd`;
- contrato `3315d7e…a3db`;
- oráculos `3f027695…7118`;
- mutantes `23881c92…0d31`;
- passo `934ca131…9fea`;
- preflight `1a2ff4f0…d5db`;
- gate `f5be011a…04a8`;
- fonte vanila ratificada `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  arquivo `compile.rs` `a943a876…0703`.

O raw de `wiring.md` mudou pelo resselo mecânico, mas o digest normativo,
calculado removendo somente `Hash do Código:` e sua quebra, permaneceu
`fbec2664…426e`. Logo, a obrigação L0 congelada não derivou.

O gate selado retornou `SEALABLE`, exit `0`: 15/15 positivos preservados,
21/21 negativos violados, oito opacos mantidos como `Unknown` com crédito
zero, nenhum `Unknown` inesperado e 30/30 mutantes rejeitados. Mutation score:
`1.0`; repetição e ordem reversa foram determinísticas.

## Materialização observada

Em `04_wiring/src/main.rs`:

- linhas 288–295: `prepare_watch_cycle_with` chama `capture` uma vez, depois
  `format_compiling` uma vez, e devolve os dois resultados sem transformação;
- linhas 297–299: o wrapper privado fornece `Instant::now` e
  `watch_block_now(&WatchStatus::Compiling)`;
- linhas 232–236: `run_watch` chama o wrapper uma vez, emite o bloco retornado,
  chama `run_compile_observed` sem recaptura e usa `started.elapsed()` no mesmo
  marcador para o status terminal;
- linhas 170–197: o teste exato usa somente closures, log e contadores em
  memória. Ele prova a ordem `capture → format`, cardinalidade unitária e
  preservação dos valores, sem relógio real, espera, I/O ou processo.

Os helpers são privados, nenhuma API pública ou novo tipo produtivo foi criado,
e a inspeção não encontrou segundo `Instant::now` entre preparação e
compilação.

## RED, GREEN e gates finais

O recibo RED contém exit `101` e `E0425`: o teste não encontrava
`prepare_watch_cycle_with`. O mesmo comando exato, após a implementação,
retornou exit `0`, executou um teste e obteve `1/1` aprovado, zero falhas.

Também constam nos recibos completos:

- build único de `typst-wiring --bin typst`: exit `0`;
- `cargo fmt --all -- --check`: exit `0`;
- `git diff --check`: exit `0`;
- `crystalline-lint .`: exit `0`, sem violação bloqueante; warnings e infos
  preexistentes permanecem visíveis no recibo.

## Memória e proveniência

Os comandos dinâmicos registrados foram sequenciais, com
`CARGO_BUILD_JOBS=1` e timeout máximo de 120 segundos. Nenhum comando
registrado iniciou `watch`, `w`, suíte externa ou ampla. Não há sinal de
timeout, OOM ou SIGKILL.

- recibos de preflight: 7.024 bytes;
- recibos dinâmicos: 390.796 bytes;
- resselo/formatação auxiliares: 154 bytes;
- total: **397.974 bytes** de 16 MiB permitidos;
- maior stream: **277.268 bytes** de 4 MiB permitidos.

A medição final usa HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado. Em
`2026-09-13T09:50:36-03:00`, o status agregado tinha 129 entradas/5.730 bytes,
SHA-256 `985084e6…ee52`; o diff stat contra HEAD tinha 26 linhas/1.685 bytes,
SHA-256 `319be077…58b1`. Esse estado inclui P1356–P1365 e outras mudanças: serve
para reprodução do contexto, não para atribuição exclusiva do delta.

## Limitações e scope-out

O workspace partilhado impede atestação técnica de isolamento, e o
implementador relatou que viu o teste já presente ao inspecionar o arquivo;
portanto a independência A/B é limitada. O teste não executa um processo
`watch` real.

Continuam fora: valor absoluto/precisão/overhead do relógio; tempo posterior ao
status terminal; cor, fullscreen, cursor, limpeza, buffering e flush; ordem do
status terminal perante diagnósticos; polling, dependências, staging,
publicação, abandono, espera e eviction; execução externa de `watch`/`w`; e
paridade funcional geral de terminal ou duração com o vanila.

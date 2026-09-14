# P1366 — relatório final da ordem status-diagnósticos

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** A implementação satisfaz o fragmento
congelado: cada saída de `run_compile_observed` anuncia o status terminal antes
dos diagnósticos ou da mensagem operacional da tentativa. O caminho `watch`
usa o marcador P1365 dentro da callback e não emite um segundo status depois do
retorno; `compile` fornece uma callback vazia.

Regime: protocolo completo, **executado sem atestação de isolamento**. O
verificador fez somente inspeção dos artefatos e recibos existentes. Não
executou Cargo, teste, build, binário, `watch`, `w`, lint, gate ou rede, e não
alterou nenhum artefato verificado.

## Cadeia selada

As entradas raw continuam idênticas ao selo:

- manifesto `137c8a78…f8729`;
- contrato `d38752f2…95f4d`;
- oráculos `bb6fc97a…0cd6`;
- mutantes `5333e9d2…be4c`;
- passo `692cfef7…06ff`;
- preflight `31cfa756…f40f`;
- gate `d568fbaf…c29`;
- fonte vanila ratificada no commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`, arquivo `compile.rs`
  `a943a876…0703`.

O raw atual de `wiring.md` é `0ea1526b…e0d`, diferente do raw selado por
causa do resselo mecânico. Removendo exclusivamente a linha iniciada por
`Hash do Código:` e sua quebra, o digest normativo continua
`878b0fdd…dc6`; portanto a obrigação L0 congelada foi preservada.

O gate selado retornou `SEALABLE`, exit `0`: 16/16 positivos preservados,
22/22 negativos violados, oito opacos mantidos como `Unknown` com crédito zero,
nenhum `Unknown` inesperado e **32/32 mutantes rejeitados**. Mutation score:
`1.0`; repetição e ordem reversa foram determinísticas.

## Materialização observada

Em `04_wiring/src/main.rs`:

- linhas 343–346: `report_in_order` é privado e chama `status` uma vez antes
  de `diagnostics` uma vez;
- linhas 460–531 e 705–719: a callback `FnMut(ExitCode, bool)` ocorre nas três
  falhas preliminares e na saída comum dos resultados compilados. Assim, todas
  as saídas da função são cobertas sem duplicação;
- linhas 666–719: resultado, artefato e eventual arquivo de timings são
  resolvidos antes de um status de sucesso. Depois do status, a ordem é warning
  experimental HTML quando aplicável, warnings, errors e mensagem operacional;
- linhas 253–264: `watch` deriva o status dos argumentos tipados e de
  `started.elapsed()`, emite o bloco P1364 na callback e não tem emissão
  terminal pós-retorno;
- linhas 455–458: `compile` passa callback vazia e apenas conserva o
  `ExitCode` retornado;
- linhas 199–221: o teste exato usa duas closures, log e contadores em memória.
  Ele prova ordem `status → diagnostics` e cardinalidade 1/1, sem relógio,
  espera, filesystem ou processo.

O hash raw atual do candidato é `36eac05d…62e3`. O helper e a callback ficam
privados e não foi criada API pública.

## RED, GREEN e gates finais

O recibo RED registra exit `101` e `E0425`: o teste ainda não encontrava
`report_in_order`. Há uma limitação importante: esse stderr preservado foi
reconstruído/truncado após a captura original. Ele conserva a causa causal e o
exit, mas não é o stream RED original integral.

O mesmo comando exato, após a implementação, retornou exit `0`, executou um
teste e obteve **1/1 aprovado**, zero falhas e sete testes filtrados.

Os recibos finais registram:

- build único de `typst-wiring --bin typst`: exit `0`;
- `cargo fmt --all -- --check`: exit `0`;
- `git diff --check`: exit `0`;
- `crystalline-lint .`: exit `0`, sem violação bloqueante; warnings e infos
  preexistentes permanecem no recibo.

## Memória e proveniência

Os comandos dinâmicos registrados foram sequenciais, com
`CARGO_BUILD_JOBS=1` e timeout máximo de 120 segundos. Nenhum comando iniciou
`watch`, `w`, suíte externa ou ampla. Não há sinal de timeout, OOM ou SIGKILL.

- recibos de preflight: 7.507 bytes;
- gate selado: 6.963 bytes;
- recibos dinâmicos e gates finais: 348.689 bytes;
- resselo/formatação auxiliares: 154 bytes;
- total: **363.313 bytes** de 16 MiB permitidos;
- maior stream: **277.268 bytes** de 4 MiB permitidos.

A verificação usa HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado. Em
`2026-09-13T11:16:20-03:00`, o status agregado tinha 139 entradas/6.212 bytes,
SHA-256 `92dc0ae2…eff6`; o diff stat contra HEAD tinha 26 linhas/1.678 bytes,
SHA-256 `475d5ded…de8`. Esse estado agrega P1356–P1366 e outras mudanças, servindo
para reprodução do contexto, não para atribuição exclusiva.

## Limitações e scope-out

O workspace partilhado impede atestação técnica de isolamento, e o teste ficou
visível no mesmo arquivo. O teste dinâmico cobre somente o helper: a cobertura
de todas as saídas da callback, o roteamento `watch`/`compile` e a ordem completa
foram confirmados por inspeção estática, não por transcript de todos os ramos.
Não foi executado um processo `watch` real.

Continuam fora: conteúdo e formatação interna dos diagnósticos; cor,
fullscreen, cursor, limpeza, buffering e flush; medições opt-in posteriores;
polling, dependências, staging, publicação, abandono, espera e eviction;
duração absoluta do relógio; execução externa de `watch`/`w`; e paridade
funcional geral de terminal ou diagnósticos com o vanila.

# P1367 — relatório final do flush dos status

## Veredito

**PASS_WITH_LIMITED_ATTESTATION.** A implementação satisfaz o fragmento
congelado: cada um dos dois blocos exclusivos de status de `watch` passa
integralmente por `write_all` e, somente após escrita bem-sucedida, por
`flush`. Falha de escrita impede o flush; falha de flush é propagada; os
diagnósticos permanecem fora dessa fronteira.

Regime: protocolo completo, **executado sem atestação de isolamento**. O
verificador fez apenas inspeção dos artefatos e recibos existentes. Não
executou Cargo, teste, build, binário, `watch`, `w`, lint, gate ou rede e não
alterou código, testes ou entradas protegidas.

## Cadeia selada

Os hashes raw continuam idênticos ao selo:

- manifesto `e8a1a213…5369a7`;
- contrato `a1e53731…ba71`;
- oráculos `a45331ec…f04c`;
- mutantes `b4dfc8fd…b6ad`;
- passo `d3094568…bb8`;
- preflight `750b3864…7270`;
- gate `e1aeaab1…9b28`;
- fontes do vanila ratificado no commit
  `586e1bd43fae6c9a973218163d3165c53ab8d16d`: `watch.rs`
  `3d5ee88a…b585` e `terminal.rs` `0fbde0a2…04a5`.

O raw atual de `wiring.md` é `1965a6ba…91c6`, diferente do raw selado por
causa do resselo mecânico. Removendo exclusivamente a linha iniciada por
`Hash do Código:` e sua quebra, o digest normativo continua
`26a68f44…4bec`. A obrigação L0 congelada foi preservada.

O gate selado retornou `SEALABLE`, exit `0`: 15/15 positivos preservados,
22/22 negativos violados, oito opacos mantidos como `Unknown` com crédito
zero, nenhum `Unknown` inesperado e **30/30 mutantes rejeitados**. Mutation
score `1.0`; repetição e ordem reversa foram determinísticas.

## Materialização observada

Em `04_wiring/src/main.rs`:

- linhas 386–389: o helper privado chama `write_all(block.as_bytes())` e,
  depois do seu sucesso, `flush()`;
- linhas 391–395: o wrapper privado obtém `stderr`, mantém seu lock durante a
  delegação e usa `expect`, portanto a falha não é descartada;
- linhas 346–352: há exatamente dois call sites produtivos de status — o bloco
  `Compiling` e o terminal — e nenhum `eprint!` de status legado;
- linhas 806–820 e 967–1025: warning HTML, warnings, errors, mensagem
  operacional e drains de diagnóstico têm zero chamada aos helpers P1367;
- linhas 224–311: um único teste P1367 usa `FakeWriter` em memória. Sucesso
  prova bytes integrais, ordem `write → flush` e cardinalidade 1/1;
  `BrokenPipe` prova uma escrita, zero flush e o mesmo `ErrorKind`;
  `TimedOut` prova uma escrita, um flush e propagação do mesmo `ErrorKind`.

O hash raw atual do candidato é `5908f9ab…fb756`. Os helpers são privados e
nenhuma API pública foi criada.

## RED, GREEN e gates finais

O RED exato retornou `101` e registrou três diagnósticos `E0425`: o teste ainda
não encontrava `write_watch_block` em `super`. O mesmo comando, após a
implementação, retornou `0`, executou exatamente um teste e obteve **1/1
aprovado**, zero falhas e oito testes filtrados.

Os recibos finais registram:

- build único de `typst-wiring --bin typst`: exit `0`;
- `cargo fmt --all -- --check`: exit `0`;
- `git diff --check`: exit `0`;
- `crystalline-lint .`: exit `0`, sem violação bloqueante; warnings e infos
  preexistentes permanecem no recibo.

## Memória e proveniência

Os comandos dinâmicos registrados foram sequenciais, com
`CARGO_BUILD_JOBS=1` e timeout máximo de 120 segundos. Nenhum comando iniciou
`watch`, `w`, suíte externa ou ampla. Os exits e recibos não mostram timeout,
OOM, SIGKILL nem falha de alocação.

- gate selado: 7.359 bytes;
- RED/GREEN e gates finais: 384.464 bytes;
- resselo: 154 bytes;
- total contabilizado: **391.977 bytes** de 16 MiB permitidos;
- maior stream: **277.268 bytes** de 4 MiB permitidos.

O preflight adicional registra 6.304 bytes de status e 1.678 bytes de diff
stat para o HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`. A working tree está não
commitada e agrega P1356–P1367 e outras mudanças; esse contexto sustenta
reprodução, não atribuição exclusiva.

## Limitações e scope-out

O workspace partilhado impede atestação técnica de isolamento. O teste ficou
visível no mesmo arquivo ao implementador, logo a independência A/B não é
forte. O writer falso aceita o slice integral na primeira chamada e não é um
terminal real; escrita parcial, `Interrupted`, falhas reais de stderr e o
instante em que bytes chegam fisicamente ao terminal permanecem opacos.

Também continuam fora: cor, fullscreen, cursor, limpeza e TTY; conteúdo e
flush de diagnósticos não-watch; polling, dependências, staging, publicação,
abandono, espera e eviction; execução externa de `watch`/`w`; e paridade
funcional geral de terminal ou stderr com o vanila.

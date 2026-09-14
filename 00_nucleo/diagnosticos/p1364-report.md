# P1364 — relatório final da terminação dos blocos de `watch`

## Veredito

`PASS_WITH_LIMITED_ATTESTATION` — o fragmento congelado do P1364 foi
materializado: `Compiling` termina com uma LF, enquanto `Success`,
`PartialSuccess` e `Error` terminam com duas LF. O teste exato, o build e os
recibos estáticos sustentam o resultado.

A formulação correta é **executado sem atestação de isolamento**. As
autoridades foram logicamente separadas, mas trabalharam no mesmo workspace e
o implementador pôde ver o teste depois de escrever a implementação. O
resultado prova somente a terminação dos blocos não-fullscreen; não prova
paridade geral de `watch`, terminal ou diagnósticos com o vanila ratificado.

## Cadeia e proveniência

- HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado;
- vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- manifesto `999f6c38…`, contrato `f4ee4347…`, oráculos `3bbd5abb…`,
  mutantes `bcd230ca…`, gate `a430d631…` e selo `80b0b49f…` preservados;
- passo `0b73ad9b…` e preflight `f928cfaa…` permanecem byte a byte;
- o digest normativo de `wiring.md` permanece exatamente
  `14efe9983549ef437fd719ff786185416c740b79040324b3daad80bf3b5d31ee`
  após excluir somente a linha `Hash do Código:`.

O gate selado registra 14/14 positivos `Preserved`, 18/18 negativos
`Violated`, 8/8 opacos `Unknown` com crédito zero e 26/26 mutantes válidos
rejeitados, score 1,0, sem `Unknown` inesperado e com repetição/ordem reversa
estáveis. Sua única captura ficou limitada à ferramenta e não foi persistida
em `/tmp`; o verificador não a reexecutou.

Há uma inconsistência de proveniência não bloqueante: o `created_at` interno do
selo diz 09:13, enquanto o mtime do arquivo é 09:08:23 e antecede o RED de
09:10:13. A precedência causal é sustentada pelo mtime e pela ordem registrada,
mas o horário declarativo do selo não é usado como prova.

## Comportamento verificado

`watch_line_at` continua produzindo uma linha sem CR/LF. O novo
`watch_block_at` usa o `Time` literal recebido e acrescenta exatamente:

- uma LF a `[03:04:05] compiling ...`;
- duas LF aos corpos de sucesso, sucesso parcial e erro.

`watch_block_now` preserva a tentativa de hora local com fallback UTC e delega
ao helper determinístico. Os dois pontos produtivos de `run_watch` escrevem o
bloco pronto por `eprint!`, portanto não acrescentam terminador externo. Os
helpers são privados, nenhuma API ou tipo público foi criado e o caminho
normal de `compile` não os chama.

O helper privado P1363 `watch_line_now` ficou sem uso e gera `dead_code` no
teste e no build. É um aviso não bloqueante: não participa do roteamento nem
altera os bytes contratados pelo P1364.

## RED, GREEN e gates

- RED: `E0425`, `watch_block_at` inexistente, exatamente o defeito esperado;
- GREEN: `running 1 test`, `1 passed`, `0 failed`, `5 filtered out`;
- build limitado de `typst-wiring`: `Finished dev profile`;
- `cargo fmt --check` e `git diff --check`: stdout/stderr vazios;
- `crystalline-lint .`: recibo com warnings/infos, stderr vazio e sem
  `error`/violation bloqueante textual;
- `--fix-hashes`: `0 drift warnings remaining`.

Os arquivos de recibo não embutem exit code nem comando completo. Assim, o
GREEN e o build são sustentados pelos sumários `ok`/`Finished`; fmt, diff e lint
também dependem dos streams e da execução registrada pela orquestração. Essa é
uma limitação explícita da força do certificado, não uma alegação de recibo
mais completo do que o material existente.

## Contenção de memória

Não houve execução externa de `watch`/`w`, suíte externa de wiring ou suíte
ampla segundo o protocolo e o inventário de recibos. Foi executado um único
teste exato in-process e um único build, sequenciais, com
`CARGO_BUILD_JOBS=1` e timeout de 120 s por comando dinâmico.

Os 18 recibos persistidos em `/tmp` somam 391.013 bytes; o maior tem 277.268
bytes. Isso fica abaixo de 4.194.304 bytes por stream e 16.777.216 bytes no
agregado. Não há evidência textual de timeout ou OOM. A ausência de processos
proibidos é atestada pelo protocolo/conjunto de recibos, não por isolamento ou
auditoria do sistema operacional.

## Scope-outs e limites

Continuam fora: cor/fullscreen/limpeza/cursor; ordem relativa a diagnósticos;
buffering e flush; polling, dependências, staging, publish, abandon, wait e
eviction; relógio real, timezone e DST; execução externa de `watch`; e paridade
geral do terminal.

O verificador não executou cargo, testes, build, binário, `watch`, linter, gate
ou rede e não alterou nenhum artefato verificado. Leu apenas a cadeia selada,
código, teste e recibos, escrevendo este relatório e o certificado
`p1364-verification.json`.

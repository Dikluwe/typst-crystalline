# P1345 — Oracle R3, segunda e última revisão focal

Regime: `executado sem atestacao de isolamento`. Esta revisão final fecha somente `INVOCATION_JSON`, `CLI_AUTHORITY`, `PROBE_SCHEMA`, `PROBE_TOCTOU` e a entrega externa de `AUTHORITY_ROOT`. Não lê nem executa candidato e não executa full corpus.

O checker R3 exige schemas operacionais e tipos JSON exatos; a CLI usa `allow_abbrev=False`, rejeita repetição de flags e aliases de path; a saída do probe deve ser exatamente o JSON ordenado canónico seguido de um LF. O probe compilado é copiado para um memfd selado, o mesmo FD é hashado antes/depois e executado via `/proc/self/fd/N` com `pass_fds`; o FD/path original também é medido para detectar swap-and-restore.

A trust anchor não é provada pelo checker. `p1345-oracle-caller-r3.py` recebe checker, receipt e authoring root por argumentos fora da banda, rehasha os bytes canónicos antes do import e só então chama o checker. O ataque triple coordenado é julgado nessa fronteira contra os pins canónicos; uma triple alternativa autoconsistente não substitui o esperado do caller.

Pins antes do receipt:

- corpus R3: `6c5f6ce6173818e3c04373cdd0183e27fa1d5d208271322dc9ed5fb0c6cc1e88`
- checker R3: `67063acf2687a9662d7b213a8aa047c2a38f7b69a0310aff59f2dbe34809979f`
- caller R3: `a87f8f2f355ed7950ebe10a05f252816e5c0e5ddf561240c8a710991250c817e`
- source verifier R2 preservado: `9434ca3f2191cade3207d236a553b76045a016a2770da243251dc5d2d9527250`
- fixture/tabela/probe R1 preservados: `8cf4078146a3625931027d65a56f2610b132b2e9f6cf775ab4761b849a4d9e12`, `ffe9322a609c3f36ea152ce8a872407ebd7daddfef033660b495e235f0d2301f`, `863fa1588083638ec9f52b7ee663023e260a09880244c61cc948df36e946e2dc`.

Resultado focal final medido: 120/122 negativos `Violated`, quatro controles herdados preservados, quatro novos controles de fronteira aceitos, score `0.9836065573770492`, dois sobreviventes e zero regressões de controle, `full_corpus_runs=0`. Os sobreviventes são `P1344-A21-alternate-corpus-and-digest` e `P1344-R2A17-alternate-corpus-root`. O subprocesso rejeita ambos com exit 2, mas `argparse` reporta primeiro os argumentos obrigatórios ausentes; o adaptador histórico exige o literal `AUTHORITY_ROOT` no stderr e classifica os dois como `Preserved`. A mesma causa repetiu após a única correção focal de reason. Como o orçamento após R3 é zero, nenhuma nova correção é permitida e P1345 encerra sem seal: `FINAL_REVISION_SURVIVORS_NOT_SEALED`.

Proveniência: medição em `2026-09-11T08:41:08-03:00`, HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree não commitado com os artefatos R3 novos; execução somente focal pelo caller independente, temporários em `/dev/shm`, candidato não lido/executado e zero full runs.

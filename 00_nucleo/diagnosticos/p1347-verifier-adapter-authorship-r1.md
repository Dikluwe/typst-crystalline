# P1347 — autoria do adapter operacional R1

## Veredito

`ADAPTER_AUTHORED_NOT_EXECUTED`

Foi autorado o adapter operacional que liga a suíte adversarial P1347 R1 aos
alvos congelados do oráculo P1347 R2. Esta autoria não executou
`run_with_adapter`, focal do checker/caller, `ptrace`, corpus full, candidato,
RED, implementação produtiva ou pré-selo. O regime permanece **executado sem
atestacao de isolamento**.

## Papel e fronteira

- Papel exclusivo: autor do adapter operacional (`/root/p1347_adapter`), sem
  autoridade de verificador ou de veredito.
- Entradas lidas: somente passo P1347, contrato P1347 R1, suíte/recibos
  adversariais P1347 R1 e checker/caller/corpus/worker/recibos do oráculo P1347
  enumerados pela atribuição.
- Saídas: `p1347-verifier-adapter-r1.py`, este relatório de autoria e o receipt
  JSON correspondente.
- A função pública `exercise(case, targets, exact_ids)` não lê
  `expected_classification` nem `expected_reason_code`; a comparação continua
  pertencendo à suíte/verificador externo.
- O adapter valida os hashes externos de checker, caller e delivery receipt,
  percorre `caller.validate_chain()`, compara os 122 IDs ASCII na ordem exata e
  só então despacha uma operação.

## Schema de resultado

Cada retorno possui exatamente os cinco campos `case_id`, `classification`,
`evidence`, `reason_code` e `schema`. A evidência possui exatamente `api`,
`injection`, `observation`, `schema` e `target_binding_sha256`. Exceções fecham
como violação segundo o código público observado; o adapter não converte um
resultado para coincidir com a expectativa adversarial.

## Mapa de cobertura

A suíte congelada contém **49 ataques focais**, **181 replays ligados** e **11
controles**. O dispatch fechado contém **36 nomes de operação**, todos usados e
nenhum ausente.

| rota | operações | casos focais/controles |
|---|---:|---:|
| worker | 11 | 21 focais |
| processo parametrizado worker/probe | 1 | 4 focais |
| DAG | 6 | 7 focais |
| probe | 12 | 17 focais |
| replay P1346 | 1 | 181 replays |
| controles | 5 | 11 controles |

Cobertura focal exata por operação:

| operação | quantidade | API/ponto de injeção |
|---|---:|---|
| `stdout_without_child` | 2 | `validate_worker`, evidência parent-owned ausente |
| `mutate_id_sequence` | 6 | array `mutation_receipts`, omit/extra/duplicate/swap/Unicode |
| `inject_worker_pid` | 5 | campo extra `pid` no envelope fechado |
| `substitute_process_identity` | 1 | `child_pid` parent-owned substituído por sibling |
| `drop_parent_wait` | 1 | `raw_wait_status` removido |
| `mix_process_evidence` | 4 | stdout/stderr/wait do worker ou stream/status do probe |
| `truncate_worker_stdout` | 1 | bytes capturados truncados |
| `append_worker_stdout` | 1 | bytes extras após o envelope |
| `worker_timeout` / `worker_signal` / `worker_exit` | 3 | hooks do worker R1 |
| `replay_worker_challenge` | 1 | segunda validação do mesmo challenge/nonce/invocation |
| `dag_labels_without_validator` | 2 | receipt com canal de resposta |
| demais cinco operações DAG | 5 | `run_dag` com validador e anchor externos |
| `swap_fd_before_exec` | 1 | hook de probe R2 |
| `swap_fd_inside_exec` | 2 | hooks wrapper/postcheck do probe R2 |
| `dup_over_after_child_check` | 1 | hook postcheck do probe R2 |
| `swap_fd_after_exec_stop` | 1 | `dup2` após autenticação da imagem e antes da request |
| `second_exec_after_attestation` / `adversarial_image` | 2 | hooks de probe R2 |
| `remove_capability` | 3 | pidfd/ptrace/execve por FD |
| `fail_syscall` | 3 | pidfd/PTRACE_SETOPTIONS/F_GET_SEALS |
| `suppress_exec_stop` / `fail_proc_exe` | 2 | hooks de probe R2 |
| `substitute_proc_exe_image` | 1 | open de `/proc/<pid>/exe` redirecionado |
| `detach_before_terminal_wait` | 1 | PTRACE_DETACH no segundo continue |

Os controles mapeiam `canonical_worker` (1), `canonical_dag` (1),
`real_dag_validator` T08–T11 (4), `canonical_traced_probe` (1) e
`p1346_control_projection` (4).

## Replays P1346

`replay_p1346_negative` não fabrica classificação. Na futura execução externa,
o primeiro replay chama uma única vez o caller P1346 pinado com `--focus` por
`checker.parent_exec_fixed`, valida o JSON por `checker.strict_json` e mantém o
relatório em cache. Cada um dos 181 casos exige exatamente uma linha com o
`source_case_id` congelado e devolve o `actual`/`reason_code` observado nessa
linha, acompanhado pelo hash do registro e evidência parent-owned do processo.
Identidade ausente/duplicada ou schema impróprio falha fechado. Esse caminho não
foi executado durante a autoria.

## Restauração e temporários

Mutações de funções globais são instaladas por `_patched_probe` e restauradas em
ordem reversa dentro de `finally`, inclusive em exceção. O adapter nunca edita
checker, caller, worker, receipts ou entradas adversariais. Builds, árvores DAG
e materializações de worker usam exclusivamente `TemporaryDirectory` sob
`/dev/shm`; diretórios retidos em cache são limpos por `atexit`.

## Pins de autoria

| entrada | SHA-256 |
|---|---|
| `typst-passo-1347.md` | `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c` |
| `p1347-contract-spec-r1.json` | `fac6e30c7bbee3b50eec6cad8c8a9a8b30f3c09027f63293e73b4bf2341db0ea` |
| `p1347-contract-binding-r1.json` | `de1dcc87e77ab5fd6c8395fa4faacfd74fd90769dcafb5d2ef5c0bd30fda5fd1` |
| `p1347-contract-receipt-r1.json` | `85c6ad312cf9ac83ff653e2a19ac96fdd9e29ad72d2eb4af90fe8fff14fafb1d` |
| `p1347-adversary-suite-r1.py` | `d51fa54da1c68dcc5c812847b90097dfe94f1b4eb83f1009294230eb5f37dd4c` |
| `p1347-adversary-authorship-r1.md` | `dd2d67799a989eb8910e3102aa8d066b1d376ed5895d061455f54e61101b99da` |
| `p1347-adversary-authorship-receipt-r1.json` | `fd5aad168f4c22e5cc781456f7b000df0c542da6637a4a55459d5ccbccad3aa0` |
| `p1347-oracle-checker-r2.py` | `9aa0e93591195eac7ddb67069abecbcf0722abeddd9e955fc062804a89d16ce5` |
| `p1347-oracle-caller-r2.py` | `b377d09b912f4996d0712d67e9cd4b0b0eb0f7acc646f48f07d55fcf58f6071e` |
| `p1347-oracle-delivery-receipt-r2.json` | `a3e7cb3e6f29bba6293187ae967bd303f3f2ebebb79dd128679787d3b433432d` |
| `p1347-oracle-corpus-r1.json` | `24f2aa18c2f39f1647d29d9fe74c2d5a2cfc9079f812bced92a7d60456e12c2b` |
| `p1347-oracle-worker-r1.py` | `3f31593b545b7bec2160167c4395bf4bebe78a4111b194c74f4c775a4372afc5` |
| `p1347-oracle-authorship-receipt-r2.json` | `fb66120dd74f102139e2db839516c716793c5d852dfce1f6fb414d15b67a681c` |

## Validação estática e proveniência

Medição em `2026-09-11T12:36:01,015387626-03:00`, commit
`496c45ac6279d4298079f848b24b5e97ba88edc1`, working tree compartilhado e
não commitado. O escopo medido do autor era somente o adapter ainda untracked;
mudanças concorrentes fora da allowlist não são atribuídas a este papel.

- SHA-256 do adapter: `cc71d74b85b14e6624aae53630f25a3c5fe34724cab7d62222f84377d6845a7a`.
- `compile(source, ..., "exec")`: PASS, sem bytecode.
- Cobertura estática: 49/49 focais, 181 replays ligados e 11/11 controles;
  36/36 nomes mapeados, zero ausentes e zero extras.
- Pins das entradas P1347 publicadas nos receipts: PASS.
- `git diff --check` restrito ao adapter: PASS.
- Execuções: ataques=0, focal=0, full=0, ptrace=0, candidato=0.

Esta autoria prova somente que há uma ligação operacional fechada e auditável.
Não prova que o checker mata os ataques, não mede sobreviventes e não emite
pré-selo ou equivalência funcional.

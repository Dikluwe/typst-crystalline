# P1348 — autoria adversarial R1 congelada

## Veredito

`AUTHORED_NOT_EXECUTED`

A suíte foi congelada antes de qualquer leitura ou execução de checker, caller,
delivery ou adapter P1348. Nesta fase foi executado somente `--author-check`;
focal, ptrace, candidato e full permanecem em **0**. Isto é evidência de autoria
e cobertura estática, não de poder discriminatório, aprovação ou pré-selo.

Regime: `executado sem atestacao de isolamento`. O filesystem e o contexto são
compartilhados; a independência é preservada por allowlist, ordem causal e
limite de escrita, sem alegação de isolamento técnico forte.

## Papel e fronteira

- Papel: adversário independente P1348 (`/root/p1346_adversary`).
- Escrita limitada à suíte, a este relatório e ao receipt de autoria R1.
- Entradas: os passos P1348/P1347 exatos, contrato P1348, blocker P1347,
  autoria/suíte adversarial P1347 e baseline adversarial P1346 pinado.
- A suíte não descobre alvos. O verificador posterior deve fornecer checker,
  caller e delivery como `FrozenTargets`, além de um adapter externo com
  SHA-256 de arquivo inteiro.
- `run_with_adapter` é um entrypoint futuro e não foi chamado durante a autoria.

## Pins de entrada

| entrada | SHA-256 |
|---|---|
| `00_nucleo/materialization/typst-passo-1348.md` | `4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7` |
| `00_nucleo/materialization/typst-passo-1347.md` | `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c` |
| `p1348-contract-spec-r1.json` | `e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af` |
| `p1348-contract-binding-r1.json` | `de05f7c3a33bae7e131bca1aec0774a1fc370cdb3a6f43d50b1e2d050db5785f` |
| `p1348-contract-receipt-r1.json` | `1f49bdd80627edb61f115da108eca0b4216050bd81aaba093839e051f31b0daf` |
| `p1347-verifier-blocker-r1.json` | `3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522` |
| `p1347-adversary-suite-r1.py` | `d51fa54da1c68dcc5c812847b90097dfe94f1b4eb83f1009294230eb5f37dd4c` |
| `p1347-adversary-authorship-r1.md` | `dd2d67799a989eb8910e3102aa8d066b1d376ed5895d061455f54e61101b99da` |
| `p1347-adversary-authorship-receipt-r1.json` | `fd5aad168f4c22e5cc781456f7b000df0c542da6637a4a55459d5ccbccad3aa0` |
| `p1346-adversary-report-r2.json` | `60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6` |
| `p1346-adversary-report-r2.md` | `bd0ada205586718106e2e4851dfb8cfb7f50475967bd272567ef10d92512b466` |
| `p1346-adversary-receipt-r2.json` | `2fdf8efdd0f805c3dc321f34192a01715b273b9b071f51d231b80e71c88115a3` |

A identidade herdada continua sendo 122 IDs ASCII distintos na ordem exata,
com SHA-256 canônico
`e958b9c5e939a4b06b9e8594966001004de814e49e0141034d5351f1dbf4a907`.

## Fatias congeladas

O primeiro focal contém **25 casos**: os dez sobreviventes exatos P1347 e 15
novos ataques/meta-ataques P1348. Só zero falhas nessa fatia libera as **39
regressões focais P1347**. Só zero falhas nelas libera os **181 replays P1346**
e os **11 controles P1347**.

Os dez sobreviventes congelados são:

1. `P1347-I04-forged-dag-worker` → `Violated/DAG_VALIDATOR`;
2. `P1347-W11-pid-sibling` → `Violated/WORKER_AUTHORITY`;
3. `P1347-W16-partial-stdout` → `Violated/WORKER_AUTHORITY`;
4. `P1347-W17-extra-stdout-line` → `Violated/WORKER_AUTHORITY`;
5. `P1347-W21-challenge-replay` → `Violated/WORKER_AUTHORITY`;
6. `P1347-D01-labels-without-validator` → `Violated/DAG_VALIDATOR`;
7. `P1347-D03-mutant-provided-anchor` → `Violated/AUTHORITY_ROOT`;
8. `P1347-D06-coordinated-anchor-substitution` → `Violated/AUTHORITY_ROOT`;
9. `P1347-P04-swap-after-exec-stop` → `Violated/PROBE_AUTHORITY`;
10. `P1347-P17-tracer-detached` → `Violated/PROBE_AUTHORITY`.

## Variantes novas e validade

- **Transação parent-owned:** substituição coordenada de PID+pidfd com stdout
  canônico e PID terminado/reutilizado exigem `WORKER_AUTHORITY`; coincidência
  entre campos fornecidos não substitui a identidade observada pelo pai.
- **Frescor consumível:** validação dupla, inversão de slots, falha antes do
  commit seguida de retry e falha depois do commit verificam `ABSENT → PENDING
  → CONSUMED`, rollback pré-commit e irreversibilidade pós-commit. Registry
  injetado por payload/adapter não possui autoridade.
- **Razões DAG:** self-answer superficialmente válido continua
  `DAG_VALIDATOR`. Um resultado real `AUTHORITY_ROOT` não pode ser colapsado;
  adulterar streams antes de autenticá-lo deve falhar primeiro como
  `WORKER_AUTHORITY`.
- **Ciclo do tracer:** remover `TRACEEXIT`, detach no exec-stop, detach antes do
  exit-event, terminal fabricado sem exit-event e segundo exec exigem
  `PROBE_AUTHORITY`.
- **Exceção do adapter:** a regressão histórica de `original_open` exige
  `ADAPTER_EXCEPTION_BLOCKS_INTEGRATION`, sem classificação nem crédito no
  score. O P04 herdado simultaneamente exige que o adapter reparado chegue ao
  resultado real `PROBE_AUTHORITY`.

Falha pós-commit é meta-blocker e deve deixar a identidade consumida; ela não é
convertida em classificação. Os casos de double validation e rollback são
sequenciais e verificam ambas as observações, não apenas o último retorno.

## Controles e execução futura

Os onze controles preservam worker/DAG canônicos, quatro rejeições T08–T11,
quatro controles P1346 e o único `Unknown/OPAQUE_PAYLOAD`: probe canônico já
autenticado. Normal, repeat e reverse devem usar identidades frescas; igualdade
é da projeção normalizada, não dos nonces.

Qualquer survivor, razão divergente, blocker não deliberado, alteração de pin,
regressão ou `Unknown` de transporte interrompe a cadeia. O score só existe
depois de execução real; não foi calculado nesta autoria.

## Medição e validações

- Estado base: commit `496c45ac6279d4298079f848b24b5e97ba88edc1`.
- Hora da medição inicial: `2026-09-11T13:30:58-03:00`.
- Working tree: não commitado e compartilhado.
- Fingerprint NUL-safe do `git status --porcelain=v1 -z` antes deste relatório:
  `4e825bc604656d5de3f58c8b8a9b58cda5506163bd47d3fdac337b2e0d5a5da5`.
- SHA-256 da suíte:
  `14f3f41b28cfaa927197f5b5767612833e38944a9db74fb9f5ace7ee9b32e30c`.
- `--author-check`: 10 sobreviventes, 15 variantes, 39 regressões, 181
  replays, 11 controles, 122 IDs; focal/full/ptrace/candidato = 0.
- Sintaxe Python: válida por `compile(...)` sem bytecode.
- Nenhum checker, caller, delivery ou adapter P1348 foi lido ou executado.

Este relatório não afirma que os ataques matam o alvo futuro. Ele fixa o corpus
e as razões antes da entrega operacional que será julgada por outro papel.

# P1349 — autoria segregada do oráculo R1

Papel: autor independente do oráculo P1349. Regime:
`executado sem atestacao de isolamento`. Estado: `NOT_SEALED`.

## Autoridade

As entradas foram limitadas aos passos P1349/P1348/P1347 explicitamente
indicados, contrato P1349 R1, baseline de oráculo P1348 e blocker P1348. Nenhum
artefato adversarial, adapter, verificador ou candidato P1349 foi lido ou
executado. A skill de materialização segregada determinou a separação de
capacidades e a DAG de entrega.

## Implementação do oráculo

- `meta_result`, `classification_result` e `integration_blocker` são envelopes
  canônicos fechados e disjuntos, selecionados pela API antes do payload;
- cada operação supervisionada recebe executor descartável, `setsid`, pidfd
  retido, pipes dedicados, registro de PID/SID/PGID/starttime e ACK obrigatório
  antes da operação;
- deadlines usam exclusivamente `monotonic_ns`: checker em 8 s, supervisor em
  12 s, seguido por TERM, grace, KILL, reap, prova de grupo ausente e remoção do
  root `/dev/shm`;
- X06 e X15 existem apenas no meta-gate. O primeiro confirma commit consumido e
  impossível de reabrir; o segundo confirma `NameError` deliberado e restauração
  por identidade de `open`;
- qualquer exceção focal retorna `integration_blocker`, sem `classification` ou
  `reason_code`;
- o caminho externo X14 registra o tracee antes de liberá-lo, observa dois
  `PTRACE_EVENT_EXEC`, encerra pelo próprio checker, exige um
  `PTRACE_EVENT_EXIT`, `GETEVENTMSG` coerente e reap terminal antes de emitir
  `Violated/PROBE_AUTHORITY`.

## Medição própria

Estado: working tree não commitado, contrato root
`5a66f9dd44051f4a90bf55b068388bfd9b65ce2f98dc6a25df94d7eb5d250500`,
manifesto `1d2b30fd83d78d1fada381f82062309cbe5eaba2a4d32665fa59872dd9c3c907`,
corpus `045af76f9540107bb633c8d2c904e726f96624ecf8e9a91d706b99f530c56803`
e checker `f8e0311992e0e4973825a9b0ddbfaaea27e959f34bbcc4406ba5434ac1974843`.

O focal R1 executou cinco processos supervisionados em 15.750.782 ns. Meta
X06/X15 passou `2/2`; cruzamentos entre domínios foram rejeitados; ausência de
TRACEEXIT produziu `Violated/PROBE_AUTHORITY`; uma exceção deliberada produziu
blocker terminal sem razão; o controle normal permaneceu `Preserved`.
`full=0`, `candidate=0`, `ptrace=0`. X14 permanece requisito decisivo do
verificador externo e não recebe crédito nesta autoria.

## Limite causal

O receipt de autoria não conhece caller ou delivery. O caller pina somente
checker, receipt e authoring root. O delivery é a folha autoexcludente, cujo
SHA-256 integral é entregue fora da DAG. Esta autoria não calcula o root final
de execução do contrato, que ainda depende de adversário e adapter independentes.

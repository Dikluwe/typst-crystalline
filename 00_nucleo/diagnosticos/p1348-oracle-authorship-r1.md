# P1348 — autoria segregada do oráculo R1

Papel: autor do oráculo e da implementação de transporte P1348. Regime:
`executado sem atestacao de isolamento`. Veredito desta autoria:
`NOT_SEALED`.

## Fronteira de leitura e escrita

Foram lidos somente os dois passos explicitamente autorizados, o contrato P1348
R1, o bloqueio P1347 e os artefatos P1347 pinados pelo contrato. Nenhum candidato
produtivo, suíte adversarial P1348, adapter futuro ou verificador futuro foi lido
ou executado. Os únicos outputs desta autoria têm prefixo `p1348-oracle-`, além do
manifesto de autoridade P1348 autorizado.

## Mecanismo produzido

- o pai cria `fork`, `pidfd` e pipes, retém as identidades dos descritores, observa
  `waitid(P_PIDFD, WEXITED|WNOWAIT)` e efetua exatamente um `waitpid` terminal;
- o tuple de syscalls é autenticado por HMAC com segredo de sessão que nunca entra
  na projeção pública; substituir PID, pidfd, pipes, streams ou status invalida a
  transação como `WORKER_AUTHORITY`;
- um registry parent-owned reserva identidade/slot sob lock, só consome depois da
  validação completa e não reabre uma identidade após commit;
- framing parcial ou extra antecede parsing e resulta em `WORKER_AUTHORITY`; um
  receipt DAG com autojulgamento resulta em `DAG_VALIDATOR`; `AUTHORITY_ROOT` do
  validador real é transportado sem colapso;
- o caminho decisivo de probe configura exatamente `TRACEEXEC|TRACEEXIT|EXITKILL`,
  exige um único `EXEC`, a imagem real de `/proc/<pid>/exe`, um único `EXIT` com
  `GETEVENTMSG` e somente então o terminal do mesmo PID/pidfd.

## Medição focal

Estado medido: working tree não commitado, identificado pelo contrato root
`fef0a3d2c5be21c76a29562e18425679bacad6327a77744d24c49f8ef48bc8bd`,
manifesto `70754da5c833a43f1943ec7abdb453f67e8889b146b27b883e04097531bad6bb`,
corpus `3e33630290cdfe681e855dee4c3becfde0aac53560d6d1c72c4e0fe83bb25458`
e checker `fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249`.

Uma execução focal não-ptrace realizou dez subprocessos. Os ataques locais de
substituição de transação, replay, reordenação, stdout parcial/extra,
autojulgamento DAG, anchor mutante e formas inválidas de trace produziram a causa
esperada. Cinco controles executáveis foram preservados. O controle decisivo de
probe permaneceu `Unknown/EXTERNAL_PTRACE_REQUIRED`, pois ptrace deve ser executado
pelo verificador externo. `full_corpus_runs=0`, `ptrace_runs=0`,
`candidate_read_or_executed=false`.

## Limite causal

Esta autoria não sela o resultado P1348. O authorship receipt não conhece caller ou
delivery receipt; o caller conhece apenas checker, authorship receipt e authoring
root; o delivery receipt é a folha da DAG e seu SHA-256 é o trust anchor externo.

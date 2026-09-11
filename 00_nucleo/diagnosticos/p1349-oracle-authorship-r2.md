# P1349 — autoria segregada do oráculo R2

Papel: autor independente do oráculo P1349. Regime:
`executado sem atestacao de isolamento`. Estado: `NOT_SEALED`.

## Predecessores e fronteira

Esta revisão leu somente o blocker público P1349 R1 e os artefatos do próprio
oráculo P1349 R1. Nenhum adversary, adapter, verificador privado ou candidato foi
lido ou executado. Todos os artefatos R1 foram preservados.

Predecessores principais:

- blocker `ebbeedae4cc9c1fff6462abe065fca58aa402c8b67d04d90b3bb48cbaf37b3c1`;
- delivery R1 `81fa483c5a8c2ccdb0a05a720378765438a77ab0bd18f3d88b8227ce0cb71c0b`;
- checker R1 `f8e0311992e0e4973825a9b0ddbfaaea27e959f34bbcc4406ba5434ac1974843`.

## Delta causal R2

O checker R1 havia observado o segundo `EXEC`, efetuado terminação e reap, mas
convertia uma divergência do tuple terminal em `OracleFailure`. A R2 mantém
exceções para falha operacional real; após dois `EXEC`, terminação checker-owned,
um evento `EXIT` e reap terminal, retorna normalmente
`Violated/PROBE_AUTHORITY`. A igualdade entre `GETEVENTMSG` e raw status continua
registrada na evidência, sem virar fluxo excepcional de classificação.

Também foram adicionadas exatamente onze operações fechadas:

- seis de trace/lifecycle → `PROBE_AUTHORITY`;
- três de PID/pidfd/transação → `WORKER_AUTHORITY`;
- anchor substituído com streams autênticos → `AUTHORITY_ROOT`;
- self-answer sem validador real → `DAG_VALIDATOR`.

Cada operação passa por um avaliador de domínio próprio e a tabela de razões é
conferida após a classificação. A operação de falha inesperada do adapter não
mascara o segundo `EXEC`: a evidência fixa a precedência do evento do checker.

## Focal próprio

Estado medido: working tree não commitado; checker R2
`999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267`;
authoring root
`10cb199eea90852052d606937008eca8f5afe5bf5c289566cb14d064e4aa4117`.

O focal não-ptrace executou os onze avaliadores em 387.732 ns: `11/11`
produziram `Violated` e a razão exata, com hashes de evidência determinísticos.
O cruzamento de domínio foi rejeitado como `SCHEMA`; o retorno normal de X14 foi
validado estaticamente. Delta sobre o blocker: doze blockers previstos são agora
caminhos de classificação normal (`X14 + S01–S11`), sem regressões locais.
`full=0`, `ptrace=0`, `candidate=0`, `adversary=0`, `process_runs=0`.

## DAG

O authorship receipt R2 pina checker R2 e predecessores, mas não conhece caller
ou delivery. O caller R2 pina receipt/checker/root. O delivery R2 é a folha
autoexcludente e seu hash integral é o anchor externo. Nenhum selo é alegado.

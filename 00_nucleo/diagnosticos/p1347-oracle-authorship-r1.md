# P1347 — autoria independente do oráculo de transporte R1

Regime: **executado sem atestacao de isolamento**. Papel exercido:
**autor do oráculo/implementação de transporte**. Não foram lidos artefatos
`p1347-adversary-*`, código produtivo candidato ou saídas privadas do adversário.

Foram materializados um corpus sem vetor de respostas, um worker answer-free e
um checker que mantém PID, pidfd, pipes, `waitid`/`waitpid`, desafio, nonce e a
sequência exata dos 122 IDs sob autoridade do pai. O mesmo checker contém o
validador backward externo: o worker DAG apenas escreve bytes e uma receita, o
pai reabre a raiz em `/dev/shm`, fornece o anchor P1346 original fora da mutação,
observa o processo validador e só então deriva o resultado.

O probe cria e sela um memfd, chama `execve` diretamente pelo FD e exige
`PTRACE_EVENT_EXEC`; a requisição semântica permanece retida até o pai abrir e
hashear `/proc/<pid>/exe`. Falhas de pidfd, ptrace, evento, imagem, status ou
memfd são fechadas como `PROBE_AUTHORITY`, nunca `Unknown`.

## Medição e paragem obrigatória

No primeiro preflight empírico, o worker parent-owned passou com 122 IDs exatos,
e o DAG canônico/T08 passou pelo validador real. O probe não alcançou o primeiro
stop: no filho, `ptrace(PTRACE_TRACEME)` retornou `EPERM` (`errno=1`); o pai
observou `waitpid` raw `32256`, equivalente a exit `126`.

O contrato P1347 torna a indisponibilidade de ptrace um stop sem fallback,
waiver ou pré-selo. Por isso não foram executados os vetores formais, repeat,
reverse ou qualquer corpus completo. `full=0`. A revisão focal R1 foi consumida
pela descoberta; a R2 não é tentada porque não há hipótese causal de código que
remova uma proibição do ambiente.

Veredito: **`BLOCKED_PTRACE_EPERM_NOT_SEALED`**.

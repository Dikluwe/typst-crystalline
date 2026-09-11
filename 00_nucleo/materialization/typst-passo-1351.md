# Passo 1351 — corrigir a projeção de executor com verificação A/B

## Motivo

O protocolo completo aplicado aos diagnósticos tornou-se contraproducente: o
Passo 1350 criou 48 artefatos, encontrou três incompatibilidades seriais e não
chegou a executar a suíte principal. O último blocker está em
`00_nucleo/diagnosticos/p1350-adapter-integration-blocker-r3.json`, SHA-256
`00a52590c1add3122010bebb2b5fe1f1c997d7687ba0bccc297db316ba9237d9`.

Este passo usa o regime **A/B reduzido** da skill Tekt. Não há contrato de
refinamento a selar nem código produtivo: o risco relevante é somente o teste
ser escrito para aceitar a implementação. Um implementador e um autor de
testes recebem esta mesma obrigação, trabalham separadamente e um verificador
executa a integração. O workspace compartilhado impede alegar isolamento
técnico forte; o regime é `executado sem atestação de isolamento`.

## Escopo e L0

A mudança fica em `00_nucleo/diagnosticos/`. Não altera Prompt L0, contrato
público, comportamento por defeito, fase do pipeline ou consumer L1–L4. Não há
resselo de L0, candidato produtivo, RED nem full corpus funcional.

Os artefatos P1350 permanecem somente leitura. O P1351 cria no máximo:

1. um módulo de compatibilidade/supervisão;
2. uma suíte A/B independente;
3. um relatório final ou blocker.

Não criar manifests, bindings, receipts, delivery chains ou versões por cada
divergência.

## Correção

Separar os dois conceitos antes confundidos:

- `executor.path_kind`: `EXECUTOR | PUBLIC_REFUSAL`;
- `executor.identity_transport`: `PIDFD | WAITPID_ENOSYS | null`.

Para `EXECUTOR`, `path_kind` é sempre `EXECUTOR`, `identity_transport` informa
como a identidade foi vinculada, PIDs/SID/PGID/starttime são válidos e a
limpeza terminal é comprovada. Para `PUBLIC_REFUSAL`, `path_kind` é
`PUBLIC_REFUSAL`, `identity_transport` e identidades são `null`, e nenhum
processo é criado. `WAITPID_ENOSYS` só é permitido quando `pidfd_open` falha
com `ENOSYS`; outros erros continuam blockers.

O módulo P1351 é o produtor autorizado dessa projeção. Ele pode reutilizar o
supervisor P1350 R3, mas deve validar a evidência recebida antes de projetá-la;
não pode inventar PID, limpeza, evento ou classificação. Toda execução herdada
continua em executor descartável, sessão própria, deadline de 12 s, TERM/KILL,
wait terminal e grupo ausente. Temporários ficam em `/dev/shm`.

## Testes A/B

O autor B escreve os testes apenas a partir deste passo e do blocker público,
sem ler o módulo candidato. A suíte deve testar:

- branches `EXECUTOR` com `PIDFD` e `WAITPID_ENOSYS`;
- branch `PUBLIC_REFUSAL` sem processo;
- rejeição dos quatro valores quando aparecem no eixo errado;
- rejeição de fallback sem `ENOSYS`, identidade inválida e limpeza ausente;
- preservação de nonce/challenge e `meta_evidence`;
- um caso real supervisionado, com journal e grupo ausente;
- ausência de aresta que execute runtime herdado no processo principal;
- repetição com estado fresco.

Os testes devem acumular todas as falhas sem vazamento; só interrompem cedo se
houver processo residual. Não validam nomes de arquivo, hashes autorreferentes
ou cerimônia documental.

## Ordem e aceitação

1. congelar este passo e o blocker P1350 R3;
2. autor A implementa somente o módulo P1351;
3. autor B escreve somente a suíte P1351, sem ler A;
4. verificador executa primeiro os testes puros de schema;
5. depois executa os casos supervisionados em `/dev/shm`;
6. se verdes, executa uma regressão focal sobre as rotas P1350 afetadas;
7. valida `cargo build`, `crystalline-lint .` e `git diff --check`;
8. escreve um único relatório com comandos, tempos, HEAD, estado da árvore,
   contagens, processos e limitações.

Aceitação:

- todos os testes A/B passam;
- `path_kind` e `identity_transport` nunca são confundidos;
- nenhum processo, zombie, FD, registry ou journal residual;
- a regressão focal não volta a bloquear em `ptrace_stop`;
- zero alteração em P1350, L0 ou código produtivo.

Uma falha sem risco de vazamento pode ser corrigida dentro deste passo e toda a
suíte é reexecutada. Duas tentativas sem ganho na mesma causa bloqueiam. O
relatório cobre apenas esta correção diagnóstica; não prova equivalência
funcional geral nem fecha P1339/P1340.

# Passo 1350 — tornar a supervisão não contornável nas rotas herdadas

## Estado e objetivo

O Passo 1349 terminou em `BLOCKED_NOT_SEALED`. A execução normal permaneceu
ativa por pelo menos 869 segundos e deixou seis processos
`p1348-opaque-probe` simultaneamente em `ptrace_stop`, filhos diretos do
executor Python principal e no mesmo SID/PGID externo. Não houve resultado,
score nem journal por caso; o verificador precisou terminar o PGID exato.

A medição está congelada em
`00_nucleo/diagnosticos/p1349-verifier-blocker-r2.json`, SHA-256
`16780a65c46ba958b6e1535184ea039d022f558a01fb384cc0c110c261d55417`.
O adapter P1349 R2, SHA-256
`f7cc942f4875817ccd965c9974c1f030ed6e9ab638a46689e6754181d8f47775`,
supervisiona somente as doze operações novas e delega as 48 operações
herdadas diretamente ao adapter R1/P1348. O checker P1349 R2, SHA-256
`999971b4e25c6a44ae76542d0115d8304c3a8f28afbaf26bc1c9347db6edc267`,
possui deadline apenas nas rotas que efetivamente entram em `supervise`.

O objetivo deste passo é fazer da supervisão uma fronteira obrigatória para
**toda** operação focal, inclusive as herdadas, e produzir progresso causal
durável antes de executar novamente o corpus completo.

## Classificação arquitetural e L0

Este passo corrige somente o protocolo diagnóstico de calibração. Não altera
contrato público, comportamento por defeito, fase do pipeline, Prompt L0 nem
consumer produtivo L1–L4. Não autoriza candidato, RED, full corpus funcional
ou implementação das cápsulas. Portanto não há resselo de L0.

A paridade observada continua sendo a linguagem conforme ADR-0107. PID,
pidfd, sessão, grupo, deadline e journal são observáveis do mecanismo de prova,
não requisitos do produto.

## Hipótese causal medida antes da decisão

Medição estática:

- `p1349-verifier-adapter-r2.py:263` envia as operações da tabela supervisora
  ao checker;
- `p1349-verifier-adapter-r2.py:266-269` envia as operações herdadas ao adapter
  P1349 R1;
- `p1349-verifier-adapter-r1.py:330` chama diretamente
  `p1348-verifier-adapter-r1.exercise`;
- o blocker P1349 R2 registra os seis probes com PPID do Python principal,
  mesmo SID/PGID externo e `ptrace_stop`.

Inferência: as rotas herdadas contornaram a fronteira descartável e acumularam
filhos parados no executor principal. Refutariam a inferência um journal que
ligasse cada PID a um executor isolado com deadline disparado e reap concluído,
ou prova de que os PIDs já existiam antes da execução. A ausência desse journal
é parte do defeito, não licença para transformar o blocker em survivor.

Decisão: nenhuma operação focal pode invocar runtime herdado no processo do
lote. A API pública do adapter P1350 possui um único caminho, sempre
supervisionado; escolher a rota herdada ocorre somente dentro do executor
descartável.

## Contrato da fronteira não contornável

Para cada caso, o processo pai:

1. valida domínio, `case_id`, operação, sequência protegida e hashes;
2. cria pipes `CLOEXEC`, um executor por `fork` e uma capacidade aleatória
   vinculada a `case_id`, PID, starttime e nonce;
3. exige do executor `setsid()`, verifica `SID == PGID == PID` e autentica o
   handshake antes de liberar a operação;
4. abre pidfd quando disponível e registra fallback explícito por
   PID+starttime quando indisponível;
5. arma deadline monotônico máximo de 12 segundos por caso;
6. recebe exatamente um frame canônico e limitado, ou classifica blocker;
7. no sucesso ou falha, encerra o grupo exato: TERM, graça máxima de 250 ms,
   KILL se necessário, `waitpid` terminal e confirmação de inexistência do
   grupo;
8. só então devolve classificação ou blocker ao lote.

O executor descartável:

- fecha FDs não pertencentes ao protocolo antes do handshake;
- não herda o journal gravável nem autoridade para classificar limpeza;
- executa dentro da sessão própria tanto a rota P1350 quanto a rota
  P1348/P1347/P1346 herdada;
- devolve apenas o envelope fechado `classification_result` ou
  `integration_blocker`, sem exceção atravessar a fronteira;
- nunca pode promover timeout, crash, frame parcial, saída excessiva, PID
  divergente ou descendente remanescente a razão semântica.

É proibido expor runtime herdado cru por API pública, manter fallback direto
no processo principal ou aceitar flag que desative a supervisão. O checker é
o único autor de deadline, sinais, reap e evidência de limpeza; o adapter é o
único autor do mapeamento entre operação congelada e rota interna.

## Journal causal incremental

O verificador fornece ao pai um FD já aberto para um journal em `/dev/shm`.
Somente o pai supervisor escreve nele. Cada linha é JSON canônico, limitada,
com:

- `run_id`, ordem, índice, `case_id`, domínio e operação;
- evento fechado: `case_start`, `executor_bound`, `deadline_armed`,
  `frame_received`, `term_sent`, `kill_sent`, `terminal_wait`,
  `group_absent`, `case_result` ou `run_complete`;
- PID, starttime, pidfd disponível, SID/PGID, deadline monotônico e resultado
  de wait/sinal quando aplicável;
- hash da linha anterior e hash da linha corrente.

O pai faz `write` completo e `fdatasync` antes de liberar o handshake e antes
de iniciar o caso seguinte. Um parser independente rejeita linha truncada,
ordem impossível, hash quebrado, caso sem fechamento, dois casos ativos,
`run_complete` prematuro ou journal escrito pelo executor. O relatório final
é derivado dos resultados retornados; o journal serve para localizar e provar
limpeza, nunca para inventar classificação ausente.

## Ataques e controles obrigatórios

O adversário congela, antes de ler o oráculo P1350, ataques que demonstrem:

- tentativa de chamar rota herdada sem supervisor;
- executor parado antes e depois do handshake;
- deadline ignorado e TERM ignorado;
- descendente em `ptrace_stop` ao fim do caso;
- SID/PGID, PID/starttime ou pidfd divergentes;
- fork adicional, filho órfão e reap do PID errado;
- frame ausente, parcial, duplicado, não canônico ou acima do limite;
- capacidade ausente, repetida ou trocada entre casos;
- journal truncado, hash quebrado, evento fora de ordem e dois casos ativos;
- exceção do adapter e erro do checker, ambos como blockers terminais;
- execução repetida e ordem reversa sem registry, FD ou processo residual.

Controles devem preservar as classificações fechadas `Preserved`, `Violated`
e `Unknown` já legitimadas, incluindo meta X06/X15, X14, suas onze variantes,
os demais focais P1348, 39 regressões P1347, 181 replays P1346 e os 11
controles. `Unknown` continua válido somente para opacidade explicitamente
prevista; blocker nunca entra no denominador.

## Autoridades e cadeia causal

Regime: **executado sem atestação de isolamento**. O workspace compartilhado
impede alegação de isolamento forte, mas autoria, allowlists, ordem e hashes
devem ser registrados.

1. autor da intenção: escreve somente este passo;
2. autor do contrato: recebe este passo e o blocker P1349 R2; não lê solução;
3. autor do oráculo: recebe contrato congelado e baseline P1349 R2; escreve
   supervisor, checker, caller, parser de journal e recibos; não lê ataques;
4. adversário: recebe passo, contrato e baseline; congela ataques e controles
   antes de ler o oráculo;
5. autor do adapter: recebe oráculo e adversário congelados; escreve apenas a
   ponte P1350 sem executar casos;
6. verificador: valida hashes/DAG, executa gates e escreve somente report,
   attestation, receipt, preseal ou blocker; nunca corrige entradas.

Cada papel registra executor, allowlist lida/escrita, contexto herdado,
predecessor causal, hashes, custo e artefatos. Artefatos P1349 e anteriores são
somente leitura.

## Ordem, gates e orçamento

1. congelar passo, blocker e baselines;
2. contrato independente;
3. oráculo e adversário independentes;
4. adapter após ambos;
5. validar estaticamente que toda operação focal possui exatamente uma entrada
   pública e que não existe aresta direta para adapters herdados;
6. executar primeiro os ataques de bypass, hang, descendente parado e journal,
   acompanhados de controles de fronteira;
7. somente com zero blockers inesperados executar meta `2/2`, X14 e variantes;
8. somente então executar demais P1348, 39 regressões P1347, 181 replays P1346
   e 11 controles;
9. repetir normal/repeat/reverse em processos e journals frescos;
10. exigir mutation score `1.0`, zero survivors, regressões e blockers
    inesperados;
11. validar JSON, sintaxe, pins, DAG, V5/V15/V26, linter e diff-check;
12. emitir pré-selo limitado ao transporte ou blocker reproduzível.

Budget: uma revisão contratual, até duas revisões do oráculo, uma do adversário
e uma do adapter após suas versões iniciais; no máximo duas execuções completas.
Recortes focais precedem qualquer repetição completa. Duas tentativas sobre a
mesma causa sem ganho discriminatório, limpeza que exija supervisor externo,
journal incapaz de nomear o caso ativo ou qualquer processo residual bloqueiam
sem waiver.

## Aceitação e limites

- todas as operações focais atravessam um executor com sessão própria;
- cada caso termina dentro do deadline pelo supervisor interno;
- zero processos, zombies, FDs, monkeypatches ou registries residuais;
- journal completo, encadeado e determinístico identifica cada transição;
- meta `2/2`, focal e regressões mantêm reasons/classificações exatas;
- mutation score `1.0`, zero blockers inesperados;
- normal/repeat/reverse produzem o mesmo vetor sem reutilizar estado;
- nenhum artefato P1349, L0 ou consumer produtivo é alterado.

O pré-selo P1350, se emitido, cobre somente a fronteira de supervisão e o
transporte discriminatório registrado. Não fecha P1339/P1340, não prova
equivalência funcional geral e não autoriza RED, candidato produtivo ou full
corpus funcional.

# Passo 1348 — ligar identidade, frescor, razões e ciclo completo do tracer

## Regime e objetivo

Regime: **executado sem atestacao de isolamento**.

P1348 é uma cadeia nova de protocolo completo. Compõe P1347 como baseline
falhado e imutável; não altera os seus contratos, oráculos, ataques, adapter ou
blocker. O objetivo é eliminar os dez sobreviventes do focal independente P1347
sem relaxar classificação, razão pública, ordem de execução ou controles.

P1348 corrige quatro fronteiras:

1. identidade do PID ligada à observação real de `pidfd`/`waitpid`;
2. frescor consumível de challenge/nonce/invocation;
3. reason codes preservados na fronteira worker/DAG;
4. tracing obrigatório até um `PTRACE_EVENT_EXIT`, além do exec-stop.

Também corrige o defeito estritamente operacional `original_open` ausente no
adapter. Nenhum L0 ou consumer produtivo pertence ao escopo.

## Estado medido e proveniência

Medição em `2026-09-11T13:16:44-03:00`:

- `HEAD = 496c45ac6279d4298079f848b24b5e97ba88edc1`;
- working tree não commitado, com 22 paths P1347 não rastreados; a lista exata
  está pinada no campo `provenance.git_status_porcelain_paths_before_output` do
  blocker P1347;
- passo P1347, SHA-256
  `c848a04d9bc3aaa9ed935ea8fef6e615cdb77a0533c13d0fcd62ef1a87ab9a1c`;
- blocker independente P1347, SHA-256
  `3b649ced677059a459262710ce3466cb2efbd69edbfb6e60c98df07cffa64522`;
- checker P1347 R2, SHA-256
  `9aa0e93591195eac7ddb67069abecbcf0722abeddd9e955fc062804a89d16ce5`;
- adapter P1347 R1, SHA-256
  `cc71d74b85b14e6624aae53630f25a3c5fe34724cab7d62222f84377d6845a7a`;
- suíte adversarial P1347 R1, SHA-256
  `d51fa54da1c68dcc5c812847b90097dfe94f1b4eb83f1009294230eb5f37dd4c`.

O focal decisivo foi executado fora do sandbox, onde ptrace está disponível, em
`2026-09-11T12:46:27-03:00`. Normal, repeat e reverse produziram a mesma projeção
SHA-256 `327a3c1889bf6170cfc8f995412c2f2f2c0012c1ae1a936b20007c03da799015`.
Dos 49 negativos válidos, 39 tiveram classificação e razão corretas; dez
sobreviveram. Score `39/49 = 0.7959183673469388`. Replays, controles e full
permaneceram em zero pelo gate congelado.

## Sobreviventes obrigatórios

P1348 deve reexecutar e eliminar exatamente estes casos antes de qualquer
regressão ampla:

- `P1347-I04-forged-dag-worker`;
- `P1347-W11-pid-sibling`;
- `P1347-W16-partial-stdout`;
- `P1347-W17-extra-stdout-line`;
- `P1347-W21-challenge-replay`;
- `P1347-D01-labels-without-validator`;
- `P1347-D03-mutant-provided-anchor`;
- `P1347-D06-coordinated-anchor-substitution`;
- `P1347-P04-swap-after-exec-stop`;
- `P1347-P17-tracer-detached`.

Os demais 39 focais P1347 ficam como regressões obrigatórias e não podem mudar
de razão para acomodar os dez acima.

## Auditoria L0 e ADR-0127

Nenhuma mudança L0 é necessária. Esta cadeia altera somente instrumentos de
diagnóstico e verificação. Não muda contrato público do produto, comportamento
por defeito, fase do pipeline ou compatibilidade; ADR-0127 não exige paragem.

## Identidade parent-owned do worker

O checker deve produzir uma evidência parent-owned fechada e validá-la como uma
unidade, não como campos independentes. No mínimo:

- `child_pid` é inteiro estrito, positivo e o PID devolvido pelo `fork`;
- o mesmo PID é usado em `pidfd_open`, `waitid(P_PIDFD, ..., WNOWAIT)` e
  `waitpid`;
- a identidade do pidfd, os endpoints dos pipes e o raw wait status são
  registrados antes de qualquer payload ser aceito;
- substituir somente `child_pid`, pidfd, stream ou status rompe a ligação e é
  `Violated/WORKER_AUTHORITY`;
- PID sibling, PID correto sem wait correspondente e mistura de evidências de
  processos diferentes nunca são aceitos.

Uma asserção de PID dentro do stdout continua sem autoridade.

## Frescor consumível

Challenge, invocation nonce, invocation hash e digest da sequência formam uma
identidade de execução. O checker mantém um registry parent-owned por sessão de
verificação e consome essa identidade exatamente uma vez depois de validar
processo, sequência e resposta.

- primeira validação canônica pode ser `Preserved`;
- replay byte a byte na mesma sessão é `Violated/WORKER_AUTHORITY`;
- falha anterior não pode marcar uma identidade nova como consumida;
- repeat/reverse usam execuções e identidades frescas, não reciclam o registro;
- registry fornecido por worker, caso, corpus ou adapter não tem autoridade.

## Fronteiras de reason code

P1348 preserva as expectativas congeladas; não as reescreve:

- stdout parcial, linha extra, stream/status incompatível e replay são
  `WORKER_AUTHORITY`, pois falham a autenticidade do canal antes do parse
  semântico do envelope;
- PID com tipo inválido continua `SCHEMA`; PID bem tipado mas não ligado ao
  processo observado é `WORKER_AUTHORITY`;
- um receipt DAG com canal de resposta/autojulgamento é `DAG_VALIDATOR`, não
  sucesso e não `SCHEMA` genérico;
- anchor fornecido ou substituído pelo mutante preserva
  `AUTHORITY_ROOT` retornado pelo validador real;
- falha de schema interna ao artefato mutado só é `SCHEMA` quando nenhuma razão
  de autoridade ou transporte anterior se aplica.

O adapter deve transportar a primeira razão observada sem colapsar
`AUTHORITY_ROOT` em `DAG_VALIDATOR`.

## Tracing até a saída

Atestar somente `PTRACE_EVENT_EXEC` não prova que o tracer permaneceu ligado.
P1348 exige:

1. `PTRACE_SETOPTIONS` inclui `PTRACE_O_TRACEEXEC`, `PTRACE_O_TRACEEXIT` e
   `PTRACE_O_EXITKILL`;
2. exatamente um exec event é observado e a imagem parada é rehashada;
3. depois da execução do probe, o pai deve observar exatamente um
   `PTRACE_EVENT_EXIT` antes do status terminal;
4. o pai lê a mensagem/estado de saída quando disponível, continua o mesmo PID
   e somente então aceita o `waitpid` terminal;
5. detach, ausência de exit-event, segundo exec, processo sibling ou terminal
   sem evento anterior é `Violated/PROBE_AUTHORITY`;
6. apenas após o ciclo completo e os hashes/seals pós-exec o payload opaco pode
   resultar `Unknown/OPAQUE_PAYLOAD`.

## Correção do adapter

O adapter P1348 é um artefato novo. Ele deve corrigir o `NameError` do caso P04
capturando e restaurando `open` no mesmo escopo da mutação, sempre em `finally`.
Nenhuma exceção do adapter pode ser convertida na razão esperada do caso; falha
do adapter é blocker de integração.

O adapter não fabrica classificações. Ele apenas aplica a mutação congelada,
executa a API correspondente e transporta a classificação/razão observadas.

## Autoridades

1. **Autor da intenção**: escreve somente este passo.
2. **Autor do contrato P1348**: recebe passo, blocker e baseline P1347 pinados;
   escreve somente contrato/binding/receipt; não lê implementação P1348.
3. **Autor do oráculo P1348**: recebe contrato fechado e checker P1347 R2;
   escreve novos checker/caller/receipts P1348; não lê ataques P1348.
4. **Adversário P1348**: recebe passo, contrato, blocker e suíte P1347;
   congela os dez replays e variantes antes de ler o checker P1348.
5. **Autor do adapter P1348**: recebe oráculo e suíte já congelados; escreve só
   o mapeamento operacional e não executa o veredito.
6. **Verificador P1348**: revalida pins, executa e decide; não corrige entradas.

O filesystem/contexto compartilhado impede alegação de isolamento forte. Cada
papel registra entradas, allowlist, hashes, custo e ordem causal.

## Ataques adicionais obrigatórios

Além dos dez sobreviventes e dos 39 focais já mortos:

- trocar PID e pidfd de forma coordenada, mantendo stdout canônico;
- usar PID real de child anterior já terminado ou reutilizado;
- validar duas vezes a mesma identidade e reordenar as duas validações;
- causar falha antes/depois do consumo do registry;
- fornecer registry pelo adapter ou payload;
- emitir self-answer DAG em schema superficialmente válido;
- fazer o validador retornar `AUTHORITY_ROOT` com stdout/stderr adulterados;
- remover `PTRACE_O_TRACEEXIT`, destacar no exec-stop e destacar antes do
  exit-event;
- fabricar status terminal sem exit-event;
- executar segundo exec entre imagem atestada e exit-event;
- provocar a exceção histórica do adapter e exigir blocker explícito.

## Ordem e gates

1. congelar P1348 e baseline P1347;
2. contrato independente;
3. oráculo e adversário independentes a partir do contrato;
4. adapter novo somente depois de ambos congelados;
5. executar primeiro os dez sobreviventes e variantes novas;
6. zero survivors libera os outros 39 focais;
7. zero survivors libera os 181 replays e 11 controles P1347;
8. normal/repeat/reverse devem ter projeção idêntica com identidades frescas;
9. exigir score `1.0`, zero regressões e apenas o probe autenticado como
   `Unknown`;
10. validar JSON, sintaxe Python, hashes/DAG, V5/V15/V26 e diff-check;
11. emitir pré-selo P1348 limitado ao transporte ou blocker reproduzível.

Temporários ficam em `/dev/shm`. Execuções que precisam de ptrace ocorrem fora
do sandbox e registram essa diferença de ambiente. Full corpus, RED, candidato
produtivo e 38 cápsulas permanecem em zero.

## Orçamento e stop

Uma revisão contratual, até duas revisões do oráculo, uma revisão do adapter e
duas revisões adversariais. Execute recorte focal antes de qualquer regressão.
Duas revisões sem ganho causal, novo survivor válido, razão relaxada, adapter
que fabrica resultado, entrada protegida alterada ou ausência de tracing
completo bloqueiam P1348 sem waiver.

## Aceitação e limites

- dez sobreviventes P1347 e variantes P1348 são `Violated` com razão exata;
- 39 focais mortos permanecem mortos;
- 181 replays P1346 permanecem mortos;
- 11 controles passam;
- normal/repeat/reverse são determinísticos e frescos;
- score focal `1.0`, transport `Unknown = 0`;
- probe canônico autenticado continua sendo o único
  `Unknown/OPAQUE_PAYLOAD`;
- nenhum artefato P1347, L0 ou consumer produtivo é alterado.

O pré-selo P1348, se emitido, cobre somente este transporte discriminatório.
Não fecha P1339/P1340, não prova equivalência funcional geral e não autoriza
RED ou implementação das 38 cápsulas sem passo subsequente explícito.

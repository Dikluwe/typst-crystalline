# Passo 1349 — separar fault-injection meta do focal e supervisionar execuções

## Regime e objetivo

Regime: **executado sem atestacao de isolamento**.

P1349 é uma cadeia nova de protocolo completo. Compõe P1348 como baseline
falhado e imutável e corrige a incompatibilidade que tornou impossível o seu
pré-selo: uma falha deliberadamente injetada no adapter era simultaneamente um
resultado esperado da suíte e um blocker terminal do focal.

P1349 cria duas fases semanticamente distintas:

1. **meta-gate de fail-closed**: injeta falhas planejadas em processos isolados
   e verifica que produzem blockers exatos e restauração segura;
2. **focal discriminatório**: aceita somente classificações e razões do
   contrato; qualquer blocker inesperado continua terminal.

O passo também torna todo caso de trace/segundo exec supervisionado por deadline,
pidfd e limpeza de grupo de processos. Nenhum L0 ou consumer produtivo pertence
ao escopo.

## Estado medido e proveniência

Medição em `2026-09-11T14:18:49-03:00`:

- `HEAD = 496c45ac6279d4298079f848b24b5e97ba88edc1`;
- working tree não commitado com 40 paths P1347–P1348 não rastreados;
- SHA-256 de `git status --porcelain=v1`:
  `477c9906ca12e0e545f65a35582ed696dd52d6a6c7ca948834389e7145ef7a9c`;
- passo P1348, SHA-256
  `4254d294e71427c7dd54ab28db674d7b20174e560bd4882bf3413dae217224c7`;
- blocker P1348, SHA-256
  `e5b9e9423e2e3ad20064cbc20f2594e25396f7e1bffe20123a4f8392cb8501de`;
- contrato P1348 R1, SHA-256
  `e9e3e10b069604f7bc45143da4625e728c34d6d3b7223c7fa9491173fa7131af`;
- checker P1348 R1, SHA-256
  `fd1c4a0e1601ea817ffb92ad6723367083ac9f5c7a0f9bcecb155e9148764249`;
- suíte adversarial P1348 R1, SHA-256
  `14f3f41b28cfaa927197f5b5767612833e38944a9db74fb9f5ace7ee9b32e30c`;
- adapter P1348 R1, SHA-256
  `bcec1c05eed907ec3ea8f45c55940ee5244fe9498c731e66458b9817d946edfa`.

O focal P1348 foi executado fora do sandbox, com ptrace disponível. O gate
parou no caso `P1348-X06-failure-after-registry-commit`. Regressões, replays,
controles, repeat, reverse, full e candidato permaneceram em zero.

## Bloqueio herdado

Três observações são normativas:

1. `X06` produziu `integration_blocker/ADAPTER_EXCEPTION_BLOCKS_INTEGRATION`
   com identidade `CONSUMED` e `reopened=false`. A suíte tratava isso como
   resultado esperado, mas o verificador corretamente o considerou terminal.
2. `X15` produziu blocker por `NameError`, com `open` restaurado. Também era uma
   fault injection planejada, não uma classificação de mutante.
3. `X14` não terminou até interrupção; `KeyboardInterrupt` foi convertido em
   blocker e excluído de reason/score.

P1349 não reclassifica esses outputs. Ele move X06 e X15 para um domínio meta
que não participa do mutation score e torna X14 um negativo focal terminado por
supervisor.

## Auditoria L0 e ADR-0127

Nenhuma mudança L0 é necessária. O escopo é exclusivamente protocolo de
diagnóstico. Não muda API pública, comportamento por defeito, fase do pipeline
ou compatibilidade; ADR-0127 não exige paragem.

## Meta-gate planejado

O meta-gate executa antes do focal e em processos descartáveis separados. Seus
casos são declarados no contrato, não descobertos depois da implementação.

### X06 — falha depois do commit de frescor

- injetar a falha somente depois do commit da identidade;
- exigir outcome `integration_blocker` e veredito
  `ADAPTER_EXCEPTION_BLOCKS_INTEGRATION`;
- exigir identidade `CONSUMED`, `reopened=false` e nenhuma segunda aceitação;
- confirmar restauração do ambiente;
- destruir o processo e o registry usados no teste.

### X15 — exceção histórica do adapter

- injetar `NameError` no mesmo boundary histórico;
- exigir outcome `integration_blocker`, sem `classification` ou `reason_code`;
- exigir restauração de `open` e igualdade de identidade antes/depois;
- destruir o processo do teste.

Observar exatamente o blocker previsto torna o **meta-teste** `PASS`; não torna
o mutante `Violated`, não entra no score e não autoriza continuar se restauração,
isolamento ou postcondição divergirem. Qualquer blocker fora desta allowlist
fechada permanece terminal.

## Focal discriminatório

X06 e X15 deixam de pertencer à lista focal. Os demais ataques P1348, incluindo
X14, continuam negativos válidos com classificação e razão exatas. O focal não
aceita `integration_blocker`, exceção do adapter, timeout não classificado ou
resultado sem reason.

O relatório deve manter contagens separadas:

- `meta_tests_passed/meta_tests_total`;
- `focal_correctly_violated/focal_valid_negatives`;
- `mutation_score`, calculado somente sobre negativos focais;
- `unexpected_integration_blockers`, que deve ser zero.

## Supervisor de cada operação perigosa

Cada caso que usa ptrace, segundo exec, fork adicional, detach ou monkeypatch de
syscall roda em processo supervisor próprio:

1. o supervisor cria sessão/grupo de processos próprio e deadline monotônico;
2. obtém PID e pidfd do executor e liga stdout/stderr a pipes próprios;
3. o executor registra filhos antes de continuar o caso;
4. término normal exige status do PID exato e ausência de descendentes vivos;
5. no deadline, o supervisor sinaliza por pidfd e grupo de processos, espera e
   recolhe todos os filhos;
6. timeout de X14 é `Violated/PROBE_AUTHORITY` somente quando o checker detecta
   e encerra o segundo exec dentro do deadline contratual;
7. se apenas o supervisor externo precisar matar o caso, o resultado é
   `integration_blocker/SUPERVISOR_TIMEOUT`, terminal e fora do score;
8. `KeyboardInterrupt`, processo órfão ou limpeza incompleta nunca são razões
   válidas do mutante.

O checker deve observar um exec event, rejeitar um segundo exec imediatamente,
observar/forçar o exit-event permitido e garantir reap. Temporários ficam em
`/dev/shm`.

## Adapter P1349

O adapter possui duas APIs fechadas:

- `exercise_meta(case, targets) -> meta_result`;
- `exercise_focal(case, targets, exact_ids) -> classification_result`.

`meta_result` não contém classificação semântica. `classification_result` não
pode conter outcome meta. Exceções são capturadas apenas para produzir blocker
de integração e nunca para coincidir com razão esperada.

O adapter reutiliza as correções P1348 de PID, registry, reason precedence e
TRACEEXIT. Ele não altera expectativas nem fabrica resultados. Estado mutado é
restaurado em `finally` e sua identidade é verificada antes de retornar.

## Autoridades

1. **Autor da intenção**: escreve somente este passo.
2. **Autor do contrato P1349**: recebe passo e blocker P1348; formaliza a
   separação meta/focal e supervisor; não lê implementação P1349.
3. **Autor do oráculo P1349**: recebe contrato fechado e baseline P1348;
   implementa supervisor/checker/caller/receipts; não lê ataques P1349.
4. **Adversário P1349**: recebe passo, contrato e baseline P1348; congela meta,
   focais e variantes antes de ler o oráculo P1349.
5. **Autor do adapter P1349**: recebe ambos congelados; escreve somente adapter
   e não executa veredito.
6. **Verificador P1349**: executa meta, focal, regressões e controles na ordem;
   não corrige entradas.

O workspace e o contexto compartilhados impedem alegação de isolamento forte.
Cada papel registra allowlist, inputs, hashes, custo e predecessor causal.

## Ataques e controles obrigatórios

Meta-gate:

- X06 pós-commit com identidade consumida e não reaberta;
- X15 `NameError` com `open` restaurado;
- blocker errado, postcondição errada, restauração incompleta e vazamento de
  processo devem falhar o meta-gate.

Focal:

- X14 segundo exec deve terminar `Violated/PROBE_AUTHORITY`;
- segundo exec antes e depois do exit-event;
- executor ignora deadline;
- pidfd divergente, PID reutilizado e processo sibling;
- stdout parcial/extra e challenge replay;
- detach antes do exit-event;
- anchor mutante e DAG self-answer com razões exatas;
- blocker inesperado, exceção ou timeout externo devem bloquear, não pontuar.

Todos os demais negativos P1348/P1347/P1346 e os 11 controles continuam
obrigatórios depois dos gates anteriores.

## Ordem e gates

1. congelar P1349 e baseline P1348;
2. contrato independente;
3. oráculo e adversário independentes;
4. adapter depois de ambos congelados;
5. executar meta-gate X06/X15 em processos isolados e exigir `2/2`;
6. executar X14 e variantes de supervisão primeiro;
7. zero blockers/survivors libera os demais focais P1348;
8. zero survivors libera 39 regressões P1347;
9. zero survivors libera 181 replays P1346 e 11 controles;
10. repetir normal/repeat/reverse com processos e registries frescos;
11. exigir score `1.0`, zero regressões, zero blockers inesperados e somente o
    probe autenticado como `Unknown`;
12. validar JSON, sintaxe, hashes/DAG, V5/V15/V26, linter e diff-check;
13. emitir pré-selo P1349 limitado ao transporte ou blocker reproduzível.

Execuções com ptrace ocorrem fora do sandbox. Full corpus, RED, candidato
produtivo e 38 cápsulas permanecem em zero.

## Orçamento e stop

Uma revisão contratual, até duas revisões do oráculo, uma do adapter e duas do
adversário. Execute somente o recorte afetado antes de regressões. Meta-gate
divergente, supervisor externo obrigado a matar X14, processo órfão, survivor
válido, razão relaxada ou duas revisões sem ganho causal bloqueiam sem waiver.

## Aceitação e limites

- meta-gate `2/2`, com blockers planejados exatos e ambientes descartados;
- X14 e variantes terminam pelo checker dentro do deadline;
- focal score `1.0`, zero `Unknown` de transporte e zero blockers inesperados;
- 39 regressões P1347 e 181 replays P1346 permanecem mortos;
- 11 controles passam;
- normal/repeat/reverse são determinísticos com estado fresco;
- nenhum processo, FD, monkeypatch ou registry vaza entre casos;
- nenhum artefato P1348, L0 ou consumer produtivo é alterado.

O pré-selo P1349, se emitido, cobre somente o transporte discriminatório. Não
fecha P1339/P1340, não prova equivalência funcional geral e não autoriza RED ou
implementação das 38 cápsulas sem passo subsequente explícito.

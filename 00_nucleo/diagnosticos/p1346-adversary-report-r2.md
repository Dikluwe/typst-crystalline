# P1346 — relatório adversarial focal final R2

## Veredito

`ADVERSARIAL_R2_SURVIVORS_BLOCK_P1346_DEFINITIVELY`

A cadeia R2 foi rehashada a partir do trust anchor externo
`2b4ec685285153b185a3de6a92aec074de39731f34e0f52b35489ddfb4c0f5e0`
antes de importar caller ou checker. Em seguida, o adversário executou de novo,
sem consumir o recibo pós-delivery como prova:

- 122 negativos P1345 dentro do worker focal;
- 18 transportes P1346, incluindo quatro workers DAG;
- os 27 ataques adversariais R1;
- 19 ataques novos de worker, PID, receipt, memfd, fork e exec;
- quatro controles canônicos.

Os 27 ataques R1 foram todos rejeitados na R2 e os quatro controles passaram.
No conjunto total de 186 negativos válidos, 181 foram corretamente rejeitados
e cinco sobreviveram: score `181/186 = 0.9731182795698925`. Corpus full
consumido: zero.

## Classe 1 — stdout do worker é aceito como execução

Três ataques ao worker histórico sobreviveram:

- `P1346-R2A01-forged-worker-all-violated`: um `CompletedProcess` fabricado,
  sem executar o case runner, entrega 122 IDs e respostas `Violated`; o checker
  aceita o conteúdo como execução real;
- `P1346-R2A02-forged-worker-duplicate-ids`: 122 cópias do mesmo ID também são
  aceitas, pois o pai não valida unicidade, ordem nem identidade do conjunto;
- `P1346-R2A03-forged-worker-pid-string`: `pid = "not-a-pid"` é aceito como
  evidência de processo.

O SHA de stdout calculado pelo pai prova apenas quais bytes ele recebeu. O PID
é lido desses mesmos bytes e não é confrontado com identidade obtida pelo pai.
Portanto, report/receipt canônicos continuam sendo aceitos no lugar de prova de
execução — exatamente o canal que a R2 precisava fechar.

## Classe 2 — worker DAG autojulga sua própria mutação

`P1346-R2A07-forged-dag-worker` fornece somente `case_id`,
`classification=Violated` e `reason_code=AUTHORITY_ROOT`. `_dag_case` aceita
esse stdout sem exigir schema fechado, PID autêntico, hashes original/mutado ou
prova de que um validador rejeitou a mutação.

O case runner canônico também decide T08–T11 por
`sha256(mutated) != sha256(original)`. Isso demonstra mudança de bytes, não
rejeição pelo backward DAG. Assim, uma resposta autoatestada pode substituir a
execução do verificador de autoridade.

## Classe 3 — TOCTOU entre verificação no filho e `execve`

`P1346-R2A16-postcheck-execve-fd-swap` troca o descritor no filho depois de
`_child_exec` confirmar stat, hash e seals, mas antes do syscall real de exec.
O executável adversarial tem SHA-256
`606f06c1fb73f90f35a4f734b0cdc0c730066903b371b030037cc7f061fce1a6`
e emite deliberadamente `process_nonce_hex = 42…42`.

O checker devolveu `Unknown/OPAQUE_PAYLOAD`; sua própria evidência contém os 64
hexadecimais `42`, enquanto `probe_executable_sha256` e
`probe_memfd_sha256` continuam apontando para os bytes legítimos. Isso prova
que o pós-hash do descritor do pai não identifica os bytes executados pelo
filho.

O ataque é válido no modelo adversarial solicitado: ele muta o último elo de
transporte entre a verificação e `execve`. A correção R2 eliminou o swap antes
do fork, mas não tornou atômicos verificação e execução.

## Ataques corretamente rejeitados

- Todos os 27 vetores R1, inclusive propagação de resultados `Preserved`,
  preflight, JSON, delivery DAG, ausência real de memfd/seals e swap antes do
  fork.
- Worker parcial, linha extra e JSON não canônico.
- Escrita, truncamento e crescimento de memfd selado.
- Close/reuse antes da verificação do filho.
- Exceções em `memfd_create`, `F_ADD_SEALS`, `fork` e `execve`.
- Saída parcial do probe.
- Mutação do execution receipt pós-delivery e substituição coordenada do DAG
  contra o trust anchor externo.

## Proveniência

- instante: `2026-09-11T11:11:51-03:00`;
- `HEAD = 2f42d64253547734564513a1159ee6b584c1c4b4`;
- working tree não commitado;
- `sha256(git status --porcelain=v1) =
  638794a38ce4497ee138244ed6e30c4188729822faefa5d39f352f9be1b8f94c`;
- `sha256(git diff HEAD --stat) =
  9783f07bc86c21e8874ebacec9d92ce5a12208e127748377c571b091a1c92e15`;
- stat rastreado: 76 ficheiros, 10.972 inserções, 833 remoções;
- runner SHA-256:
  `1e77f7958c400f280756442fb8a185afce067502ab036bc125aa55d585608240`;
- relatório JSON SHA-256:
  `60de39bf2588fab84953f3a2fefea4cfd842dac205f1b23b669830106ea6bdc6`;
- comando:
  `PYTHONDONTWRITEBYTECODE=1 python3 -B 00_nucleo/diagnosticos/p1346-adversary-runner-r2.py --output 00_nucleo/diagnosticos/p1346-adversary-report-r2.json`;
- temporários: `/dev/shm/p1346-*`;
- execution receipt usado como prova: não;
- candidato lido ou executado: não;
- inputs protegidos alterados: não;
- corpus full: zero.

## Limite

Regime: `executado sem atestacao de isolamento`. O filesystem compartilhado e
o contexto herdado impedem atestação forte. O resultado cobre somente o gate
focal P1346 e bloqueia definitivamente esta cadeia; não sela, não autoriza RED
ou candidato e não afirma equivalência funcional geral.

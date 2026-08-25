# P1179 — auditoria da migração de hashes do linter

**Medição:** 2026-08-25T16:31:36-03:00  
**HEAD:** `ce49041de76eb64c011e990e9774a0339f979211`  
**Estado:** working tree não commitada; seis tracked paths anteriores modificados;
índice vazio  
**Decisão:** **C — regressão do linter; migração global bloqueada**

## Proveniência

O executável medido foi `/home/dikluwe/.cargo/bin/crystalline-lint`, SHA-256
`1265a0d534274c07b9d44fb152507f1c1a9052528236876394b260fecbbfb466`, tamanho
18.205.816 bytes e mtime `2026-08-25 16:18:16.689710851 -0300`. A instalação
local aponta para `crystalline-lint 0.2.0` em
`/repos/Antigravity/tekt-linter`, HEAD
`84fa3006ad6557722cfbe4d10c78c7d0de6b4195`, árvore limpa. Não havia fonte
anterior local identificada; portanto este relatório não atribui um commit
histórico específico como introdutor da regressão.

No início e após os dry-runs, `git status --short` tinha SHA-256
`6af6e3db7e4de881a75810b70bfca9ec7860081475545af0a4581b4da9a0f95b` e o diff
binário contra HEAD tinha SHA-256
`102831d5b95459d10a2287d1fcad50b99221198162381a311000a8b926c4e3c3`.
Logo, a auditoria não alterou a working tree acumulada.

## Dry-run selado e contagens

Duas execuções de `crystalline-lint --fix-hashes --dry-run .` terminaram com
exit 0, stderr vazio e stdout byte-idêntico: 421 linhas, SHA-256
`518825da010e5124649a6287eaaf39f5f6d8c0b30c7cbfcf0d4dc8d762581c78`.
O [manifesto TSV](typst-p1179-manifest-hashes-linter.tsv) contém exatamente os
421 paths, sem duplicatas nem entradas não classificadas.

Os 421 consumers referenciam 336 prompts:

- 22 prompts são compartilhados por 107 consumers;
- 314 consumers têm prompt exclusivo;
- 23 prompts não possuem uma linha canônica `Hash do Código:` no preâmbulo,
  afetando 78 consumers;
- 7 desses prompts reúnem simultaneamente 62 consumers compartilhados;
- na classificação exclusiva do manifesto: 78 `missing_metadata`, 45
  `shared_prompt` restantes e 298 `single_consumer` graváveis.

Todos os 22 prompts compartilhados receberam um `hash-b` diferente para cada
consumer. Assim, não existe um único valor de código que a metadata escalar do
prompt possa representar.

Nenhum dos seis tracked paths já modificados apareceu no dry-run: os dois
consumers HTML já estavam corretamente selados. Não houve warning V5 sem ação
proposta e o plano só propôs headers de linhagem/fontes e metadata dos prompts;
nenhuma alteração funcional foi proposta.

## O que são `hash-a`, `hash-b` e os 421 avisos

Na implementação instalada, `hash-a` é SHA-256 truncado a oito hexadecimais do
prompt L0 depois de remover a linha canônica `Hash do Código:`
(`03_infra/prompt_reader.rs:24-34`). Essa remoção evita um ciclo de hash.
`hash-b` é o SHA-256 truncado do consumer, ignorando a própria linha
`@prompt-hash` (`02_shell/fix_hashes.rs:24-35`).

Os 421 avisos são, portanto, 421 headers `@prompt-hash` que não coincidem com o
L0 recalculado pela versão atual do linter. Não são 421 falhas funcionais nem
421 erros de compilação; V5 está configurado como warning, razão pela qual o
lint comum termina com exit 0.

Triangulação independente confirmou, entre outros:

- `eval/bibtex.rs`: `hash-a=7dca55a1`, `hash-b=a1887dad`;
- `eval/bibliography.rs`: `hash-a=2dcd02da`, `hash-b=fcd1d272`, mas o prompt
  `compiler/eval.md` não tem metadata canônica;
- `stdlib/html.rs`: L0 `19720544`, igual ao header atual, e código `948866a7`,
  igual à metadata atual do prompt;
- `export/html.rs`: L0 `ba8a0c7e`, igual ao header atual, e código `b3a7ba69`,
  igual à metadata atual do prompt.

## Reprodução mínima e idempotência enganosa

Uma fixture isolada em `/tmp/p1179-fixture` continha um prompt exclusivo, um
prompt compartilhado por dois consumers e um prompt sem metadata. A primeira
aplicação terminou com exit 0 e informou:

- um `Partial write` no prompt sem metadata: o header do source foi escrito
  antes de a gravação do prompt falhar;
- dois `Applied` sucessivos no prompt compartilhado, deixando como metadata o
  `hash-b=5c40564a` do último consumer e descartando `3af314e0` do primeiro;
- um `Applied` correto no caso exclusivo.

A segunda passagem respondeu `Nothing to fix`, e
`--checks v5 --fail-on warning` respondeu `No violations found`, ambos com exit
0. Mesmo assim, o prompt sem metadata continuou incompleto e o prompt
compartilhado reteve arbitrariamente apenas o último consumer. A aparente
idempotência é, portanto, um falso fechamento.

A causa direta está em `02_shell/fix_hashes.rs:197-220`: o migrador grava o
header primeiro e a metadata depois, sem rollback. A porta documenta “inject”
(`:34-35`), mas `03_infra/prompt_io.rs:208` exige exatamente uma ocorrência e
não insere uma ausente. O plano também é produzido por violação/consumer
(`fix_hashes.rs:115-162`), sem agrupar prompts compartilhados.

Refutadores: a classificação C seria refutada por uma versão que (1) modele
explicitamente múltiplos consumers ou rejeite-os antes de escrever, (2) insira
metadata ausente ou rejeite todo o lote antes de qualquer write, (3) faça a
operação atômica/rollback e (4) tenha teste de segunda passagem que valide
também a metadata bidirecional, não apenas ausência de V5.

## Decisão e alcance

A migração não deve ser aplicada, nem parcialmente. A correção pertence ao
contrato e à implementação de `fix-hashes` no repositório do linter; este passo
não altera L0 ou código do `typst-crystalline`. Depois da correção e reinstalação,
P1179 deve ser repetido desde o dry-run.

Os 732 paths restaurados em P1178.1 foram o subconjunto externo ao escopo HTML
que uma execução mutante anterior chegou a tocar antes desta auditoria. A
proveniência imediatamente anterior provava-os limpos, permitindo restaurá-los
sem sobrescrever os seis tracked paths acumulados. Os 421 atuais são o conjunto
de divergências remanescente visto depois dessa restauração; os números medem
momentos e conjuntos distintos, não uma contradição.


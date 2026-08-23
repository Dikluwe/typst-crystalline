# P1137 — baseline inicial da matriz integral

## Proveniência

- Medição: `2026-08-23T13:22:33-03:00` (baseline final repetida após correções RED do harness).
- HEAD: `781b207b4a5de9c2bfbe5819918a193d1d9293e5`.
- Working tree: **não commitada**. No início da execução, `git diff HEAD --stat` registrou `225 files changed, 9605 insertions(+), 4486 deletions(-)`; havia ainda ficheiros não rastreados. A enumeração reproduzível do estado é `git status --short` e `git diff HEAD --stat`. Nenhum destes números fecha paridade.
- Vanilla ratificado: `upstream/main a51e02804`; `/repos/Antigravity/typst-crystalline/lab/typst-original/target/release/typst`; string observada `typst 0.15.1 (e0e8ca4d)`.
- Cristalino: `/repos/Antigravity/typst-crystalline/target/release/typst`; string observada `typst 0.15.0 (781b207b)`; `0.15.0` é erro por esquecimento e não alvo deliberado.
- Comando: `python3 lab/parity/matrix/runner.py --output /tmp/p1137-results.json`.

## Resultado por estado e eixo

Denominador explícito: **10 casos**. Resultado: **4 MATCH, 2 DIFF, 3 ABSENT, 0 PARTIAL, 0 ERROR, 1 UNMEASURED**.

| ID | Eixo | Estado | Classe/observação |
|---|---:|---|---|
| P1137-S-001 | S | MATCH | ambos rejeitam a entrada inválida; mensagem/span ainda não comparados |
| P1137-E-001 | E | DIFF | `LANGUAGE_SEMANTICS`: `sys.version` ligado a `PARITY_VERSION` está esquecido em 0.15.0 vs 0.15.1 |
| P1137-B-001 | B | ABSENT | `LANGUAGE_SEMANTICS`: vanilla avalia `calc.gcd`; CLI cristalina não expõe `eval` |
| P1137-I-001 | I | ABSENT | `LANGUAGE_SEMANTICS`: vanilla produz query JSON; CLI cristalina não expõe `query` |
| P1137-L-001 | L | MATCH | ambos produzem PDF; não afirma igualdade visual/geométrica |
| P1137-X-001 | X | MATCH | ambos produzem SVG; help cristalino contradiz a capacidade real |
| P1137-X-002 | X | ABSENT | `PUBLIC_FORMAT`: vanilla produz HTML com `--features html`; cristalino rejeita o formato |
| P1137-X-003 | X | MATCH | ambos produzem PNG; help cristalino contradiz a capacidade real |
| P1137-C-001 | C | DIFF | `PUBLIC_CLI`: help, comandos, opções e defaults diferem |
| P1137-C-002 | C | UNMEASURED | package `@preview`: falta fixture local; rede não foi usada |

Lista integral não-MATCH: `P1137-E-001`, `P1137-B-001`, `P1137-I-001`, `P1137-X-002`, `P1137-C-001`, `P1137-C-002`. Não houve `PARTIAL` nem `ERROR`.

## RED → GREEN do harness

O primeiro RED encontrou manifesto inválido (`classe=null` num item `UNMEASURED`). O segundo RED detectou opção global vanilla na posição errada, diretórios de artefatos inexistentes, formato `repr/raw` inadequado e fixture de counter dependente de contexto. Depois das correções, 7 testes unitários passam e todas as expectativas das sentinelas coincidem. Divergências esperadas continuam `DIFF`/`ABSENT` e não fazem o runner falhar.

## Limitações

- `MATCH` de layout/export nesta baseline comprova somente sucesso e presença do artefato, não bytes, geometria, morfologia ou pixels.
- Diagnóstico sintático mede somente exit code; mensagem e span ficam no denominador futuro.
- `sys.version` usa o `--version` como leitura da mesma constante pública, comprovada em `02_shell/src/cli.rs:84-96` e `01_core/src/entities/version.rs:22`; o hash impresso não prova revisão.
- Não há ainda DTO tipado para eval, extração integral de scopes/args/defaults, comparação de árvores SVG/HTML, PDF semântico, raster ou geometria.
- A árvore já estava extensamente modificada. Nenhum código L1–L4 foi alterado por P1137.

## Prioridade seguinte

1. `P1137-E-001` — semântica pública incorreta por versão esquecida.
2. `P1137-B-001` e `P1137-I-001` — contratos existentes internamente mas sem superfície CLI comparável.
3. `P1137-X-002` — HTML ausente.
4. `P1137-C-001` — CLI pública e help; inclui corrigir as descrições falsas de PNG/SVG.
5. `P1137-C-002` — construir fixture local antes de medir packages.

## Gates finais

- `python3 -m unittest lab/parity/matrix/test_runner.py`: **PASS**, 7 testes.
- `python3 lab/parity/matrix/runner.py --output /tmp/p1137-results.json`: **PASS**, todas as expectativas das 10 sentinelas coincidiram.
- `crystalline-lint .`: **PASS**, exit code 0; emitiu avisos não bloqueantes da árvore de produção preexistente.
- `git diff --check`: **PASS**.
- `cargo test --manifest-path lab/parity/Cargo.toml`: **FAIL**, depois de corrigidas as referências obsoletas do harness e o denominador antigo de 46 para 47 ficheiros. Resultado relevante: 43/50 testes de `parse_parity` passaram e 7 falharam (`espaco_simples`, `lista_bullets`, `parbreak`, `texto_simples`, `strong_nested`, `parity_equacao_inline_em_texto`, `corpus_completo`). O vanilla agrega espaços dentro de folhas de texto onde o cristalino produz folhas `space` separadas. Isto é divergência morfológica real; não foi normalizada nem corrigida em L1–L4 porque P1137 proíbe correções de produção durante a baseline.

Consequentemente, a infraestrutura e a baseline P1137 estão executadas, mas o checkbox de suíte Cargo integral permanece aberto até um passo de correção de paridade com auditoria L0 própria.

## Adendo de correção 1–2 — 2026-08-23

Por confirmação do dono, `0.15.0` era esquecimento: o cristalino acompanha a versão pública do vanilla ratificado `a51e02804`. Foram corrigidos `sys.version`/`--version` para `0.15.1` e a morfologia lexical de espaços internos entre caracteres alfanuméricos. O lexer agora absorve esses espaços na mesma folha `Text`; pontuação e tabulação continuam a produzir trivia separada.

Proveniência da remedição:

- Medição: `2026-08-23T14:47:59-03:00`.
- HEAD: `781b207b4a5de9c2bfbe5819918a193d1d9293e5`.
- Working tree: **não commitada**; `git diff HEAD --stat` registrou `253 files changed, 9864 insertions(+), 4636 deletions(-)`, além de ficheiros não rastreados. Este número identifica o estado amplo da árvore, não atribui todos os ficheiros a esta correção.
- Binário cristalino reconstruído: `typst 0.15.1 (781b207b)`.
- Runner: **5 MATCH, 1 DIFF, 3 ABSENT, 0 PARTIAL, 0 ERROR, 1 UNMEASURED**. `P1137-E-001` passou de `DIFF` para `MATCH`; os demais estados permaneceram iguais.

Gates após as correções:

- `cargo build --release --bin typst`: **PASS**.
- `cargo test --manifest-path lab/parity/Cargo.toml --quiet`: **PASS** — grupos observados: 11, 1, 1, 50, 34 e 2 testes; zero falhas.
- `python3 -m unittest lab/parity/matrix/test_runner.py`: **PASS**, 7 testes.
- `python3 lab/parity/matrix/runner.py --output /tmp/p1137-results-after.json`: **PASS**, expectativas das 10 sentinelas coincidiram.
- `crystalline-lint .`: **PASS**, exit code 0; avisos não bloqueantes permanecem.
- `git diff --check`: **PASS**.

O antigo `FAIL` do gate Cargo acima fica preservado como baseline histórica; este adendo é o estado vigente após as correções 1–2. A prioridade seguinte começa agora em `P1137-B-001`/`P1137-I-001`.

## Adendo `P1137-B-001` — `typst eval`

Em `2026-08-23T15:11:21-03:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada
(`259 files changed, 10346 insertions(+), 4657 deletions(-)`, além de não
rastreados), o subcomando `eval` foi ligado para expressão code isolada.

- `calc.gcd(12, 18)` em JSON produz `6\n` nos dois binários;
- dict/array usa JSON estrutural; raw string preserva bytes sem newline;
- raw sobre inteiro e expressão sintaticamente inválida retornam exit 1;
- a invocação legada de compilação continua produzindo PDF;
- a matriz passa a **6 MATCH, 1 DIFF, 2 ABSENT e 1 UNMEASURED**.

`P1137-B-001` passou de `ABSENT` para `MATCH`. `P1137-I-001` (`query`)
permanece `ABSENT` e torna-se a próxima prioridade.

## Adendo `P1137-I-001` — `typst query`

Em `2026-08-23T15:37:50-03:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada
(`260 files changed, 10655 insertions(+), 4659 deletions(-)`, além de não
rastreados), o subcomando deprecated `query` foi exposto para headings.

- objeto heading completo coincide byte a byte no stdout com o vanilla;
- `--field level` produz `[1]\n` e `--one` produz o objeto sem array;
- a ordem vem de `Introspector::query_by_kind` e os elementos reais de
  `Introspector::element_at`;
- tipos de conteúdo ainda não serializados falham explicitamente.

A matriz passa a **7 MATCH, 1 DIFF, 1 ABSENT e 1 UNMEASURED**.
`P1137-I-001` passou de `ABSENT` para `MATCH`.

## Adendo `P1137-X-002` — HTML semântico inicial

Em `2026-08-23T18:51:57.264770+00:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada
(`git diff HEAD --stat`: `260 files changed, 10761 insertions(+), 4663
deletions(-)`, além de não rastreados), foi ligado o primeiro subconjunto do
backend HTML sem passar pelo layout paginado.

- o ADR-0128 e os L0 de eval, CLI, pipeline, exporter e wiring precedem o código;
- `EvalTarget::Html` chega à avaliação e `target()` distingue `html` de `paged`;
- `.html` e `--format html` selecionam o pipeline semântico experimental;
- a fixture `plain.typ` produz 175 bytes, byte-idênticos ao vanilla ratificado;
- ambos os artefactos têm SHA-256
  `c8049e71950f48ba1b3cbcdd0dd6ca75a6357a762dd6a3056670f75a73b35f92`;
- o subconjunto continua explicitamente incompleto: texto/parágrafo, heading,
  strong, emph, linebreak e escape; variantes restantes falham com diagnóstico.

Gates: 2 testes do exporter HTML e 1 teste da resolução do formato passaram;
build release passou; `P1137-X-002` passou como `MATCH`; `crystalline-lint .`
passou com exit 0 e apenas avisos não bloqueantes. A matriz integral passa a
**8 MATCH, 1 DIFF e 1 UNMEASURED**, sem casos `ABSENT`.

## Adendo `P1137-C-002` — package local sem rede

Em `2026-08-23T18:56:48.821844+00:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada
(`git diff HEAD --stat`: `260 files changed, 10817 insertions(+), 4663
deletions(-)`, além de não rastreados), foi adicionada uma fixture versionada
`@preview/example:0.1.0`.

O runner injeta `TYPST_PACKAGE_PATH` somente no vanilla e `XDG_DATA_HOME`
somente no cristalino, apontando ambos para o mesmo conteúdo local. Nenhuma
rede é usada. Os dois processos terminaram com exit 0, stdout/stderr vazios e
artefacto PDF presente. Os 8 testes do runner passaram.

`P1137-C-002` passa de `UNMEASURED` para `MATCH`. A matriz integral fica em
**9 MATCH e 1 DIFF**; o único resíduo é `P1137-C-001`.

Para `C-001`, a medição mostrou diferença pública de parsing e estrutura, não
apenas texto editorial. Foi redigida atualização L0 em `shell/cli.md` e
`wiring.md`; por alterar contrato público, ela aguarda confirmação obrigatória
do dono antes de código (ADR-0127). Os hashes não foram ressellados enquanto
este gate estiver aberto.

## Adendo `P1137-C-001` — `compile` canónico e help estrutural

Após confirmação explícita do dono, o L0 foi ressellado e implementado. Na
remedição de `2026-08-23T19:01:54.458272+00:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, a working tree permanecia não
commitada (`git diff HEAD --stat`: `260 files changed, 10908 insertions(+),
4668 deletions(-)`, além de não rastreados).

- `typst compile INPUT OUTPUT` é a grafia pública canónica;
- `typst c INPUT OUTPUT` é alias funcional;
- a grafia histórica `typst INPUT OUTPUT` continua funcional por tradução
  anterior ao clap, mas deixou de poluir o help principal;
- `query` continua funcional por compatibilidade, porém oculto do help;
- as opções de compilação passaram ao help de `compile`; `--color` permanece
  global;
- comandos ainda ausentes não são anunciados por stubs.

A sentinela passou a comparar inventário estrutural, não stdout literal: o
hash/identidade próprios e wrapping editorial não são igualdade mecânica
exigida. O estado melhora de `DIFF` para `PARTIAL`, pois `watch`/`w`, `init`,
`fonts`, `completions`, `info` e `--cert` ainda não existem.

Gates: 2 testes unitários de normalização, 2 testes de integração específicos,
os **39 testes** integrais do CLI, build release, as três grafias de compilação,
os **9 testes** do runner, `git diff --check` e `crystalline-lint .` passaram.
A matriz fica em **9 MATCH e 1 PARTIAL**, sem `DIFF`, `ABSENT`, `ERROR` ou
`UNMEASURED`.

## Adendo `P1137-C-001` — `fonts` e `completions`

Em `2026-08-23T19:15:01.284720+00:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, working tree não commitada
(`git diff HEAD --stat`: `263 files changed, 11145 insertions(+), 4671
deletions(-)`, além de não rastreados), foram ligados mais dois comandos reais:

- `typst fonts`, com `--font-path`, `--ignore-system-fonts` e `--variants`,
  reutilizando descoberta embutida/sistema/projecto de L3 e formatter L2;
- `typst completions` para Bash, Elvish, Fish, PowerShell e Zsh, gerado da
  própria árvore clap. A medição corrigiu o L0: o vanilla também mantém o
  `query` oculto nas tabelas internas da completion Bash.

Gates: **43 testes** integrais do CLI, testes unitários dos dois formatters,
build release, runner (**9 testes**), matriz integral, `git diff --check` e
`crystalline-lint .` passaram. A matriz permanece **9 MATCH e 1 PARTIAL**;
o resíduo de C-001 agora é `info`, `--cert`, `init` e `watch`.

## Adendo final — fechamento de `P1137-C-001` e do Passo 1137

Remedição em `2026-08-23T21:17:03.141776+00:00`, HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5`, com working tree não
commitada (`git diff HEAD --stat` no registro final: `271 files changed,
12130 insertions(+), 4697 deletions(-)`, além de ficheiros não rastreados).

Os comandos `info`, `--cert`, `init` e `watch`/`w` foram materializados após os
adendos anteriores. A remedição estrutural do help mediu o mesmo conjunto de
comandos e opções públicas relevante nos dois binários; identidade própria e
wrapping editorial continuam excluídos por serem mecânica, conforme ADR-0107.
`P1137-C-001` passou de `PARTIAL` para `MATCH`.

Resultado final do denominador explícito: **10 casos — 10 MATCH, 0 DIFF,
0 ABSENT, 0 PARTIAL, 0 ERROR e 0 UNMEASURED**. Não existe item não-MATCH a
listar nesta execução. Isto fecha o denominador inicial; não afirma paridade
integral de geometria, raster, PDF bruto ou de todo o corpus da linguagem.

Proveniência dos binários:

- vanilla ratificado `a51e02804`:
  `lab/typst-original/target/release/typst`, saída observada
  `typst 0.15.1 (e0e8ca4d)`;
- cristalino no estado acima: `target/release/typst`, saída observada
  `typst 0.15.1 (781b207b)`.

Gates finais:

- `cargo build --release --bin typst`: **PASS**;
- `python3 -m unittest lab/parity/matrix/test_runner.py`: **PASS**, 9 testes;
- `python3 lab/parity/matrix/runner.py --validate-only`: **PASS**, 10 casos;
- runner integral para `/tmp/p1137-results-final.json`: **PASS**, 10 MATCH;
- `cargo test --manifest-path lab/parity/Cargo.toml --quiet`: **PASS** — grupos
  de 11, 1, 1, 50, 34 e 2 testes, zero falhas;
- `crystalline-lint .`: **PASS**, zero violações bloqueantes;
- `git diff --check`: **PASS**.

Com estes gates, os critérios de aceitação do Passo 1137 ficam integralmente
satisfeitos. INFO-2, expansão do HTML e comparadores profundos permanecem
frentes futuras independentes, não resíduos do denominador inicial.

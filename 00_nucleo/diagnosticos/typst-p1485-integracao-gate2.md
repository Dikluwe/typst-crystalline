# P1485 — integração segregada e gate de continuidade global

**Estado:** working tree não commitada sobre HEAD
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`.
Snapshot de 2026-09-24T00:08:11-03:00:
`git diff HEAD --binary | sha256sum` =
`8606b8a690576b373d2ec27224ee0a6b119890d4ebe1e2d86c00ab042f96a026`.
A lista exata de arquivos está no fim deste diagnóstico. Este próprio
diagnóstico foi criado depois do snapshot e não fazia parte dele.
Vanilla pinado `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
`/usr/local/bin/typst` SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## Protocolo A/B/C

A e B partiram do mesmo HEAD e do mesmo L0 confirmado, em worktrees
isolados `/tmp/p1485-agent-a.gLdZ5b` e
`/tmp/p1485-agent-b.gq0srH`. A alterou apenas produção; B alterou
apenas testes e mediu fixtures no vanilla sem consultar o código A.
C integrou os diffs, executou os RED/GREEN e fez três mutações em
cópia descartável em `/dev/shm/p1485-executor.0jgjDZ/mutant`,
com target separado para evitar falso sobrevivente por binário em cache.

## Medições decisórias

| Prova | Resultado |
|---|---|
| RED baseline B, CLI `p1485_` | 5/5 falharam sem bundle |
| RED baseline B, core `p1485_` | 2/2 falharam sem asset |
| GREEN core completo | 6140/6140 |
| GREEN shell completo | 84/84 |
| GREEN infra, excluído teste lento P1431 | 1051/1051 |
| GREEN CLI com `TYPST_COMMIT_SHA` fixado, excluído o gate P1485.2 | 85/85 |
| B bundle integrado, incluindo PDF, SVG, PNG, HTML, assets, watch/HTTP, frame e query | 8/9; único RED: numeração entre PDFs |
| Mutações dirigidas em RAM | 3/3 mortas: duplicata, asset fora do bundle, perda de payload |
| `cargo build` e `crystalline-lint .` | sucesso; zero violações |
| Teste P1431 de rasterização isolado | timeout após 180 s; não usado como prova de regressão P1485 |

O teste `p1485_bundle_heading_numbering_continua_entre_pdfs` usa
`#set heading(numbering: "1.")`, `document("a.pdf")[= Alpha]` e
`document("b.pdf")[= Beta]`. `pdftotext` extrai `1. Alpha`/
`2. Beta` no vanilla e `1. Alpha`/`1. Beta` no cristalino.
A comparação é do texto da linguagem, não dos bytes PDF.
A falha ocorre porque a introspecção percorre todos os bodies
com um `Locator` global, mas cada layout reinicia `Locator::new()`.
O caminho de estabilização contextual selecionada também
reintrospecta somente um body; sua paridade cruzada ainda exige
prova. P1485 não está encerrado.

O primeiro ensaio de mutação usou o target compartilhado e retornou
falso sobrevivente porque o binário de teste não foi reconstruído
com a cópia mutada. O ensaio decisório usou
`CARGO_TARGET_DIR=/dev/shm/p1485-executor.0jgjDZ/target-mutant`,
`CARGO_INCREMENTAL=0` e debug 0; os três mutantes então falharam
pelos asserts esperados. Não se atribui crédito ao ensaio com cache.

A primeira execução da suíte CLI ampla encontrou
`p1137_info_json_expoe_schema_e_commit_integral` vermelho porque
o build local tinha `build.commit: null`. Reexecutar com
`TYPST_COMMIT_SHA=bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`
fixado, conforme a receita de proveniência do repositório, deu
85/85 verde (com apenas o gate de numeração filtrado).
Não é regressão do bundle.

## Gate ADR-0127 P1485.2

A proposta L0 foi redigida e selada, **sem código funcional novo**
para seed/layout/contexto. Hashes SHA-256 completos:

```text
entities/locator.md                           5ce899ea09e45d6c11eca8156336106e769a453d927b49e21b309366c8225405
compiler/introspect.md                        bfc8b1e13b2166c250d0b1fd2ea15721857f6575bcdd25270b5eb04d6c9c74c1
compiler/layout.md                            14de7aa87256744cdc47e9d108c8a67a75003cbcd6f6fe5402d0a6e5868ea5ff
infra/pipeline/context_stabilization.md      b87c127dfd1112eeceda1a26804a12ae92291888af121230578500a1dfa168cc
infra/pipeline.md                            2fa2c70155ecb4796579616a27624f5142098e14595ccb18c538201ea229037a
```

O contrato proposto acrescenta `LocatorSeed`/checkpoint,
introspecção global com inícios por body e entrypoint de layout
com seed; a coordenação L3 revalida contextos no conjunto inteiro
antes de exportar. Como há APIs públicas L1 novas, ADR-0127 exige
confirmação humana depois destes L0s e antes do código. A
aceitação inclui numeração `1. Alpha`/`2. Beta`, queries/refs/
counters/state entre documentos, HTML/Paged mistos e publicação
sem arquivos parciais. Não converter o RED em scope-out.

## Snapshot Git anterior a este diagnóstico

```text
 M 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/eval/bindings/field_access.md
 M 00_nucleo/prompts/compiler/eval/tests.md
 M 00_nucleo/prompts/compiler/introspect.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/stdlib/format.md
 M 00_nucleo/prompts/compiler/stdlib/structural.md
 M 00_nucleo/prompts/compiler/stdlib/structural/document.md
 M 00_nucleo/prompts/entities/compiler_features.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/locator.md
 M 00_nucleo/prompts/infra/integration_tests.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/shell/cli.md
 M 00_nucleo/prompts/shell/watch.md
 M 00_nucleo/prompts/wiring.md
 M 00_nucleo/prompts/wiring/tests/cli.md
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/mod.rs
 M 01_core/src/compiler/eval/repr.rs
 M 01_core/src/compiler/eval/rules.rs
 M 01_core/src/compiler/eval/tests.rs
 M 01_core/src/compiler/introspect.rs
 M 01_core/src/compiler/layout/mod.rs
 M 01_core/src/compiler/stdlib/format.rs
 M 01_core/src/compiler/stdlib/foundations/query.rs
 M 01_core/src/compiler/stdlib/structural/document.rs
 M 01_core/src/compiler/stdlib/structural/mod.rs
 M 01_core/src/entities/compiler_features.rs
 M 01_core/src/entities/content.rs
 M 01_core/src/entities/locator.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/integration_tests.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/watch.rs
 M 04_wiring/src/main.rs
 M 04_wiring/tests/cli.rs
?? 00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
?? 00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
?? 00_nucleo/materialization/typst-passo-1484.md
?? 00_nucleo/materialization/typst-passo-1485.md
?? 00_nucleo/materialization/typst-passo-1486.md
?? 00_nucleo/materialization/typst-passo-1487.md
?? 00_nucleo/materialization/typst-passo-1488.md
```
## Continuação P1485.2 e novo gate P1485.3 — 2026-09-24 04:26:50 UTC

O dono confirmou o primeiro gate P1485.2. Foram implementados checkpoints
de Locator, seed de layout, introspecção global dos bodies, estabilização
contextual de bundle e leitura lógica de `counter(page).get/at` em Paged.
Os ensaios concluídos antes do último RED: bundle 15/15, L1 6142/6142,
infra lib 1051/1051 (um teste raster P1431 preexistente filtrado) e contrato
Rustdoc watch 1/1. A corrida infra agregada encontrou inicialmente
6/7 em `p1297_watch_capability_contract`; o allowlist foi corrigido e o
teste focal passou. Não tratar o 6/7 histórico como verde.
Logs em `/dev/shm/p1485-gate2.kDyg70/`:
`bundle-final-candidate.log`, `core-shell-infra.log`, `watch-contract.log`.

Nova sonda RED `04_wiring/tests/cli.rs:3003`:
`p1485_bundle_html_nao_herda_page_counter_do_pdf`. PDF faz
`counter(page).update(5)`; HTML seguinte faz `counter(page).get()`.
O teste em `red-html-page.log` terminou 0/1, com body
`<body><p>5</p></body>` em vez de 1. Output manual do mesmo fonte:
vanilla ratificado `586e1bd43` deu `<body><p>1</p></body>`;
cristalino deu `<body><p>5</p></body>`.
Em `pagehtmlupdate.typ`, update 7 no HTML vanilla deu
`<body><p>7</p></body>`. Arquivos de prova:
`/dev/shm/p1485-gate2.kDyg70/pagehtml.typ`,
`/dev/shm/p1485-gate2.kDyg70/pagehtmlupdate.typ`,
`vanilla-pagehtml/two.html`, `crystalline-pagehtml/two.html`,
`vanilla-pagehtmlupdate/h.html`. Mtimes locais -03:00:
vanilla base 2026-09-24 01:11:37, update 01:12:48,
RED 01:14:16. O estado do código das três medições foi working tree
não commitada sobre o HEAD abaixo; entre elas só variaram fixtures/sondas,
não a implementação produtiva de Page. A leitura manual confirma o RED,
não autoriza copiar um bug vanilla não medido.

Causa: introspecção pura recebe os bodies e seus starts; introspecção
runtime recebe uma Sequence sem fronteiras. Um reset de Page isolado
também não basta para `CounterUpdate::Func`, que hoje acrescenta
snapshots no pós-pass. Os L0s P1485.3 propõem uma entrypoint pública
L1 com bodies e starts, e replay Page em ordem causal. Seis L0s foram
redigidos antes do código; `crystalline-lint --fix-hashes .` reportou
0 drift e `crystalline-lint --quiet .` retornou 0. Só os headers de
linhagem foram alterados pelo resselo. Hashes SHA-256 brutos dos L0s:

```text
compiler/introspect.md                        1450fb5482fc934b2c4950b1d24e034cd7711978d527c6f48344a62098353a99
compiler/introspect/from_tags.md              f176d7443f5005d81040105baf9fc1371624379ac18f3769cf3c2487db18bed9
compiler/stdlib/counter.md                    376d80be8a249c822235f473e457ea4eb5b163f7a94725f4cd5c3af5682e694a
entities/counter_registry.md                  8e62e42e860f4c120dbd6146444b4350afeb781fdc722b34fc66c05f2e3b1f76
infra/pipeline.md                             1f387291bc61f26b24fc74eb3c21937656f945eb4510d85e69e514e83ab96f6f
infra/pipeline/context_stabilization.md       7417753d489bf2c32f09be947203bf8d188c397229a67598ebaf56488dd8dcec
```

`counter(page).final()` multi-documento continua fora desta emenda;
não declarar P1485 fechado. Nenhum código funcional da nova entrypoint
foi escrito; novo gate ADR-0127 aguarda confirmação humana. Não houve commit.

Proveniência do snapshot acima: HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`;
`git diff HEAD --stat` em 2026-09-24 04:26:50 UTC:

```text
 .../typst-cobertura-vanilla-vs-cristalino.md       |   8 +-
 00_nucleo/prompts/compiler/eval.md                 |  73 ++-
 .../prompts/compiler/eval/bindings/field_access.md |  53 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  45 +-
 00_nucleo/prompts/compiler/introspect.md           | 112 ++++-
 00_nucleo/prompts/compiler/introspect/from_tags.md |  27 +-
 00_nucleo/prompts/compiler/layout.md               |  47 +-
 00_nucleo/prompts/compiler/stdlib/counter.md       |  47 +-
 00_nucleo/prompts/compiler/stdlib/format.md        |  34 ++
 00_nucleo/prompts/compiler/stdlib/structural.md    |  24 +-
 .../prompts/compiler/stdlib/structural/document.md |  88 +++-
 00_nucleo/prompts/entities/compiler_features.md    |  33 +-
 00_nucleo/prompts/entities/content.md              |  38 +-
 00_nucleo/prompts/entities/counter_registry.md     |  23 +
 00_nucleo/prompts/entities/locator.md              |  41 +-
 00_nucleo/prompts/infra/integration_tests.md       |  49 +-
 00_nucleo/prompts/infra/pipeline.md                | 162 +++++-
 .../infra/pipeline/context_stabilization.md        |  93 +++-
 .../infra/tests/p1297_watch_capability_contract.md |  28 +-
 00_nucleo/prompts/shell/cli.md                     |  72 ++-
 00_nucleo/prompts/shell/watch.md                   |  75 +++
 00_nucleo/prompts/wiring.md                        |  60 ++-
 00_nucleo/prompts/wiring/tests/cli.md              |  26 +
 01_core/src/compiler/eval/bindings/field_access.rs |  34 +-
 01_core/src/compiler/eval/mod.rs                   |  46 +-
 01_core/src/compiler/eval/repr.rs                  |  10 +-
 01_core/src/compiler/eval/rules.rs                 |   5 +-
 01_core/src/compiler/eval/tests.rs                 | 113 ++++-
 01_core/src/compiler/introspect.rs                 | 115 ++++-
 01_core/src/compiler/introspect/from_tags.rs       |   2 +-
 01_core/src/compiler/layout/mod.rs                 |  41 +-
 01_core/src/compiler/stdlib/counter.rs             |  42 +-
 01_core/src/compiler/stdlib/format.rs              |  32 +-
 01_core/src/compiler/stdlib/foundations/query.rs   |   1 +
 01_core/src/compiler/stdlib/structural/document.rs | 174 ++++---
 01_core/src/compiler/stdlib/structural/mod.rs      | 172 ++++---
 01_core/src/entities/compiler_features.rs          |  21 +-
 01_core/src/entities/content.rs                    |  44 +-
 01_core/src/entities/counter_registry.rs           |   2 +-
 01_core/src/entities/locator.rs                    |  27 +-
 02_shell/src/cli.rs                                |  80 ++-
 03_infra/src/integration_tests.rs                  |  31 +-
 03_infra/src/pipeline.rs                           | 455 ++++++++++++++++-
 03_infra/src/pipeline/context_stabilization.rs     | 281 ++++++++++-
 03_infra/src/watch.rs                              | 283 ++++++++++-
 03_infra/tests/p1297_watch_capability_contract.rs  |   4 +-
 04_wiring/src/main.rs                              | 162 +++++-
 04_wiring/tests/cli.rs                             | 551 ++++++++++++++++++++-
 48 files changed, 3679 insertions(+), 307 deletions(-)
```

Não rastreados no mesmo instante (`git ls-files --others --exclude-standard`):

```text
00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1487.md
00_nucleo/materialization/typst-passo-1488.md
```


## P1485.3 após confirmação — integração final (2026-09-24 05:25:08 UTC)

Estado medido: working tree **não commitada** sobre
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`. A e B receberam o
mesmo L0, escreveram produção/testes em arquivos separados; C integrou.
A correção inicial passou 21/22; o RED restante mostrava Page Func no PDF
como 1 em vez de 5. A correção focal usa o estado Page runtime resolvido
do próprio body para a numeração paginada, sem reexecutar o callback.
A sequência final, após essa correção, foi:

- bundle 22/22, log concluído 2026-09-24 02:08:17 -03:00;
- L1 6143/6143, 02:10:28 -03:00;
- L3 1051/1051, com um único teste raster P1431 lento filtrado,
  02:12:16 -03:00;
- CLI integral 99/99, 02:13:31 -03:00;
- `cargo build -q`, `crystalline-lint --quiet .` e
  `git diff --check` retornaram 0 no mesmo working tree.

Mutation testing dirigido, baseline nos mesmos casos e mesmas flags,
com cópia de fonte em RAM separada e o mesmo cache descartável:
**3 mutantes mortos/3 tentados**. Reinício Page em 9 foi rejeitado por
`p1485_bundle_html_nao_herda_page_counter_do_pdf` (HTML 9 contra 1,
02:17:20 -03:00); reset adicional indevido de `counter("shared")`
foi rejeitado por `p1485_bundle_body_vazio_reinicia_so_page`
(HTML 0 contra 4, 02:19:50 -03:00); engolir erro de Page Func foi
rejeitado por `p1485_bundle_page_func_erro_impede_publicacao_parcial`
(status 0 contra falha requerida, 02:21:44 -03:00). Não houve mutante
sobrevivente. As cópias e logs RAM são descartáveis e são removidos após
transcrever os observáveis; não entram no Git.

Os L0s P1485.3 tiveram gate ADR-0127 confirmado pelo dono e
`crystalline-lint --fix-hashes .` terminou com zero drift antes da
implementação. SHA-256 bruto no snapshot final:

```text
19977229fe3eaa615f3ff408b4c3ad3001589f90930a6fa82e3e8f07848e1a16  00_nucleo/prompts/compiler/introspect.md
ce580c45284df1348226e6da3bbe3565e66fbec0f99f8aba6a83152de78a6eed  00_nucleo/prompts/compiler/introspect/from_tags.md
197930b23cf6bfa56382a7762059501732c69b62e81a92f76182df4d84efc701  00_nucleo/prompts/compiler/stdlib/counter.md
2059565fffcf3df862683d83bc6586cbfc7ff46397fb8fbfb9001e3d97cb5364  00_nucleo/prompts/entities/counter_registry.md
a6dced64efefc1e3fe96567096047b2001a9874028e6f33ebe338b424eb3bfbd  00_nucleo/prompts/infra/pipeline.md
d9c42175c0932e7e9a0bec0dd92e7d66174ba72fc5196f3d885672ca08c85591  00_nucleo/prompts/infra/pipeline/context_stabilization.md
```

A correção é semântica própria: reset só de Page por body, demais
contadores/labels/headings globais, Funções em ordem causal, HTML sem
render de CounterUpdate apenas depois da introspecção. Não se declarou
paridade por igualdade de bytes ou mecânica vanilla. O contrato
`counter(page).final()` multi-documento permanece scope-out explícito
de `compiler/stdlib/counter.md`; nenhuma sonda acima o cobre.

Proveniência exata do snapshot final: `git diff HEAD --stat` em
2026-09-24 05:25:08 UTC:

```text
 .../typst-cobertura-vanilla-vs-cristalino.md       |   8 +-
 00_nucleo/prompts/compiler/eval.md                 |  73 ++-
 .../prompts/compiler/eval/bindings/field_access.md |  53 +-
 00_nucleo/prompts/compiler/eval/tests.md           |  45 +-
 00_nucleo/prompts/compiler/introspect.md           | 113 +++-
 00_nucleo/prompts/compiler/introspect/from_tags.md |  27 +-
 00_nucleo/prompts/compiler/layout.md               |  47 +-
 00_nucleo/prompts/compiler/stdlib/counter.md       |  47 +-
 00_nucleo/prompts/compiler/stdlib/format.md        |  34 ++
 00_nucleo/prompts/compiler/stdlib/structural.md    |  24 +-
 .../prompts/compiler/stdlib/structural/document.md |  88 ++-
 00_nucleo/prompts/entities/compiler_features.md    |  33 +-
 00_nucleo/prompts/entities/content.md              |  38 +-
 00_nucleo/prompts/entities/counter_registry.md     |  23 +
 00_nucleo/prompts/entities/locator.md              |  41 +-
 00_nucleo/prompts/infra/integration_tests.md       |  49 +-
 00_nucleo/prompts/infra/pipeline.md                | 162 ++++-
 .../infra/pipeline/context_stabilization.md        | 102 +++-
 .../infra/tests/p1297_watch_capability_contract.md |  28 +-
 00_nucleo/prompts/shell/cli.md                     |  72 ++-
 00_nucleo/prompts/shell/watch.md                   |  75 +++
 00_nucleo/prompts/wiring.md                        |  60 +-
 00_nucleo/prompts/wiring/tests/cli.md              |  26 +
 01_core/src/compiler/eval/bindings/field_access.rs |  34 +-
 01_core/src/compiler/eval/mod.rs                   |  46 +-
 01_core/src/compiler/eval/repr.rs                  |  10 +-
 01_core/src/compiler/eval/rules.rs                 |   5 +-
 01_core/src/compiler/eval/tests.rs                 | 113 +++-
 01_core/src/compiler/introspect.rs                 | 182 +++++-
 01_core/src/compiler/introspect/from_tags.rs       | 129 +++-
 01_core/src/compiler/layout/mod.rs                 |  41 +-
 01_core/src/compiler/stdlib/counter.rs             |  42 +-
 01_core/src/compiler/stdlib/format.rs              |  32 +-
 01_core/src/compiler/stdlib/foundations/query.rs   |   1 +
 01_core/src/compiler/stdlib/structural/document.rs | 174 +++---
 01_core/src/compiler/stdlib/structural/mod.rs      | 172 ++++--
 01_core/src/entities/compiler_features.rs          |  21 +-
 01_core/src/entities/content.rs                    |  44 +-
 01_core/src/entities/counter_registry.rs           |  10 +-
 01_core/src/entities/locator.rs                    |  27 +-
 02_shell/src/cli.rs                                |  80 ++-
 03_infra/src/integration_tests.rs                  |  31 +-
 03_infra/src/pipeline.rs                           | 498 +++++++++++++++-
 03_infra/src/pipeline/context_stabilization.rs     | 326 ++++++++++-
 03_infra/src/watch.rs                              | 283 ++++++++-
 03_infra/tests/p1297_watch_capability_contract.rs  |   4 +-
 04_wiring/src/main.rs                              | 162 ++++-
 04_wiring/tests/cli.rs                             | 649 ++++++++++++++++++++-
 48 files changed, 4044 insertions(+), 340 deletions(-)
```

Não rastreados no mesmo instante:

```text
00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1487.md
00_nucleo/materialization/typst-passo-1488.md
```

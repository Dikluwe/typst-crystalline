# P1486 — medição bilateral de rótulos de páginas PDF

## Proveniência

Medição em 2026-09-24 entre 08:20 e 08:32 UTC; snapshot de Git às 2026-09-24 08:33:43 UTC.
HEAD: `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree não
commitada. As três alterações L0 de P1486 foram escritas **depois** dos
probes: `entities/layout_types.md`, `infra/pipeline.md` e
`infra/export/builder.md`. Nenhum consumer P1486 foi alterado. O
`git diff HEAD -- 01_core/src/entities/layout_types.rs
03_infra/src/export/builder.rs` estava vazio antes e depois dos probes.
`03_infra/src/pipeline.rs` já estava modificado por P1485 antes deles.

Vanilla ratificado: `/usr/local/bin/typst`, SHA-256
`eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
fonte pinada `586e1bd43fae6c9a973218163d3165c53ab8d16d`.
Cristalino usado na sonda negativa: `target/debug/typst`, SHA-256
`459e434d51e97039491134203b64b8ccd2dbbdb227706f142e54942b92874607`.
Este binário de debug pré-existente pode anteceder parte da working tree;
a ausência de emissão também foi confirmada diretamente em
`03_infra/src/export/builder.rs`, que continua sem `/PageLabels`.

Snapshot de `git diff HEAD --stat=200` às 2026-09-24 08:33:43 UTC:

```text
 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md  |   8 +-
 00_nucleo/prompts/compiler/eval.md                               |  73 ++++++++++++++-
 00_nucleo/prompts/compiler/eval/bindings/field_access.md         |  53 ++++++++++-
 00_nucleo/prompts/compiler/eval/tests.md                         |  45 ++++++++-
 00_nucleo/prompts/compiler/introspect.md                         | 113 +++++++++++++++++++++-
 00_nucleo/prompts/compiler/introspect/from_tags.md               |  27 +++++-
 00_nucleo/prompts/compiler/layout.md                             |  47 +++++++++-
 00_nucleo/prompts/compiler/stdlib/counter.md                     |  47 +++++++++-
 00_nucleo/prompts/compiler/stdlib/format.md                      |  34 +++++++
 00_nucleo/prompts/compiler/stdlib/structural.md                  |  24 ++++-
 00_nucleo/prompts/compiler/stdlib/structural/document.md         |  88 +++++++++++++++---
 00_nucleo/prompts/entities/compiler_features.md                  |  33 +++++--
 00_nucleo/prompts/entities/content.md                            |  38 +++++++-
 00_nucleo/prompts/entities/counter_registry.md                   |  23 +++++
 00_nucleo/prompts/entities/layout_types.md                       |  33 +++++++
 00_nucleo/prompts/entities/locator.md                            |  41 +++++++-
 00_nucleo/prompts/infra/export/builder.md                        |  64 +++++++++++++
 00_nucleo/prompts/infra/integration_tests.md                     |  49 +++++++++-
 00_nucleo/prompts/infra/pipeline.md                              | 187 ++++++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md        | 102 +++++++++++++++++++-
 00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md |  28 +++++-
 00_nucleo/prompts/shell/cli.md                                   |  72 +++++++++++++-
 00_nucleo/prompts/shell/watch.md                                 |  75 +++++++++++++++
 00_nucleo/prompts/wiring.md                                      |  60 +++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md                            |  26 ++++++
 01_core/src/compiler/eval/bindings/field_access.rs               |  34 ++++++-
 01_core/src/compiler/eval/mod.rs                                 |  46 ++++++---
 01_core/src/compiler/eval/repr.rs                                |  10 +-
 01_core/src/compiler/eval/rules.rs                               |   5 +-
 01_core/src/compiler/eval/tests.rs                               | 113 +++++++++++++++++++++-
 01_core/src/compiler/introspect.rs                               | 182 +++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/introspect/from_tags.rs                     | 129 +++++++++++++++++++-------
 01_core/src/compiler/layout/mod.rs                               |  41 ++++++--
 01_core/src/compiler/stdlib/counter.rs                           |  42 +++++----
 01_core/src/compiler/stdlib/format.rs                            |  32 +++++--
 01_core/src/compiler/stdlib/foundations/query.rs                 |   1 +
 01_core/src/compiler/stdlib/structural/document.rs               | 174 ++++++++++++++++++++--------------
 01_core/src/compiler/stdlib/structural/mod.rs                    | 172 ++++++++++++++++++++++------------
 01_core/src/entities/compiler_features.rs                        |  21 ++++-
 01_core/src/entities/content.rs                                  |  44 +++++----
 01_core/src/entities/counter_registry.rs                         |  10 +-
 01_core/src/entities/locator.rs                                  |  27 +++++-
 02_shell/src/cli.rs                                              |  80 +++++++++++++---
 03_infra/src/integration_tests.rs                                |  31 ++++---
 03_infra/src/pipeline.rs                                         | 498 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/pipeline/context_stabilization.rs                   | 326 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/watch.rs                                            | 283 ++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/tests/p1297_watch_capability_contract.rs                |   4 +-
 04_wiring/src/main.rs                                            | 162 +++++++++++++++++++++++++++++---
 04_wiring/tests/cli.rs                                           | 649 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 50 files changed, 4166 insertions(+), 340 deletions(-)
```

Não rastreados no mesmo snapshot (o presente relatório ainda não existia):

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

## Fixture e comandos

Quatro páginas de texto `One`, `Two`, `Three`, `Four`, separadas por
`#pagebreak()`. Prefixo de cada fixture:

```typst
// arabic.typ
#set page(numbering: "1")
// roman.typ: #set page(numbering: "i")
// none.typ: sem #set page(numbering: ...)
// shift.typ:
#set page(numbering: "1")
#counter(page).update(10)
```

Cada fixture foi compilada com o vanilla para exportação total (sem
`--pages`), `--pages 1-4`, `--pages 2-4`, `--pages 1,3` e
`--pages 4`. Exemplo reproduzível:

```sh
/usr/local/bin/typst compile shift.typ shift-1_3.pdf --pages 1,3
qpdf --check shift-1_3.pdf
mutool show shift-1_3.pdf trailer/Root/PageLabels
mutool show shift-1_3.pdf 3 4
python3 -c 'from pypdf import PdfReader; r=PdfReader("shift-1_3.pdf"); print(r.page_labels); print("/PageLabels" in r.trailer["/Root"])'
```

A extração usou o Python empacotado do workspace com `pypdf`:
`/home/dikluwe/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3`.

## Rótulos observados

| Fixture | Total | 1–4 | 2–4 | 1+3 | 4 |
|---|---|---|---|---|---|
| Arábica | 1, 2, 3, 4 | 1, 2, 3, 4 | 2, 3, 4 | 1, 3 | 4 |
| Romana | i, ii, iii, iv | i, ii, iii, iv | ii, iii, iv | i, iii | iv |
| Sem padrão | 1, 2, 3, 4 | 1, 2, 3, 4 | 2, 3, 4 | 1, 3 | 4 |
| Contador iniciado em 10 | 10, 11, 12, 13 | 10, 11, 12, 13 | 11, 12, 13 | 10, 12 | 13 |

No vanilla, `/PageLabels` existe em todos os casos com padrão, inclusive
sem seleção, e no caso sem padrão **só** se houve omissão efetiva.
Para seleção 1+3, a number tree contém `/Nums [0 3 0 R 1 4 0 R]`;
os objetos têm `/Type /PageLabel`. Arábica e fallback usam `/S /D`,
romana usa `/S /r`; os `/St` da fixture deslocada são 10 e 12,
e os do fallback são 1 e 3. `qpdf --check` não apontou erro nas sondas
vanilla arábica/deslocada nem na sonda cristalina.

O cristalino atual, em `--pages 1,3`, deu `['1', '2']` tanto para
a fixture arábica como para a deslocada, com `/PageLabels` ausente.
O rótulo implícito do leitor é, portanto, o índice do PDF exportado,
não o da página Typst original.

## Classificação e conclusão

O rótulo visto pelo leitor é observável semântico. A numeração de
objetos, bytes PDF e forma concreta da number tree são mecânica,
exceto a validade estrutural necessária ao leitor (ADR-0107).
A fonte vanilla `typst-pdf/src/convert.rs:130-149` seleciona o número
lógico para `Numbering::Pattern` e o físico para o fallback após
omissão; `typst-pdf/src/page.rs:17-60` decide estilo nativo ou texto.
Não há evidência de bug de vanilla nesta regra, diferentemente do bug
de outlines já rejeitado em P1468.

Inferência refutada: `Page.numbering` + índice físico bastariam.
A fixture `counter(page).update(10)` exige informação lógica 10/12,
ausente de `Page` e presente no `PageStore` realizado. Esta decisão
propõe `Page.logical_number` público, portanto aciona o gate ADR-0127.
A inferência de que o `PageStore` final está alinhado com cada
`Page` deve ser verificada nos testes RED→GREEN após confirmação.

## Adendo de implementação — 2026-09-24 11:18:44 UTC

O gate ADR-0127 do campo público foi confirmado pelo dono. HEAD
`bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`, working tree não
commitada. SHA-256 do binário cristalino reconstruído:
`27e0b7057a1f9b2a1c1862e2722bd4b11f8edef3c0b52f99e2a0d8216f3f1a9c`;
vanilla pinado: `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.
Fixture versionada: `lab/parity/corpus/p1486-page-labels.typ`. PDFs da
sonda 1+3: cristalino SHA-256
`ca681e2ab4bf9a3e37389f2eb9e3b8aec1b22810cf6afb362d6928c0a893ab40`,
vanilla `b7b29c29965646f7f135e4450fe45e55cfd54feb2b63efe21ec7c7bf5eed61df`.

Snapshot exato de caminhos alterados no instante da medição (66 tracked,
12 untracked; o presente diagnóstico já era untracked). Os caminhos
tracked constam de `git diff HEAD --stat` e os demais de
`git ls-files --others --exclude-standard`:

```text
M 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md
M 00_nucleo/prompts/compiler/eval.md
M 00_nucleo/prompts/compiler/eval/bindings/field_access.md
M 00_nucleo/prompts/compiler/eval/tests.md
M 00_nucleo/prompts/compiler/introspect.md
M 00_nucleo/prompts/compiler/introspect/from_tags.md
M 00_nucleo/prompts/compiler/layout.md
M 00_nucleo/prompts/compiler/layout/cursor.md
M 00_nucleo/prompts/compiler/layout/html_frame.md
M 00_nucleo/prompts/compiler/stdlib/counter.md
M 00_nucleo/prompts/compiler/stdlib/format.md
M 00_nucleo/prompts/compiler/stdlib/structural.md
M 00_nucleo/prompts/compiler/stdlib/structural/document.md
M 00_nucleo/prompts/entities/compiler_features.md
M 00_nucleo/prompts/entities/content.md
M 00_nucleo/prompts/entities/counter_registry.md
M 00_nucleo/prompts/entities/layout_types.md
M 00_nucleo/prompts/entities/locator.md
M 00_nucleo/prompts/infra/export/builder.md
M 00_nucleo/prompts/infra/export/tests/mod.md
M 00_nucleo/prompts/infra/integration_tests.md
M 00_nucleo/prompts/infra/pipeline.md
M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
M 00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md
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
M 01_core/src/compiler/introspect/from_tags.rs
M 01_core/src/compiler/layout/cursor.rs
M 01_core/src/compiler/layout/html_frame.rs
M 01_core/src/compiler/layout/mod.rs
M 01_core/src/compiler/stdlib/counter.rs
M 01_core/src/compiler/stdlib/format.rs
M 01_core/src/compiler/stdlib/foundations/query.rs
M 01_core/src/compiler/stdlib/structural/document.rs
M 01_core/src/compiler/stdlib/structural/mod.rs
M 01_core/src/entities/compiler_features.rs
M 01_core/src/entities/content.rs
M 01_core/src/entities/counter_registry.rs
M 01_core/src/entities/layout_types.rs
M 01_core/src/entities/locator.rs
M 02_shell/src/cli.rs
M 03_infra/src/export/builder.rs
M 03_infra/src/export/fonts.rs
M 03_infra/src/export/render.rs
M 03_infra/src/export/stream.rs
M 03_infra/src/export/svg.rs
M 03_infra/src/export/tests/mod.rs
M 03_infra/src/export/tests/p1481_pdfua_structure_tests.rs
M 03_infra/src/export/tests/p1483_pdf_spot_tests.rs
M 03_infra/src/integration_tests.rs
M 03_infra/src/layout_bidi.rs
M 03_infra/src/pipeline.rs
M 03_infra/src/pipeline/context_stabilization.rs
M 03_infra/src/shaper.rs
M 03_infra/src/watch.rs
M 03_infra/tests/p1297_watch_capability_contract.rs
M 04_wiring/src/main.rs
M 04_wiring/tests/cli.rs
?? 00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
?? 00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
?? 00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
?? 00_nucleo/diagnosticos/typst-p1486-page-labels.md
?? 00_nucleo/materialization/typst-passo-1484.md
?? 00_nucleo/materialization/typst-passo-1485.md
?? 00_nucleo/materialization/typst-passo-1486.md
?? 00_nucleo/materialization/typst-passo-1487.md
?? 00_nucleo/materialization/typst-passo-1488.md
?? 00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md
?? 03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs
?? lab/parity/corpus/p1486-page-labels.typ
```

Comandos: `target/debug/typst compile lab/parity/corpus/p1486-page-labels.typ
crystal-final.pdf --pages 1,3`, mesmo comando com `/usr/local/bin/typst`,
`qpdf --check` individual e `pypdf.PdfReader(...).page_labels`. Ambos os
leitores devolvem `['10', '12']` e `qpdf` não detecta erro estrutural.
`cargo test -p typst-infra --lib p1486_`: 8/8; regressão CLI P1485
`p1485_bundle_page`: 6/6. `CARGO_BUILD_JOBS=1 cargo build` e
`crystalline-lint --quiet .` passaram. A suíte L3 completa foi interrompida
após três minutos no teste raster P1431 para limitar recursos, sem falha
observada antes da interrupção. A compilação dos testes L1 foi interrompida
ao atingir 3,2 GB de RAM; o compilador reportou ICE durante a interrupção,
portanto não é evidência de regressão funcional.

Distinção semântica: `pdftotext` da mesma sonda mostra texto/margem
`One, 1; Three, 3` no cristalino e `One, 10; Three, 12` no vanilla.
P1486 corrige os **rótulos do leitor PDF**, não a numeração desenhada:
o L0 de `infra/export/builder.md` preserva explicitamente esse conteúdo.
Não declarar paridade da margem nem mascarar a diferença como metadado.

Rerun às 2026-09-24 11:21:31 UTC, com o mesmo conjunto de caminhos de
working tree acima (apenas este diagnóstico untracked recebeu o adendo):
8/8 testes P1486 e 6/6 regressões CLI P1485 novamente passaram. SHA-256
dos consumers decisivos nesse rerun: `pipeline.rs`
`ec808570198ecc034831ce6ccba394dbd8498264fbb7a1b619967203bffcd921`,
`pipeline/context_stabilization.rs`
`673a88612cdcadd55422e28b81e1d9bb2075dd12549cffccb9a5d10daec2bb84`.

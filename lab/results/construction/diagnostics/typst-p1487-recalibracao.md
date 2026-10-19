# P1487 — oráculo P1464 e matriz de paridade recalibrada

## Proveniência e âmbito

Medição consolidada em 2026-09-24 13:34:37 UTC. HEAD: `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08`; working tree não commitada. O snapshot exato das alterações no momento desta medição está abaixo. Este relatório foi criado depois do snapshot e, portanto, não consta da lista de não rastreados.

Binários medidos (SHA-256):

- vanilla ratificado `/usr/local/bin/typst`: `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f` (pin `586e1bd43fae6c9a973218163d3165c53ab8d16d`);
- cristalino `target/debug/typst`: `81cbcb240620cfccac5677cbab236b9609cb7209252109f8ca63ba2b64275fa2`;
- extrator vanilla `lab/typst-original/target/release/p1140-inventory`: `80a04a0434252814caebe48c4d09965c205aaaabf585dce3011609aa1099946f`;
- extrator cristalino `lab/surface-inventory/target/release/surface-inventory-p1140`: `449dd2cb1757ab114eb70d7eba2525a146e89a2e1044cbfb0089e062df33fb36`.

Snapshot de `git diff HEAD --stat=200`:

```text
 00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md  |   8 +-
 00_nucleo/prompts/compiler/eval.md                               |  73 ++++++++++++-
 00_nucleo/prompts/compiler/eval/bindings/field_access.md         |  53 ++++++++-
 00_nucleo/prompts/compiler/eval/tests.md                         |  45 +++++++-
 00_nucleo/prompts/compiler/introspect.md                         | 113 +++++++++++++++++++-
 00_nucleo/prompts/compiler/introspect/from_tags.md               |  27 ++++-
 00_nucleo/prompts/compiler/layout.md                             |  94 +++++++++++++++-
 00_nucleo/prompts/compiler/layout/cursor.md                      |  37 ++++++-
 00_nucleo/prompts/compiler/layout/html_frame.md                  |  15 ++-
 00_nucleo/prompts/compiler/stdlib/counter.md                     |  47 +++++++-
 00_nucleo/prompts/compiler/stdlib/format.md                      |  34 ++++++
 00_nucleo/prompts/compiler/stdlib/structural.md                  |  24 ++++-
 00_nucleo/prompts/compiler/stdlib/structural/document.md         |  88 +++++++++++++--
 00_nucleo/prompts/entities/compiler_features.md                  |  33 ++++--
 00_nucleo/prompts/entities/content.md                            |  38 ++++++-
 00_nucleo/prompts/entities/counter_registry.md                   |  23 ++++
 00_nucleo/prompts/entities/layout_types.md                       |  35 +++++-
 00_nucleo/prompts/entities/locator.md                            |  41 ++++++-
 00_nucleo/prompts/infra/export/builder.md                        |  82 ++++++++++++--
 00_nucleo/prompts/infra/export/tests/mod.md                      |  10 +-
 00_nucleo/prompts/infra/integration_tests.md                     |  49 ++++++++-
 00_nucleo/prompts/infra/pipeline.md                              | 235 +++++++++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md        | 130 ++++++++++++++++++++++-
 00_nucleo/prompts/infra/tests/p1297_watch_capability_contract.md |  28 ++++-
 00_nucleo/prompts/shell/cli.md                                   |  72 ++++++++++++-
 00_nucleo/prompts/shell/watch.md                                 |  75 +++++++++++++
 00_nucleo/prompts/wiring.md                                      |  60 ++++++++++-
 00_nucleo/prompts/wiring/tests/cli.md                            |  26 +++++
 00_nucleo/prompts/wiring/tests/p1464-format-pdf.md               |  28 ++++-
 01_core/src/compiler/eval/bindings/field_access.rs               |  34 +++++-
 01_core/src/compiler/eval/mod.rs                                 |  46 ++++++--
 01_core/src/compiler/eval/repr.rs                                |  10 +-
 01_core/src/compiler/eval/rules.rs                               |   5 +-
 01_core/src/compiler/eval/tests.rs                               | 113 +++++++++++++++++++-
 01_core/src/compiler/introspect.rs                               | 182 ++++++++++++++++++++++++++++++-
 01_core/src/compiler/introspect/from_tags.rs                     | 129 ++++++++++++++++------
 01_core/src/compiler/layout/cursor.rs                            |  12 ++-
 01_core/src/compiler/layout/html_frame.rs                        |   3 +-
 01_core/src/compiler/layout/mod.rs                               |  56 +++++++---
 01_core/src/compiler/stdlib/counter.rs                           |  42 ++++----
 01_core/src/compiler/stdlib/format.rs                            |  32 ++++--
 01_core/src/compiler/stdlib/foundations/query.rs                 |   1 +
 01_core/src/compiler/stdlib/structural/document.rs               | 174 ++++++++++++++++++------------
 01_core/src/compiler/stdlib/structural/mod.rs                    | 172 ++++++++++++++++++++----------
 01_core/src/entities/compiler_features.rs                        |  21 +++-
 01_core/src/entities/content.rs                                  |  44 ++++----
 01_core/src/entities/counter_registry.rs                         |  10 +-
 01_core/src/entities/layout_types.rs                             |   7 +-
 01_core/src/entities/locator.rs                                  |  27 ++++-
 02_shell/src/cli.rs                                              |  80 +++++++++++---
 03_infra/src/export/builder.rs                                   |  89 +++++++++++++++-
 03_infra/src/export/fonts.rs                                     |   2 +
 03_infra/src/export/render.rs                                    |   2 +
 03_infra/src/export/stream.rs                                    |   3 +
 03_infra/src/export/svg.rs                                       |   7 ++
 03_infra/src/export/tests/mod.rs                                 | 112 ++++++++++++++++++-
 03_infra/src/export/tests/p1481_pdfua_structure_tests.rs         |   1 +
 03_infra/src/export/tests/p1483_pdf_spot_tests.rs                |   1 +
 03_infra/src/integration_tests.rs                                |  31 +++---
 03_infra/src/layout_bidi.rs                                      |   1 +
 03_infra/src/pipeline.rs                                         | 744 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/pipeline/context_stabilization.rs                   | 339 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/shaper.rs                                           |   2 +
 03_infra/src/watch.rs                                            | 283 +++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/tests/p1297_watch_capability_contract.rs                |   4 +-
 04_wiring/src/main.rs                                            | 162 +++++++++++++++++++++++++---
 04_wiring/tests/cli.rs                                           | 649 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 04_wiring/tests/p1464_format_pdf.rs                              |   4 +-
 lab/parity/matrix/README.md                                      |  23 ++++
 lab/parity/matrix/manifest.yaml                                  |  45 +++++++-
 lab/parity/matrix/runner.py                                      | 148 +++++++++++++++++++++++++-
 lab/parity/matrix/schema.json                                    |  12 ++-
 lab/parity/matrix/test_runner.py                                 |  90 +++++++++++++++-
 lab/surface-inventory/merge.py                                   |  18 +++-
 lab/surface-inventory/src/main.rs                                |   7 +-
 lab/surface-inventory/test_merge.py                              |  25 ++++-
 lab/typst-original/crates/p1140-inventory/src/main.rs            |   5 +-
 77 files changed, 5242 insertions(+), 390 deletions(-)
```

Não rastreados no mesmo snapshot:

```text
00_nucleo/diagnosticos/typst-p1484-matriz-bundle.md
00_nucleo/diagnosticos/typst-p1485-bundle-sondas.md
00_nucleo/diagnosticos/typst-p1485-integracao-gate2.md
00_nucleo/diagnosticos/typst-p1486-page-labels.md
00_nucleo/diagnosticos/typst-p1487-pixel-diff.png
00_nucleo/materialization/typst-passo-1484.md
00_nucleo/materialization/typst-passo-1485.md
00_nucleo/materialization/typst-passo-1486.md
00_nucleo/materialization/typst-passo-1487.md
00_nucleo/materialization/typst-passo-1488.md
00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md
03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs
lab/parity/corpus/p1486-page-labels.typ
lab/parity/matrix/fixtures/bundle-asset.typ
lab/parity/matrix/fixtures/bundle-html.typ
lab/parity/matrix/fixtures/bundle-mixed.typ
lab/parity/matrix/fixtures/page-labels.typ
lab/parity/matrix/fixtures/pdfua.typ
```

## Decisão após medição

A sonda bilateral de `type(pdf)`, `repr(pdf)`, `pdf.attach` e `pdf.artifact` devolveu `function`, `pdf`, `function`, `attach`, `function`, `artifact` em ambos os binários. O RED do teste P1464 tinha uma falha por cada um dos quatro perfis apenas na expectativa antiga de módulo. O L0 proprietário foi atualizado antes do consumer, e o teste agora exige função global com namespace. Nenhum código produtivo foi mudado por P1487.

A matriz distingue `DIFFERENCE` (observação sem testemunho semântico) de `VIOLATED` (morfologia, semântica ou formato público divergente). Não existe tolerância automática de pixels. `UNVERIFIED_METADATA` no inventário permanece distinto de ausência e de prova funcional: por exemplo, `pdf` existe e é função dos dois lados, mas os parâmetros não são observados bilateralmente pelo extrator.

## Matriz funcional

Manifest com 32 casos; cada perfil foi executado em ordem direta e inversa com digests idênticos. Não houve falha de expectativa.

| Perfil | MATCH | DIFFERENCE | DISABLED_BY_PROFILE | VIOLATED | UNKNOWN |
|---|---:|---:|---:|---:|---:|
| default | 22 | 2 | 8 | 0 | 0 |
| html | 24 | 2 | 6 | 0 | 0 |
| a11y-extras | 23 | 2 | 7 | 0 | 0 |
| html+a11y | 25 | 2 | 5 | 0 | 0 |
| bundle | 26 | 2 | 4 | 0 | 0 |
| html+bundle | 29 | 2 | 1 | 0 | 0 |

Os novos probes cobrem `color.spot.tint` válido e inválido; `asset` funcional, caminho inválido, asset puro e bundle PDF/HTML; feature bundle desativada; PDF/UA positivo e `--no-pdf-tags` negativo; seleção de páginas 1,3 (rótulos 10,12) e página 2 (rótulo 11), incluindo número visível na margem. Os controles negativos exigem rejeição bilateral, não igualdade textual de diagnósticos além do contrato do caso.

As duas `DIFFERENCE` preexistentes continuam mensuráveis: PNG de 1191×1684 com 11/2.005.644 pixels diferentes, delta máximo de canal 1 e delta médio 0,000004113392; o [artefato de diferença](typst-p1487-pixel-diff.png) tem SHA-256 `38a6f9161797b60dc17e0533abdbd2f59c68529ffeee1d882ea9ad583b150d9d`. No PDF, página, tamanho e texto `Hello, parity.` coincidem; só o nome de fonte difere (`LibertinusSerif-Regular` versus `LibertinusSerif-Regular-Identity-H`). Isso não é creditado como `MATCH` nem promovido a violação da linguagem sem testemunho adicional.

## Catálogos de superfície

Extratores regenerados bilateralmente; todos os seis merges passaram a validação de esquema, revisão e vetor de features, sem `UNKNOWN` ou `MISSING_*`. Contagens são de caminhos catalogados, não percentagem de paridade da linguagem.

| Perfil | MATCH | UNVERIFIED_METADATA | EXTRA_BINDING |
|---|---:|---:|---:|
| default | 1552 | 469 | 47 |
| html | 1552 | 699 | 48 |
| a11y-extras | 1552 | 475 | 47 |
| html+a11y | 1552 | 705 | 48 |
| bundle | 1552 | 472 | 47 |
| html+bundle | 1552 | 702 | 48 |

No vanilla, `asset`, `format.bundle` e `format.bundle.asset` surgem em `bundle` e não no default; os seis membros `pdf.*`/`format.pdf.*` de acessibilidade surgem somente em `a11y-extras`. Ausência num perfil desativado não foi classificada como defeito. Um binding `UNVERIFIED_METADATA` não recebe crédito de execução sem probe.

## Reprodução e gates

Comandos usados (executados serialmente, `CARGO_BUILD_JOBS=1` nos builds/testes Rust):

```sh
python3 -m unittest lab/surface-inventory/test_merge.py lab/surface-inventory/test_run_probes.py lab/parity/matrix/test_runner.py -q
CARGO_BUILD_JOBS=1 cargo build --manifest-path lab/typst-original/Cargo.toml -p p1140-inventory --release -q
CARGO_BUILD_JOBS=1 cargo build --manifest-path lab/surface-inventory/Cargo.toml --release -q
for profile in default html a11y-extras html+a11y bundle html+bundle; do
  lab/typst-original/target/release/p1140-inventory /tmp/p1487-probes-codex-20260924.k4h0qQ/vanilla-$profile.json "$profile" /usr/local/bin/typst
  lab/surface-inventory/target/release/surface-inventory-p1140 /tmp/p1487-probes-codex-20260924.k4h0qQ/crystal-$profile.json /tmp/p1487-probes-codex-20260924.k4h0qQ/vanilla-$profile.json "$profile" lab/surface-inventory/extra_seeds.json target/debug/typst
  python3 lab/surface-inventory/merge.py /tmp/p1487-probes-codex-20260924.k4h0qQ/vanilla-$profile.json /tmp/p1487-probes-codex-20260924.k4h0qQ/crystal-$profile.json /tmp/p1487-probes-codex-20260924.k4h0qQ/inventory-$profile.json --profile "$profile"
  python3 lab/parity/matrix/runner.py --vanilla /usr/local/bin/typst --crystalline target/debug/typst --profile "$profile" --output /tmp/p1487-probes-codex-20260924.k4h0qQ/matrix-$profile.json
done
CARGO_BUILD_JOBS=1 cargo test -p typst-wiring --test p1464_format_pdf -- --nocapture
CARGO_BUILD_JOBS=1 cargo build --quiet
CARGO_BUILD_JOBS=1 cargo test --manifest-path lab/surface-inventory/Cargo.toml --release -q
CARGO_BUILD_JOBS=1 cargo test --manifest-path lab/typst-original/Cargo.toml -p p1140-inventory --release -q
crystalline-lint --checks v5,v15,v26 --fix-hashes .
git diff --check
```

Resultado: 79 testes Python, 4 testes P1464 e 2 testes em cada extrator Rust passaram; build passou. `crystalline-lint --checks v5,v15,v26 --fix-hashes .` ressellou apenas o header P1464. Após criar este relatório, `crystalline-lint .` e `crystalline-lint --checks v5,v15,v26 --quiet .` saíram com código 0; o lint completo ainda emite avisos informativos não relacionados. `qpdf --check` no PDF/UA positivo cristalino não encontrou erro de sintaxe nem stream.

Limite: a matriz cobre apenas estes 32 casos. Ela não fecha a pendência de continuidade global do bundle descrita em P1485. Nenhuma contagem aqui deve ser apresentada como percentual de paridade total.

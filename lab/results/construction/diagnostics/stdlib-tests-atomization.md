# Atomização dos testes da stdlib — 2026-09-26

## Proveniência e medição anterior à decisão

HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`.
Working tree não commitada, com alterações prévias e alterações concorrentes.
Inventário final recolhido em `2026-09-26T11:26:08.749Z`.
A inspeção inicial de 2026-09-26T08:16:50-03:00 e o baseline desta execução
identificaram 14.087 linhas em `01_core/src/compiler/stdlib/mod.rs`,
742 testes inline em :231–14087 e dois helpers produtivos em :210–228.
O registro global permanece em `01_core/src/compiler/eval/mod.rs`.

A lista exata dos arquivos alterados e o stat da árvore observada nesta
verificação estão abaixo. Arquivos novos têm inventário expandido; hashes dos
consumers desta tarefa estão na tabela. O estado concorrente impede atribuir
um resultado global a um commit limpo.

## Decisão e resultado

Extração interna em fluxo contínuo ADR-0127, L0 primeiro. A fachada passa a
232 linhas, com ligação `#[cfg(test)] mod tests;`. A suíte fica em 22 famílias,
um hub e support. Cada arquivo novo tem owner exclusivo em
`00_nucleo/prompts/compiler/stdlib/tests/`.

A extração preserva os 742 nomes locais e os corpos completos dos testes,
inclusive atributos adicionais, fixtures e assertions. Imports ajustados ficam
fora dos corpos. Os helpers de suporte ganham somente visibilidade privada
`pub(super)`. A comparação mecânica confirmou identidade dos corpos e da
fachada produtiva excluindo a linha de hash e a ligação test-only.

Não houve alteração de comportamento da linguagem ou medição de paridade;
não foi necessário invocar um vanilla diferente do pin vigente. A atomização
do registro global é a próxima fatia, fora deste recorte.

## Verificação

- Baseline: `cargo test -p typst-core --lib compiler::stdlib::tests:: -- --list`
  descobriu 742 testes; execução do binário anterior: 742 passaram.
- Extração: o mesmo comando compilou a suíte e descobriu 742 testes.
- Execução direta do binário compilado: 742 passaram, zero falhas ou ignorados.
- Binário: `target/debug/deps/typst_core-32f8dd8720b9a85a`;
  SHA-256 `b19b74527b32cd1356917c5f750758426e4cdf8403c8076241d6b6f6d8a0f69c`.
- `git diff --check`: sem problemas.
- `crystalline-lint .`: nenhum error/warning nos consumers desta tarefa.
  Avisos informativos V20 dos patterns históricos foram preservados.
  A validação global encontrou pins V26 desatualizados nos owners concorrentes
  de `compiler/eval/observation`; não há certificação global de zero violations.
- `cargo build`: a tentativa global encontrou `layout/measurement` declarado
  antes de seu arquivo existir e imports ainda incompletos em
  `eval/observation/mod.rs`, ambos introduzidos concorrentemente fora deste
  recorte. O build global não está fechado por esta execução.

Recheck posterior: `layout/measurement.rs` já havia sido criado, mas o build
ainda falhou nos imports de `eval/observation/mod.rs` (E0432). A checagem
focal dos gates arquiteturais também encontrou metadata incompleta no L0
concorrente de measurement (V0), além dos pins V26 de observation. Não foram
alterados esses owners. Logs adicionais: `stdlib-build-recheck.log` e
`stdlib-lint-contracts.log`. A suíte desta extração permanece comprovada pelo
binário compilado e executado antes dessas alterações concorrentes.

Logs desta execução em /tmp: `stdlib-tests-before.log`,
`stdlib-tests-after.log`, `stdlib-tests-baseline-run.log`,
`stdlib-tests-extracted-run.log`, `stdlib-build.log` e
`stdlib-lint-final.log`. A prova mecânica foi feita antes e após os ajustes
de imports, com os corpos congelados em `stdlib-tests-original.json`.

## Manifesto dos consumers

| Arquivo | Testes | Linhas | SHA-256 |
| --- | ---: | ---: | --- |
| `arguments.rs` | 2 | 23 | `dadc808bd43147d9349048ea926f91292296ba3a04e852c39aa6f865da07e8e6` |
| `assertions.rs` | 16 | 189 | `d3fde2c6db0e8cb9a8d42f790bd7e456e5ac088b753d90521a1e386ecc4f61ce` |
| `bibliography.rs` | 43 | 799 | `b43e7df17b98cf5fa538653c1ed72a0d60f3c606c264ceb33fd40339ed6345d5` |
| `calc.rs` | 71 | 2014 | `0ea6e22be90acbf7e946fc80f88015a341a982c046f99e2ed8899a8b694b7085` |
| `color.rs` | 9 | 116 | `80dac149484c6e16bb810cf078870d1bd16a9ebbe3bd0de329fe9b1aef8167e9` |
| `contextual.rs` | 20 | 397 | `89466bbe2dc956b417f6118f6db1c3376aa005d6daf2eb6a6a669e9759e3968e` |
| `eval.rs` | 8 | 99 | `0b446d00340923e5f0fbdf1f73338d368caac99c3f9a8d1c9ce13242df846400` |
| `figure_image.rs` | 21 | 333 | `d0aef0f9d08329294a3216ed11b10847e5a0977289fe42e19b69bd64f1953a85` |
| `footnote.rs` | 8 | 108 | `2c584333c6a385fdd60dc8a1e5ad5b2c7ea8c679d7a82a952acb2a04abe03adb` |
| `foundations.rs` | 30 | 616 | `9c085135409315009d4574e25b5e32b229a113e2e324e31b4abcefbce2f33b34` |
| `gradients.rs` | 41 | 729 | `bb10fc95be562569bb0012de68428a08f1f7afc8f4c7c571569ea0404165d337` |
| `introspection.rs` | 33 | 654 | `aaaf8084c3ff92721469fa22056a375fdc934e4d0552b419ccfeec15a9f9e808` |
| `layout_containers.rs` | 87 | 1379 | `0cab1e7267f5928274c85ffaf74249178dd947eeb70e44eaaa3efb333e3b4a32` |
| `layout_context.rs` | 39 | 473 | `09455701b7d0d2b45752fdae748da09999af14811c4ad1ef99de654809bd7077` |
| `layout_spacing.rs` | 40 | 591 | `041233eb275a6d5b1451b7ca6903a549ea88a40a160bc9e400f615398ae3fb5a` |
| `math.rs` | 62 | 780 | `700eb16d59b261d51fab0e4b11bd33d7b73a72eb700e8740c43159fad29d91c2` |
| `mod.rs` | 0 | 104 | `c5aa052fdcc1a79dbdad71e90b227a92e5edd3fa16567ec07bc61110da91d332` |
| `shapes.rs` | 52 | 1220 | `9558869b5c49430880a4e5dea817bcf3a72f5133daf359f754c608cd515c51bf` |
| `stroke.rs` | 19 | 409 | `c154617398c6c2cfec78a3ca9411aaaba7f31c67e45a048a64345bd12544ce56` |
| `support.rs` | 0 | 370 | `6acf68ef4245ccea539fe8d1a096c86ef6afcfb3452a0d20d814da925121dfa4` |
| `symbols.rs` | 3 | 43 | `1391d38b0d48e5a4468fb71179009eff8be2c8ecc20abe10f472d5b13f1912a4` |
| `table_grid.rs` | 90 | 1492 | `cf798d86782e34c9aa286efa2d44300454caa4d6e748a374dbf3c62164ea204a` |
| `text.rs` | 34 | 520 | `4f083fdde6903ed7520490aefe6b965aff62f81d562b2cefca1da3e4083cf599` |
| `transforms.rs` | 14 | 235 | `8e5ee54a6c1c67ddd4c62e6faa1a43b34b32012dddd543d5f84cdcd57a29ea66` |

## Estado completo observado

`git diff HEAD --stat`:

```text
 00_nucleo/adr/README.md                            |  4301 +-----
 00_nucleo/adr/adr-stub-vs-fallback.md              |    76 -
 00_nucleo/adr/template-adr.md                      |    77 +-
 .../adr/typst-adr-0002-hierarquia-contencao.md     |   182 -
 00_nucleo/adr/typst-adr-0003-comemo-contencao.md   |   159 -
 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md |   198 -
 00_nucleo/adr/typst-adr-0007-rustc-hash.md         |    85 -
 00_nucleo/adr/typst-adr-0028-typographic-types.md  |    52 -
 .../adr/typst-adr-0060-model-structural-roadmap.md |   779 --
 .../adr/typst-adr-0061-layout-fase-x-roadmap.md    |  1645 ---
 .../typst-adr-0066-introspection-runtime-adiada.md |   496 -
 .../adr/typst-adr-0078-column-flow-algorithm.md    |     3 +-
 .../adr/typst-adr-0079-layout-fase-5-roadmap.md    |  2628 ----
 ...dr-0081-m7-plus-pipeline-restructuring-scope.md |  1085 --
 ...090-gradient-conic-strategy-type-4-vs-type-1.md |   561 -
 ...tern-debt-37-campo-layouter-consumer-pending.md |   259 -
 ...t-adr-0097-scope-out-reconfirmado-por-fase-a.md |   269 -
 .../adr/typst-adr-0130-referencias-canonicas-l0.md |    42 +-
 00_nucleo/prompts/compiler/eval.md                 |    19 +
 00_nucleo/prompts/compiler/layout.md               |    15 +
 00_nucleo/prompts/compiler/layout/equation.md      |     4 +
 00_nucleo/prompts/compiler/layout/page_geometry.md |     5 +
 00_nucleo/prompts/compiler/layout/placement.md     |     4 +
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    19 +-
 00_nucleo/prompts/entities/content.md              |   119 +-
 00_nucleo/prompts/infra/export/builder.md          |    21 +
 00_nucleo/prompts/infra/pipeline.md                |  1668 +--
 .../infra/pipeline/context_stabilization.md        |     9 +
 01_core/src/compiler/eval/bibliography.rs          |     2 +-
 01_core/src/compiler/eval/bindings/access.rs       |    29 +-
 01_core/src/compiler/eval/bindings/binding.rs      |     4 +-
 01_core/src/compiler/eval/bindings/field_access.rs |    88 +-
 .../src/compiler/eval/bindings/method_dispatch.rs  |    34 +-
 .../src/compiler/eval/bindings/value_methods.rs    |    24 +-
 01_core/src/compiler/eval/call_dispatch.rs         |    49 +-
 01_core/src/compiler/eval/closures.rs              |     6 +-
 01_core/src/compiler/eval/control_flow.rs          |     6 +-
 01_core/src/compiler/eval/font_dict.rs             |     7 +-
 01_core/src/compiler/eval/markup.rs                |     4 +-
 01_core/src/compiler/eval/math.rs                  |    12 +-
 01_core/src/compiler/eval/mod.rs                   |  5769 +-------
 01_core/src/compiler/eval/modules.rs               |    18 +-
 01_core/src/compiler/eval/operators/arithmetic.rs  |    12 +-
 01_core/src/compiler/eval/p1427_tests.rs           |     6 +-
 01_core/src/compiler/eval/p1430_tests.rs           |     2 +-
 01_core/src/compiler/eval/p1431_tests.rs           |     2 +-
 01_core/src/compiler/eval/repr.rs                  |    36 +-
 01_core/src/compiler/eval/rules.rs                 |   406 +-
 01_core/src/compiler/eval/selector_matching.rs     |     9 +-
 01_core/src/compiler/eval/show_rule_termination.rs |     6 +-
 01_core/src/compiler/eval/tests.rs                 |   947 +-
 01_core/src/compiler/introspect.rs                 |    54 +-
 01_core/src/compiler/introspect/fixpoint.rs        |     8 +-
 01_core/src/compiler/introspect/from_tags.rs       |    41 +-
 01_core/src/compiler/introspect/locatable.rs       |     4 +-
 01_core/src/compiler/layout/columns.rs             |    10 +-
 01_core/src/compiler/layout/decorations.rs         |     4 +-
 01_core/src/compiler/layout/equation.rs            |    84 +-
 01_core/src/compiler/layout/format_options.rs      |    43 +-
 01_core/src/compiler/layout/mod.rs                 |   466 +-
 01_core/src/compiler/layout/sequence.rs            |    63 +-
 01_core/src/compiler/layout/set_page.rs            |    75 +-
 01_core/src/compiler/layout/tests.rs               |  1078 +-
 01_core/src/compiler/layout/word_segment.rs        |    44 +-
 01_core/src/compiler/math/layout/accent.rs         |     4 +-
 01_core/src/compiler/math/layout/attach.rs         |     4 +-
 01_core/src/compiler/math/layout/mod.rs            |    47 +-
 01_core/src/compiler/math/layout/p1430_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/p1431_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/spacing.rs        |    18 +-
 01_core/src/compiler/math/layout/tests.rs          |     2 +-
 01_core/src/compiler/stdlib/color.rs               |   219 +-
 01_core/src/compiler/stdlib/counter.rs             |    10 +-
 01_core/src/compiler/stdlib/format.rs              |    12 +-
 01_core/src/compiler/stdlib/foundations/color.rs   |     6 +-
 01_core/src/compiler/stdlib/gradients.rs           |    11 +-
 01_core/src/compiler/stdlib/layout.rs              |    36 +-
 01_core/src/compiler/stdlib/loading.rs             |     5 +-
 01_core/src/compiler/stdlib/mod.rs                 | 13852 +------------------
 01_core/src/compiler/stdlib/structural/document.rs |    45 +-
 01_core/src/compiler/stdlib/structural/math.rs     |     2 +-
 01_core/src/compiler/stdlib/structural/mod.rs      |    54 +-
 01_core/src/compiler/stdlib/structural/outline.rs  |     6 +-
 .../src/compiler/stdlib/structural/p1429_tests.rs  |     2 +-
 .../src/compiler/stdlib/structural/p1430_tests.rs  |     2 +-
 01_core/src/compiler/stdlib/structural/par.rs      |     2 +-
 01_core/src/compiler/stdlib/text/constructor.rs    |     2 +-
 01_core/src/entities/color.rs                      |    31 +-
 01_core/src/entities/content.rs                    |  6763 +--------
 01_core/src/entities/content_hash.rs               |    10 +-
 01_core/src/entities/elements/figure_caption.rs    |     6 +-
 01_core/src/entities/elements/footnote_entry.rs    |     8 +-
 01_core/src/entities/gradient.rs                   |    24 +-
 01_core/src/entities/locator.rs                    |     1 -
 01_core/src/entities/style.rs                      |     4 +-
 01_core/src/entities/style_chain.rs                |    18 +-
 02_shell/src/cli.rs                                |    34 +-
 03_infra/src/export/builder.rs                     |  4116 +-----
 03_infra/src/export/html.rs                        |    48 +-
 03_infra/src/export/svg.rs                         |    21 +-
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6004 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     5 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/watch.rs                              |    67 +-
 03_infra/tests/p1250_svg_integrated.rs             |     3 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 benches/Cargo.toml                                 |     5 +
 115 files changed, 3556 insertions(+), 52668 deletions(-)
```

`git status --short --untracked-files=all`:

```text
 M 00_nucleo/adr/README.md
 D 00_nucleo/adr/adr-stub-vs-fallback.md
 M 00_nucleo/adr/template-adr.md
 D 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
 D 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
 D 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md
 D 00_nucleo/adr/typst-adr-0007-rustc-hash.md
 D 00_nucleo/adr/typst-adr-0028-typographic-types.md
 D 00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md
 D 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
 D 00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md
 M 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
 D 00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md
 D 00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
 D 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
 D 00_nucleo/adr/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
 D 00_nucleo/adr/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
 M 00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/page_geometry.md
 M 00_nucleo/prompts/compiler/layout/placement.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 00_nucleo/prompts/compiler/stdlib/_comum.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 01_core/src/compiler/eval/bibliography.rs
 M 01_core/src/compiler/eval/bindings/access.rs
 M 01_core/src/compiler/eval/bindings/binding.rs
 M 01_core/src/compiler/eval/bindings/field_access.rs
 M 01_core/src/compiler/eval/bindings/method_dispatch.rs
 M 01_core/src/compiler/eval/bindings/value_methods.rs
 M 01_core/src/compiler/eval/call_dispatch.rs
 M 01_core/src/compiler/eval/closures.rs
 M 01_core/src/compiler/eval/control_flow.rs
 M 01_core/src/compiler/eval/font_dict.rs
 M 01_core/src/compiler/eval/markup.rs
 M 01_core/src/compiler/eval/math.rs
 M 01_core/src/compiler/eval/mod.rs
 M 01_core/src/compiler/eval/modules.rs
 M 01_core/src/compiler/eval/operators/arithmetic.rs
 M 01_core/src/compiler/eval/p1427_tests.rs
 M 01_core/src/compiler/eval/p1430_tests.rs
 M 01_core/src/compiler/eval/p1431_tests.rs
 M 01_core/src/compiler/eval/repr.rs
 M 01_core/src/compiler/eval/rules.rs
 M 01_core/src/compiler/eval/selector_matching.rs
 M 01_core/src/compiler/eval/show_rule_termination.rs
 M 01_core/src/compiler/eval/tests.rs
 M 01_core/src/compiler/introspect.rs
 M 01_core/src/compiler/introspect/fixpoint.rs
 M 01_core/src/compiler/introspect/from_tags.rs
 M 01_core/src/compiler/introspect/locatable.rs
 M 01_core/src/compiler/layout/columns.rs
 M 01_core/src/compiler/layout/decorations.rs
 M 01_core/src/compiler/layout/equation.rs
 M 01_core/src/compiler/layout/format_options.rs
 M 01_core/src/compiler/layout/mod.rs
 M 01_core/src/compiler/layout/sequence.rs
 M 01_core/src/compiler/layout/set_page.rs
 M 01_core/src/compiler/layout/tests.rs
 M 01_core/src/compiler/layout/word_segment.rs
 M 01_core/src/compiler/math/layout/accent.rs
 M 01_core/src/compiler/math/layout/attach.rs
 M 01_core/src/compiler/math/layout/mod.rs
 M 01_core/src/compiler/math/layout/p1430_tests.rs
 M 01_core/src/compiler/math/layout/p1431_tests.rs
 M 01_core/src/compiler/math/layout/spacing.rs
 M 01_core/src/compiler/math/layout/tests.rs
 M 01_core/src/compiler/stdlib/color.rs
 M 01_core/src/compiler/stdlib/counter.rs
 M 01_core/src/compiler/stdlib/format.rs
 M 01_core/src/compiler/stdlib/foundations/color.rs
 M 01_core/src/compiler/stdlib/gradients.rs
 M 01_core/src/compiler/stdlib/layout.rs
 M 01_core/src/compiler/stdlib/loading.rs
 M 01_core/src/compiler/stdlib/mod.rs
 M 01_core/src/compiler/stdlib/structural/document.rs
 M 01_core/src/compiler/stdlib/structural/math.rs
 M 01_core/src/compiler/stdlib/structural/mod.rs
 M 01_core/src/compiler/stdlib/structural/outline.rs
 M 01_core/src/compiler/stdlib/structural/p1429_tests.rs
 M 01_core/src/compiler/stdlib/structural/p1430_tests.rs
 M 01_core/src/compiler/stdlib/structural/par.rs
 M 01_core/src/compiler/stdlib/text/constructor.rs
 M 01_core/src/entities/color.rs
 M 01_core/src/entities/content.rs
 M 01_core/src/entities/content_hash.rs
 M 01_core/src/entities/elements/figure_caption.rs
 M 01_core/src/entities/elements/footnote_entry.rs
 M 01_core/src/entities/gradient.rs
 M 01_core/src/entities/locator.rs
 M 01_core/src/entities/style.rs
 M 01_core/src/entities/style_chain.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/export/builder.rs
 M 03_infra/src/export/html.rs
 M 03_infra/src/export/svg.rs
 M 03_infra/src/export/tests/mod.rs
 M 03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs
 M 03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs
 M 03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs
 M 03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs
 M 03_infra/src/integration_tests.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/query_helpers.rs
 M 03_infra/src/watch.rs
 M 03_infra/tests/p1250_svg_integrated.rs
 M 04_wiring/src/main.rs
 M 04_wiring/tests/cli.rs
 M benches/Cargo.toml
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-archive/README.md
?? 00_nucleo/diagnosticos/adr-archive/adr-0113-draft-stub-vs-fallback-portuguese.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0004-passo1-descobertas.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0060-model-structural-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0061-layout-fase-x-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0079-layout-fase-5-roadmap.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
?? 00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
?? 00_nucleo/diagnosticos/adr-archive/redirects.tsv
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0002-hierarquia-contencao.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0003-comemo-contencao.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0007-rustc-hash.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0028-typographic-types.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0066-introspection-runtime-adiada.md
?? 00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
?? 00_nucleo/diagnosticos/adr-cleanup-classification.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/materialization/tekt-english-reference-migration.md
?? 00_nucleo/prompts/_nuclei/eval/observation-relation.toml
?? 00_nucleo/prompts/compiler/eval/observation.md
?? 00_nucleo/prompts/compiler/eval/observation/content.md
?? 00_nucleo/prompts/compiler/eval/observation/elements.md
?? 00_nucleo/prompts/compiler/eval/observation/geometry.md
?? 00_nucleo/prompts/compiler/eval/observation/math.md
?? 00_nucleo/prompts/compiler/eval/observation/page.md
?? 00_nucleo/prompts/compiler/eval/observation/style.md
?? 00_nucleo/prompts/compiler/eval/observation/value.md
?? 00_nucleo/prompts/compiler/layout.md.orig
?? 00_nucleo/prompts/compiler/layout/equation.md.orig
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/stdlib/tests/arguments.md
?? 00_nucleo/prompts/compiler/stdlib/tests/assertions.md
?? 00_nucleo/prompts/compiler/stdlib/tests/bibliography.md
?? 00_nucleo/prompts/compiler/stdlib/tests/calc.md
?? 00_nucleo/prompts/compiler/stdlib/tests/color.md
?? 00_nucleo/prompts/compiler/stdlib/tests/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/tests/eval.md
?? 00_nucleo/prompts/compiler/stdlib/tests/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/tests/footnote.md
?? 00_nucleo/prompts/compiler/stdlib/tests/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/tests/gradients.md
?? 00_nucleo/prompts/compiler/stdlib/tests/introspection.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_containers.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_context.md
?? 00_nucleo/prompts/compiler/stdlib/tests/layout_spacing.md
?? 00_nucleo/prompts/compiler/stdlib/tests/math.md
?? 00_nucleo/prompts/compiler/stdlib/tests/mod.md
?? 00_nucleo/prompts/compiler/stdlib/tests/shapes.md
?? 00_nucleo/prompts/compiler/stdlib/tests/stroke.md
?? 00_nucleo/prompts/compiler/stdlib/tests/support.md
?? 00_nucleo/prompts/compiler/stdlib/tests/symbols.md
?? 00_nucleo/prompts/compiler/stdlib/tests/table_grid.md
?? 00_nucleo/prompts/compiler/stdlib/tests/text.md
?? 00_nucleo/prompts/compiler/stdlib/tests/transforms.md
?? 00_nucleo/prompts/entities/content/construction.md
?? 00_nucleo/prompts/entities/content/emptiness.md
?? 00_nucleo/prompts/entities/content/equality.md
?? 00_nucleo/prompts/entities/content/fields.md
?? 00_nucleo/prompts/entities/content/identity.md
?? 00_nucleo/prompts/entities/content/morphology.md
?? 00_nucleo/prompts/entities/content/page-structure.md
?? 00_nucleo/prompts/entities/content/plain-text.md
?? 00_nucleo/prompts/entities/content/rust-hash.md
?? 00_nucleo/prompts/entities/content/sequence.md
?? 00_nucleo/prompts/entities/content/tests.md
?? 00_nucleo/prompts/entities/content/tests/morphology.md
?? 00_nucleo/prompts/entities/content/tests/projections.md
?? 00_nucleo/prompts/entities/content/tests/representation.md
?? 00_nucleo/prompts/entities/content/tests/sequence.md
?? 00_nucleo/prompts/entities/content/tests/traversal.md
?? 00_nucleo/prompts/entities/content/traversal.md
?? 00_nucleo/prompts/infra/export/builder/fonts.md
?? 00_nucleo/prompts/infra/export/builder/metadata.md
?? 00_nucleo/prompts/infra/export/builder/navigation.md
?? 00_nucleo/prompts/infra/export/builder/resources.md
?? 00_nucleo/prompts/infra/export/builder/serialization.md
?? 00_nucleo/prompts/infra/export/builder/streams.md
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 01_core/src/compiler/eval/observation/content.rs
?? 01_core/src/compiler/eval/observation/elements.rs
?? 01_core/src/compiler/eval/observation/geometry.rs
?? 01_core/src/compiler/eval/observation/math.rs
?? 01_core/src/compiler/eval/observation/mod.rs
?? 01_core/src/compiler/eval/observation/page.rs
?? 01_core/src/compiler/eval/observation/style.rs
?? 01_core/src/compiler/eval/observation/value.rs
?? 01_core/src/compiler/layout/measurement.rs
?? 01_core/src/compiler/stdlib/tests/arguments.rs
?? 01_core/src/compiler/stdlib/tests/assertions.rs
?? 01_core/src/compiler/stdlib/tests/bibliography.rs
?? 01_core/src/compiler/stdlib/tests/calc.rs
?? 01_core/src/compiler/stdlib/tests/color.rs
?? 01_core/src/compiler/stdlib/tests/contextual.rs
?? 01_core/src/compiler/stdlib/tests/eval.rs
?? 01_core/src/compiler/stdlib/tests/figure_image.rs
?? 01_core/src/compiler/stdlib/tests/footnote.rs
?? 01_core/src/compiler/stdlib/tests/foundations.rs
?? 01_core/src/compiler/stdlib/tests/gradients.rs
?? 01_core/src/compiler/stdlib/tests/introspection.rs
?? 01_core/src/compiler/stdlib/tests/layout_containers.rs
?? 01_core/src/compiler/stdlib/tests/layout_context.rs
?? 01_core/src/compiler/stdlib/tests/layout_spacing.rs
?? 01_core/src/compiler/stdlib/tests/math.rs
?? 01_core/src/compiler/stdlib/tests/mod.rs
?? 01_core/src/compiler/stdlib/tests/shapes.rs
?? 01_core/src/compiler/stdlib/tests/stroke.rs
?? 01_core/src/compiler/stdlib/tests/support.rs
?? 01_core/src/compiler/stdlib/tests/symbols.rs
?? 01_core/src/compiler/stdlib/tests/table_grid.rs
?? 01_core/src/compiler/stdlib/tests/text.rs
?? 01_core/src/compiler/stdlib/tests/transforms.rs
?? 01_core/src/entities/content/construction.rs
?? 01_core/src/entities/content/emptiness.rs
?? 01_core/src/entities/content/equality.rs
?? 01_core/src/entities/content/fields.rs
?? 01_core/src/entities/content/identity.rs
?? 01_core/src/entities/content/morphology.rs
?? 01_core/src/entities/content/page_structure.rs
?? 01_core/src/entities/content/plain_text.rs
?? 01_core/src/entities/content/rust_hash.rs
?? 01_core/src/entities/content/sequence.rs
?? 01_core/src/entities/content/tests/mod.rs
?? 01_core/src/entities/content/tests/morphology.rs
?? 01_core/src/entities/content/tests/projections.rs
?? 01_core/src/entities/content/tests/representation.rs
?? 01_core/src/entities/content/tests/sequence.rs
?? 01_core/src/entities/content/tests/traversal.rs
?? 01_core/src/entities/content/traversal.rs
?? 03_infra/src/export/builder/fonts.rs
?? 03_infra/src/export/builder/metadata.rs
?? 03_infra/src/export/builder/navigation.rs
?? 03_infra/src/export/builder/resources.rs
?? 03_infra/src/export/builder/serialization.rs
?? 03_infra/src/export/builder/streams.rs
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? benches/content/construction.rs
?? benches/content/coverage.rs
?? benches/content/fixtures.rs
?? benches/content/identity.rs
?? benches/content/morphology.rs
?? benches/content/page_structure.rs
?? benches/content/projections.rs
?? benches/content/representation.rs
?? benches/content/sequence.rs
?? benches/content/traversal.rs
?? benches/content_bench.rs
```

# P1506 — atomizacao do builder PDF

## Resultado

`03_infra/src/export/builder.rs` conserva `PdfBuilder`, estado inicial,
setters, selecao do caminho e orquestracao Helvetica. As familias fontes,
recursos, streams, metadados, navegacao e serializacao vivem em seus seis
modulos descendentes, com owners L0 exclusivos e imports explicitos.
O nucleo tem 373 linhas no checkpoint abaixo. Os testes locais de
streams e escapes acompanham os respectivos modulos.

Extracao interna conforme ADR-0109/0127/0129, sem mudanca funcional ou de
assinaturas externas. Os L0 preparados na working tree foram preservados,
com metadata canonica e referencias de medicao acrescentadas. A selagem
foi validada em snapshot restrito e transferiu somente os hashes dos sete
pares; nenhum reparo global foi usado por esta execucao.

Microbenchmarks: `benches/pdf_builder_bench.rs`. Documentacao de uso em
`00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md`. Sao exportacoes
completas com fixtures por familia; nao ha alegacao de ganho de desempenho
nem medidas isoladas de fases internas.

## Validacao e limites

- `cargo build` passou na ronda registrada em `/tmp/pdf-builder-build.log`.
- Build mais recente nesta captura: PASSOU
- Checagem do benchmark passou; smoke test: 12 cenarios executados
  com sucesso, incluindo fonte unica e duas fontes alternadas.
- Snapshots P307b: test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 1067 filtered out; finished in 0.30s
- Exportacao PDF, recompilada da arvore atual: test result: ok. 485 passed; 0 failed; 0 ignored; 0 measured; 591 filtered out; finished in 2.09s
- `crystalline-lint --checks v5,v6,v15,v26 .`: sem violacoes na ronda
  registrada em `/tmp/pdf-builder-lineage-latest.log`.
- `crystalline-lint .` nesta captura: Sem erros/fatais; avisos e informacoes nao bloqueantes permanecem.
- `git diff --check` dos arquivos rastreados do escopo passou.

A corrida ampla `export::` foi interrompida no teste
`p1431_raster_mapeia_butt_round_square_e_auto_square`, que rasteriza a
16 pixels por ponto e nao exercita o builder PDF. A regressao final foi
recompilada da arvore atual e exclui somente esse caso. Comandos:

```sh
cargo test -p typst-infra --lib export:: -- --test-threads=1 --skip p1431_raster_mapeia_butt_round_square_e_auto_square
cargo test -p typst-infra --lib p307b_snapshot_tests -- --test-threads=1
cargo test -p typst-benches --bench pdf_builder_bench -- --test
```

Uma ronda intermediaria encontrou erros em arquivos modificados
simultaneamente de `compiler/stdlib/registry`, incluindo imports de `Func`
e visibilidade de `make_stdlib_with_features`. Esses erros foram resolvidos
pela frente externa antes do build final e da recompilacao da regressao.
O binario de testes validado tem SHA-256
`696130feb74bf17d69f29b26d923ca97a5e1c68e1caa48cb0e7a2090041784ab`
e path `target/debug/deps/typst_infra-d65d9d78ec3ade2f`. Os testes e
benchmarks funcionais nao constituem medicao bilateral nova contra vanilla.

## Proveniencia do checkpoint

Horario da captura: `2026-09-26T08:42:11.424153-03:00`.
HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; working tree nao commitada, compartilhada com outras
atomizacoes em curso. Os checksums abaixo identificam o builder desta
entrega. `git diff HEAD --stat` e a lista individual dos nao rastreados
foram capturados antes de gravar este diagnostico; incluem alteracoes
externas ao escopo, nao atribuidas a esta execucao.

```text
d67731402f573b168c0191fc07ae7fcc187e48798a9d739d1c3c83c402b35bac  03_infra/src/export/builder.rs
c8e2c754405e6ce8cf579a0563e7b8fd8ed7e662cbb06533f5dc5134c13118a5  03_infra/src/export/builder/fonts.rs
03b5f112e11d1ac3c18736398a0d84335cb7cdff714961c4105a18bdb195760a  03_infra/src/export/builder/metadata.rs
56c9a76bbfd57ab67dd1f5e65a32006231ad3b06687e801c829a00b00f446a00  03_infra/src/export/builder/navigation.rs
f3463baf0f4c74f68ec0ee6fd078e470f7a78e8d78dc534a3e80281daa058188  03_infra/src/export/builder/resources.rs
53c39841e32a68f689d1c7bfe0a82d22ae2611d38094a5fd5bfe07bb46eed743  03_infra/src/export/builder/serialization.rs
4f18fdfcf4948c099e7844c0e143b32b5370468fa881a135b059e2186cf1dee6  03_infra/src/export/builder/streams.rs
```

### git diff HEAD --stat

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
 00_nucleo/prompts/compiler/eval.md                 |    43 +-
 00_nucleo/prompts/compiler/layout.md               |    23 +-
 00_nucleo/prompts/compiler/layout/equation.md      |     6 +-
 00_nucleo/prompts/compiler/layout/page_geometry.md |     7 +-
 00_nucleo/prompts/compiler/layout/placement.md     |     6 +-
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    28 +-
 00_nucleo/prompts/entities/content.md              |   119 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 01_core/Cargo.toml                                 |     3 +
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
 01_core/src/compiler/eval/mod.rs                   |  6587 +--------
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
 01_core/src/compiler/layout/equation.rs            |   153 +-
 01_core/src/compiler/layout/format_options.rs      |    43 +-
 01_core/src/compiler/layout/mod.rs                 |   914 +-
 01_core/src/compiler/layout/page_geometry.rs       |    47 +-
 01_core/src/compiler/layout/placement.rs           |   128 +-
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
 01_core/src/compiler/stdlib/mod.rs                 | 13854 +------------------
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
 03_infra/src/export/builder.rs                     |  4274 +-----
 03_infra/src/export/html.rs                        |    48 +-
 03_infra/src/export/svg.rs                         |    21 +-
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6005 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     7 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/watch.rs                              |    67 +-
 03_infra/tests/p1250_svg_integrated.rs             |     3 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    17 +
 119 files changed, 3847 insertions(+), 54127 deletions(-)
```

### git ls-files --others --exclude-standard

```text
00_nucleo/adr/ORDER.md
00_nucleo/diagnosticos/adr-archive/README.md
00_nucleo/diagnosticos/adr-archive/adr-0113-draft-stub-vs-fallback-portuguese.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0004-passo1-descobertas.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0060-model-structural-roadmap.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0061-layout-fase-x-roadmap.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0079-layout-fase-5-roadmap.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
00_nucleo/diagnosticos/adr-archive/process-roadmaps/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
00_nucleo/diagnosticos/adr-archive/redirects.tsv
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0002-hierarquia-contencao.md
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0003-comemo-contencao.md
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0007-rustc-hash.md
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0028-typographic-types.md
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0066-introspection-runtime-adiada.md
00_nucleo/diagnosticos/adr-archive/superseded/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
00_nucleo/diagnosticos/adr-cleanup-classification.md
00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
00_nucleo/diagnosticos/content-microbenchmarks.md
00_nucleo/diagnosticos/p1505-content-representation-ab.md
00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
00_nucleo/diagnosticos/pipeline-atomization.md
00_nucleo/diagnosticos/stdlib-tests-atomization.md
00_nucleo/diagnosticos/tekt-english-reference-pilot.md
00_nucleo/materialization/tekt-english-reference-migration.md
00_nucleo/prompts/_nuclei/eval/observation-relation.toml
00_nucleo/prompts/compiler/eval/observation.md
00_nucleo/prompts/compiler/eval/observation/content.md
00_nucleo/prompts/compiler/eval/observation/elements.md
00_nucleo/prompts/compiler/eval/observation/geometry.md
00_nucleo/prompts/compiler/eval/observation/math.md
00_nucleo/prompts/compiler/eval/observation/page.md
00_nucleo/prompts/compiler/eval/observation/style.md
00_nucleo/prompts/compiler/eval/observation/value.md
00_nucleo/prompts/compiler/layout.md.orig
00_nucleo/prompts/compiler/layout/equation.md.orig
00_nucleo/prompts/compiler/layout/measurement.md
00_nucleo/prompts/compiler/layout/measurement.md.orig
00_nucleo/prompts/compiler/layout/page_geometry.md.orig
00_nucleo/prompts/compiler/layout/par_line_callbacks.md
00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
00_nucleo/prompts/compiler/layout/style_scope.md
00_nucleo/prompts/compiler/layout/style_scope.md.orig
00_nucleo/prompts/compiler/stdlib/registry/contextual.md
00_nucleo/prompts/compiler/stdlib/registry/data.md
00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md
00_nucleo/prompts/compiler/stdlib/registry/figure_image.md
00_nucleo/prompts/compiler/stdlib/registry/formats.md
00_nucleo/prompts/compiler/stdlib/registry/foundations.md
00_nucleo/prompts/compiler/stdlib/registry/layout.md
00_nucleo/prompts/compiler/stdlib/registry/math.md
00_nucleo/prompts/compiler/stdlib/registry/mod.md
00_nucleo/prompts/compiler/stdlib/registry/modules.md
00_nucleo/prompts/compiler/stdlib/registry/shapes.md
00_nucleo/prompts/compiler/stdlib/registry/structural.md
00_nucleo/prompts/compiler/stdlib/registry/table_grid.md
00_nucleo/prompts/compiler/stdlib/registry/tests.md
00_nucleo/prompts/compiler/stdlib/registry/text.md
00_nucleo/prompts/compiler/stdlib/tests/arguments.md
00_nucleo/prompts/compiler/stdlib/tests/assertions.md
00_nucleo/prompts/compiler/stdlib/tests/bibliography.md
00_nucleo/prompts/compiler/stdlib/tests/calc.md
00_nucleo/prompts/compiler/stdlib/tests/color.md
00_nucleo/prompts/compiler/stdlib/tests/contextual.md
00_nucleo/prompts/compiler/stdlib/tests/eval.md
00_nucleo/prompts/compiler/stdlib/tests/figure_image.md
00_nucleo/prompts/compiler/stdlib/tests/footnote.md
00_nucleo/prompts/compiler/stdlib/tests/foundations.md
00_nucleo/prompts/compiler/stdlib/tests/gradients.md
00_nucleo/prompts/compiler/stdlib/tests/introspection.md
00_nucleo/prompts/compiler/stdlib/tests/layout_containers.md
00_nucleo/prompts/compiler/stdlib/tests/layout_context.md
00_nucleo/prompts/compiler/stdlib/tests/layout_spacing.md
00_nucleo/prompts/compiler/stdlib/tests/math.md
00_nucleo/prompts/compiler/stdlib/tests/mod.md
00_nucleo/prompts/compiler/stdlib/tests/shapes.md
00_nucleo/prompts/compiler/stdlib/tests/stroke.md
00_nucleo/prompts/compiler/stdlib/tests/support.md
00_nucleo/prompts/compiler/stdlib/tests/symbols.md
00_nucleo/prompts/compiler/stdlib/tests/table_grid.md
00_nucleo/prompts/compiler/stdlib/tests/text.md
00_nucleo/prompts/compiler/stdlib/tests/transforms.md
00_nucleo/prompts/entities/content/construction.md
00_nucleo/prompts/entities/content/emptiness.md
00_nucleo/prompts/entities/content/equality.md
00_nucleo/prompts/entities/content/fields.md
00_nucleo/prompts/entities/content/identity.md
00_nucleo/prompts/entities/content/morphology.md
00_nucleo/prompts/entities/content/page-structure.md
00_nucleo/prompts/entities/content/plain-text.md
00_nucleo/prompts/entities/content/rust-hash.md
00_nucleo/prompts/entities/content/sequence.md
00_nucleo/prompts/entities/content/tests.md
00_nucleo/prompts/entities/content/tests/morphology.md
00_nucleo/prompts/entities/content/tests/projections.md
00_nucleo/prompts/entities/content/tests/representation.md
00_nucleo/prompts/entities/content/tests/sequence.md
00_nucleo/prompts/entities/content/tests/traversal.md
00_nucleo/prompts/entities/content/traversal.md
00_nucleo/prompts/infra/export/builder/fonts.md
00_nucleo/prompts/infra/export/builder/metadata.md
00_nucleo/prompts/infra/export/builder/navigation.md
00_nucleo/prompts/infra/export/builder/resources.md
00_nucleo/prompts/infra/export/builder/serialization.md
00_nucleo/prompts/infra/export/builder/streams.md
00_nucleo/prompts/infra/pipeline/bundle.md
00_nucleo/prompts/infra/pipeline/html.md
00_nucleo/prompts/infra/pipeline/paged.md
00_nucleo/prompts/infra/pipeline/pdf.md
00_nucleo/prompts/infra/pipeline/raster.md
00_nucleo/prompts/infra/pipeline/resources.md
00_nucleo/prompts/infra/pipeline/svg.md
00_nucleo/prompts/infra/pipeline/tests.md
01_core/src/compiler/eval/observation/content.rs
01_core/src/compiler/eval/observation/elements.rs
01_core/src/compiler/eval/observation/geometry.rs
01_core/src/compiler/eval/observation/math.rs
01_core/src/compiler/eval/observation/mod.rs
01_core/src/compiler/eval/observation/page.rs
01_core/src/compiler/eval/observation/style.rs
01_core/src/compiler/eval/observation/value.rs
01_core/src/compiler/layout/equation.rs.orig
01_core/src/compiler/layout/measurement.rs
01_core/src/compiler/layout/measurement.rs.orig
01_core/src/compiler/layout/mod.rs.orig
01_core/src/compiler/layout/page_geometry.rs.orig
01_core/src/compiler/layout/par_line_callbacks.rs
01_core/src/compiler/layout/par_line_callbacks.rs.orig
01_core/src/compiler/layout/placement.rs.orig
01_core/src/compiler/layout/style_scope.rs
01_core/src/compiler/layout/style_scope.rs.orig
01_core/src/compiler/stdlib/registry/contextual.rs
01_core/src/compiler/stdlib/registry/data.rs
01_core/src/compiler/stdlib/registry/diagnostics.rs
01_core/src/compiler/stdlib/registry/figure_image.rs
01_core/src/compiler/stdlib/registry/formats.rs
01_core/src/compiler/stdlib/registry/foundations.rs
01_core/src/compiler/stdlib/registry/layout.rs
01_core/src/compiler/stdlib/registry/math.rs
01_core/src/compiler/stdlib/registry/mod.rs
01_core/src/compiler/stdlib/registry/modules.rs
01_core/src/compiler/stdlib/registry/shapes.rs
01_core/src/compiler/stdlib/registry/structural.rs
01_core/src/compiler/stdlib/registry/table_grid.rs
01_core/src/compiler/stdlib/registry/tests.rs
01_core/src/compiler/stdlib/registry/text.rs
01_core/src/compiler/stdlib/tests/arguments.rs
01_core/src/compiler/stdlib/tests/assertions.rs
01_core/src/compiler/stdlib/tests/bibliography.rs
01_core/src/compiler/stdlib/tests/calc.rs
01_core/src/compiler/stdlib/tests/color.rs
01_core/src/compiler/stdlib/tests/contextual.rs
01_core/src/compiler/stdlib/tests/eval.rs
01_core/src/compiler/stdlib/tests/figure_image.rs
01_core/src/compiler/stdlib/tests/footnote.rs
01_core/src/compiler/stdlib/tests/foundations.rs
01_core/src/compiler/stdlib/tests/gradients.rs
01_core/src/compiler/stdlib/tests/introspection.rs
01_core/src/compiler/stdlib/tests/layout_containers.rs
01_core/src/compiler/stdlib/tests/layout_context.rs
01_core/src/compiler/stdlib/tests/layout_spacing.rs
01_core/src/compiler/stdlib/tests/math.rs
01_core/src/compiler/stdlib/tests/mod.rs
01_core/src/compiler/stdlib/tests/shapes.rs
01_core/src/compiler/stdlib/tests/stroke.rs
01_core/src/compiler/stdlib/tests/support.rs
01_core/src/compiler/stdlib/tests/symbols.rs
01_core/src/compiler/stdlib/tests/table_grid.rs
01_core/src/compiler/stdlib/tests/text.rs
01_core/src/compiler/stdlib/tests/transforms.rs
01_core/src/entities/content/construction.rs
01_core/src/entities/content/emptiness.rs
01_core/src/entities/content/equality.rs
01_core/src/entities/content/fields.rs
01_core/src/entities/content/identity.rs
01_core/src/entities/content/morphology.rs
01_core/src/entities/content/page_structure.rs
01_core/src/entities/content/plain_text.rs
01_core/src/entities/content/rust_hash.rs
01_core/src/entities/content/sequence.rs
01_core/src/entities/content/tests/mod.rs
01_core/src/entities/content/tests/morphology.rs
01_core/src/entities/content/tests/projections.rs
01_core/src/entities/content/tests/representation.rs
01_core/src/entities/content/tests/sequence.rs
01_core/src/entities/content/tests/traversal.rs
01_core/src/entities/content/traversal.rs
03_infra/src/export/builder/fonts.rs
03_infra/src/export/builder/metadata.rs
03_infra/src/export/builder/navigation.rs
03_infra/src/export/builder/resources.rs
03_infra/src/export/builder/serialization.rs
03_infra/src/export/builder/streams.rs
03_infra/src/pipeline/bundle.rs
03_infra/src/pipeline/html.rs
03_infra/src/pipeline/paged.rs
03_infra/src/pipeline/pdf.rs
03_infra/src/pipeline/raster.rs
03_infra/src/pipeline/resources.rs
03_infra/src/pipeline/svg.rs
03_infra/src/pipeline/tests.rs
benches/content/construction.rs
benches/content/coverage.rs
benches/content/fixtures.rs
benches/content/identity.rs
benches/content/morphology.rs
benches/content/page_structure.rs
benches/content/projections.rs
benches/content/representation.rs
benches/content/sequence.rs
benches/content/traversal.rs
benches/content_bench.rs
benches/pdf_builder_bench.rs
benches/pipeline_bench.rs
```

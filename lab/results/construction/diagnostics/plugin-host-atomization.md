# Atomização do host de plugins

Separação exclusivamente interna a L3: loading (compilação, validação de memory e exports), execution (instanciação, chamada e transition), typst_env (linker e protocolo), snapshot (captura/restauração da memória). A fachada preserva WasmiPluginHost, new/Default, estado privado e impl PluginHost com delegação estática. Suíte movida integralmente para tests.rs. Cada consumer tem L0 exclusivo e o L0 anterior está arquivado em plugin-host-l0-before-atomization.md.

Sem pool novo, alterações ao runtime/features, API L1, precedência de erros ou mensagens. Instância fresca por chamada e snapshots somente de memória linear, sem globals, continuam iguais. Não foi medida performance nem nova paridade contra o vanilla.

## Validação

- Baseline `cargo test -p typst-infra --lib plugin_host:: -- --test-threads=1`: 20 passaram.
- Mesmo comando após extração: 20 passaram.
- `cargo test -p typst-wiring --test cli cli_plugin_ -- --test-threads=1`: 2 passaram (transition e mensagens/spans).
- `cargo build`: passou.
- `crystalline-lint .` após o resselo final: exit 0, sem erros bloqueantes nem warnings atribuídos ao plugin_host; warnings de outros módulos permanecem no estado global.
- Ownership: `crystalline-lint --checks v15,v26 .` sem violations. Resselo aplicado somente aos seis pares do plugin_host, a partir do dry-run global; arquivos de outras tarefas não foram resselados por esta execução.
- Comparação estrutural: loading, execution, typst_env, snapshot e testes mantêm corpos, descontando imports, visibilidade ao módulo pai, receiver self para parâmetro host, formatação e extração do registro do linker. Todas as strings produtivas preservadas verbatim. Esta comparação estrutural não substitui os critérios de linguagem.
- A primeira tentativa pós-extração encontrou erros em export/stream e world, de alterações simultâneas. Nenhum desses arquivos foi modificado nesta tarefa; a repetição normal passou após sua estabilização. Um harness temporário não é usado como prova de aceitação.

## Distribuição medida

| Consumer | Linhas |
|---|---:|
| `03_infra/src/plugin_host.rs` | 103 |
| `03_infra/src/plugin_host/execution.rs` | 157 |
| `03_infra/src/plugin_host/loading.rs` | 58 |
| `03_infra/src/plugin_host/snapshot.rs` | 55 |
| `03_infra/src/plugin_host/tests.rs` | 518 |
| `03_infra/src/plugin_host/typst_env.rs` | 82 |

Fonte anterior: 907 linhas; SHA-256 `d2c95fa476a8f38f48d395b0ccd71a7e8c3377b4623090374715023fe2fff149`. L0 anterior SHA-256 `f2f4a29db3d2a6fe47baef9c62fdced5ae0c679d6be42804bc5aa5902dbb2484`.

## Proveniência

HEAD baseline `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; HEAD pós-extração `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; working tree não commitado. Captura baseline: `2026-09-26T09:11:38-03:00`; captura pós-extração: `2026-09-26T09:19:10-03:00`. Estes resultados são do estado não commitado, com alterações concorrentes preservadas, não de uma revisão limpa.

### Baseline: git diff HEAD --stat

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
 00_nucleo/prompts/entities/syntax-node.md          |   230 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
 00_nucleo/prompts/infra/export/stream.md           |    31 +-
 00_nucleo/prompts/infra/font_metrics.md            |  1085 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 00_nucleo/prompts/infra/shaper.md                  |   954 +-
 00_nucleo/prompts/infra/system-world.md            |   427 +-
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
 01_core/src/compiler/layout/tests.rs               |  1083 +-
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
 01_core/src/entities/syntax_node.rs                |   104 +
 02_shell/src/cli.rs                                |    34 +-
 03_infra/src/export/builder.rs                     |  4274 +-----
 03_infra/src/export/html.rs                        |    48 +-
 03_infra/src/export/svg.rs                         |    21 +-
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/font_metrics.rs                       |  3373 +----
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6005 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     7 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/shaper.rs                             |  2648 +---
 03_infra/src/watch.rs                              |    67 +-
 03_infra/tests/p1250_svg_integrated.rs             |     6 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    22 +
 128 files changed, 4171 insertions(+), 62674 deletions(-)
```

### Baseline: untracked

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
00_nucleo/diagnosticos/font-metrics-atomization.md
00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
00_nucleo/diagnosticos/layout-microbenchmarks.md
00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
00_nucleo/diagnosticos/observation-microbenchmarks.md
00_nucleo/diagnosticos/p1505-content-representation-ab.md
00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
00_nucleo/diagnosticos/pipeline-atomization.md
00_nucleo/diagnosticos/pipeline-microbenchmarks-baseline.json
00_nucleo/diagnosticos/pipeline-microbenchmarks.md
00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md
00_nucleo/diagnosticos/stdlib-microbenchmarks.md
00_nucleo/diagnosticos/stdlib-registry-atomization.md
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
00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md
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
00_nucleo/prompts/entities/syntax-node/errors.md
00_nucleo/prompts/entities/syntax-node/linked.md
00_nucleo/prompts/entities/syntax-node/representation.md
00_nucleo/prompts/infra/export/builder/fonts.md
00_nucleo/prompts/infra/export/builder/metadata.md
00_nucleo/prompts/infra/export/builder/navigation.md
00_nucleo/prompts/infra/export/builder/resources.md
00_nucleo/prompts/infra/export/builder/serialization.md
00_nucleo/prompts/infra/export/builder/streams.md
00_nucleo/prompts/infra/export/stream/draw.md
00_nucleo/prompts/infra/export/stream/page.md
00_nucleo/prompts/infra/export/stream/shape.md
00_nucleo/prompts/infra/export/stream/text.md
00_nucleo/prompts/infra/font_metrics/basic.md
00_nucleo/prompts/infra/font_metrics/caches.md
00_nucleo/prompts/infra/font_metrics/fallback.md
00_nucleo/prompts/infra/font_metrics/math.md
00_nucleo/prompts/infra/font_metrics/resolution.md
00_nucleo/prompts/infra/pipeline/bundle.md
00_nucleo/prompts/infra/pipeline/html.md
00_nucleo/prompts/infra/pipeline/paged.md
00_nucleo/prompts/infra/pipeline/pdf.md
00_nucleo/prompts/infra/pipeline/raster.md
00_nucleo/prompts/infra/pipeline/resources.md
00_nucleo/prompts/infra/pipeline/svg.md
00_nucleo/prompts/infra/pipeline/tests.md
00_nucleo/prompts/infra/shaper/bidi.md
00_nucleo/prompts/infra/shaper/cache.md
00_nucleo/prompts/infra/shaper/font_selection.md
00_nucleo/prompts/infra/shaper/reconciliation.md
00_nucleo/prompts/infra/shaper/shaping.md
00_nucleo/prompts/infra/shaper/tests.md
00_nucleo/prompts/infra/shaper/traversal.md
00_nucleo/prompts/infra/system-world/cache.md
00_nucleo/prompts/infra/system-world/fonts.md
00_nucleo/prompts/infra/system-world/packages.md
00_nucleo/prompts/infra/system-world/paths.md
00_nucleo/prompts/infra/system-world/tests.md
00_nucleo/prompts/infra/system-world/world_impl.md
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
01_core/src/compiler/stdlib/registry/microbenchmarks.rs
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
03_infra/src/font_metrics/basic.rs
03_infra/src/font_metrics/caches.rs
03_infra/src/font_metrics/fallback.rs
03_infra/src/font_metrics/math.rs
03_infra/src/font_metrics/resolution.rs
03_infra/src/pipeline/bundle.rs
03_infra/src/pipeline/html.rs
03_infra/src/pipeline/paged.rs
03_infra/src/pipeline/pdf.rs
03_infra/src/pipeline/raster.rs
03_infra/src/pipeline/resources.rs
03_infra/src/pipeline/svg.rs
03_infra/src/pipeline/tests.rs
03_infra/src/shaper/bidi.rs
03_infra/src/shaper/cache.rs
03_infra/src/shaper/font_selection.rs
03_infra/src/shaper/reconciliation.rs
03_infra/src/shaper/shaping.rs
03_infra/src/shaper/tests.rs
03_infra/src/shaper/traversal.rs
benches/Cargo.toml.orig
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
benches/layout/fixtures.rs
benches/layout/measurement.rs
benches/layout/style_scope.rs
benches/layout_bench.rs
benches/pdf_builder_bench.rs
benches/pipeline_bench.rs
```

### Pós-extração: git diff HEAD --stat

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
 00_nucleo/prompts/entities/syntax-node.md          |   230 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 00_nucleo/prompts/infra/export/images.md           |   206 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
 00_nucleo/prompts/infra/export/stream.md           |    35 +-
 00_nucleo/prompts/infra/font_metrics.md            |  1085 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 00_nucleo/prompts/infra/plugin_host.md             |   333 +-
 00_nucleo/prompts/infra/shaper.md                  |   954 +-
 00_nucleo/prompts/infra/system-world.md            |   431 +-
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
 01_core/src/compiler/layout/tests.rs               |  1083 +-
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
 01_core/src/entities/syntax_node.rs                |  1122 +-
 02_shell/src/cli.rs                                |    34 +-
 03_infra/src/export/builder.rs                     |  4274 +-----
 03_infra/src/export/html.rs                        |    48 +-
 03_infra/src/export/images.rs                      |   596 +-
 03_infra/src/export/stream.rs                      |  2133 +--
 03_infra/src/export/svg.rs                         |    21 +-
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/font_metrics.rs                       |  3373 +----
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6005 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     7 +-
 03_infra/src/plugin_host.rs                        |   838 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/shaper.rs                             |  2648 +---
 03_infra/src/watch.rs                              |    67 +-
 03_infra/src/world.rs                              |  1131 +-
 03_infra/tests/p1250_svg_integrated.rs             |     6 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    22 +
 134 files changed, 4350 insertions(+), 68758 deletions(-)
```

### Pós-extração: untracked

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
00_nucleo/diagnosticos/font-metrics-atomization.md
00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
00_nucleo/diagnosticos/images-l0-before-atomization.md
00_nucleo/diagnosticos/layout-microbenchmarks.md
00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
00_nucleo/diagnosticos/observation-microbenchmarks.md
00_nucleo/diagnosticos/p1505-content-representation-ab.md
00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
00_nucleo/diagnosticos/pipeline-atomization.md
00_nucleo/diagnosticos/pipeline-microbenchmarks-baseline.json
00_nucleo/diagnosticos/pipeline-microbenchmarks.md
00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md
00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md
00_nucleo/diagnosticos/stdlib-microbenchmarks.md
00_nucleo/diagnosticos/stdlib-registry-atomization.md
00_nucleo/diagnosticos/stdlib-tests-atomization.md
00_nucleo/diagnosticos/stream-atomization.md
00_nucleo/diagnosticos/syntax-node-atomization.md
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
00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md
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
00_nucleo/prompts/entities/syntax-node/errors.md
00_nucleo/prompts/entities/syntax-node/linked.md
00_nucleo/prompts/entities/syntax-node/representation.md
00_nucleo/prompts/infra/export/builder/fonts.md
00_nucleo/prompts/infra/export/builder/metadata.md
00_nucleo/prompts/infra/export/builder/navigation.md
00_nucleo/prompts/infra/export/builder/resources.md
00_nucleo/prompts/infra/export/builder/serialization.md
00_nucleo/prompts/infra/export/builder/streams.md
00_nucleo/prompts/infra/export/images/icc.md
00_nucleo/prompts/infra/export/images/jpeg.md
00_nucleo/prompts/infra/export/images/png.md
00_nucleo/prompts/infra/export/images/scan.md
00_nucleo/prompts/infra/export/images/xobjects.md
00_nucleo/prompts/infra/export/stream/draw.md
00_nucleo/prompts/infra/export/stream/page.md
00_nucleo/prompts/infra/export/stream/shape.md
00_nucleo/prompts/infra/export/stream/text.md
00_nucleo/prompts/infra/font_metrics/basic.md
00_nucleo/prompts/infra/font_metrics/caches.md
00_nucleo/prompts/infra/font_metrics/fallback.md
00_nucleo/prompts/infra/font_metrics/math.md
00_nucleo/prompts/infra/font_metrics/resolution.md
00_nucleo/prompts/infra/pipeline/bundle.md
00_nucleo/prompts/infra/pipeline/html.md
00_nucleo/prompts/infra/pipeline/paged.md
00_nucleo/prompts/infra/pipeline/pdf.md
00_nucleo/prompts/infra/pipeline/raster.md
00_nucleo/prompts/infra/pipeline/resources.md
00_nucleo/prompts/infra/pipeline/svg.md
00_nucleo/prompts/infra/pipeline/tests.md
00_nucleo/prompts/infra/plugin_host/execution.md
00_nucleo/prompts/infra/plugin_host/loading.md
00_nucleo/prompts/infra/plugin_host/snapshot.md
00_nucleo/prompts/infra/plugin_host/tests.md
00_nucleo/prompts/infra/plugin_host/typst_env.md
00_nucleo/prompts/infra/shaper/bidi.md
00_nucleo/prompts/infra/shaper/cache.md
00_nucleo/prompts/infra/shaper/font_selection.md
00_nucleo/prompts/infra/shaper/reconciliation.md
00_nucleo/prompts/infra/shaper/shaping.md
00_nucleo/prompts/infra/shaper/tests.md
00_nucleo/prompts/infra/shaper/traversal.md
00_nucleo/prompts/infra/system-world/cache.md
00_nucleo/prompts/infra/system-world/fonts.md
00_nucleo/prompts/infra/system-world/packages.md
00_nucleo/prompts/infra/system-world/paths.md
00_nucleo/prompts/infra/system-world/tests.md
00_nucleo/prompts/infra/system-world/world_impl.md
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
01_core/src/compiler/stdlib/registry/microbenchmarks.rs
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
01_core/src/entities/syntax_node/errors.rs
01_core/src/entities/syntax_node/linked.rs
01_core/src/entities/syntax_node/representation.rs
03_infra/src/export/builder/fonts.rs
03_infra/src/export/builder/metadata.rs
03_infra/src/export/builder/navigation.rs
03_infra/src/export/builder/resources.rs
03_infra/src/export/builder/serialization.rs
03_infra/src/export/builder/streams.rs
03_infra/src/export/images/icc.rs
03_infra/src/export/images/jpeg.rs
03_infra/src/export/images/png.rs
03_infra/src/export/images/scan.rs
03_infra/src/export/images/xobjects.rs
03_infra/src/export/stream/draw.rs
03_infra/src/export/stream/page.rs
03_infra/src/export/stream/shape.rs
03_infra/src/export/stream/text.rs
03_infra/src/font_metrics/basic.rs
03_infra/src/font_metrics/caches.rs
03_infra/src/font_metrics/fallback.rs
03_infra/src/font_metrics/math.rs
03_infra/src/font_metrics/resolution.rs
03_infra/src/pipeline/bundle.rs
03_infra/src/pipeline/html.rs
03_infra/src/pipeline/paged.rs
03_infra/src/pipeline/pdf.rs
03_infra/src/pipeline/raster.rs
03_infra/src/pipeline/resources.rs
03_infra/src/pipeline/svg.rs
03_infra/src/pipeline/tests.rs
03_infra/src/plugin_host/execution.rs
03_infra/src/plugin_host/loading.rs
03_infra/src/plugin_host/snapshot.rs
03_infra/src/plugin_host/tests.rs
03_infra/src/plugin_host/typst_env.rs
03_infra/src/shaper/bidi.rs
03_infra/src/shaper/cache.rs
03_infra/src/shaper/font_selection.rs
03_infra/src/shaper/reconciliation.rs
03_infra/src/shaper/shaping.rs
03_infra/src/shaper/tests.rs
03_infra/src/shaper/traversal.rs
03_infra/src/world/cache.rs
03_infra/src/world/fonts.rs
03_infra/src/world/packages.rs
03_infra/src/world/paths.rs
03_infra/src/world/tests.rs
03_infra/src/world/world_impl.rs
benches/Cargo.toml.orig
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
benches/layout/fixtures.rs
benches/layout/measurement.rs
benches/layout/style_scope.rs
benches/layout_bench.rs
benches/pdf_builder_bench.rs
benches/pipeline_bench.rs
benches/stream_bench.rs
```

O próprio diagnóstico é acrescentado após esta captura. Alterações posteriores restritas a marcadores de linhagem não modificam os algoritmos testados.

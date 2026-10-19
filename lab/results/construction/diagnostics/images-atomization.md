# Atomização de images no exportador PDF

## Escopo e resultado

ICC, leitura de cabeçalhos JPEG, decodificação raster PNG/GIF/WebP,
varredura/deduplicação/validação e construção de XObjects PDF têm consumers
próprios. O hub mantém tipos, compress_zlib e reexports nos caminhos existentes.
Preserva visibilidade, assinaturas, catálogos e comportamento por defeito.
O nome process_png_for_pdf continua histórico e formato-genérico.

| Consumer | Linhas após formatação | Testes locais |
| --- | ---: | ---: |
| `03_infra/src/export/images.rs` | 71 | 0 |
| `03_infra/src/export/images/icc.rs` | 156 | 2 |
| `03_infra/src/export/images/jpeg.rs` | 81 | 1 |
| `03_infra/src/export/images/png.rs` | 102 | 1 |
| `03_infra/src/export/images/scan.rs` | 368 | 2 |
| `03_infra/src/export/images/xobjects.rs` | 111 | 1 |

O source original tinha 625 linhas; o hub ficou com 71. Os 20 corpos de
funções produtivas foram comparados antes/depois (normalização de whitespace
e vírgulas de formatação) e preservados. Essa auditoria é de movimentação
mecânica, não prova nova de paridade da linguagem.

Os arrays ICC foram comparados byte a byte e mantêm identidade:

| Perfil | Bytes | SHA-256 |
| --- | ---: | --- |
| sRGB v4 | 480 | c56e1685d888f5edb92fe07f2750f387f8fe8e91b32ff8fb0b56bfbbb9458353 |
| sRGB v2 | 736 | af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4 |

Cada consumer tem owner 1:1 em 00_nucleo/prompts/infra/export/images/;
images.md é o owner do hub. O L0 anterior está registrado em
00_nucleo/diagnosticos/images-l0-before-atomization.md.

Medição corrigida no novo L0: validate_document_images chama diretamente os
decoders; o cache thread-local por ponteiro era consultado apenas no scan.
A frase histórica que atribuía memoização à validação não correspondia ao
código. Foi corrigida a documentação, preservando o comportamento atual,
incluindo escopo/lifetime/política do cache. Não se amplia garantia de cache
para além do documento.

## Validação

Baseline E2E antes da movimentação:

```text
test result: ok. 359 passed; 0 failed; 0 ignored; 0 measured; 717 filtered out; finished in 0.58s
```

Suíte E2E após movimentação:

```text
test result: ok. 359 passed; 0 failed; 0 ignored; 0 measured; 734 filtered out; finished in 0.52s
```

Testes locais:

```text
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 1086 filtered out; finished in 0.01s
```

Cobrem versão/cabeçalhos ICC e roundtrip do perfil comprimido; SOF0/SOF2,
canais grayscale/RGB/CMYK e headers truncados; pixels RGB/alpha descomprimidos;
Arc compartilhado versus bytes iguais em outro Arc dentro de Groups; ordem
SMask/main e validação de imagem aninhada; streams binários, ICCBased e
referência opcional SMask. Os testes E2E existentes não foram editados.
A primeira compilação dos testes locais, ainda sem implementações nos novos
consumers, acusou símbolos ausentes. Depois da movimentação os testes passam.
A verificação de identidade SHA-256 dos perfis foi feita na auditoria de
bytes, sem acrescentar dependência ao crate para os testes Rust.

```sh
cargo test -p typst-infra --lib export::images -- --nocapture
cargo test -p typst-infra --lib export::tests -- --nocapture
cargo build
crystalline-lint --checks v5,v15,v26 .
crystalline-lint .
rustfmt --edition 2024 --check 03_infra/src/export/images.rs
```

Build, suites e linter completo terminaram com exit code 0. V5/V15/V26 não
apontaram problemas no escopo images. O linter completo preserva informação
V19 dos braços agrupados deslocados e avisos fora deste escopo nas alterações
concorrentes. Não houve correção de código alheio para obter esse resultado.

## Proveniência

HEAD: 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5. Working tree não commitado,
com edições concorrentes. Snapshot source/inventário anterior capturado em
2026-09-26T12:11:12Z; suíte local e comparação de fontes/ICC conferidas em
2026-09-26T12:18:30Z. Suíte E2E final, build e inventário final conferidos em
2026-09-26T12:19:11Z. Os inventários abaixo identificam todos os arquivos
alterados e não rastreados nos dois estados capturados.

Identidades das fontes desta extração:

```text
03e35e9d079571cee19b1a8d2e32f8857a3a7f4b0ff7707119cd2d417e2db33d  /tmp/images-before.rs
a4fff522cf18e5262da5a2dcc14912a6fff982071fc0caa2495add6916cbf475  03_infra/src/export/images.rs
415db95aad151ca352b5b41ed93563dfe54527b7c5301be8cd4dd26883a1cc38  03_infra/src/export/images/icc.rs
ab9aca09870ad26942366b3adf99e548e64b4c00a7eea38eeca258577b93ebc1  03_infra/src/export/images/jpeg.rs
b23cfc69109a31da482fb2a5d0e10d347724051d9c7db13773a24f4ba7088ed1  03_infra/src/export/images/png.rs
bc9d13c9094402903a08526d9fa91925a6bc36fb26e629c417a062542f070665  03_infra/src/export/images/scan.rs
83d56f10e1c169dab5d7340d5df6ef1200bb4d834ff0606dbf22e43e0ad3920d  03_infra/src/export/images/xobjects.rs
```

<details>
<summary>Antes: git diff HEAD --stat</summary>

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
 127 files changed, 4141 insertions(+), 62673 deletions(-)
```

</details>

<details>
<summary>Antes: git status --short</summary>

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
 M 00_nucleo/prompts/entities/syntax-node.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/infra/system-world.md
 M 01_core/Cargo.toml
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
 M 01_core/src/compiler/layout/page_geometry.rs
 M 01_core/src/compiler/layout/placement.rs
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
 M 01_core/src/entities/syntax_node.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/export/builder.rs
 M 03_infra/src/export/html.rs
 M 03_infra/src/export/svg.rs
 M 03_infra/src/export/tests/mod.rs
 M 03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs
 M 03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs
 M 03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs
 M 03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs
 M 03_infra/src/font_metrics.rs
 M 03_infra/src/integration_tests.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/query_helpers.rs
 M 03_infra/src/shaper.rs
 M 03_infra/src/watch.rs
 M 03_infra/tests/p1250_svg_integrated.rs
 M 04_wiring/src/main.rs
 M 04_wiring/tests/cli.rs
 M Cargo.lock
 M benches/Cargo.toml
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-archive/
?? 00_nucleo/diagnosticos/adr-cleanup-classification.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/font-metrics-atomization.md
?? 00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
?? 00_nucleo/diagnosticos/observation-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
?? 00_nucleo/diagnosticos/pipeline-atomization.md
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks-baseline.json
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks.md
?? 00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md
?? 00_nucleo/diagnosticos/stdlib-microbenchmarks.md
?? 00_nucleo/diagnosticos/stdlib-registry-atomization.md
?? 00_nucleo/diagnosticos/stdlib-tests-atomization.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/materialization/tekt-english-reference-migration.md
?? 00_nucleo/prompts/_nuclei/eval/observation-relation.toml
?? 00_nucleo/prompts/compiler/eval/observation.md
?? 00_nucleo/prompts/compiler/eval/observation/
?? 00_nucleo/prompts/compiler/layout.md.orig
?? 00_nucleo/prompts/compiler/layout/equation.md.orig
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/measurement.md.orig
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md.orig
?? 00_nucleo/prompts/compiler/stdlib/registry/
?? 00_nucleo/prompts/compiler/stdlib/tests/
?? 00_nucleo/prompts/entities/content/
?? 00_nucleo/prompts/entities/syntax-node/
?? 00_nucleo/prompts/infra/export/builder/
?? 00_nucleo/prompts/infra/font_metrics/
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 00_nucleo/prompts/infra/shaper/
?? 00_nucleo/prompts/infra/system-world/
?? 01_core/src/compiler/eval/observation/
?? 01_core/src/compiler/layout/equation.rs.orig
?? 01_core/src/compiler/layout/measurement.rs
?? 01_core/src/compiler/layout/measurement.rs.orig
?? 01_core/src/compiler/layout/mod.rs.orig
?? 01_core/src/compiler/layout/page_geometry.rs.orig
?? 01_core/src/compiler/layout/par_line_callbacks.rs
?? 01_core/src/compiler/layout/par_line_callbacks.rs.orig
?? 01_core/src/compiler/layout/placement.rs.orig
?? 01_core/src/compiler/layout/style_scope.rs
?? 01_core/src/compiler/layout/style_scope.rs.orig
?? 01_core/src/compiler/stdlib/registry/
?? 01_core/src/compiler/stdlib/tests/
?? 01_core/src/entities/content/
?? 03_infra/src/export/builder/
?? 03_infra/src/font_metrics/
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? 03_infra/src/shaper/
?? benches/Cargo.toml.orig
?? benches/content/
?? benches/content_bench.rs
?? benches/layout/
?? benches/layout_bench.rs
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
```

</details>

<details>
<summary>Depois: git diff HEAD --stat</summary>

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

</details>

<details>
<summary>Depois: git status --short</summary>

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
 M 00_nucleo/prompts/entities/syntax-node.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/images.md
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/export/stream.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/plugin_host.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/infra/system-world.md
 M 01_core/Cargo.toml
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
 M 01_core/src/compiler/layout/page_geometry.rs
 M 01_core/src/compiler/layout/placement.rs
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
 M 01_core/src/entities/syntax_node.rs
 M 02_shell/src/cli.rs
 M 03_infra/src/export/builder.rs
 M 03_infra/src/export/html.rs
 M 03_infra/src/export/images.rs
 M 03_infra/src/export/stream.rs
 M 03_infra/src/export/svg.rs
 M 03_infra/src/export/tests/mod.rs
 M 03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs
 M 03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs
 M 03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs
 M 03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs
 M 03_infra/src/font_metrics.rs
 M 03_infra/src/integration_tests.rs
 M 03_infra/src/pipeline.rs
 M 03_infra/src/pipeline/context_stabilization.rs
 M 03_infra/src/plugin_host.rs
 M 03_infra/src/query_helpers.rs
 M 03_infra/src/shaper.rs
 M 03_infra/src/watch.rs
 M 03_infra/src/world.rs
 M 03_infra/tests/p1250_svg_integrated.rs
 M 04_wiring/src/main.rs
 M 04_wiring/tests/cli.rs
 M Cargo.lock
 M benches/Cargo.toml
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-archive/
?? 00_nucleo/diagnosticos/adr-cleanup-classification.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/font-metrics-atomization.md
?? 00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
?? 00_nucleo/diagnosticos/images-l0-before-atomization.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
?? 00_nucleo/diagnosticos/observation-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
?? 00_nucleo/diagnosticos/pipeline-atomization.md
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks-baseline.json
?? 00_nucleo/diagnosticos/pipeline-microbenchmarks.md
?? 00_nucleo/diagnosticos/plugin-host-atomization.md
?? 00_nucleo/diagnosticos/plugin-host-l0-before-atomization.md
?? 00_nucleo/diagnosticos/shaper-atomizacao-2026-09-26.md
?? 00_nucleo/diagnosticos/stdlib-microbenchmarks.md
?? 00_nucleo/diagnosticos/stdlib-registry-atomization.md
?? 00_nucleo/diagnosticos/stdlib-tests-atomization.md
?? 00_nucleo/diagnosticos/stream-atomization.md
?? 00_nucleo/diagnosticos/syntax-node-atomization.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/materialization/tekt-english-reference-migration.md
?? 00_nucleo/prompts/_nuclei/eval/observation-relation.toml
?? 00_nucleo/prompts/compiler/eval/observation.md
?? 00_nucleo/prompts/compiler/eval/observation/
?? 00_nucleo/prompts/compiler/layout.md.orig
?? 00_nucleo/prompts/compiler/layout/equation.md.orig
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/measurement.md.orig
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md.orig
?? 00_nucleo/prompts/compiler/stdlib/registry/
?? 00_nucleo/prompts/compiler/stdlib/tests/
?? 00_nucleo/prompts/entities/content/
?? 00_nucleo/prompts/entities/syntax-node/
?? 00_nucleo/prompts/infra/export/builder/
?? 00_nucleo/prompts/infra/export/images/
?? 00_nucleo/prompts/infra/export/stream/
?? 00_nucleo/prompts/infra/font_metrics/
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 00_nucleo/prompts/infra/plugin_host/
?? 00_nucleo/prompts/infra/shaper/
?? 00_nucleo/prompts/infra/system-world/
?? 01_core/src/compiler/eval/observation/
?? 01_core/src/compiler/layout/equation.rs.orig
?? 01_core/src/compiler/layout/measurement.rs
?? 01_core/src/compiler/layout/measurement.rs.orig
?? 01_core/src/compiler/layout/mod.rs.orig
?? 01_core/src/compiler/layout/page_geometry.rs.orig
?? 01_core/src/compiler/layout/par_line_callbacks.rs
?? 01_core/src/compiler/layout/par_line_callbacks.rs.orig
?? 01_core/src/compiler/layout/placement.rs.orig
?? 01_core/src/compiler/layout/style_scope.rs
?? 01_core/src/compiler/layout/style_scope.rs.orig
?? 01_core/src/compiler/stdlib/registry/
?? 01_core/src/compiler/stdlib/tests/
?? 01_core/src/entities/content/
?? 01_core/src/entities/syntax_node/
?? 03_infra/src/export/builder/
?? 03_infra/src/export/images/
?? 03_infra/src/export/stream/
?? 03_infra/src/font_metrics/
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? 03_infra/src/plugin_host/
?? 03_infra/src/shaper/
?? 03_infra/src/world/
?? benches/Cargo.toml.orig
?? benches/content/
?? benches/content_bench.rs
?? benches/layout/
?? benches/layout_bench.rs
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
?? benches/stream_bench.rs
```

</details>

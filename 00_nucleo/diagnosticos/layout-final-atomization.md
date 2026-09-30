# Finalização dos braços de layout — ADR-0109

## Medição e proveniência anterior à decisão

HEAD 6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5, working tree não commitada. Captura inicial 2026-09-26T23:34:06.813Z; captura final 2026-09-26T23:43:20.198Z. Modificações substanciais preexistentes e concorrentes foram preservadas. Fonte inicial mod.rs: 2795 linhas, SHA-256 1107c3cfdba3c939b02a9afa53332d190702053b8cb0c79142b0cfbab921f9a3. L0 inicial: SHA-256 c2c06b2733965e1657bf03e4ebbacc494585dd6dfca2d939a86acdb56a6ffa7a, header efetivo 4b9ee811. Fonte congelada e snapshots em /tmp/layout-finish-baseline.json.

Em 01_core/src/compiler/layout/mod.rs, corpos restantes: StateDisplay :1495, CounterDisplayCallback :1514, Space/SpaceFromNewline :1528, Parbreak :1549, CounterDisplay :1612, OutlineEntry :1712, Align :1782 e ContextBlock :2007. Esta medição localiza mecânica; não é nova medição de paridade vanilla.

## Decisão e execução

L0 atualizado primeiro: owner central, testes e oito owners comportamentais 1:1. Sete novos módulos privados descendentes mais parbreak.rs existente. Cada braço delega a free function pub(super) layout genérica em FontMetrics/ImageSizer. Preservados enum fechado, match exaustivo e despacho estático. Não se introduziu dyn/vtable, import reverso, pub(crate), contrato público, campo, default ou fase.

StateDisplay/CounterDisplayCallback somente consomem conteúdo pré-realizado causal. Space/SpaceFromNewline conservam supressão, avanço, justificação e wrap. Parbreak conserva marcador, spacing/leading e colapso. CounterDisplay mantém resolução location-aware e fallback. OutlineEntry conserva body válido/erro ignorado. Align delega ao helper existente de placement. ContextBlock mantém captura opt-in, geometria e imaterialidade; closures nunca são executadas pelo layouter.

Comparação /tmp/layout-finish-verify.cjs confirma oito corpos conservados após normalizar whitespace, self→layouter e caminhos locais de parbreak. Reconstruído o dispatcher, o núcleo é byte-idêntico ao baseline, exceto headers e sete declarações de módulo. Estado, checkpoint, entrypoints, demais braços e ordem não mudaram. mark_boundary preexistente permanece igual modulo import e formatação local. mod.rs: 2795 → 2692 linhas. Nenhuma alegação de performance ou microbenchmark.

## Testes e validação

Adicionados 11 testes focais nos owners e um gate estrutural em tests.rs; testes anteriores preservados.

RED estrutural isolado: cargo test --offline -p typst-core --lib atomized_remaining_behavior_arms_delegate_statically falhou antes da extração por delegação ausente. Na preparação, fixtures tiveram erros de compilação e dois testes confundiram Location instalada com a nova Location do dispatcher: corrigidos para testar diretamente a função dona. Não se tratou isso como regressão produtiva. A primeira compilação da extração também precisou do import html_frame em parbreak, corrigido antes do GREEN.

GREEN focal: cargo test --offline -p typst-core --lib atomized, 12 aprovados. Suíte integral solicitada: cargo test -p typst-core --lib:
```text
test result: ok. 6280 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 8.14s
```

Os dois ignored são preexistentes; nenhum novo teste foi ignorado. cargo build --offline e crystalline-lint .: exit 0. Gates v3/v4/v5/v13/v14/v15/v26: exit 0, No violations found. Build/lint integral ainda contêm warnings/infos preexistentes, e infos V20 de padrões movidos para parbreak; não se afirma zero warnings global. Resselo apenas dos dez pares afetados, incluindo Hash do Código e @prompt-hash; nenhum --fix-hashes global.

Rustfmt limitado aos oito owners comportamentais com skip_children=true. mod.rs e tests.rs não passaram pelo formatador. Nenhum .orig foi escrito/alterado. git diff --check passou. Logs temporários: /tmp/layout-finish-{structural-red,green,core-tests,build,final-gates,full-lint,conservation}.log.

## Consumers finais

| Consumer | Linhas | SHA-256 integral | Prompt efetivo |
| --- | ---: | --- | --- |
| 01_core/src/compiler/layout/mod.rs | 2692 | 140c63bb283726aa1682b22b040465356404661007c479c652c69ad5ee9265a3 | 5341cc52 |
| 01_core/src/compiler/layout/tests.rs | 23157 | af3283885894ab374ae86e2f0851a48ee4e777f146da91b0ddd37b82b2c57b0a | 51620bf1 |
| 01_core/src/compiler/layout/state_display.rs | 72 | dcc6ab1157af2b823ee848afbb5b86e29407e6173c8919004bd2dab6e41c6846 | 7a493dbf |
| 01_core/src/compiler/layout/counter_display_callback.rs | 71 | 6bb2148cd8722d92de290689d36fc7f400dd82cdebcedceef0293f53b7f10cfe | ecd13d40 |
| 01_core/src/compiler/layout/space.rs | 81 | 5f80957d21f27bc3e91a0e7377ff2211c7101e9094386ae4513efa50af9312ea | 456be9aa |
| 01_core/src/compiler/layout/parbreak.rs | 141 | b884c4b47cc9a18c9daaacc674aaf3fae9ffde514fd4fc919e72ee7fb4992e15 | 20c8e4b9 |
| 01_core/src/compiler/layout/counter_display.rs | 56 | 567b98840cf12caebc79c87c010c9d6da9167dea135005dbf719434754141d69 | 889646b1 |
| 01_core/src/compiler/layout/outline_entry.rs | 36 | a73c9d8475ab9ae60b19fdba105486d0579a3ee1c804b7809d163a80aa8b24cd | ca0ed0ff |
| 01_core/src/compiler/layout/align.rs | 47 | 5c1b30f715559889eca2b2174c3c4e5200584281b5ef5e92d996f42ec1311085 | 8740ea70 |
| 01_core/src/compiler/layout/context_block.rs | 91 | 78b4ed581afe3caf339c66cc92401ccd6ecfd0819121f6f6fee2bdfada8eb5a0 | 3012ea7e |

## Árvore inicial

git diff HEAD --stat:
```text
 00_nucleo/adr/README.md                            |  4371 +-----
 00_nucleo/adr/adr-stub-vs-fallback.md              |    76 -
 00_nucleo/adr/template-adr.md                      |    79 +-
 .../adr/typst-adr-0001-estrategia-migracao.md      |     2 +-
 .../adr/typst-adr-0002-hierarquia-contencao.md     |   182 -
 00_nucleo/adr/typst-adr-0003-comemo-contencao.md   |   159 -
 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md |   198 -
 00_nucleo/adr/typst-adr-0005-packagespec-world.md  |     2 +-
 00_nucleo/adr/typst-adr-0006-typst-timing.md       |     2 +-
 00_nucleo/adr/typst-adr-0007-rustc-hash.md         |    85 -
 00_nucleo/adr/typst-adr-0008-defer-inline.md       |     2 +-
 00_nucleo/adr/typst-adr-0009-math-class.md         |     2 +-
 00_nucleo/adr/typst-adr-0010-unicode-ident.md      |     2 +-
 00_nucleo/adr/typst-adr-0011-unicode-math-class.md |     2 +-
 00_nucleo/adr/typst-adr-0012-unicode-script.md     |     2 +-
 .../adr/typst-adr-0013-unicode-segmentation.md     |     2 +-
 00_nucleo/adr/typst-adr-0014-unscanny.md           |     2 +-
 00_nucleo/adr/typst-adr-0015-ecow.md               |     2 +-
 00_nucleo/adr/typst-adr-0016-lazyhash.md           |     2 +-
 .../typst-adr-0017-adiamento-eval-typst-library.md |     2 +-
 00_nucleo/adr/typst-adr-0018-rustc-hash.md         |     2 +-
 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md      |     2 +-
 00_nucleo/adr/typst-adr-0020-fontdb.md             |     2 +-
 00_nucleo/adr/typst-adr-0021-datetime.md           |     2 +-
 00_nucleo/adr/typst-adr-0022-fontbook.md           |     2 +-
 00_nucleo/adr/typst-adr-0023-indexmap.md           |     2 +-
 00_nucleo/adr/typst-adr-0024-ecow-value.md         |     2 +-
 00_nucleo/adr/typst-adr-0025-int-eq-float.md       |     2 +-
 00_nucleo/adr/typst-adr-0026-R1-content-arc.md     |     2 +-
 .../adr/typst-adr-0026-content-divergencia.md      |     2 +-
 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md |     2 +-
 00_nucleo/adr/typst-adr-0028-typographic-types.md  |    52 -
 ...typst-adr-0029-pureza-fisica-revoga-adr-0028.md |     2 +-
 .../adr/typst-adr-0030-performance-dominio-l1.md   |     2 +-
 .../adr/typst-adr-0031-early-hashing-source.md     |     2 +-
 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md       |     2 +-
 .../typst-adr-0033-paridade-funcional-vanilla.md   |     2 +-
 .../typst-adr-0034-diagnostico-tipos-vanilla.md    |     2 +-
 00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md |     2 +-
 .../adr/typst-adr-0036-atomizacao-progressiva.md   |     2 +-
 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md |     2 +-
 00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md |     2 +-
 00_nucleo/adr/typst-adr-0039-frameitem-style.md    |     2 +-
 00_nucleo/adr/typst-adr-0040-activacao-set-rule.md |     2 +-
 .../adr/typst-adr-0041-activacao-show-rule.md      |     2 +-
 00_nucleo/adr/typst-adr-0042-sink-materializado.md |     2 +-
 00_nucleo/adr/typst-adr-0043-canal-sink-saida.md   |     2 +-
 00_nucleo/adr/typst-adr-0044-engine-agregador.md   |     2 +-
 .../adr/typst-adr-0045-formato-diagnosticos.md     |     3 +-
 00_nucleo/adr/typst-adr-0046-cli-minima.md         |     2 +-
 00_nucleo/adr/typst-adr-0047-cli-clap.md           |     2 +-
 .../adr/typst-adr-0048-diagnosticos-com-cor.md     |     2 +-
 00_nucleo/adr/typst-adr-0049-cli-em-l2.md          |     2 +-
 00_nucleo/adr/typst-adr-0050-formatter-em-l2.md    |     2 +-
 00_nucleo/adr/typst-adr-0051-flags-funcionais.md   |     2 +-
 .../adr/typst-adr-0052-lang-tipo-semantico.md      |     2 +-
 00_nucleo/adr/typst-adr-0053-font-tipo-composto.md |     2 +-
 .../adr/typst-adr-0054-criterio-fecho-debt-1.md    |     2 +-
 .../adr/typst-adr-0055-font-consumer-cidfont.md    |     2 +-
 00_nucleo/adr/typst-adr-0057-lang-hyphenation.md   |     2 +-
 .../adr/typst-adr-0058-value-type-simplificado.md  |     2 +-
 00_nucleo/adr/typst-adr-0059-args-tipo-separado.md |     2 +-
 .../adr/typst-adr-0060-model-structural-roadmap.md |   779 --
 .../adr/typst-adr-0061-layout-fase-x-roadmap.md    |  1645 ---
 ...ypst-adr-0062-hayagriva-bibliography-parsing.md |     3 +-
 .../typst-adr-0064-smart-para-option-default.md    |     2 +-
 .../adr/typst-adr-0065-inventariar-primeiro.md     |     2 +-
 .../typst-adr-0066-introspection-runtime-adiada.md |   496 -
 .../typst-adr-0067-attribute-grammar-scoping.md    |     2 +-
 .../adr/typst-adr-0068-location-aware-layouter.md  |     2 +-
 .../typst-adr-0069-post-recursion-tag-emission.md  |     2 +-
 ...pst-adr-0070-eliminacao-counter-state-legacy.md |     3 +-
 .../adr/typst-adr-0071-walk-pipeline-redesign.md   |     3 +-
 .../typst-adr-0072-m7-fixpoint-runtime-fechado.md  |     2 +-
 .../adr/typst-adr-0073-comemo-introspector.md      |     3 +-
 ...pst-adr-0074-f3-layouter-substores-trackable.md |     3 +-
 .../adr/typst-adr-0075-vanilla-integration.md      |     3 +-
 .../adr/typst-adr-0076-introspector-completion.md  |     3 +-
 00_nucleo/adr/typst-adr-0077-regex-l1.md           |     3 +-
 .../adr/typst-adr-0078-column-flow-algorithm.md    |     6 +-
 .../adr/typst-adr-0079-layout-fase-5-roadmap.md    |  2628 ----
 .../typst-adr-0080-l0-minimal-para-refactors.md    |     2 +-
 ...dr-0081-m7-plus-pipeline-restructuring-scope.md |  1085 --
 ...t-adr-0082-promocoes-reais-scope-outs-graded.md |     3 +-
 ...or-paridade-vanilla-com-subset-materializado.md |     3 +-
 .../adr/typst-adr-0084-auditoria-condicional.md    |     2 +-
 .../adr/typst-adr-0085-diagnostico-imutavel.md     |     2 +-
 .../adr/typst-adr-0086-paint-wrapper-solid-only.md |     2 +-
 .../adr/typst-adr-0087-gradient-linear-only.md     |     2 +-
 .../adr/typst-adr-0088-gradient-radial-only.md     |     2 +-
 .../adr/typst-adr-0089-gradient-conic-only.md      |     2 +-
 ...090-gradient-conic-strategy-type-4-vs-type-1.md |   561 -
 ...091-gradient-space-runtime-and-cmyk-strategy.md |     3 +-
 .../typst-adr-0092-conic-coons-patches-rgb-cmyk.md |     3 +-
 ...ypst-adr-0093-meta-metodologia-evolucao-adrs.md |     3 +-
 .../adr/typst-adr-0094-meta-operacional-specs.md   |     3 +-
 .../typst-adr-0095-dedup-arc-as-ptr-resources.md   |     3 +-
 ...tern-debt-37-campo-layouter-consumer-pending.md |   259 -
 ...t-adr-0097-scope-out-reconfirmado-por-fase-a.md |   269 -
 ...8-single-source-of-truth-invariante-anti-bug.md |     3 +-
 ...-adr-0099-activacao-posterior-feature-graded.md |     3 +-
 .../adr/typst-adr-0100-coesao-por-dominio-l3.md    |     2 +-
 .../adr/typst-adr-0101-ieee754-restricao-stdlib.md |     2 +-
 .../adr/typst-adr-0102-math-style-mechanism.md     |     2 +-
 .../adr/typst-adr-0103-math-style-composition.md   |     2 +-
 .../adr/typst-adr-0104-atomicidade-para-agentes.md |     2 +-
 ...t-adr-0105-modelo-elemento-d-agora-f-destino.md |     2 +-
 .../adr/typst-adr-0106-fronteira-de-extensao-e1.md |     2 +-
 ...pst-adr-0107-paridade-linguagem-nao-mecanica.md |     2 +-
 .../adr/typst-adr-0108-disciplina-anti-deriva.md   |     2 +-
 00_nucleo/adr/typst-adr-0109-atomizacao.md         |     3 +-
 00_nucleo/adr/typst-adr-0110-marcador-vanilla.md   |     3 +-
 .../adr/typst-adr-0111-crates-parsing-loading.md   |     3 +-
 00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md    |     3 +-
 ...-0113-stub-transparente-vs-fallback-software.md |     2 +-
 .../adr/typst-adr-0114-sonda-antes-da-spec.md      |     2 +-
 .../adr/typst-adr-0115-infra-benchmark-scanner.md  |     3 +-
 ...typst-adr-0116-excecao-get-unchecked-scanner.md |     3 +-
 00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md |     3 +-
 .../adr/typst-adr-0118-runtime-state-context.md    |     3 +-
 .../adr/typst-adr-0119-disciplina-verificacao.md   |     2 +-
 .../adr/typst-adr-0120-textshaped-rustybuzz.md     |     3 +-
 .../adr/typst-adr-0121-proveniencia-medicao.md     |     2 +-
 .../adr/typst-adr-0122-paridade-defeitos-testes.md |     2 +-
 .../adr/typst-adr-0123-geometria-tipografica.md    |     2 +-
 .../adr/typst-adr-0124-checklist-sublayouts.md     |     2 +-
 .../typst-adr-0125-implementacao-vs-linguagem.md   |     2 +-
 .../adr/typst-adr-0126-modo-verboso-primeiro.md    |     2 +-
 .../adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md |     2 +-
 .../adr/typst-adr-0128-html-target-semantico.md    |     2 +-
 ...typst-adr-0129-nucleos-tekt-l0-compartilhado.md |     3 +-
 .../adr/typst-adr-0130-referencias-canonicas-l0.md |    94 +-
 00_nucleo/prompts/compiler/eval.md                 |    43 +-
 00_nucleo/prompts/compiler/layout.md               |    23 +-
 00_nucleo/prompts/compiler/layout/equation.md      |     6 +-
 00_nucleo/prompts/compiler/layout/page_geometry.md |     7 +-
 00_nucleo/prompts/compiler/layout/placement.md     |     6 +-
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/parse/parser.md         |    35 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    28 +-
 00_nucleo/prompts/entities/content.md              |  3580 +----
 00_nucleo/prompts/entities/content_hash.md         |   170 +-
 00_nucleo/prompts/entities/syntax-node.md          |   238 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 00_nucleo/prompts/infra/export/html.md             |   405 +-
 00_nucleo/prompts/infra/export/images.md           |   206 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
 00_nucleo/prompts/infra/export/stream.md           |    37 +-
 00_nucleo/prompts/infra/export/svg.md              |   556 +-
 00_nucleo/prompts/infra/font_metrics.md            |  1085 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 00_nucleo/prompts/infra/plugin_host.md             |   333 +-
 00_nucleo/prompts/infra/shaper.md                  |   954 +-
 00_nucleo/prompts/infra/system-world.md            |   431 +-
 00_nucleo/prompts/wiring.md                        |   978 +-
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
 01_core/src/compiler/parse/parser.rs               |   629 +-
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
 03_infra/src/export/html.rs                        |  1286 +-
 03_infra/src/export/images.rs                      |   596 +-
 03_infra/src/export/stream.rs                      |  2135 +--
 03_infra/src/export/svg.rs                         |  3271 +----
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
 04_wiring/src/main.rs                              |  1358 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    37 +
 benches/scanner_bench.rs                           |    11 +-
 255 files changed, 4856 insertions(+), 80715 deletions(-)
```

git status --short --untracked-files=all:
```text
 M 00_nucleo/adr/README.md
 D 00_nucleo/adr/adr-stub-vs-fallback.md
 M 00_nucleo/adr/template-adr.md
 M 00_nucleo/adr/typst-adr-0001-estrategia-migracao.md
 D 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
 D 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
 D 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md
 M 00_nucleo/adr/typst-adr-0005-packagespec-world.md
 M 00_nucleo/adr/typst-adr-0006-typst-timing.md
 D 00_nucleo/adr/typst-adr-0007-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0008-defer-inline.md
 M 00_nucleo/adr/typst-adr-0009-math-class.md
 M 00_nucleo/adr/typst-adr-0010-unicode-ident.md
 M 00_nucleo/adr/typst-adr-0011-unicode-math-class.md
 M 00_nucleo/adr/typst-adr-0012-unicode-script.md
 M 00_nucleo/adr/typst-adr-0013-unicode-segmentation.md
 M 00_nucleo/adr/typst-adr-0014-unscanny.md
 M 00_nucleo/adr/typst-adr-0015-ecow.md
 M 00_nucleo/adr/typst-adr-0016-lazyhash.md
 M 00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md
 M 00_nucleo/adr/typst-adr-0018-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0020-fontdb.md
 M 00_nucleo/adr/typst-adr-0021-datetime.md
 M 00_nucleo/adr/typst-adr-0022-fontbook.md
 M 00_nucleo/adr/typst-adr-0023-indexmap.md
 M 00_nucleo/adr/typst-adr-0024-ecow-value.md
 M 00_nucleo/adr/typst-adr-0025-int-eq-float.md
 M 00_nucleo/adr/typst-adr-0026-R1-content-arc.md
 M 00_nucleo/adr/typst-adr-0026-content-divergencia.md
 M 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md
 D 00_nucleo/adr/typst-adr-0028-typographic-types.md
 M 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md
 M 00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md
 M 00_nucleo/adr/typst-adr-0031-early-hashing-source.md
 M 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md
 M 00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md
 M 00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md
 M 00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md
 M 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md
 M 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md
 M 00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md
 M 00_nucleo/adr/typst-adr-0039-frameitem-style.md
 M 00_nucleo/adr/typst-adr-0040-activacao-set-rule.md
 M 00_nucleo/adr/typst-adr-0041-activacao-show-rule.md
 M 00_nucleo/adr/typst-adr-0042-sink-materializado.md
 M 00_nucleo/adr/typst-adr-0043-canal-sink-saida.md
 M 00_nucleo/adr/typst-adr-0044-engine-agregador.md
 M 00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md
 M 00_nucleo/adr/typst-adr-0046-cli-minima.md
 M 00_nucleo/adr/typst-adr-0047-cli-clap.md
 M 00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md
 M 00_nucleo/adr/typst-adr-0049-cli-em-l2.md
 M 00_nucleo/adr/typst-adr-0050-formatter-em-l2.md
 M 00_nucleo/adr/typst-adr-0051-flags-funcionais.md
 M 00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md
 M 00_nucleo/adr/typst-adr-0053-font-tipo-composto.md
 M 00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md
 M 00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md
 M 00_nucleo/adr/typst-adr-0057-lang-hyphenation.md
 M 00_nucleo/adr/typst-adr-0058-value-type-simplificado.md
 M 00_nucleo/adr/typst-adr-0059-args-tipo-separado.md
 D 00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md
 D 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
 M 00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md
 M 00_nucleo/adr/typst-adr-0064-smart-para-option-default.md
 M 00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md
 D 00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md
 M 00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md
 M 00_nucleo/adr/typst-adr-0068-location-aware-layouter.md
 M 00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md
 M 00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md
 M 00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md
 M 00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md
 M 00_nucleo/adr/typst-adr-0073-comemo-introspector.md
 M 00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md
 M 00_nucleo/adr/typst-adr-0075-vanilla-integration.md
 M 00_nucleo/adr/typst-adr-0076-introspector-completion.md
 M 00_nucleo/adr/typst-adr-0077-regex-l1.md
 M 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
 D 00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md
 M 00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md
 D 00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
 M 00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md
 M 00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md
 M 00_nucleo/adr/typst-adr-0084-auditoria-condicional.md
 M 00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md
 M 00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md
 M 00_nucleo/adr/typst-adr-0087-gradient-linear-only.md
 M 00_nucleo/adr/typst-adr-0088-gradient-radial-only.md
 M 00_nucleo/adr/typst-adr-0089-gradient-conic-only.md
 D 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
 M 00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md
 M 00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md
 M 00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md
 M 00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md
 M 00_nucleo/adr/typst-adr-0095-dedup-arc-as-ptr-resources.md
 D 00_nucleo/adr/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
 D 00_nucleo/adr/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
 M 00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md
 M 00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md
 M 00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md
 M 00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md
 M 00_nucleo/adr/typst-adr-0102-math-style-mechanism.md
 M 00_nucleo/adr/typst-adr-0103-math-style-composition.md
 M 00_nucleo/adr/typst-adr-0104-atomicidade-para-agentes.md
 M 00_nucleo/adr/typst-adr-0105-modelo-elemento-d-agora-f-destino.md
 M 00_nucleo/adr/typst-adr-0106-fronteira-de-extensao-e1.md
 M 00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md
 M 00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md
 M 00_nucleo/adr/typst-adr-0109-atomizacao.md
 M 00_nucleo/adr/typst-adr-0110-marcador-vanilla.md
 M 00_nucleo/adr/typst-adr-0111-crates-parsing-loading.md
 M 00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md
 M 00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md
 M 00_nucleo/adr/typst-adr-0114-sonda-antes-da-spec.md
 M 00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md
 M 00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md
 M 00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md
 M 00_nucleo/adr/typst-adr-0118-runtime-state-context.md
 M 00_nucleo/adr/typst-adr-0119-disciplina-verificacao.md
 M 00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md
 M 00_nucleo/adr/typst-adr-0122-paridade-defeitos-testes.md
 M 00_nucleo/adr/typst-adr-0123-geometria-tipografica.md
 M 00_nucleo/adr/typst-adr-0124-checklist-sublayouts.md
 M 00_nucleo/adr/typst-adr-0125-implementacao-vs-linguagem.md
 M 00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md
 M 00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md
 M 00_nucleo/adr/typst-adr-0128-html-target-semantico.md
 M 00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md
 M 00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/page_geometry.md
 M 00_nucleo/prompts/compiler/layout/placement.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 00_nucleo/prompts/compiler/parse/parser.md
 M 00_nucleo/prompts/compiler/stdlib/_comum.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/content_hash.md
 M 00_nucleo/prompts/entities/syntax-node.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/html.md
 M 00_nucleo/prompts/infra/export/images.md
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/export/stream.md
 M 00_nucleo/prompts/infra/export/svg.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/plugin_host.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/infra/system-world.md
 M 00_nucleo/prompts/wiring.md
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
 M 01_core/src/compiler/parse/parser.rs
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
 M benches/scanner_bench.rs
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-0130-step-1060-history.md
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
?? 00_nucleo/diagnosticos/adr-governance-reconciliation.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/benchmark-suite-review.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-history.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-manifest.json
?? 00_nucleo/diagnosticos/core-l0-sanitization.md
?? 00_nucleo/diagnosticos/font-metrics-atomization.md
?? 00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
?? 00_nucleo/diagnosticos/html-atomization-manifest.json
?? 00_nucleo/diagnosticos/html-atomization.md
?? 00_nucleo/diagnosticos/images-atomization.md
?? 00_nucleo/diagnosticos/images-l0-before-atomization.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
?? 00_nucleo/diagnosticos/observation-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/parser-atomization.md
?? 00_nucleo/diagnosticos/parser-l0-before-atomization.md
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
?? 00_nucleo/diagnosticos/svg-atomization.md
?? 00_nucleo/diagnosticos/svg-l0-before-atomization.md
?? 00_nucleo/diagnosticos/syntax-node-atomization.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/diagnosticos/wiring-main-atomization.md
?? 00_nucleo/diagnosticos/world-atomization-manifest.json
?? 00_nucleo/diagnosticos/world-atomization.md
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
?? 00_nucleo/prompts/compiler/layout/measurement.md.orig
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md.orig
?? 00_nucleo/prompts/compiler/parse/parser/checkpoints.md
?? 00_nucleo/prompts/compiler/parse/parser/diagnostics.md
?? 00_nucleo/prompts/compiler/parse/parser/state.md
?? 00_nucleo/prompts/compiler/stdlib/registry/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/registry/data.md
?? 00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md
?? 00_nucleo/prompts/compiler/stdlib/registry/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/registry/formats.md
?? 00_nucleo/prompts/compiler/stdlib/registry/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/registry/layout.md
?? 00_nucleo/prompts/compiler/stdlib/registry/math.md
?? 00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md
?? 00_nucleo/prompts/compiler/stdlib/registry/mod.md
?? 00_nucleo/prompts/compiler/stdlib/registry/modules.md
?? 00_nucleo/prompts/compiler/stdlib/registry/shapes.md
?? 00_nucleo/prompts/compiler/stdlib/registry/structural.md
?? 00_nucleo/prompts/compiler/stdlib/registry/table_grid.md
?? 00_nucleo/prompts/compiler/stdlib/registry/tests.md
?? 00_nucleo/prompts/compiler/stdlib/registry/text.md
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
?? 00_nucleo/prompts/entities/syntax-node/errors.md
?? 00_nucleo/prompts/entities/syntax-node/linked.md
?? 00_nucleo/prompts/entities/syntax-node/representation.md
?? 00_nucleo/prompts/infra/export/builder/fonts.md
?? 00_nucleo/prompts/infra/export/builder/metadata.md
?? 00_nucleo/prompts/infra/export/builder/navigation.md
?? 00_nucleo/prompts/infra/export/builder/resources.md
?? 00_nucleo/prompts/infra/export/builder/serialization.md
?? 00_nucleo/prompts/infra/export/builder/streams.md
?? 00_nucleo/prompts/infra/export/html/document.md
?? 00_nucleo/prompts/infra/export/html/encoding.md
?? 00_nucleo/prompts/infra/export/html/inline.md
?? 00_nucleo/prompts/infra/export/html/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/html/pretty.md
?? 00_nucleo/prompts/infra/export/html/table.md
?? 00_nucleo/prompts/infra/export/html/tags.md
?? 00_nucleo/prompts/infra/export/html/tests.md
?? 00_nucleo/prompts/infra/export/html/whitespace.md
?? 00_nucleo/prompts/infra/export/images/icc.md
?? 00_nucleo/prompts/infra/export/images/jpeg.md
?? 00_nucleo/prompts/infra/export/images/png.md
?? 00_nucleo/prompts/infra/export/images/scan.md
?? 00_nucleo/prompts/infra/export/images/xobjects.md
?? 00_nucleo/prompts/infra/export/stream/draw.md
?? 00_nucleo/prompts/infra/export/stream/page.md
?? 00_nucleo/prompts/infra/export/stream/shape.md
?? 00_nucleo/prompts/infra/export/stream/text.md
?? 00_nucleo/prompts/infra/export/svg/geometry.md
?? 00_nucleo/prompts/infra/export/svg/glyphs.md
?? 00_nucleo/prompts/infra/export/svg/groups.md
?? 00_nucleo/prompts/infra/export/svg/images.md
?? 00_nucleo/prompts/infra/export/svg/navigation.md
?? 00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/svg/paints.md
?? 00_nucleo/prompts/infra/export/svg/serialization.md
?? 00_nucleo/prompts/infra/export/svg/tests.md
?? 00_nucleo/prompts/infra/font_metrics/basic.md
?? 00_nucleo/prompts/infra/font_metrics/caches.md
?? 00_nucleo/prompts/infra/font_metrics/fallback.md
?? 00_nucleo/prompts/infra/font_metrics/math.md
?? 00_nucleo/prompts/infra/font_metrics/resolution.md
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 00_nucleo/prompts/infra/plugin_host/execution.md
?? 00_nucleo/prompts/infra/plugin_host/loading.md
?? 00_nucleo/prompts/infra/plugin_host/snapshot.md
?? 00_nucleo/prompts/infra/plugin_host/tests.md
?? 00_nucleo/prompts/infra/plugin_host/typst_env.md
?? 00_nucleo/prompts/infra/shaper/bidi.md
?? 00_nucleo/prompts/infra/shaper/cache.md
?? 00_nucleo/prompts/infra/shaper/font_selection.md
?? 00_nucleo/prompts/infra/shaper/reconciliation.md
?? 00_nucleo/prompts/infra/shaper/shaping.md
?? 00_nucleo/prompts/infra/shaper/tests.md
?? 00_nucleo/prompts/infra/shaper/traversal.md
?? 00_nucleo/prompts/infra/system-world/cache.md
?? 00_nucleo/prompts/infra/system-world/fonts.md
?? 00_nucleo/prompts/infra/system-world/packages.md
?? 00_nucleo/prompts/infra/system-world/paths.md
?? 00_nucleo/prompts/infra/system-world/tests.md
?? 00_nucleo/prompts/infra/system-world/world_impl.md
?? 00_nucleo/prompts/wiring/compile.md
?? 00_nucleo/prompts/wiring/completions.md
?? 00_nucleo/prompts/wiring/diagnostics.md
?? 00_nucleo/prompts/wiring/eval.md
?? 00_nucleo/prompts/wiring/fonts.md
?? 00_nucleo/prompts/wiring/info.md
?? 00_nucleo/prompts/wiring/init.md
?? 00_nucleo/prompts/wiring/query.md
?? 00_nucleo/prompts/wiring/watch.md
?? 01_core/src/compiler/eval/observation/content.rs
?? 01_core/src/compiler/eval/observation/elements.rs
?? 01_core/src/compiler/eval/observation/geometry.rs
?? 01_core/src/compiler/eval/observation/math.rs
?? 01_core/src/compiler/eval/observation/mod.rs
?? 01_core/src/compiler/eval/observation/page.rs
?? 01_core/src/compiler/eval/observation/style.rs
?? 01_core/src/compiler/eval/observation/value.rs
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
?? 01_core/src/compiler/parse/parser/checkpoints.rs
?? 01_core/src/compiler/parse/parser/diagnostics.rs
?? 01_core/src/compiler/parse/parser/state.rs
?? 01_core/src/compiler/stdlib/registry/contextual.rs
?? 01_core/src/compiler/stdlib/registry/data.rs
?? 01_core/src/compiler/stdlib/registry/diagnostics.rs
?? 01_core/src/compiler/stdlib/registry/figure_image.rs
?? 01_core/src/compiler/stdlib/registry/formats.rs
?? 01_core/src/compiler/stdlib/registry/foundations.rs
?? 01_core/src/compiler/stdlib/registry/layout.rs
?? 01_core/src/compiler/stdlib/registry/math.rs
?? 01_core/src/compiler/stdlib/registry/microbenchmarks.rs
?? 01_core/src/compiler/stdlib/registry/mod.rs
?? 01_core/src/compiler/stdlib/registry/modules.rs
?? 01_core/src/compiler/stdlib/registry/shapes.rs
?? 01_core/src/compiler/stdlib/registry/structural.rs
?? 01_core/src/compiler/stdlib/registry/table_grid.rs
?? 01_core/src/compiler/stdlib/registry/tests.rs
?? 01_core/src/compiler/stdlib/registry/text.rs
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
?? 01_core/src/entities/syntax_node/errors.rs
?? 01_core/src/entities/syntax_node/linked.rs
?? 01_core/src/entities/syntax_node/representation.rs
?? 03_infra/src/export/builder/fonts.rs
?? 03_infra/src/export/builder/metadata.rs
?? 03_infra/src/export/builder/navigation.rs
?? 03_infra/src/export/builder/resources.rs
?? 03_infra/src/export/builder/serialization.rs
?? 03_infra/src/export/builder/streams.rs
?? 03_infra/src/export/html/document.rs
?? 03_infra/src/export/html/encoding.rs
?? 03_infra/src/export/html/inline.rs
?? 03_infra/src/export/html/p1360_ab_tests.rs
?? 03_infra/src/export/html/pretty.rs
?? 03_infra/src/export/html/table.rs
?? 03_infra/src/export/html/tags.rs
?? 03_infra/src/export/html/tests.rs
?? 03_infra/src/export/html/whitespace.rs
?? 03_infra/src/export/images/icc.rs
?? 03_infra/src/export/images/jpeg.rs
?? 03_infra/src/export/images/png.rs
?? 03_infra/src/export/images/scan.rs
?? 03_infra/src/export/images/xobjects.rs
?? 03_infra/src/export/stream/draw.rs
?? 03_infra/src/export/stream/page.rs
?? 03_infra/src/export/stream/shape.rs
?? 03_infra/src/export/stream/text.rs
?? 03_infra/src/export/svg/geometry.rs
?? 03_infra/src/export/svg/glyphs.rs
?? 03_infra/src/export/svg/groups.rs
?? 03_infra/src/export/svg/images.rs
?? 03_infra/src/export/svg/navigation.rs
?? 03_infra/src/export/svg/p1360_ab_tests.rs
?? 03_infra/src/export/svg/paints.rs
?? 03_infra/src/export/svg/serialization.rs
?? 03_infra/src/export/svg/tests.rs
?? 03_infra/src/font_metrics/basic.rs
?? 03_infra/src/font_metrics/caches.rs
?? 03_infra/src/font_metrics/fallback.rs
?? 03_infra/src/font_metrics/math.rs
?? 03_infra/src/font_metrics/resolution.rs
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? 03_infra/src/plugin_host/execution.rs
?? 03_infra/src/plugin_host/loading.rs
?? 03_infra/src/plugin_host/snapshot.rs
?? 03_infra/src/plugin_host/tests.rs
?? 03_infra/src/plugin_host/typst_env.rs
?? 03_infra/src/shaper/bidi.rs
?? 03_infra/src/shaper/cache.rs
?? 03_infra/src/shaper/font_selection.rs
?? 03_infra/src/shaper/reconciliation.rs
?? 03_infra/src/shaper/shaping.rs
?? 03_infra/src/shaper/tests.rs
?? 03_infra/src/shaper/traversal.rs
?? 03_infra/src/world/cache.rs
?? 03_infra/src/world/fonts.rs
?? 03_infra/src/world/packages.rs
?? 03_infra/src/world/paths.rs
?? 03_infra/src/world/tests.rs
?? 03_infra/src/world/world_impl.rs
?? 04_wiring/src/compile.rs
?? 04_wiring/src/completions.rs
?? 04_wiring/src/diagnostics.rs
?? 04_wiring/src/eval.rs
?? 04_wiring/src/fonts.rs
?? 04_wiring/src/info.rs
?? 04_wiring/src/init.rs
?? 04_wiring/src/query.rs
?? 04_wiring/src/watch.rs
?? benches/Cargo.toml.orig
?? benches/README.md
?? benches/build.rs
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
?? benches/layout/fixtures.rs
?? benches/layout/measurement.rs
?? benches/layout/style_scope.rs
?? benches/layout_bench.rs
?? benches/observation_bench.rs
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
?? benches/run_core_benchmark.py
?? benches/stdlib_bench.rs
?? benches/stream_bench.rs
?? benches/support/core_runner.rs
?? benches/support/profile.rs
?? benches/tests/test_benchmark_guards.py
```

## Árvore final antes deste diagnóstico

git diff HEAD --stat (não inclui untracked):
```text
 00_nucleo/adr/README.md                            |  4371 +-----
 00_nucleo/adr/adr-stub-vs-fallback.md              |    76 -
 00_nucleo/adr/template-adr.md                      |    79 +-
 .../adr/typst-adr-0001-estrategia-migracao.md      |     2 +-
 .../adr/typst-adr-0002-hierarquia-contencao.md     |   182 -
 00_nucleo/adr/typst-adr-0003-comemo-contencao.md   |   159 -
 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md |   198 -
 00_nucleo/adr/typst-adr-0005-packagespec-world.md  |     2 +-
 00_nucleo/adr/typst-adr-0006-typst-timing.md       |     2 +-
 00_nucleo/adr/typst-adr-0007-rustc-hash.md         |    85 -
 00_nucleo/adr/typst-adr-0008-defer-inline.md       |     2 +-
 00_nucleo/adr/typst-adr-0009-math-class.md         |     2 +-
 00_nucleo/adr/typst-adr-0010-unicode-ident.md      |     2 +-
 00_nucleo/adr/typst-adr-0011-unicode-math-class.md |     2 +-
 00_nucleo/adr/typst-adr-0012-unicode-script.md     |     2 +-
 .../adr/typst-adr-0013-unicode-segmentation.md     |     2 +-
 00_nucleo/adr/typst-adr-0014-unscanny.md           |     2 +-
 00_nucleo/adr/typst-adr-0015-ecow.md               |     2 +-
 00_nucleo/adr/typst-adr-0016-lazyhash.md           |     2 +-
 .../typst-adr-0017-adiamento-eval-typst-library.md |     2 +-
 00_nucleo/adr/typst-adr-0018-rustc-hash.md         |     2 +-
 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md      |     2 +-
 00_nucleo/adr/typst-adr-0020-fontdb.md             |     2 +-
 00_nucleo/adr/typst-adr-0021-datetime.md           |     2 +-
 00_nucleo/adr/typst-adr-0022-fontbook.md           |     2 +-
 00_nucleo/adr/typst-adr-0023-indexmap.md           |     2 +-
 00_nucleo/adr/typst-adr-0024-ecow-value.md         |     2 +-
 00_nucleo/adr/typst-adr-0025-int-eq-float.md       |     2 +-
 00_nucleo/adr/typst-adr-0026-R1-content-arc.md     |     2 +-
 .../adr/typst-adr-0026-content-divergencia.md      |     2 +-
 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md |     2 +-
 00_nucleo/adr/typst-adr-0028-typographic-types.md  |    52 -
 ...typst-adr-0029-pureza-fisica-revoga-adr-0028.md |     2 +-
 .../adr/typst-adr-0030-performance-dominio-l1.md   |     2 +-
 .../adr/typst-adr-0031-early-hashing-source.md     |     2 +-
 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md       |     2 +-
 .../typst-adr-0033-paridade-funcional-vanilla.md   |     2 +-
 .../typst-adr-0034-diagnostico-tipos-vanilla.md    |     2 +-
 00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md |     2 +-
 .../adr/typst-adr-0036-atomizacao-progressiva.md   |     2 +-
 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md |     2 +-
 00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md |     2 +-
 00_nucleo/adr/typst-adr-0039-frameitem-style.md    |     2 +-
 00_nucleo/adr/typst-adr-0040-activacao-set-rule.md |     2 +-
 .../adr/typst-adr-0041-activacao-show-rule.md      |     2 +-
 00_nucleo/adr/typst-adr-0042-sink-materializado.md |     2 +-
 00_nucleo/adr/typst-adr-0043-canal-sink-saida.md   |     2 +-
 00_nucleo/adr/typst-adr-0044-engine-agregador.md   |     2 +-
 .../adr/typst-adr-0045-formato-diagnosticos.md     |     3 +-
 00_nucleo/adr/typst-adr-0046-cli-minima.md         |     2 +-
 00_nucleo/adr/typst-adr-0047-cli-clap.md           |     2 +-
 .../adr/typst-adr-0048-diagnosticos-com-cor.md     |     2 +-
 00_nucleo/adr/typst-adr-0049-cli-em-l2.md          |     2 +-
 00_nucleo/adr/typst-adr-0050-formatter-em-l2.md    |     2 +-
 00_nucleo/adr/typst-adr-0051-flags-funcionais.md   |     2 +-
 .../adr/typst-adr-0052-lang-tipo-semantico.md      |     2 +-
 00_nucleo/adr/typst-adr-0053-font-tipo-composto.md |     2 +-
 .../adr/typst-adr-0054-criterio-fecho-debt-1.md    |     2 +-
 .../adr/typst-adr-0055-font-consumer-cidfont.md    |     2 +-
 00_nucleo/adr/typst-adr-0057-lang-hyphenation.md   |     2 +-
 .../adr/typst-adr-0058-value-type-simplificado.md  |     2 +-
 00_nucleo/adr/typst-adr-0059-args-tipo-separado.md |     2 +-
 .../adr/typst-adr-0060-model-structural-roadmap.md |   779 --
 .../adr/typst-adr-0061-layout-fase-x-roadmap.md    |  1645 ---
 ...ypst-adr-0062-hayagriva-bibliography-parsing.md |     3 +-
 .../typst-adr-0064-smart-para-option-default.md    |     2 +-
 .../adr/typst-adr-0065-inventariar-primeiro.md     |     2 +-
 .../typst-adr-0066-introspection-runtime-adiada.md |   496 -
 .../typst-adr-0067-attribute-grammar-scoping.md    |     2 +-
 .../adr/typst-adr-0068-location-aware-layouter.md  |     2 +-
 .../typst-adr-0069-post-recursion-tag-emission.md  |     2 +-
 ...pst-adr-0070-eliminacao-counter-state-legacy.md |     3 +-
 .../adr/typst-adr-0071-walk-pipeline-redesign.md   |     3 +-
 .../typst-adr-0072-m7-fixpoint-runtime-fechado.md  |     2 +-
 .../adr/typst-adr-0073-comemo-introspector.md      |     3 +-
 ...pst-adr-0074-f3-layouter-substores-trackable.md |     3 +-
 .../adr/typst-adr-0075-vanilla-integration.md      |     3 +-
 .../adr/typst-adr-0076-introspector-completion.md  |     3 +-
 00_nucleo/adr/typst-adr-0077-regex-l1.md           |     3 +-
 .../adr/typst-adr-0078-column-flow-algorithm.md    |     6 +-
 .../adr/typst-adr-0079-layout-fase-5-roadmap.md    |  2628 ----
 .../typst-adr-0080-l0-minimal-para-refactors.md    |     2 +-
 ...dr-0081-m7-plus-pipeline-restructuring-scope.md |  1085 --
 ...t-adr-0082-promocoes-reais-scope-outs-graded.md |     3 +-
 ...or-paridade-vanilla-com-subset-materializado.md |     3 +-
 .../adr/typst-adr-0084-auditoria-condicional.md    |     2 +-
 .../adr/typst-adr-0085-diagnostico-imutavel.md     |     2 +-
 .../adr/typst-adr-0086-paint-wrapper-solid-only.md |     2 +-
 .../adr/typst-adr-0087-gradient-linear-only.md     |     2 +-
 .../adr/typst-adr-0088-gradient-radial-only.md     |     2 +-
 .../adr/typst-adr-0089-gradient-conic-only.md      |     2 +-
 ...090-gradient-conic-strategy-type-4-vs-type-1.md |   561 -
 ...091-gradient-space-runtime-and-cmyk-strategy.md |     3 +-
 .../typst-adr-0092-conic-coons-patches-rgb-cmyk.md |     3 +-
 ...ypst-adr-0093-meta-metodologia-evolucao-adrs.md |     3 +-
 .../adr/typst-adr-0094-meta-operacional-specs.md   |     3 +-
 .../typst-adr-0095-dedup-arc-as-ptr-resources.md   |     3 +-
 ...tern-debt-37-campo-layouter-consumer-pending.md |   259 -
 ...t-adr-0097-scope-out-reconfirmado-por-fase-a.md |   269 -
 ...8-single-source-of-truth-invariante-anti-bug.md |     3 +-
 ...-adr-0099-activacao-posterior-feature-graded.md |     3 +-
 .../adr/typst-adr-0100-coesao-por-dominio-l3.md    |     2 +-
 .../adr/typst-adr-0101-ieee754-restricao-stdlib.md |     2 +-
 .../adr/typst-adr-0102-math-style-mechanism.md     |     2 +-
 .../adr/typst-adr-0103-math-style-composition.md   |     2 +-
 .../adr/typst-adr-0104-atomicidade-para-agentes.md |     2 +-
 ...t-adr-0105-modelo-elemento-d-agora-f-destino.md |     2 +-
 .../adr/typst-adr-0106-fronteira-de-extensao-e1.md |     2 +-
 ...pst-adr-0107-paridade-linguagem-nao-mecanica.md |     2 +-
 .../adr/typst-adr-0108-disciplina-anti-deriva.md   |     2 +-
 00_nucleo/adr/typst-adr-0109-atomizacao.md         |     3 +-
 00_nucleo/adr/typst-adr-0110-marcador-vanilla.md   |     3 +-
 .../adr/typst-adr-0111-crates-parsing-loading.md   |     3 +-
 00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md    |     3 +-
 ...-0113-stub-transparente-vs-fallback-software.md |     2 +-
 .../adr/typst-adr-0114-sonda-antes-da-spec.md      |     2 +-
 .../adr/typst-adr-0115-infra-benchmark-scanner.md  |     3 +-
 ...typst-adr-0116-excecao-get-unchecked-scanner.md |     3 +-
 00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md |     3 +-
 .../adr/typst-adr-0118-runtime-state-context.md    |     3 +-
 .../adr/typst-adr-0119-disciplina-verificacao.md   |     2 +-
 .../adr/typst-adr-0120-textshaped-rustybuzz.md     |     3 +-
 .../adr/typst-adr-0121-proveniencia-medicao.md     |     2 +-
 .../adr/typst-adr-0122-paridade-defeitos-testes.md |     2 +-
 .../adr/typst-adr-0123-geometria-tipografica.md    |     2 +-
 .../adr/typst-adr-0124-checklist-sublayouts.md     |     2 +-
 .../typst-adr-0125-implementacao-vs-linguagem.md   |     2 +-
 .../adr/typst-adr-0126-modo-verboso-primeiro.md    |     2 +-
 .../adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md |     2 +-
 .../adr/typst-adr-0128-html-target-semantico.md    |     2 +-
 ...typst-adr-0129-nucleos-tekt-l0-compartilhado.md |     3 +-
 .../adr/typst-adr-0130-referencias-canonicas-l0.md |    94 +-
 00_nucleo/prompts/compiler/eval.md                 |    43 +-
 00_nucleo/prompts/compiler/layout.md               |    31 +-
 00_nucleo/prompts/compiler/layout/equation.md      |     6 +-
 00_nucleo/prompts/compiler/layout/page_geometry.md |     7 +-
 00_nucleo/prompts/compiler/layout/parbreak.md      |    16 +-
 00_nucleo/prompts/compiler/layout/placement.md     |     6 +-
 00_nucleo/prompts/compiler/layout/tests.md         |    10 +-
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/parse/parser.md         |    35 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    28 +-
 00_nucleo/prompts/entities/content.md              |  3580 +----
 00_nucleo/prompts/entities/content_hash.md         |   170 +-
 00_nucleo/prompts/entities/syntax-node.md          |   238 +-
 00_nucleo/prompts/infra/export/builder.md          |  2273 +--
 00_nucleo/prompts/infra/export/html.md             |   386 +-
 00_nucleo/prompts/infra/export/images.md           |   175 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
 00_nucleo/prompts/infra/export/stream.md           |   674 +-
 00_nucleo/prompts/infra/export/svg.md              |   683 +-
 00_nucleo/prompts/infra/font_metrics.md            |  1085 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |    11 +-
 00_nucleo/prompts/infra/plugin_host.md             |   333 +-
 00_nucleo/prompts/infra/shaper.md                  |   954 +-
 00_nucleo/prompts/infra/system-world.md            |   431 +-
 00_nucleo/prompts/wiring.md                        |   978 +-
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
 01_core/src/compiler/layout/mod.rs                 |  1049 +-
 01_core/src/compiler/layout/page_geometry.rs       |    47 +-
 01_core/src/compiler/layout/parbreak.rs            |   109 +-
 01_core/src/compiler/layout/placement.rs           |   128 +-
 01_core/src/compiler/layout/sequence.rs            |    63 +-
 01_core/src/compiler/layout/set_page.rs            |    75 +-
 01_core/src/compiler/layout/tests.rs               |  1104 +-
 01_core/src/compiler/layout/word_segment.rs        |    44 +-
 01_core/src/compiler/math/layout/accent.rs         |     4 +-
 01_core/src/compiler/math/layout/attach.rs         |     4 +-
 01_core/src/compiler/math/layout/mod.rs            |    47 +-
 01_core/src/compiler/math/layout/p1430_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/p1431_tests.rs    |     2 +-
 01_core/src/compiler/math/layout/spacing.rs        |    18 +-
 01_core/src/compiler/math/layout/tests.rs          |     2 +-
 01_core/src/compiler/parse/parser.rs               |   629 +-
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
 01_core/src/entities/content_hash.rs               |    12 +-
 01_core/src/entities/elements/figure_caption.rs    |     6 +-
 01_core/src/entities/elements/footnote_entry.rs    |     8 +-
 01_core/src/entities/gradient.rs                   |    24 +-
 01_core/src/entities/locator.rs                    |     1 -
 01_core/src/entities/style.rs                      |     4 +-
 01_core/src/entities/style_chain.rs                |    18 +-
 01_core/src/entities/syntax_node.rs                |  1122 +-
 02_shell/src/cli.rs                                |    34 +-
 03_infra/src/export/builder.rs                     |  4274 +-----
 03_infra/src/export/html.rs                        |  1286 +-
 03_infra/src/export/images.rs                      |   596 +-
 03_infra/src/export/stream.rs                      |  2135 +--
 03_infra/src/export/svg.rs                         |  3271 +----
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
 04_wiring/src/main.rs                              |  1358 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    37 +
 benches/scanner_bench.rs                           |    11 +-
 258 files changed, 4989 insertions(+), 83847 deletions(-)
```

git status --short --untracked-files=all:
```text
 M 00_nucleo/adr/README.md
 D 00_nucleo/adr/adr-stub-vs-fallback.md
 M 00_nucleo/adr/template-adr.md
 M 00_nucleo/adr/typst-adr-0001-estrategia-migracao.md
 D 00_nucleo/adr/typst-adr-0002-hierarquia-contencao.md
 D 00_nucleo/adr/typst-adr-0003-comemo-contencao.md
 D 00_nucleo/adr/typst-adr-0004-passo1-descobertas.md
 M 00_nucleo/adr/typst-adr-0005-packagespec-world.md
 M 00_nucleo/adr/typst-adr-0006-typst-timing.md
 D 00_nucleo/adr/typst-adr-0007-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0008-defer-inline.md
 M 00_nucleo/adr/typst-adr-0009-math-class.md
 M 00_nucleo/adr/typst-adr-0010-unicode-ident.md
 M 00_nucleo/adr/typst-adr-0011-unicode-math-class.md
 M 00_nucleo/adr/typst-adr-0012-unicode-script.md
 M 00_nucleo/adr/typst-adr-0013-unicode-segmentation.md
 M 00_nucleo/adr/typst-adr-0014-unscanny.md
 M 00_nucleo/adr/typst-adr-0015-ecow.md
 M 00_nucleo/adr/typst-adr-0016-lazyhash.md
 M 00_nucleo/adr/typst-adr-0017-adiamento-eval-typst-library.md
 M 00_nucleo/adr/typst-adr-0018-rustc-hash.md
 M 00_nucleo/adr/typst-adr-0019-ttf-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0020-fontdb.md
 M 00_nucleo/adr/typst-adr-0021-datetime.md
 M 00_nucleo/adr/typst-adr-0022-fontbook.md
 M 00_nucleo/adr/typst-adr-0023-indexmap.md
 M 00_nucleo/adr/typst-adr-0024-ecow-value.md
 M 00_nucleo/adr/typst-adr-0025-int-eq-float.md
 M 00_nucleo/adr/typst-adr-0026-R1-content-arc.md
 M 00_nucleo/adr/typst-adr-0026-content-divergencia.md
 M 00_nucleo/adr/typst-adr-0027-cidfont-subsetting.md
 D 00_nucleo/adr/typst-adr-0028-typographic-types.md
 M 00_nucleo/adr/typst-adr-0029-pureza-fisica-revoga-adr-0028.md
 M 00_nucleo/adr/typst-adr-0030-performance-dominio-l1.md
 M 00_nucleo/adr/typst-adr-0031-early-hashing-source.md
 M 00_nucleo/adr/typst-adr-0032-unsafe-em-l1.md
 M 00_nucleo/adr/typst-adr-0033-paridade-funcional-vanilla.md
 M 00_nucleo/adr/typst-adr-0034-diagnostico-tipos-vanilla.md
 M 00_nucleo/adr/typst-adr-0035-ecovec-autorizacao.md
 M 00_nucleo/adr/typst-adr-0036-atomizacao-progressiva.md
 M 00_nucleo/adr/typst-adr-0037-coesao-por-dominio.md
 M 00_nucleo/adr/typst-adr-0038-sistema-estilos-l1.md
 M 00_nucleo/adr/typst-adr-0039-frameitem-style.md
 M 00_nucleo/adr/typst-adr-0040-activacao-set-rule.md
 M 00_nucleo/adr/typst-adr-0041-activacao-show-rule.md
 M 00_nucleo/adr/typst-adr-0042-sink-materializado.md
 M 00_nucleo/adr/typst-adr-0043-canal-sink-saida.md
 M 00_nucleo/adr/typst-adr-0044-engine-agregador.md
 M 00_nucleo/adr/typst-adr-0045-formato-diagnosticos.md
 M 00_nucleo/adr/typst-adr-0046-cli-minima.md
 M 00_nucleo/adr/typst-adr-0047-cli-clap.md
 M 00_nucleo/adr/typst-adr-0048-diagnosticos-com-cor.md
 M 00_nucleo/adr/typst-adr-0049-cli-em-l2.md
 M 00_nucleo/adr/typst-adr-0050-formatter-em-l2.md
 M 00_nucleo/adr/typst-adr-0051-flags-funcionais.md
 M 00_nucleo/adr/typst-adr-0052-lang-tipo-semantico.md
 M 00_nucleo/adr/typst-adr-0053-font-tipo-composto.md
 M 00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md
 M 00_nucleo/adr/typst-adr-0055-font-consumer-cidfont.md
 M 00_nucleo/adr/typst-adr-0057-lang-hyphenation.md
 M 00_nucleo/adr/typst-adr-0058-value-type-simplificado.md
 M 00_nucleo/adr/typst-adr-0059-args-tipo-separado.md
 D 00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md
 D 00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md
 M 00_nucleo/adr/typst-adr-0062-hayagriva-bibliography-parsing.md
 M 00_nucleo/adr/typst-adr-0064-smart-para-option-default.md
 M 00_nucleo/adr/typst-adr-0065-inventariar-primeiro.md
 D 00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md
 M 00_nucleo/adr/typst-adr-0067-attribute-grammar-scoping.md
 M 00_nucleo/adr/typst-adr-0068-location-aware-layouter.md
 M 00_nucleo/adr/typst-adr-0069-post-recursion-tag-emission.md
 M 00_nucleo/adr/typst-adr-0070-eliminacao-counter-state-legacy.md
 M 00_nucleo/adr/typst-adr-0071-walk-pipeline-redesign.md
 M 00_nucleo/adr/typst-adr-0072-m7-fixpoint-runtime-fechado.md
 M 00_nucleo/adr/typst-adr-0073-comemo-introspector.md
 M 00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md
 M 00_nucleo/adr/typst-adr-0075-vanilla-integration.md
 M 00_nucleo/adr/typst-adr-0076-introspector-completion.md
 M 00_nucleo/adr/typst-adr-0077-regex-l1.md
 M 00_nucleo/adr/typst-adr-0078-column-flow-algorithm.md
 D 00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md
 M 00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md
 D 00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md
 M 00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md
 M 00_nucleo/adr/typst-adr-0083-color-paridade-vanilla-com-subset-materializado.md
 M 00_nucleo/adr/typst-adr-0084-auditoria-condicional.md
 M 00_nucleo/adr/typst-adr-0085-diagnostico-imutavel.md
 M 00_nucleo/adr/typst-adr-0086-paint-wrapper-solid-only.md
 M 00_nucleo/adr/typst-adr-0087-gradient-linear-only.md
 M 00_nucleo/adr/typst-adr-0088-gradient-radial-only.md
 M 00_nucleo/adr/typst-adr-0089-gradient-conic-only.md
 D 00_nucleo/adr/typst-adr-0090-gradient-conic-strategy-type-4-vs-type-1.md
 M 00_nucleo/adr/typst-adr-0091-gradient-space-runtime-and-cmyk-strategy.md
 M 00_nucleo/adr/typst-adr-0092-conic-coons-patches-rgb-cmyk.md
 M 00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md
 M 00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md
 M 00_nucleo/adr/typst-adr-0095-dedup-arc-as-ptr-resources.md
 D 00_nucleo/adr/typst-adr-0096-pattern-debt-37-campo-layouter-consumer-pending.md
 D 00_nucleo/adr/typst-adr-0097-scope-out-reconfirmado-por-fase-a.md
 M 00_nucleo/adr/typst-adr-0098-single-source-of-truth-invariante-anti-bug.md
 M 00_nucleo/adr/typst-adr-0099-activacao-posterior-feature-graded.md
 M 00_nucleo/adr/typst-adr-0100-coesao-por-dominio-l3.md
 M 00_nucleo/adr/typst-adr-0101-ieee754-restricao-stdlib.md
 M 00_nucleo/adr/typst-adr-0102-math-style-mechanism.md
 M 00_nucleo/adr/typst-adr-0103-math-style-composition.md
 M 00_nucleo/adr/typst-adr-0104-atomicidade-para-agentes.md
 M 00_nucleo/adr/typst-adr-0105-modelo-elemento-d-agora-f-destino.md
 M 00_nucleo/adr/typst-adr-0106-fronteira-de-extensao-e1.md
 M 00_nucleo/adr/typst-adr-0107-paridade-linguagem-nao-mecanica.md
 M 00_nucleo/adr/typst-adr-0108-disciplina-anti-deriva.md
 M 00_nucleo/adr/typst-adr-0109-atomizacao.md
 M 00_nucleo/adr/typst-adr-0110-marcador-vanilla.md
 M 00_nucleo/adr/typst-adr-0111-crates-parsing-loading.md
 M 00_nucleo/adr/typst-adr-0112-rust-decimal-l1.md
 M 00_nucleo/adr/typst-adr-0113-stub-transparente-vs-fallback-software.md
 M 00_nucleo/adr/typst-adr-0114-sonda-antes-da-spec.md
 M 00_nucleo/adr/typst-adr-0115-infra-benchmark-scanner.md
 M 00_nucleo/adr/typst-adr-0116-excecao-get-unchecked-scanner.md
 M 00_nucleo/adr/typst-adr-0117-sonda-a0-mecanismo.md
 M 00_nucleo/adr/typst-adr-0118-runtime-state-context.md
 M 00_nucleo/adr/typst-adr-0119-disciplina-verificacao.md
 M 00_nucleo/adr/typst-adr-0120-textshaped-rustybuzz.md
 M 00_nucleo/adr/typst-adr-0121-proveniencia-medicao.md
 M 00_nucleo/adr/typst-adr-0122-paridade-defeitos-testes.md
 M 00_nucleo/adr/typst-adr-0123-geometria-tipografica.md
 M 00_nucleo/adr/typst-adr-0124-checklist-sublayouts.md
 M 00_nucleo/adr/typst-adr-0125-implementacao-vs-linguagem.md
 M 00_nucleo/adr/typst-adr-0126-modo-verboso-primeiro.md
 M 00_nucleo/adr/typst-adr-0127-gate-l0-paragem-vs-fluxo.md
 M 00_nucleo/adr/typst-adr-0128-html-target-semantico.md
 M 00_nucleo/adr/typst-adr-0129-nucleos-tekt-l0-compartilhado.md
 M 00_nucleo/adr/typst-adr-0130-referencias-canonicas-l0.md
 M 00_nucleo/prompts/compiler/eval.md
 M 00_nucleo/prompts/compiler/layout.md
 M 00_nucleo/prompts/compiler/layout/equation.md
 M 00_nucleo/prompts/compiler/layout/page_geometry.md
 M 00_nucleo/prompts/compiler/layout/parbreak.md
 M 00_nucleo/prompts/compiler/layout/placement.md
 M 00_nucleo/prompts/compiler/layout/tests.md
 M 00_nucleo/prompts/compiler/layout/word_segment.md
 M 00_nucleo/prompts/compiler/parse/parser.md
 M 00_nucleo/prompts/compiler/stdlib/_comum.md
 M 00_nucleo/prompts/entities/content.md
 M 00_nucleo/prompts/entities/content_hash.md
 M 00_nucleo/prompts/entities/syntax-node.md
 M 00_nucleo/prompts/infra/export/builder.md
 M 00_nucleo/prompts/infra/export/html.md
 M 00_nucleo/prompts/infra/export/images.md
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/export/stream.md
 M 00_nucleo/prompts/infra/export/svg.md
 M 00_nucleo/prompts/infra/font_metrics.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
 M 00_nucleo/prompts/infra/plugin_host.md
 M 00_nucleo/prompts/infra/shaper.md
 M 00_nucleo/prompts/infra/system-world.md
 M 00_nucleo/prompts/wiring.md
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
 M 01_core/src/compiler/layout/parbreak.rs
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
 M 01_core/src/compiler/parse/parser.rs
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
 M benches/scanner_bench.rs
?? 00_nucleo/adr/ORDER.md
?? 00_nucleo/diagnosticos/adr-0130-step-1060-history.md
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
?? 00_nucleo/diagnosticos/adr-governance-reconciliation.md
?? 00_nucleo/diagnosticos/adr-performance-quantity-analysis.md
?? 00_nucleo/diagnosticos/adr-reference-legacy-portuguese.md
?? 00_nucleo/diagnosticos/adr-template-legacy-portuguese.md
?? 00_nucleo/diagnosticos/benchmark-suite-review-state.json
?? 00_nucleo/diagnosticos/benchmark-suite-review.md
?? 00_nucleo/diagnosticos/content-microbenchmarks.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-history.md
?? 00_nucleo/diagnosticos/core-l0-sanitization-manifest.json
?? 00_nucleo/diagnosticos/core-l0-sanitization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/README.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/fonts.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/metadata.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/navigation.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/resources.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/serialization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/builder/streams.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/document.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/encoding.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/inline.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/p1360_ab_tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/pretty.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/table.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/tags.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/html/whitespace.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/icc.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/jpeg.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/png.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/scan.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/images/xobjects.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/manifest.json
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/draw.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/page.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/shape.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/stream/text.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/geometry.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/glyphs.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/groups.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/images.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/navigation.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/p1360_ab_tests.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/paints.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/serialization.md
?? 00_nucleo/diagnosticos/exporters-l0-history/2026-09-26/svg/tests.md
?? 00_nucleo/diagnosticos/font-metrics-atomization.md
?? 00_nucleo/diagnosticos/font-metrics-l0-before-atomization.md
?? 00_nucleo/diagnosticos/html-atomization-manifest.json
?? 00_nucleo/diagnosticos/html-atomization.md
?? 00_nucleo/diagnosticos/images-atomization.md
?? 00_nucleo/diagnosticos/images-l0-before-atomization.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/basic.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/caches.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/fallback.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/math.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/font_metrics/resolution.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/bundle.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/context_stabilization.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/html.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/paged.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/pdf.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/raster.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/resources.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/svg.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/pipeline/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/execution.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/loading.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/snapshot.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/plugin_host/typst_env.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/bidi.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/cache.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/font_selection.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/reconciliation.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/shaping.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/shaper/traversal.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/cache.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/fonts.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/packages.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/paths.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/tests.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/infra/system-world/world_impl.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/compile.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/completions.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/diagnostics.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/eval.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/eviction.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/fonts.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/info.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/init.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/query.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/cli.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/crystalline_lint.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1289_float_is_infinite.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1291_callback_runtime.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1292_contract.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1293_contract.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1462-format-svg.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1463-format-png.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1464-format-pdf.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1465-format-html.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1467-format-pdf-tagged-pretty.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1468-format-pdf-pages.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/tests/p1469-format-pdf-standard.md
?? 00_nucleo/diagnosticos/l0-infra-wiring-history/wiring/watch.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md.orig
?? 00_nucleo/diagnosticos/observation-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/parser-atomization.md
?? 00_nucleo/diagnosticos/parser-l0-before-atomization.md
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
?? 00_nucleo/diagnosticos/svg-atomization.md
?? 00_nucleo/diagnosticos/svg-l0-before-atomization.md
?? 00_nucleo/diagnosticos/syntax-node-atomization.md
?? 00_nucleo/diagnosticos/tekt-english-reference-pilot.md
?? 00_nucleo/diagnosticos/wiring-main-atomization.md
?? 00_nucleo/diagnosticos/world-atomization-manifest.json
?? 00_nucleo/diagnosticos/world-atomization.md
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
?? 00_nucleo/prompts/compiler/layout/align.md
?? 00_nucleo/prompts/compiler/layout/context_block.md
?? 00_nucleo/prompts/compiler/layout/counter_display.md
?? 00_nucleo/prompts/compiler/layout/counter_display_callback.md
?? 00_nucleo/prompts/compiler/layout/equation.md.orig
?? 00_nucleo/prompts/compiler/layout/measurement.md
?? 00_nucleo/prompts/compiler/layout/measurement.md.orig
?? 00_nucleo/prompts/compiler/layout/outline_entry.md
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
?? 00_nucleo/prompts/compiler/layout/space.md
?? 00_nucleo/prompts/compiler/layout/state_display.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md.orig
?? 00_nucleo/prompts/compiler/parse/parser/checkpoints.md
?? 00_nucleo/prompts/compiler/parse/parser/diagnostics.md
?? 00_nucleo/prompts/compiler/parse/parser/state.md
?? 00_nucleo/prompts/compiler/stdlib/registry/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/registry/data.md
?? 00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md
?? 00_nucleo/prompts/compiler/stdlib/registry/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/registry/formats.md
?? 00_nucleo/prompts/compiler/stdlib/registry/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/registry/layout.md
?? 00_nucleo/prompts/compiler/stdlib/registry/math.md
?? 00_nucleo/prompts/compiler/stdlib/registry/microbenchmarks.md
?? 00_nucleo/prompts/compiler/stdlib/registry/mod.md
?? 00_nucleo/prompts/compiler/stdlib/registry/modules.md
?? 00_nucleo/prompts/compiler/stdlib/registry/shapes.md
?? 00_nucleo/prompts/compiler/stdlib/registry/structural.md
?? 00_nucleo/prompts/compiler/stdlib/registry/table_grid.md
?? 00_nucleo/prompts/compiler/stdlib/registry/tests.md
?? 00_nucleo/prompts/compiler/stdlib/registry/text.md
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
?? 00_nucleo/prompts/entities/syntax-node/errors.md
?? 00_nucleo/prompts/entities/syntax-node/linked.md
?? 00_nucleo/prompts/entities/syntax-node/representation.md
?? 00_nucleo/prompts/infra/export/builder/fonts.md
?? 00_nucleo/prompts/infra/export/builder/metadata.md
?? 00_nucleo/prompts/infra/export/builder/navigation.md
?? 00_nucleo/prompts/infra/export/builder/resources.md
?? 00_nucleo/prompts/infra/export/builder/serialization.md
?? 00_nucleo/prompts/infra/export/builder/streams.md
?? 00_nucleo/prompts/infra/export/html/document.md
?? 00_nucleo/prompts/infra/export/html/encoding.md
?? 00_nucleo/prompts/infra/export/html/inline.md
?? 00_nucleo/prompts/infra/export/html/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/html/pretty.md
?? 00_nucleo/prompts/infra/export/html/table.md
?? 00_nucleo/prompts/infra/export/html/tags.md
?? 00_nucleo/prompts/infra/export/html/tests.md
?? 00_nucleo/prompts/infra/export/html/whitespace.md
?? 00_nucleo/prompts/infra/export/images/icc.md
?? 00_nucleo/prompts/infra/export/images/jpeg.md
?? 00_nucleo/prompts/infra/export/images/png.md
?? 00_nucleo/prompts/infra/export/images/scan.md
?? 00_nucleo/prompts/infra/export/images/xobjects.md
?? 00_nucleo/prompts/infra/export/stream/draw.md
?? 00_nucleo/prompts/infra/export/stream/page.md
?? 00_nucleo/prompts/infra/export/stream/shape.md
?? 00_nucleo/prompts/infra/export/stream/text.md
?? 00_nucleo/prompts/infra/export/svg/geometry.md
?? 00_nucleo/prompts/infra/export/svg/glyphs.md
?? 00_nucleo/prompts/infra/export/svg/groups.md
?? 00_nucleo/prompts/infra/export/svg/images.md
?? 00_nucleo/prompts/infra/export/svg/navigation.md
?? 00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md
?? 00_nucleo/prompts/infra/export/svg/paints.md
?? 00_nucleo/prompts/infra/export/svg/serialization.md
?? 00_nucleo/prompts/infra/export/svg/tests.md
?? 00_nucleo/prompts/infra/font_metrics/basic.md
?? 00_nucleo/prompts/infra/font_metrics/caches.md
?? 00_nucleo/prompts/infra/font_metrics/fallback.md
?? 00_nucleo/prompts/infra/font_metrics/math.md
?? 00_nucleo/prompts/infra/font_metrics/resolution.md
?? 00_nucleo/prompts/infra/pipeline/bundle.md
?? 00_nucleo/prompts/infra/pipeline/html.md
?? 00_nucleo/prompts/infra/pipeline/paged.md
?? 00_nucleo/prompts/infra/pipeline/pdf.md
?? 00_nucleo/prompts/infra/pipeline/raster.md
?? 00_nucleo/prompts/infra/pipeline/resources.md
?? 00_nucleo/prompts/infra/pipeline/svg.md
?? 00_nucleo/prompts/infra/pipeline/tests.md
?? 00_nucleo/prompts/infra/plugin_host/execution.md
?? 00_nucleo/prompts/infra/plugin_host/loading.md
?? 00_nucleo/prompts/infra/plugin_host/snapshot.md
?? 00_nucleo/prompts/infra/plugin_host/tests.md
?? 00_nucleo/prompts/infra/plugin_host/typst_env.md
?? 00_nucleo/prompts/infra/shaper/bidi.md
?? 00_nucleo/prompts/infra/shaper/cache.md
?? 00_nucleo/prompts/infra/shaper/font_selection.md
?? 00_nucleo/prompts/infra/shaper/reconciliation.md
?? 00_nucleo/prompts/infra/shaper/shaping.md
?? 00_nucleo/prompts/infra/shaper/tests.md
?? 00_nucleo/prompts/infra/shaper/traversal.md
?? 00_nucleo/prompts/infra/system-world/cache.md
?? 00_nucleo/prompts/infra/system-world/fonts.md
?? 00_nucleo/prompts/infra/system-world/packages.md
?? 00_nucleo/prompts/infra/system-world/paths.md
?? 00_nucleo/prompts/infra/system-world/tests.md
?? 00_nucleo/prompts/infra/system-world/world_impl.md
?? 00_nucleo/prompts/wiring/compile.md
?? 00_nucleo/prompts/wiring/completions.md
?? 00_nucleo/prompts/wiring/diagnostics.md
?? 00_nucleo/prompts/wiring/eval.md
?? 00_nucleo/prompts/wiring/fonts.md
?? 00_nucleo/prompts/wiring/info.md
?? 00_nucleo/prompts/wiring/init.md
?? 00_nucleo/prompts/wiring/query.md
?? 00_nucleo/prompts/wiring/watch.md
?? 01_core/src/compiler/eval/observation/content.rs
?? 01_core/src/compiler/eval/observation/elements.rs
?? 01_core/src/compiler/eval/observation/geometry.rs
?? 01_core/src/compiler/eval/observation/math.rs
?? 01_core/src/compiler/eval/observation/mod.rs
?? 01_core/src/compiler/eval/observation/page.rs
?? 01_core/src/compiler/eval/observation/style.rs
?? 01_core/src/compiler/eval/observation/value.rs
?? 01_core/src/compiler/layout/align.rs
?? 01_core/src/compiler/layout/context_block.rs
?? 01_core/src/compiler/layout/counter_display.rs
?? 01_core/src/compiler/layout/counter_display_callback.rs
?? 01_core/src/compiler/layout/equation.rs.orig
?? 01_core/src/compiler/layout/measurement.rs
?? 01_core/src/compiler/layout/measurement.rs.orig
?? 01_core/src/compiler/layout/mod.rs.orig
?? 01_core/src/compiler/layout/outline_entry.rs
?? 01_core/src/compiler/layout/page_geometry.rs.orig
?? 01_core/src/compiler/layout/par_line_callbacks.rs
?? 01_core/src/compiler/layout/par_line_callbacks.rs.orig
?? 01_core/src/compiler/layout/placement.rs.orig
?? 01_core/src/compiler/layout/space.rs
?? 01_core/src/compiler/layout/state_display.rs
?? 01_core/src/compiler/layout/style_scope.rs
?? 01_core/src/compiler/layout/style_scope.rs.orig
?? 01_core/src/compiler/parse/parser/checkpoints.rs
?? 01_core/src/compiler/parse/parser/diagnostics.rs
?? 01_core/src/compiler/parse/parser/state.rs
?? 01_core/src/compiler/stdlib/registry/contextual.rs
?? 01_core/src/compiler/stdlib/registry/data.rs
?? 01_core/src/compiler/stdlib/registry/diagnostics.rs
?? 01_core/src/compiler/stdlib/registry/figure_image.rs
?? 01_core/src/compiler/stdlib/registry/formats.rs
?? 01_core/src/compiler/stdlib/registry/foundations.rs
?? 01_core/src/compiler/stdlib/registry/layout.rs
?? 01_core/src/compiler/stdlib/registry/math.rs
?? 01_core/src/compiler/stdlib/registry/microbenchmarks.rs
?? 01_core/src/compiler/stdlib/registry/mod.rs
?? 01_core/src/compiler/stdlib/registry/modules.rs
?? 01_core/src/compiler/stdlib/registry/shapes.rs
?? 01_core/src/compiler/stdlib/registry/structural.rs
?? 01_core/src/compiler/stdlib/registry/table_grid.rs
?? 01_core/src/compiler/stdlib/registry/tests.rs
?? 01_core/src/compiler/stdlib/registry/text.rs
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
?? 01_core/src/entities/syntax_node/errors.rs
?? 01_core/src/entities/syntax_node/linked.rs
?? 01_core/src/entities/syntax_node/representation.rs
?? 03_infra/src/export/builder/fonts.rs
?? 03_infra/src/export/builder/metadata.rs
?? 03_infra/src/export/builder/navigation.rs
?? 03_infra/src/export/builder/resources.rs
?? 03_infra/src/export/builder/serialization.rs
?? 03_infra/src/export/builder/streams.rs
?? 03_infra/src/export/html/document.rs
?? 03_infra/src/export/html/encoding.rs
?? 03_infra/src/export/html/inline.rs
?? 03_infra/src/export/html/p1360_ab_tests.rs
?? 03_infra/src/export/html/pretty.rs
?? 03_infra/src/export/html/table.rs
?? 03_infra/src/export/html/tags.rs
?? 03_infra/src/export/html/tests.rs
?? 03_infra/src/export/html/whitespace.rs
?? 03_infra/src/export/images/icc.rs
?? 03_infra/src/export/images/jpeg.rs
?? 03_infra/src/export/images/png.rs
?? 03_infra/src/export/images/scan.rs
?? 03_infra/src/export/images/xobjects.rs
?? 03_infra/src/export/stream/draw.rs
?? 03_infra/src/export/stream/page.rs
?? 03_infra/src/export/stream/shape.rs
?? 03_infra/src/export/stream/text.rs
?? 03_infra/src/export/svg/geometry.rs
?? 03_infra/src/export/svg/glyphs.rs
?? 03_infra/src/export/svg/groups.rs
?? 03_infra/src/export/svg/images.rs
?? 03_infra/src/export/svg/navigation.rs
?? 03_infra/src/export/svg/p1360_ab_tests.rs
?? 03_infra/src/export/svg/paints.rs
?? 03_infra/src/export/svg/serialization.rs
?? 03_infra/src/export/svg/tests.rs
?? 03_infra/src/font_metrics/basic.rs
?? 03_infra/src/font_metrics/caches.rs
?? 03_infra/src/font_metrics/fallback.rs
?? 03_infra/src/font_metrics/math.rs
?? 03_infra/src/font_metrics/resolution.rs
?? 03_infra/src/pipeline/bundle.rs
?? 03_infra/src/pipeline/html.rs
?? 03_infra/src/pipeline/paged.rs
?? 03_infra/src/pipeline/pdf.rs
?? 03_infra/src/pipeline/raster.rs
?? 03_infra/src/pipeline/resources.rs
?? 03_infra/src/pipeline/svg.rs
?? 03_infra/src/pipeline/tests.rs
?? 03_infra/src/plugin_host/execution.rs
?? 03_infra/src/plugin_host/loading.rs
?? 03_infra/src/plugin_host/snapshot.rs
?? 03_infra/src/plugin_host/tests.rs
?? 03_infra/src/plugin_host/typst_env.rs
?? 03_infra/src/shaper/bidi.rs
?? 03_infra/src/shaper/cache.rs
?? 03_infra/src/shaper/font_selection.rs
?? 03_infra/src/shaper/reconciliation.rs
?? 03_infra/src/shaper/shaping.rs
?? 03_infra/src/shaper/tests.rs
?? 03_infra/src/shaper/traversal.rs
?? 03_infra/src/world/cache.rs
?? 03_infra/src/world/fonts.rs
?? 03_infra/src/world/packages.rs
?? 03_infra/src/world/paths.rs
?? 03_infra/src/world/tests.rs
?? 03_infra/src/world/world_impl.rs
?? 04_wiring/src/compile.rs
?? 04_wiring/src/completions.rs
?? 04_wiring/src/diagnostics.rs
?? 04_wiring/src/eval.rs
?? 04_wiring/src/fonts.rs
?? 04_wiring/src/info.rs
?? 04_wiring/src/init.rs
?? 04_wiring/src/query.rs
?? 04_wiring/src/watch.rs
?? benches/Cargo.toml.orig
?? benches/README.md
?? benches/build.rs
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
?? benches/layout/fixtures.rs
?? benches/layout/measurement.rs
?? benches/layout/style_scope.rs
?? benches/layout_bench.rs
?? benches/observation_bench.rs
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
?? benches/run_core_benchmark.py
?? benches/stdlib_bench.rs
?? benches/stream_bench.rs
?? benches/support/core_runner.rs
?? benches/support/profile.rs
?? benches/tests/test_benchmark_guards.py
```

Este diagnóstico é o único arquivo adicional após a captura. Alterações fora dos dez consumers/owners listados são preexistentes ou concorrentes, não desta execução. Nenhum commit foi criado.

# Atomização do registro da stdlib — 2026-09-26

## Proveniência da medição e decisão

HEAD: `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`; working tree não
commitada e com alterações concorrentes. Baseline congelado em
`2026-09-26T11:35:54.472Z`; estado final recolhido em `2026-09-26T11:49:50.957Z`.
O stat e a lista expandida de arquivos alterados em ambos os momentos estão
abaixo. Consumers novos têm hashes próprios na tabela.

A leitura de `01_core/src/compiler/eval/mod.rs:2260–3144` identificou a
composição monolítica e 18 trechos consecutivos de registro. A lista congelada
contém 179 nomes quando Bundle está ativa e 178 quando está desligada.
A referência de fonte integral congelada tem SHA-256
`613683d0f9716e899d49da74c7e86e8dd8b85fb5422774e3e0909f79dada357b` e 3.322 linhas.

Decisão interna ADR-0127 em fluxo contínuo: atualizar L0 primeiro, conservar
wrappers privados de eval e extrair a composição para
`01_core/src/compiler/stdlib/registry/`, com owners exclusivos em
`00_nucleo/prompts/compiler/stdlib/registry/`.
A atomização move montagem de bindings dentro da mesma fase de eval;
entrypoints, despacho, aquisição de inputs/features e projeção de std/global
permanecem no owner `00_nucleo/prompts/compiler/eval.md`.

## Materialização

O compositor usa 13 famílias e 18 chamadas ordenadas, além de testes próprios.
Reentradas em text, structural, layout e foundations conservam a ordem anterior.
Formats constrói PDF e HTML uma única vez, entrega as mesmas projeções à
factory de format e devolve PDF para sua inserção posterior. Asset é único e
seu binding global continua condicionado exclusivamente à feature Bundle.

A fachada stdlib declara registry privado e reexporta somente a factory interna
em pub(crate). Cada arquivo tem um Prompt L0 exclusivo; cláusulas históricas
de registro em eval ficam como proveniência e têm encaminhamento para os novos
owners. Nenhuma validação de Args, cast, função nativa ou lógica de exportação
foi incorporada ao registry. Helpers de argumentos continuam em seus owners.

A comparação mecânica confirmou as 18 etapas na ordem original, com todas as
instruções preservadas após normalização apenas de whitespace. As únicas
adaptações de corpo foram retirar a criação de Scope do trecho formats (ela
fica no compositor) e devolver seu PDF. Os tipos e nomes públicos são
verificados por testes e sondas da linguagem; a conservação textual e da ordem
é controle da refatoração, não alegação de paridade com mecânica vanilla.
Não houve nova medição contra vanilla ou mudança do pin de referência.

Eval fica com 2.441 linhas na árvore final desta medição. Registry/mod.rs tem
49 linhas, e o maior consumer produtivo da família tem 215 linhas. A tabela
abaixo permite reproduzir os tamanhos sem misturar estados concorrentes.

## Verificação e limites

- Build produtivo no workspace: `cargo build`, exit 0, após formatação e
  limpeza dos imports. Log: `/tmp/stdlib-registry-build-final.log`.
- Gates v1,v2,v3,v4,v5,v6,v13,v14,v15,v26: sem violações.
  Log: `/tmp/stdlib-registry-gates-final.log`.
- Lint completo: sem erros nos arquivos novos; warnings/infos existentes no
  restante da árvore continuam. Não se reivindica zero violations global.
  Log: `/tmp/stdlib-registry-lint-final.log`.
- `git diff --check`: exit 0; verificação de resselo repetida: nenhum patch
  pendente nos owners desta extração.
- CLI pública: sucesso nos oito perfis independentes de Html/A11yExtras/Bundle,
  com 8 sondas positivas compostas e
  28 controles de ausência/gate. Os valores
  positivos verificam nomes curtos, kinds, encoders e identidades das projeções
  de PDF/HTML/asset; negativos exigem diagnóstico. Evidência integral em
  `/tmp/stdlib-registry-cli-results.json`.
- Binário CLI: `target/debug/typst`, SHA-256 `5127d6487cc4a9a503c4c10dcc6dae509972bc395cf7d0e80c7c9533f8586568`.

O baseline real de `compiler::eval::p1471_tests` passou antes da mudança.
A compilação dos controles novos no workspace foi bloqueada por ausência do
import de Align2D no helper preexistente
`01_core/src/compiler/layout/tests.rs:3177`. O mesmo bloqueio apareceu antes
e depois da extração. O arquivo concorrente não foi editado por esta tarefa.

Para completar a verificação, foi congelada uma cópia de 01_core, Cargo.toml
e Cargo.lock em `/tmp/stdlib-registry-verification.ybMfhK`. Somente na cópia:

1. workspace reduzido ao membro 01_core;
2. import de Align2D adicionado ao módulo de testes de layout;
3. função monolítica congelada adicionada como baseline_stdlib_with_features
   sob cfg(test), mais uma cópia dos controles que a chama.

O código produtivo do registry na cópia é idêntico ao workspace, desconsiderando
somente headers de hash. Os controles exercitam lado a lado a versão anterior
e a versão nova. Resultado: 66 testes selecionados por registry passaram,
zero falhas. Incluem os quatro controles na versão anterior, os quatro na nova,
os testes das 13 famílias e testes anteriores cujo nome contém registry.
Um expected provisório presumiu que pdf e format.pdf teriam cardinalidades
iguais; ambos os candidatos o refutaram igualmente, porque a projeção possui
opções de exportação próprias. Esse assert foi retirado, conservando o controle
de identidade de todos os membros projetados. Nenhum código produtivo mudou
para satisfazer essa expectativa incorreta.

Regressões na mesma cópia:

- compiler::stdlib::: 1.432 passaram, zero falhas ou ignorados;
- compiler::eval::: 1.626 passaram, zero falhas e um teste já ignorado.

Os grupos sobrepõem testes; suas contagens não devem ser somadas. Commands:
`cargo test --offline --manifest-path /tmp/stdlib-registry-verification.ybMfhK/Cargo.toml -p typst-core --lib registry`
e os mesmos argumentos com filtros compiler::stdlib:: e compiler::eval::.
Logs: `/tmp/stdlib-registry-isolated-tests-final.log`,
`/tmp/stdlib-registry-stdlib-regressions.log`,
`/tmp/stdlib-registry-eval-regressions.log`.
SHA-256 do binário de testes isolado: `ver log de execução`.
A execução normal de cargo test no workspace ainda depende da correção do
import concorrente; o build produtivo normal já passou.

## Manifesto dos novos consumers

| Arquivo | Linhas | SHA-256 |
| --- | ---: | --- |
| `contextual.rs` | 129 | `d9e1d09bdf89a41b893242a4bea0c66733821baaa1db50d5a4ac25d030a72025` |
| `data.rs` | 93 | `8c87629ff8f09a48b7ba63cb4d192c8832a5e87cd138ef6de979aa5c06870482` |
| `diagnostics.rs` | 57 | `1a03f51a36b11e124f4b6aff0b5dd066349444b45a09f759001e662576a5e891` |
| `figure_image.rs` | 48 | `244aaaa3fb7cd2bac7d12d6afcdd1f3a1f7cca847f4c0ab3c44a59251f0095ce` |
| `formats.rs` | 42 | `1a10f3846c167d3bef9c5fe35fc3b586430d97d4392e747bf13e24cc70dde23f` |
| `foundations.rs` | 99 | `6a90e9aab830b519c62f43d22ff7968546cf1803a1035d5a5b7edcc1327b9a4d` |
| `layout.rs` | 132 | `cbe72078f8922acf9b764ccaeacebc8a87df0005d481f1b64d640459f2491db1` |
| `math.rs` | 55 | `f1bbbf71f5f107d66662252058936667c6777f70c0a80ab76feba3714191f548` |
| `mod.rs` | 49 | `5bd60b922cb05b0905a4eb0f21c6bb2632ae32e0b93006686ae0a737ec70ebb2` |
| `modules.rs` | 58 | `128d2b4692fa29ec2807b492a7b0a0619334390c07359ed4f2247f9543a5d304` |
| `shapes.rs` | 84 | `8d2afde75d4c2a71357d6fc2657f10775cf00faecbb2daf4570e2f9d558bdf29` |
| `structural.rs` | 215 | `6a45d7de462c61c5c987ec64ed96703192aa19efff6e83390ee9c20be553355f` |
| `table_grid.rs` | 88 | `18131e39dd4cdfe7e3bd10e6ff5e488fcff5e255bb4dcd63434ec4bf7690ab05` |
| `tests.rs` | 343 | `cc6a88fd8b7f92769244baef74bb1185aa063fa7e7a5e7243134513b47dbb9a3` |
| `text.rs` | 60 | `aeb4c42dd964a04c57245542d2b0500ba9c3f62cba34dd03c177c83de296a8a9` |

## Working tree do baseline congelado

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
 00_nucleo/prompts/compiler/eval.md                 |    21 +-
 00_nucleo/prompts/compiler/layout.md               |    21 +
 00_nucleo/prompts/compiler/layout/equation.md      |     4 +
 00_nucleo/prompts/compiler/layout/page_geometry.md |     5 +
 00_nucleo/prompts/compiler/layout/placement.md     |     4 +
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    19 +-
 00_nucleo/prompts/entities/content.md              |   119 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 00_nucleo/prompts/infra/pipeline.md                |  1676 +--
 .../infra/pipeline/context_stabilization.md        |     9 +
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
 01_core/src/compiler/eval/mod.rs                   |  5770 +-------
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
 01_core/src/compiler/layout/equation.rs            |   151 +-
 01_core/src/compiler/layout/format_options.rs      |    43 +-
 01_core/src/compiler/layout/mod.rs                 |   912 +-
 01_core/src/compiler/layout/page_geometry.rs       |    45 +
 01_core/src/compiler/layout/placement.rs           |   126 +
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
 03_infra/src/export/builder.rs                     |  4272 +-----
 03_infra/src/export/html.rs                        |    48 +-
 03_infra/src/export/svg.rs                         |    21 +-
 03_infra/src/export/tests/mod.rs                   |     2 +-
 .../tests/p1475_pdfa_conformance_core_tests.rs     |     8 +-
 .../tests/p1476_pdfa_attachments_af_tests.rs       |    14 +-
 .../p1477_pdfa_icc_version_and_cidset_tests.rs     |    12 +-
 .../p1478_pdfa4f_empty_embedded_files_tests.rs     |     9 +-
 03_infra/src/integration_tests.rs                  |     2 +-
 03_infra/src/pipeline.rs                           |  6003 +-------
 03_infra/src/pipeline/context_stabilization.rs     |     5 +-
 03_infra/src/query_helpers.rs                      |     9 +-
 03_infra/src/watch.rs                              |    67 +-
 03_infra/tests/p1250_svg_integrated.rs             |     3 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    17 +
 119 files changed, 3834 insertions(+), 53266 deletions(-)
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
 M Cargo.lock
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
?? 00_nucleo/diagnosticos/stdlib-tests-atomization.md
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
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md
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
?? 01_core/src/compiler/layout/mod.rs.orig
?? 01_core/src/compiler/layout/page_geometry.rs.orig
?? 01_core/src/compiler/layout/par_line_callbacks.rs
?? 01_core/src/compiler/layout/style_scope.rs
?? 01_core/src/compiler/layout/style_scope.rs.orig
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
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
```

## Working tree da verificação final

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
 00_nucleo/prompts/compiler/eval.md                 |    43 +-
 00_nucleo/prompts/compiler/layout.md               |    23 +-
 00_nucleo/prompts/compiler/layout/equation.md      |     6 +-
 00_nucleo/prompts/compiler/layout/page_geometry.md |     7 +-
 00_nucleo/prompts/compiler/layout/placement.md     |     6 +-
 00_nucleo/prompts/compiler/layout/word_segment.md  |    55 +-
 00_nucleo/prompts/compiler/stdlib/_comum.md        |    28 +-
 00_nucleo/prompts/entities/content.md              |   119 +-
 00_nucleo/prompts/infra/export/builder.md          |    23 +-
 .../prompts/infra/export/p1250_svg_integrated.md   |     6 +-
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
 03_infra/tests/p1250_svg_integrated.rs             |     6 +-
 04_wiring/src/main.rs                              |    38 +-
 04_wiring/tests/cli.rs                             |   330 +-
 Cargo.lock                                         |     3 +
 benches/Cargo.toml                                 |    22 +
 120 files changed, 3863 insertions(+), 54130 deletions(-)
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
 M 00_nucleo/prompts/infra/export/p1250_svg_integrated.md
 M 00_nucleo/prompts/infra/pipeline.md
 M 00_nucleo/prompts/infra/pipeline/context_stabilization.md
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
 M Cargo.lock
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
?? 00_nucleo/diagnosticos/layout-microbenchmarks.md
?? 00_nucleo/diagnosticos/p1505-content-representation-ab.md
?? 00_nucleo/diagnosticos/p1506-pdf-builder-atomizacao.md
?? 00_nucleo/diagnosticos/pdf-builder-microbenchmarks.md
?? 00_nucleo/diagnosticos/pipeline-atomization.md
?? 00_nucleo/diagnosticos/stdlib-tests-atomization.md
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
?? 00_nucleo/prompts/compiler/layout/measurement.md.orig
?? 00_nucleo/prompts/compiler/layout/page_geometry.md.orig
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md
?? 00_nucleo/prompts/compiler/layout/par_line_callbacks.md.orig
?? 00_nucleo/prompts/compiler/layout/style_scope.md
?? 00_nucleo/prompts/compiler/layout/style_scope.md.orig
?? 00_nucleo/prompts/compiler/stdlib/registry/contextual.md
?? 00_nucleo/prompts/compiler/stdlib/registry/data.md
?? 00_nucleo/prompts/compiler/stdlib/registry/diagnostics.md
?? 00_nucleo/prompts/compiler/stdlib/registry/figure_image.md
?? 00_nucleo/prompts/compiler/stdlib/registry/formats.md
?? 00_nucleo/prompts/compiler/stdlib/registry/foundations.md
?? 00_nucleo/prompts/compiler/stdlib/registry/layout.md
?? 00_nucleo/prompts/compiler/stdlib/registry/math.md
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
?? 01_core/src/compiler/stdlib/registry/contextual.rs
?? 01_core/src/compiler/stdlib/registry/data.rs
?? 01_core/src/compiler/stdlib/registry/diagnostics.rs
?? 01_core/src/compiler/stdlib/registry/figure_image.rs
?? 01_core/src/compiler/stdlib/registry/formats.rs
?? 01_core/src/compiler/stdlib/registry/foundations.rs
?? 01_core/src/compiler/stdlib/registry/layout.rs
?? 01_core/src/compiler/stdlib/registry/math.rs
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
?? benches/Cargo.toml.orig
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
?? benches/pdf_builder_bench.rs
?? benches/pipeline_bench.rs
```

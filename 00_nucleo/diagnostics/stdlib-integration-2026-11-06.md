# Integração do reúso adiado da stdlib

O reúso adiado foi integrado ao produto, com L0s atualizados antes do código. O ELF final preservou os controles funcionais e reduziu CPU nos documentos com vários imports, tanto na CLI quanto no watch. O controle sem imports não apresentou penalidade estável. Não foram integrados outros candidatos de otimização.

As verificações globais não estão todas verdes no estado recebido: permanecem uma expectativa antiga de paginação, avisos de Clippy, um benchmark que não compila e diferenças de formatação. Os impedimentos foram confrontados com o baseline; não foram corrigidos ou ocultados nesta integração.

## Estado e fronteira da mudança

Baseline: HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-06T14:55:42.061560+00:00`, com 53302 arquivos protegidos. O diffstat recebido é exatamente o apêndice de [stdlib-reuse-2026-11-05.md](stdlib-reuse-2026-11-05.md): 184 arquivos, 8388 inserções e 537 remoções. As alterações anteriores de fontes, imagens, layout e plugins compõem o baseline.

Foram modificados somente os dois consumidores existentes de eval/imports e seus dois owners; foram acrescentados um helper, seu testfile e os dois owners correspondentes. Não houve edição de Cargo/lockfile, configuração de build, World, fontes, parser, layout ou exportação. O código de laboratório anterior foi preservado. Sem commit, staging, publicação ou instalação de um binário fora da RAM.

| Consumer produtivo | Owner L0 |
| --- | --- |
| `01_core/src/compiler/eval/mod.rs` | `00_nucleo/prompts/core/compiler/eval/mod.md` |
| `01_core/src/compiler/eval/modules.rs` | `00_nucleo/prompts/core/compiler/eval/modules.md` |
| `01_core/src/compiler/eval/stdlib_snapshot.rs` | `00_nucleo/prompts/core/compiler/eval/stdlib_snapshot.md` |
| `01_core/src/compiler/eval/stdlib_snapshot/tests.rs` | `00_nucleo/prompts/core/compiler/eval/stdlib_snapshot/tests.md` |

O novo helper tem owner exclusivo e consome o núcleo existente de identidade builtin. Não se criou novo Nucleus ou relação 1:N de ownership. A integração foi explicitamente autorizada pelo usuário; é mecanismo privado sem mudança de contrato público/default/fase, no fluxo contínuo da ADR-0127.

Cada EvalContext começa com slot vazio. O root e as fachadas de expressão mantêm seus builders e Scope owned. Só o primeiro import cria o template; imports seguintes precisam coincidir em todos os pares ordenados de inputs e Features completas. Mismatch recebe um seed para o próprio subtree, sem substituir o seed do caller. A instanciação recria identidades Module/Element/With e preserva aliases, spans, argumentos e autoridade Std/Format. O grafo continua limitado ao registry builtin qualificado, não a módulos avaliados, closures/plugins ou construtores arbitrários.

Não há memoização de Module, cache entre compilações, seletor de laboratório ou novo método em World. A prova de frescor das fontes e a ordem original de consultas de inputs/fontes/arquivos permanecem no pipeline.

## Materialização, testes e linhagem

Foi aplicado o [skill Tekt de código/testes independentes](/home/dikluwe/.codex/skills/tekt-materializacao-segregada/SKILL.md): A escreveu a implementação em cópia RAM, B escreveu os testes sem consultar A, C executou o RED antes de instalar A. Um adaptador diagnóstico com comportamento legado de builder sem seed compilou e executou os testes: **24 passaram e quatro falharam** na nova obrigação de seed/instanciação. Não foi uma falha de compilação contada como RED funcional.

A instalação ocorreu em `2026-11-06T15:10:14.562891+00:00`. A primeira fonte selada ficou congelada em `15:16:51.508171+00:00`. A limpeza posterior foi exclusivamente a retirada de uma conversão EcoString redundante no testfile, sem alteração de asserções; a fonte final foi congelada em `15:38:43.547940+00:00`. O release foi recompilado e seus bytes ficaram **idênticos** aos anteriores à limpeza test-only. As medições abaixo usam esse release final, com contagens repetidas após a recompilação.

- **29 testes novos passaram**, incluindo o constructor real com slot vazio, inputs até 4096 pares/mais de 1 MiB Unicode, ordem e diferenças no primeiro/meio/último par, todas as 64 transições de Features no helper, ancestrais/subtrees, grafo real fechado, Module/raw.line/With, ocorrências/spans e controles públicos de imports/edições/diagnósticos. Os 29 foram repetidos depois da limpeza de Clippy.
- **Core completo: 8368 passaram, cinco ignorados.** Essa execução precede somente a conversão test-only descrita acima; nenhuma asserção ou implementação produtiva mudou depois dela.
- **Infra completa: 1652 passaram, dois ignorados.** O primeiro launcher não herdou o PATH com `rsvg-convert` e produziu três falhas ambientais; a execução completa com PATH corrigido passou, sem modificar esses testes.
- Os demais alvos compilados de integração foram executados. A suíte `language_and_export_contract` passou nos dez testes com `TYPST_VANILLA` apontando para o oráculo **atual** ratificado `64c5e8c02fe0394d913ead36e42ef8c1334d2107`, cujos bytes foram verificados contra o SHA-256 aprovado no guia. A mensagem histórica do teste menciona bd5, mas esse pin antigo não foi usado. Isso é uma nova execução desse corpus pequeno contra o pin atual, não uma certificação geral de paridade.
- **Doctests: cinco passaram. Rustdoc estrito do workspace passou.**
- **6/6 mutações compiladas foram rejeitadas por asserções executadas:** compartilhar identidades, ignorar inputs, validar só o comprimento, ignorar Features, substituir o seed do ancestor e capturar antecipadamente no constructor real. Cinco exercitam helpers mutados privados; a última modifica o constructor na cópia RAM. Não são seis campanhas completas de pipeline, nem crédito por erro de compilação.
- **Lint arquitetural estrito sem erros/avisos**, ownership V15/núcleos V26 válidos e forward/reverse seals consistentes. O dry-run final de reparo retornou `Nothing to fix`. Não se adicionou supressão.

Os testes independentes de criação de contexto foram mantidos co-localizados no helper e a suíte maior no filho test-only. Isso resolve o reconhecimento V2 sem duplicar gates cfg ou criar um teste fictício; as asserções de B permaneceram as mesmas.

### Pendências anteriores preservadas

O teste `p1292_d_flush_prefix_suffix_clearance_nested_and_noop` continua falhando: NESTED_AFTER aparece na página 2, enquanto o teste espera 1. O teste **original e inalterado** foi também compilado/executado apontando para a CLI baseline: falhou na mesma asserção. A fixture isolada produziu os mesmos textos, páginas e coordenadas nos releases baseline/produto. Não se alterou a expectativa nem o layout para fazer a suíte passar.

Clippy estrito parou nos mesmos três `large_enum_variant` do baseline (`page_canvas.rs`, `paint.rs`, `tiling.rs`). A auditoria sem promoção global de warnings confirmou, em ambos, o aviso existente de inicialização em `svg_image.rs` e o erro E0063 de `lab/benches/pdf_builder_bench.rs:116`, que omite o campo `format`. A checagem completa de lib/tests do core foi executada separadamente: a única advertência nova no testfile foi corrigida; a rodada final não emitiu diagnósticos nos quatro consumers alterados. O aviso de itens após módulo de testes em `layout/grid/row_groups.rs` é anterior e esse arquivo permaneceu intacto.

`cargo fmt --all -- --check` retornou exatamente o mesmo diff no baseline e no produto: seis arquivos anteriores em introspect/layout. Os consumers alterados estão formatados. Não se formatou o trabalho anterior para esconder essa pendência. Assim, não se afirma que o workspace inteiro terminou com todos os gates globais verdes.

## Receita das medições

Tudo descartável foi compilado/executado em `/dev/shm/tekt-stdlib-integration-20261106`: workspaces, targets, ELFs, mutantes, caches, corpus de pacotes, logs e PDFs. Rust 1.92.0; Cargo offline/locked, jobs=1, incremental off, release recebido s/ThinLTO/uma unidade e exceção zune-jpeg preservada; `.cargo/config.toml` completo com `-C llvm-args=-align-all-functions=3`, sem override de rustflags. Launchers de build limitados a 6 GiB/swap=0, no máximo dois builds simultâneos; builds funcionais de testes separados em dev/debug=0. O cache RAM anterior foi apenas copiado.

As bancadas reutilizam os drivers/checkers independentes de `stdlib-qualification` e `stdlib-cli-residual`, com apenas o diretório RAM rebaseado pelo launcher. Não se integrou qualquer sonda ao produto.

Corpus real: Oxifmt 1.0.0, Suiji 0.3.0 e Numbly 0.1.0, em cache privado preparado dos arquivos versionados de `packages.typst.org/preview`, com fontes normais herdadas. As listas de fontes dos ELFs coincidiram integralmente: 3203 linhas. Dois perfis: vazio e 512 pares/74734 bytes UTF-8, mesmos arquivos da qualificação anterior. Em outro controle funcional, editar **somente** o pacote Oxifmt numa cópia privada acionou o watch e mudou A→B nos dois releases; o corpus dos relógios ficou imutável.

Lote principal: 12 rodadas, uma repetição, cinco fixtures, três rótulos — baseline `bas`, hardlink do mesmo baseline `dup` (A/A) e produto `prd`. CLI, startup do watch e dois pares same-bytes/edit por processo. A ordem cobre as posições dos três rótulos. Foram 720 processos/2160 publicações medidos de `15:57:14.473682` a `16:02:22.674127` UTC. Depois, controle CLI sem imports com fonte/root/cwd compartilhados, 36 rodadas/duas variantes por perfil: 144 processos/publicações, até `16:02:38.531494` UTC. CPU0, Rayon=1, stack 64 MiB, epoch=0, PDF com `--pdf-tagged=false`, cache de SO aquecido. Nenhum build/teste/debugger/tracer/extrator de PDF correu durante os relógios.

CPU da CLI é wait4; CPU do watch é delta por ciclo. Wall inclui publicação e debounce. RSS é pico do processo inteiro, não por fase/ciclo. Médias dentro de cada rodada e medianas de razões pareadas; bootstrap de 10000 reamostragens/seed=0, IC exploratórios de 95%, sem correção de múltiplas comparações. Valores absolutos e percentuais pareados não se derivam um do outro pela razão entre medianas. Não houve descarte posterior de rodadas.

## Ganhos no ELF produtivo

Redução de **CPU**, produto contra baseline, com 512 inputs:

| Fixture | CLI | Startup watch | Watch após edição |
| --- | ---: | ---: | ---: |
| 16 imports locais | 8,16% | 4,28% | 21,94% |
| Oxifmt repetido | 4,69% | 3,65% | 18,68% |
| Suiji | 3,71% | inconclusivo | 10,48% |
| Numbly, um import | sem ganho conclusivo | sem ganho conclusivo | sem ganho conclusivo |

Oxifmt repetido: ICs CLI [−6,42%; −3,00%], startup [−8,13%; −1,82%] e edição [−22,07%; −16,56%]. Contra o rótulo A/A `dup`, as três reduções também tiveram ICs negativos. Suiji startup teve ponto −1,50%, mas IC [−3,27%; +3,01%]; não é apresentado como ganho confirmado. Sem inputs, Oxifmt repetido reduziu CPU em 5,67%/3,84%/21,62%, e 16 imports em 6,58%/6,52%/20,89%, respectivamente.

No controle CLI compartilhado sem imports, delta de CPU com 512 pares **−0,60% [−1,13%; +0,35%]**, ou −0,599 ms [−1,143; +0,340]; no vazio, −0,12% [−0,64%; +0,55%]. Não apareceu a penalidade estável de +0,74%. Isso não prova custo zero ou equivalência sob uma margem previamente aprovada. Os controles principais sem imports e com Numbly também tiveram ICs incluindo zero; o lote grande foi mais ruidoso, inclusive contra o mesmo ELF `dup`. Não se generalizam ganhos a todo documento ou a qualquer host.

O custo de memória não é apresentado como vantagem: no watch/512, pico mediano de fanout 16 foi 26,89→28,52 MiB e Oxifmt repetido 40,94→41,33 MiB; Suiji 39,30→38,71 MiB. São picos em sessões curtas, não prova de retenção em sessões longas.

As contagens externas, sem usar duração do debugger, confirmaram no release final:

| Caso | Builders baseline → produto | World.inputs/Sources |
| --- | ---: | --- |
| CLI sem imports | 1→1 | iguais: 1/1 |
| CLI 16 imports | 17→2 | iguais: 17/17 |
| CLI Oxifmt repetido | 17→2 | iguais: 17/3 |
| CLI Suiji | 9→2 | iguais: 9/7 |
| Watch 16 imports, acumulado startup/mesmos bytes/edição | 17/34/51→2/4/6 | iguais: 17/34/51 em ambos os contadores |

No watch sem imports, builders e queries acumulam 1/2/3 nos dois lados; Sources 1/1/2. Portanto a economia vem de composição da stdlib, sem supressão de leitura/avaliação dos imports ou da aquisição de inputs.

As **2304 publicações** passaram no checker e em **1512 comparações** de texto/posições/fontes entre variantes. Uma seleção de **50 pares/100 páginas** também coincidiu em rasterização a 50 dpi. Esses são controles adicionais de regressão no mesmo compilador, não uma definição de paridade como igualdade dos bytes do PDF.

[Medições de domínio](../../lab/results/construction/diagnostics/stdlib-integration-2026-11-06/) incluem CSVs brutos, comparações contra `bas` e `dup`, RSS, contagens e o controle CLI compartilhado. `sample_epoch` registra o instante de cada observação. Código/testes permanecem no produto; logs, recibos, ELFs, PDFs, caches próprios e mutantes não são persistidos no repositório.

## Diffstat exato do estado produtivo medido

O apêndice inclui as alterações prévias recebidas. Além deste diffstat de arquivos rastreados, os quatro arquivos novos de código/L0 são os dois consumers e dois owners de `stdlib_snapshot` listados acima. O diagnóstico e os CSVs são resultados posteriores, sem efeito no ELF.

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 ++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/mod.md                                   |  33 ++++++++-
 00_nucleo/prompts/core/compiler/eval/modules.md                               |  36 +++++++++-
 00_nucleo/prompts/core/compiler/eval/observation/elements.md                  |  63 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/eval/repr/content.md                          |  30 +++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set.md                             |  41 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set/text.md                        |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/eval/tests.md                                 |  29 +++++++-
 00_nucleo/prompts/core/compiler/introspect.md                                 |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/convergence.md                     |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/figure_demands.md                  |  19 ++++-
 00_nucleo/prompts/core/compiler/introspect/materialize_time.md                |  32 ++++++++-
 00_nucleo/prompts/core/compiler/introspect/page_running.md                    |  23 +++++-
 00_nucleo/prompts/core/compiler/introspect/walk.md                            |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/bidi_origin.md                         |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/block.md                               |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 ++++++++++++++--
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 ++++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 ++++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  40 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 ++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 ++++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 ++++++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  37 +++++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 ++-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 ++++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  37 +++++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 ++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 ++++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  37 +++++++++-
 00_nucleo/prompts/core/entities/content/construction.md                       |  70 ++++++++++++++++++-
 00_nucleo/prompts/core/entities/content/fields.md                             |  30 +++++++-
 00_nucleo/prompts/core/entities/content/tests/traversal.md                    |  19 ++++-
 00_nucleo/prompts/core/entities/content/traversal.md                          |  20 +++++-
 00_nucleo/prompts/core/entities/elements/figure.md                            |  20 +++++-
 00_nucleo/prompts/core/entities/elements/image.md                             |  64 ++++++++++++++++-
 00_nucleo/prompts/core/entities/image_format.md                               |  26 ++++++-
 00_nucleo/prompts/core/entities/image_sizer.md                                |  26 ++++++-
 00_nucleo/prompts/core/entities/layout_types.md                               |  31 +++++++-
 00_nucleo/prompts/core/entities/mod.md                                        |  24 ++++++-
 00_nucleo/prompts/core/entities/plugin_func.md                                |  72 ++++++++++++++-----
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 ++++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 +++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  82 ++++++++++++++++++++--
 00_nucleo/prompts/infra/export/render.md                                      |  36 +++++++++-
 00_nucleo/prompts/infra/export/stream/draw.md                                 |  10 ++-
 00_nucleo/prompts/infra/export/svg/images.md                                  |  19 ++++-
 00_nucleo/prompts/infra/export/svg/tests.md                                   |  10 ++-
 00_nucleo/prompts/infra/export/tests/mod.md                                   |  27 ++++++-
 00_nucleo/prompts/infra/fontdb/selective.md                                   |  10 ++-
 00_nucleo/prompts/infra/lib.md                                                |  17 ++++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md                     |  19 ++++-
 00_nucleo/prompts/infra/pipeline/paged.md                                     |  17 ++++-
 00_nucleo/prompts/infra/pipeline/tests.md                                     | 125 ++++++++++++++++++++++++++++++++-
 00_nucleo/prompts/infra/shaper/bidi.md                                        |  37 +++++++++-
 00_nucleo/prompts/infra/shaper/shaping.md                                     |  35 +++++++++-
 00_nucleo/prompts/infra/tests/svg_composite_graph.md                          |  10 ++-
 00_nucleo/prompts/shell/cli.md                                                |  19 ++++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++--
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/mod.rs                                              |   7 +-
 01_core/src/compiler/eval/modules.rs                                          |   9 ++-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 +++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 +++++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  34 ++++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++++++---
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          |   9 +--
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 ++++-
 01_core/src/compiler/layout/columns.rs                                        |  32 +++++----
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 ++++++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 +++-----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 ++++++++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  75 +++++++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |   9 +--
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 +++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 ++++++++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 ++++++++++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/tracks.rs                                    | 114 ++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 ++++++++++++++++++---
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +----
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 ++++++++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  78 ++++++++++++++++++---
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +--
 01_core/src/compiler/layout/place.rs                                          |  67 ++++++++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |   9 +--
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  86 +++++++++++++++++++++--
 01_core/src/compiler/layout/repeat.rs                                         |   7 +-
 01_core/src/compiler/layout/set_page.rs                                       |  16 +----
 01_core/src/compiler/layout/shape.rs                                          |   7 +-
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++--
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  12 +++-
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 ++-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 ++-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++++++--
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 ++++++++++++++++++++++++------
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++++-
 01_core/src/compiler/stdlib/visualize.rs                                      |   7 +-
 01_core/src/entities/content/construction.rs                                  |   9 ++-
 01_core/src/entities/content/fields.rs                                        |   4 +-
 01_core/src/entities/content/tests/traversal.rs                               |   4 +-
 01_core/src/entities/content/traversal.rs                                     |   4 +-
 01_core/src/entities/elements/figure.rs                                       |  79 ++++++++++++++++++++-
 01_core/src/entities/elements/image.rs                                        |  21 ++++--
 01_core/src/entities/image_format.rs                                          |   9 ++-
 01_core/src/entities/image_sizer.rs                                           |  13 +++-
 01_core/src/entities/layout_types.rs                                          |   4 +-
 01_core/src/entities/mod.rs                                                   |   4 +-
 01_core/src/entities/plugin_func.rs                                           | 379 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++------------
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            | 288 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--------
 03_infra/src/export/render.rs                                                 |  58 ++++++++++++++-
 03_infra/src/export/stream/draw.rs                                            |   3 +-
 03_infra/src/export/svg/images.rs                                             |   8 ++-
 03_infra/src/export/svg/tests.rs                                              |   4 +-
 03_infra/src/export/tests/mod.rs                                              |  15 +++-
 03_infra/src/fontdb/selective.rs                                              | 534 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++--
 03_infra/src/lib.rs                                                           |   4 +-
 03_infra/src/pipeline/context_stabilization.rs                                |   4 +-
 03_infra/src/pipeline/paged.rs                                                |   4 +-
 03_infra/src/pipeline/tests.rs                                                | 593 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                                                   |  78 ++++++++++++++++++++-
 03_infra/src/shaper/shaping.rs                                                |  14 ++--
 03_infra/tests/svg_composite_graph.rs                                         |   3 +-
 Cargo.lock                                                                    | 275 ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++---
 188 files changed, 8466 insertions(+), 544 deletions(-)
```

Conferência final em `2026-11-06T16:22:10.894210+00:00`: os 53298 arquivos recebidos fora das quatro edições autorizadas mantiveram seus bytes; o diffstat e as fontes produtivas medidas continuaram idênticos. Os quatro reverse seals também foram conferidos por conteúdo.

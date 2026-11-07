# Reúso da biblioteca padrão por compilação

O experimento confirmou custo evitável na construção repetida da stdlib. O candidato de laboratório reduz o builder de N+1 para uma chamada por avaliação, sem memoizar imports e sem compartilhar identidades observáveis entre arquivos. O ganho se concentra em projetos com muitos imports. **Não foi integrado ao produto.** A preservação de linguagem passou no corpus; isso não constitui prova geral de qualquer biblioteca/registro futuro.

## Estado medido

HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, working tree não commitada, congelada em `2026-11-05T21:46:35.699489+00:00`. A lista exata recebida está no apêndice: 184 arquivos, 8388 inserções, 537 remoções. As correções anteriores de imagens/plugins estão incluídas. Nenhum arquivo produtivo ou L0 foi alterado neste experimento.

O baseline veio da cópia congelada. O candidato altera apenas, na cópia RAM, `compiler/eval/mod.rs`, `compiler/eval/modules.rs` e o novo helper `lab_stdlib_snapshot.rs`. Seu código definitivo está em [lab/experiments/stdlib-reuse](../../lab/experiments/stdlib-reuse/). A escreveu patch/helper/driver; B escreveu testes independentemente; C aplicou, compilou e executou. Os agentes compartilharam o checkout original, mas não as soluções antes da conclusão independente; não se afirma isolamento de filesystem.

Rust 1.92.0; Cargo offline/locked; release `opt-level=s`, thin LTO, codegen-units=1 e override existente de zune-jpeg. Um job por build, MemoryHigh 8 GiB, MemoryMax 12 GiB, swap zero. Fontes, caches e executáveis descartáveis ficaram na área exclusiva `/dev/shm/tekt-stdlib-reuse-20261105`, posteriormente removida. O cache anterior foi somente copiado. Baseline SHA-256 `a1f1a7033a991f6443bb8345c4f5fc2f72e05cd58ba9598a0e4db98fe8096b3d`; candidato final `cbc5951b59d22029c7de04badad6173e199f9abb285a799dbf9a2f5d9e9e41d8`.

O primeiro protótipo foi recusado: sua suposição de funções apenas nativas provocou panic em 12 controles. `registry/structural.rs:97` registra `raw.line` via `Func::element`, com construtor estático em `entities/elements/raw.rs`. A revisão foi congelada em `2026-11-05T22:09:56.175825+00:00`, recompilada e validada antes dos relógios. Os números abaixo são exclusivamente dessa revisão. O controle específico de identidade de `raw.line` foi acrescentado sem enfraquecer expectativas anteriores.

## Mecanismo e correção

A fonte atual constrói stdlib em `eval/mod.rs:640`, `:848` e `eval/modules.rs:60`. `Module::PartialEq` (`entities/module.rs:65`) usa identidade de Arc; `Func::PartialEq` distingue Element/With por identidade. Esses mecanismos tornam a igualdade **na linguagem** entre arquivos observável. Os L0s atuais de eval/imports exigem preservar a divergência cross-file existente, não corrigi-la incidentalmente com uma otimização.

O snapshot é privado no EvalContext e novo a cada entrada. Cada import continua consultando World.inputs na posição original; o guard compara todos os pares na ordem e todas as features. Divergência cria um template novo apenas naquele subtree. Não há cache global/TLS/comemo, mudança de World/Engine, cache de Module avaliado ou compartilhamento de scopes locais, estilos, show-rules ou registry do usuário.

Cada arquivo recebe Scope e invólucros Module novos, preservando kind Std/Format, nomes, carriers e aliases. Namespaces são percorridos; raw.line recebe Func Element novo com o mesmo construtor estático. O memo local usa identidade de Arc, evitando confundir nativas de mesmo nome com diferentes chamadas/namespaces. O código trata With, mas sua produção identificada em `structural/math/accent.rs:80` é dinâmica e não parte da composição estática medida; não se afirma cobertura geral dessa clonagem. Closure/Plugin no template continuam recusados. Isso é uma implementação limitada à stdlib deste estado, não uma API geral de cópia.

O façade público `__lab_resolve_stdlib` existe **somente no overlay**, para exercitar o mesmo guard dos imports. Mismatch de features no helper é diagnóstico, não caminho normal da CLI: features normalmente permanecem fixas no root. Esse façade não é proposta de API produtiva.

Passaram 17 controles públicos idênticos em baseline/candidato e dois controles adicionais do guard no candidato. Cobrem igualdade em linguagem de módulos e raw.line, aliases, imports repetidos/aninhados/pacote, inputs alterados/reordenados, roots/Worlds/features/targets intercalados, registros/shadowing/include/show isolados, erros/hints/spans/traces e host de plugins. Três mutações compiladas foram rejeitadas por execução: compartilhar identidades, ignorar inputs e ignorar features. Não se aceitou erro de compilação como rejeição.

## Construção, Sources e layout

Contagens GDB em executáveis release, fora dos relógios:

|Imports, fan-out|Builder baseline → candidato|Source::new_with_parser, ambos|Convergência / Layouter, ambos|
|---|---|---|---|
|0|1 → 1|1|1 / 1|
|1|2 → 1|2|1 / 1|
|4|5 → 1|5|1 / 1|
|8|9 → 1|9|1 / 1|
|16|17 → 1|17|1 / 1|
|64|65 → 1|65|1 / 1|

Nested/repeated 64 também reduzem 65 → 1 builders. No watch com 16 imports, builders cumulativos são 17/34/51 no baseline e 1/2/3 no candidato (startup, mesmos bytes, edição). Sources permanecem 17/34/51 em ambos. Com 64, são 65/130/195 Sources em ambos e builders 65/130/195 → 1/2/3. O limite existente da SourceSession não foi alterado. Source::new_with_parser conta entradas, não tempo exclusivo de parse; funções inline não observadas continuam fora do contador. Não há alegação de reduzir layout: as fronteiras observadas permanecem iguais.

## Tempos no pipeline

Lote principal: 13 fixtures, 10 rodadas × duas repetições, pares alternados/rotacionados, 1040 processos medidos e 2080 publicações registradas. Aquecimento separado. CPU do filho fixada no primeiro CPU permitido (0), Rayon 1, stack 64 MiB, epoch 0, UTC, fontes padrão do sistema. Sem builds, depuradores, profilers, contadores de alocação ou extração de PDF durante os relógios. Campos CPU do watch vêm de clock por processo; a soma dos ciclos foi conferida contra wait4. RSS é pico do processo inteiro, repetido nas linhas de watch, não memória de cada fase.

Cada célula abaixo é mediana das médias de duas repetições por rodada. Percentuais/intervalos vêm dos dez deltas pareados por rodada, bootstrap de medianas (10 mil reamostragens, seed 0); diferença das medianas absolutas não precisa coincidir com o delta pareado.

|Fixture|CLI wall, ms|Startup watch wall, ms|Edição main: CPU, ms|Edição main: wall, ms|
|---|---:|---:|---:|---:|
|0 imports|93,28 → 94,38|136,38 → 138,40|18,97 → 19,26|88,53 → 88,92|
|1 import|96,00 → 94,83|139,70 → 137,65|20,19 → 19,21|89,97 → 88,44|
|4 imports|97,87 → 96,26|141,25 → 140,79|22,84 → 20,65|92,40 → 90,30|
|8 imports|101,88 → 97,48|146,18 → 141,38|26,32 → 22,60|95,60 → 92,12|
|16 imports|107,98 → 101,73|151,86 → 146,21|33,93 → 26,19|103,32 → 95,33|
|64 imports|153,80 → 122,49|200,91 → 166,99|77,41 → 47,80|146,95 → 117,44|
|Nested 64|178,76 → 146,48|221,73 → 187,68|79,21 → 52,00|148,55 → 121,42|
|Repeated 64|151,62 → 121,27|196,02 → 164,96|74,18 → 45,52|143,79 → 115,11|

16 imports: CLI −5,92% [−6,39; −5,36], startup −3,78% [−4,21; −3,07], CPU quente −21,20%. 64: CLI −20,38% [−20,71; −20,29], startup −17,02% [−17,75; −16,66], CPU quente −38,01%. Os intervalos completos, incluindo regravação de mesmos bytes, estão nos CSVs. Wall quente inclui aproximadamente 70 ms de eventos/debounce: não é tempo puro de compilação.

Os controles não autorizam ganho universal: outline ficou inconclusivo; import_compute não melhorou a CLI; paragraph registrou CLI +0,65% e startup +1,25%. São controles de outros custos, não amostras de todas as cargas reais.

### Repetição motivada pelo controle negativo

O lote principal indicou 0 imports CLI +1,06% [0,71;1,56] e startup +1,41% [1,13;2,14]. Isso motivou **novo lote separado**, mesmos binários, 0/16/64 imports, outras dez rodadas × duas repetições: 240 processos, 480 publicações.

|Imports|CLI wall, ms|Startup wall, ms|CPU quente, ms|
|---|---:|---:|---:|
|0|93,25 → 93,62|136,50 → 137,25|19,41 → 18,99|
|16|108,07 → 100,55|152,75 → 144,38|33,63 → 26,10|
|64|153,10 → 122,61|199,57 → 167,50|77,80 → 48,23|

Na repetição, CLI 0 imports ficou +0,20% [0,06;0,71]; startup +0,23% [−0,28;0,81]. O acréscimo maior não se reproduziu de forma estável, mas não foi demonstrada regressão zero. Não combinar os lotes para esconder o primeiro resultado. A causa do residual cold **não foi atribuída**. O eval isolado abaixo não mostra perda no caso zero; isso impede atribuir automaticamente todo o residual à captura do snapshot.

## Eval separado de fontes, parse, layout e exportação

Sete rodadas pareadas em executáveis de laboratório otimizados com `-C opt-level=s`, ligados às mesmas bibliotecas release. Sources pré-parseadas, World sem logs por chamada, quatro avaliações e descarte completo por caso. O relógio inclui eval/realização pura e pequenas verificações iguais nos dois lados. Não há layout/exportação/fontes ou parse nesse intervalo, nem LTO completo do harness; seus absolutos não devem ser subtraídos dos tempos da CLI.

|Imports|Eval baseline → candidato por chamada, ms|Variação pareada|
|---|---:|---:|
|0|1,094 → 1,082|−0,51% [−2,97;2,36]|
|1|1,853 → 1,390|−24,31%|
|4|4,279 → 2,528|−40,81%|
|8|7,700 → 4,172|−46,01%|
|16|14,585 → 7,421|−49,07% [−49,31;−48,68]|
|64|56,657 → 27,969|−50,78% [−51,71;−50,63]|
|Nested 64|64,677 → 36,526|−43,25%|
|Repeated 64|56,699 → 28,045|−50,54%|

## Alocações em bancada separada

Reutilizado o forwarding System allocator de `catalog-tests`, com controles independentes zeroed/grow/shrink/free aprovados em ambos os binários. O código do contador não mudou. Foram observadas três avaliações no mesmo SystemWorld real, fontes embutidas; import I/O/parse ainda pode compor essa janela, diferente do mock pré-parseado. Não usar os tempos desses processos. Pedidos somam capacidades de alloc/realloc: **não são bytes copiados, RSS ou memória simultaneamente viva**; mmap e malloc estrangeiro ficam fora.

Na primeira avaliação, 16 imports: 113058 → 82707 alocações, 35,60 → 24,05 MB solicitados. Com 64: 432264 → 310857 alocações e 136,10 → 89,92 MB. O saldo após descarte permanece igual entre lados em cada fixture; não se conclui vazamento a partir dele. O pico cumulativo do alocador em nested64 caiu de 58,51 para 53,74 MB, sem promessa equivalente de RSS. O caso zero acrescenta uma alocação e 136 bytes nessa janela.

## Saída e delimitação

Conferidos todos os **2560 PDFs** dos dois lotes: 1280 pares com texto/geometria/fontes equivalentes e epochs corretos. Dezesseis pares de amostra tiveram todas as suas páginas rasterizadas a 50 dpi e pixels iguais. Não se exigiu igualdade binária de PDF. Em seis watches adicionais, editar apenas a dependência atualizou fan-out/nested16 de Total 16 para 17 e repeated16 para 32 nos dois lados. As verificações de edição não entram nos relógios.

Os dados de domínio ficam em [stdlib-reuse-2026-11-05](../../lab/results/construction/diagnostics/stdlib-reuse-2026-11-05/). Fixtures sintéticas e testes estão com o código experimental; controles adicionais acompanham os dados. Logs, mutantes, caches e executáveis não foram conservados no repositório.

**Conclusão aplicável:** há um candidato funcionalmente compatível com o corpus, com ganho reproduzido em imports. A construção deixou de ser N+1; a instanciação por arquivo e as cópias de scopes continuam custando, e as fontes continuam dominando muitos casos pequenos. O experimento não sustenta melhorar qualquer documento nem eliminar o gargalo de fontes.

Antes de propor integração: (1) atribuir o residual cold/zero imports com controle da política no mesmo código medido e fases L3 separadas de eval, sem ler ambiente em L1; (2) só então escolher materialização sob L0s próprios para snapshot, eval e imports, com pins/owners válidos, eliminando o façade diagnóstico; (3) revalidar o corpus em uma amostra de pacotes reais e os controles de zero imports/inputs grandes. Não alterar igualdade de Module/Func para viabilizar cache. Compartilhamento de corpos Scope é outra hipótese, não resultado deste experimento: o owner atual de Module ainda especifica scope:Scope e precisaria ser revisto antes de qualquer materialização produtiva. Nenhuma dessas alterações foi implementada.

## Diffstat exato do estado recebido

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 ++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++++-
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
 184 files changed, 8388 insertions(+), 537 deletions(-)
```

Integridade conferida em `2026-11-05T22:53:22.379361+00:00`: os 52935 arquivos recebidos mantêm os mesmos bytes, incluindo todos os L0s e as quatro camadas produtivas. O diffstat recebido permanece idêntico. Adições limitadas ao novo laboratório/diagnóstico e dados de domínio.

Os builds release isolados e `crystalline-lint --quiet .` passaram. A aprovação estrutural refere-se ao produto ativo preservado; os overlays experimentais não foram apresentados como materialização produtiva selada.

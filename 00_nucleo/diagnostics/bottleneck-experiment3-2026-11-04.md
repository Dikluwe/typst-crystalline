# Experimento 3 — catálogo atual e reúso de recursos

Resultado: a separação preservou os resultados dos controles do modelo e da bancada com fontes reais, sob a premissa de catálogo integralmente validado. Foi possível reutilizar recursos não alterados entre gerações sem conservar a seleção anterior. **Não demonstra benefício na CLI/watch nem autoriza integração ao produto.**

## Método e fronteira

Skill tekt-materializacao-segregada: A escreveu modelo e bancada; B escreveu testes a partir da mesma especificação sem consultar A; C compilou, executou e aplicou mutantes. Tudo experimental em /dev/shm/tekt-e3-20261104. Rust 1.92.0; modelo/tests std-only via rustc --edition=2021 --test; bancada Cargo standalone offline/release, um job e TMPDIR/target na RAM, fontdb 0.23.0 e ttf-parser 0.25.1. Primeira compilação 11,79 s; segunda locked. Nenhuma alteração produtiva ou de L0.

Modelo: snapshot.verified é uma premissa externa, nunca inferida do número da geração. Rejeição limpa cache. A seleção é refeita na lista atual (primeira família exata válida, ordem explícita). A chave do payload inclui caminho, índice da face, identidade Unix completa, bytes exatos e época do parser. Não retém negativos nem índices crus de slots. Cache por instância, Arc de bytes próprios preserva Worlds antigos. Orçamento limita apenas bytes dos payloads retidos, não chaves, estruturas ou Arcs externos; não é teto de RSS. Baseline é o mesmo modelo com orçamento zero.

Bancada real: diretório controlado e plano, redescoberto integralmente via fontdb a cada etapa; observações explícitas anteriores/posteriores com diretório, lstat/stat e bytes, recusando diferença. Essa verificação cobre a fixture estável, **não substitui a prova fontconfig/raízes/configuração/concorrência do Tekt**. Seleção simplificada difere do FontBook real. Um memo auxiliar Weak/Arc evita derivar métricas novamente quando o payload vivo é o mesmo. Métricas reais comparadas: units_per_em, quantidade de glifos e glyph_index de A, além da igualdade dos bytes/face/payload; parse fresco adicional em cada demanda controla o resultado. Não cobre shaping, cobertura completa, renderização, variações ou diagnósticos do compilador.

## Correção nos controles

31 testes independentes passaram: mudança não usada, novo preferido da mesma família, remoção/fallback, reordenação, geração igual com entradas diferentes, bytes diferentes com identidade igual, todas oito componentes da identidade, face, época do parser, negativa reparada, catálogo não verificado, isolamento de instâncias, orçamento exato/zero/oversize e payload antigo preservado.

17 mutantes compilados foram rejeitados, 17/17: omitir caminho, face, bytes, época do parser, verificação, validade, seleção atual, limpeza após recusa ou orçamento; e omitir separadamente cada um dos oito campos de identidade. Nenhum sobrevivente. A sensibilidade a esses erros não certifica completude ou a captura do filesystem.

Três rodadas finais independentes da sequência real abaixo, com três demandas por etapa, tiveram os mesmos contadores e resultados esperados. Fontes: NotoSans-Regular.ttf do repositório e DejaVuSans.ttf do host. A fonte preferida da mesma família usa os mesmos bytes em outro caminho e ordem explicitamente alterada; não demonstra o algoritmo de prioridade real do produto.

|Etapa|Faces no catálogo|Parses sem retenção|Parses com reúso|Hits|
|---|---:|---:|---:|---:|
|base|1|3|1|2|
|add-unused|2|3|0|3|
|new-preferred-same-family|3|3|1|2|
|remove-preferred|2|3|0|3|
|invalid-candidate|2|3|1|2|
|repaired-preferred|3|3|1|2|
|atomic-replace|3|3|1|2|
|symlink-before|3|3|0|3|
|symlink-retarget|3|3|0|3|

Etapas: inicial, adição não usada, novo preferido mesma família, remoção do preferido, candidato inválido, reparo com preferência, substituição atômica, symlink antes/depois de retarget. Fontdb canonicaliza/deduplica alguns caminhos: a bancada de symlink confirma o resultado da nova descoberta, enquanto os testes do modelo cobrem troca explícita de identidade/bytes. O caso inválido da bancada não testa uma demanda ausente exata antes/depois; isso foi coberto nos testes independentes.

Cada rodada: 27 parses sem retenção, cinco com reúso, 22 hits e 27 parses adicionais de validação fresca. A adição não usada produziu três hits e nenhum novo parse do recurso usado; novo preferido e reparo produziram payload novo; remoção refez a seleção atual e pôde recuperar recurso ainda válido. O Arc do World inicial conservou seus 556.216 bytes após todas as mudanças.

## Custos e limites de benefício

Cada rodada manteve 72 leituras explícitas de arquivos e 46,057,780 bytes lidos no catálogo controlado. **Exclui I/O interno do fontdb**, portanto não é contagem total do filesystem. Mediana da soma dos nove envelopes de catálogo: 20.133 ms.

|Métrica da sequência inteira|Mediana de três rodadas|
|---|---:|
|Resolve sem retenção (seleção, chave, cópias e construção)|2.581 ms|
|Resolve com cache (seleção, chave, cópias e lookup)|1.367 ms|
|Parse de recursos sem retenção|0.038334 ms|
|Parse de recursos com reúso|0.002645 ms|

Esses envelopes não são latência do compilador. A diferença de resolve inclui cópias específicas do modelo; o baseline com orçamento zero também constrói uma chave, o que não representa o custo exato do FontSlot produtivo. A contagem de parses evitados é real nesta bancada, mas o parse pequeno é barato e não comprova benefício geral. Os parses internos de descoberta de fontdb continuam ocorrendo e não entram no contador de parses de recursos. Não se calculou speedup do produto nem se subtraíram medianas para inventar fases.

Controle TTC separado: NotoSansCJK-Regular.ttc do host, dez faces e duas rodadas de demanda por face. Vinte demandas, dez parses e dez hits; comparação com parse fresco por demanda. A representação ingênua reteve 194.847.840 bytes de payload, mais a mesma quantidade aproximada de bytes nas chaves, antes de overhead. Ela não compartilha os bytes da coleção entre faces. O RSS pico do processo completo (incluindo o controle TTC e ambos os caminhos) ficou em [463880, 463852, 463736] KiB. Não são medições separadas de RSS baseline/candidato; não provam economia de memória.

Na sequência simples, o cache terminou com 3.188.088 bytes de payload e também conservou recursos de seleções anteriores sob o limite. Arcs externos podem prolongar retenção após eviction. O limite de payload do modelo não resolve orçamento total do produto.

## Implicação para Tekt

A hipótese é viável nos controles **se catálogo atual continuar autoridade, seleção for refeita, payload for realmente imutável e chave completa for estabelecida**. Reúso pode preservar uma fonte utilizada quando outra parte do catálogo muda; vigiar apenas a fonte antiga continuaria incorreto, porque uma nova fonte pode vencer a seleção. O experimento 2 não é contornado: nenhuma leitura da prova foi deduplicada aqui.

O gargalo de aquisição/validação não foi removido. O ganho potencial fica em reconstrução de recursos/derivações, dependente da carga. O produto já retém o par FontBook/slots por geração; essa bancada só estuda reúso adicional quando a geração muda. Sem mudança do catálogo, a retenção já existente pode tornar o ganho marginal nulo.

Não transportar diretamente os objetos atuais: fontes simples podem usar mmap (fonts.rs e world_types.rs), enquanto este modelo usa Vec próprio. Arc compartilha propriedade, não demonstra imutabilidade do arquivo mapeado diante de escrita externa. TTC também requer preservar índice e reconstrução da face; aqui somente parse indexado foi testado, sem a extração produtiva.

L0 font_session exige uma geração pareada, sem ampliar lifetime ou guardar faces/slots separadamente; nenhuma extensão foi autorizada por este resultado. Antes de eventual integração: medir recursos realmente demandados pela CLI/watch e seu custo de reconstrução em uma mudança real de catálogo, definir identidade/lifetime de payload imutável e limite total de memória, e testar seleção/cobertura/shaping/render/diagnósticos reais. Esta etapa não rastreou os lookups das cinco fixtures no compilador completo e não demonstra equivalência da seleção simplificada com Typst.

## Proveniência

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, working tree não commitado. Início registrado 2026-11-04T18:15:12.128031+00:00; verificação final 2026-11-04T18:21:13.008677+00:00. Os 2411 arquivos registrados de fontes/L0/diagnósticos permanecem byte a byte iguais. Única adição persistente desta tarefa: este diagnóstico. Fontes, testes e resultados experimentais são descartáveis na RAM e removidos no encerramento, conforme skill.

<details><summary>git diff HEAD --stat=200 exato do estado medido</summary>

```text
 00_nucleo/prompts/core/compiler/eval/bindings/field_access.md                 |  19 +++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md |  51 +++++++++-
 00_nucleo/prompts/core/compiler/eval/bindings/field_access/fields.md          |  30 +++++-
 00_nucleo/prompts/core/compiler/eval/observation/elements.md                  |  63 ++++++++++++-
 00_nucleo/prompts/core/compiler/eval/repr/content.md                          |  30 +++++-
 00_nucleo/prompts/core/compiler/eval/rules/set.md                             |  41 +++++++-
 00_nucleo/prompts/core/compiler/eval/rules/set/text.md                        |  42 ++++++++-
 00_nucleo/prompts/core/compiler/eval/tests.md                                 |  29 +++++-
 00_nucleo/prompts/core/compiler/introspect.md                                 |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/convergence.md                     |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/figure_demands.md                  |  19 +++-
 00_nucleo/prompts/core/compiler/introspect/materialize_time.md                |  32 ++++++-
 00_nucleo/prompts/core/compiler/introspect/page_running.md                    |  23 ++++-
 00_nucleo/prompts/core/compiler/introspect/walk.md                            |  16 +++-
 00_nucleo/prompts/core/compiler/layout/bidi_origin.md                         |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/block.md                               |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/checkpoint.md                          |  61 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns.md                             |  60 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/columns/flow.md                        |  18 +++-
 00_nucleo/prompts/core/compiler/layout/columns/segmented.md                   |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/columns/transitions.md                 |  16 +++-
 00_nucleo/prompts/core/compiler/layout/cursor.md                              |  40 +++++++-
 00_nucleo/prompts/core/compiler/layout/equation.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/figure.md                              |  32 ++++++-
 00_nucleo/prompts/core/compiler/layout/flush.md                               |  19 +++-
 00_nucleo/prompts/core/compiler/layout/footnote.md                            |  39 +++++++-
 00_nucleo/prompts/core/compiler/layout/footnote_flush.md                      |  24 ++++-
 00_nucleo/prompts/core/compiler/layout/footnote_fragments.md                  |  31 +++++-
 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md                  |  95 ++++++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/grid/tracks.md                         |  81 +++++++++++++++-
 00_nucleo/prompts/core/compiler/layout/helpers.md                             |  10 +-
 00_nucleo/prompts/core/compiler/layout/image.md                               |  58 +++++++++++-
 00_nucleo/prompts/core/compiler/layout/measurement.md                         |  19 +++-
 00_nucleo/prompts/core/compiler/layout/mod.md                                 |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/page_run.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/page_running_requests.md               |  53 ++++++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph.md                           |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/paragraph_indent.md                    |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/place.md                               |  32 ++++++-
 00_nucleo/prompts/core/compiler/layout/placement.md                           |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/references.md                          |  48 +++++++++-
 00_nucleo/prompts/core/compiler/layout/region_progress.md                     |  42 ++++++++-
 00_nucleo/prompts/core/compiler/layout/repeat.md                              |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/set_page.md                            |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/shape.md                               |  37 +++++++-
 00_nucleo/prompts/core/compiler/layout/slicing.md                             |  10 +-
 00_nucleo/prompts/core/compiler/layout/stack.md                               |  69 +++++++++++++-
 00_nucleo/prompts/core/compiler/layout/sub_frame.md                           |  17 +++-
 00_nucleo/prompts/core/compiler/layout/text/style.md                          |  34 ++++++-
 00_nucleo/prompts/core/compiler/layout/tiling.md                              |  37 +++++++-
 00_nucleo/prompts/core/compiler/math/layout/delimited.md                      |  80 +++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/external_bridge.md                |  34 ++++++-
 00_nucleo/prompts/core/compiler/math/layout/mod.md                            |  81 +++++++++++++++-
 00_nucleo/prompts/core/compiler/math/layout/op.md                             |  52 +++++++++-
 00_nucleo/prompts/core/compiler/realize/figure.md                             |  32 ++++++-
 00_nucleo/prompts/core/compiler/realize/figure_kind.md                        |  19 +++-
 00_nucleo/prompts/core/compiler/stdlib/figure_image.md                        |  87 ++++++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/tests/figure_image.md                  |  65 ++++++++++++-
 00_nucleo/prompts/core/compiler/stdlib/text/constructor.md                    |  42 ++++++++-
 00_nucleo/prompts/core/compiler/stdlib/visualize.md                           |  37 +++++++-
 00_nucleo/prompts/core/entities/content/construction.md                       |  70 +++++++++++++-
 00_nucleo/prompts/core/entities/content/fields.md                             |  30 +++++-
 00_nucleo/prompts/core/entities/content/tests/traversal.md                    |  19 +++-
 00_nucleo/prompts/core/entities/content/traversal.md                          |  20 +++-
 00_nucleo/prompts/core/entities/elements/figure.md                            |  20 +++-
 00_nucleo/prompts/core/entities/elements/image.md                             |  64 ++++++++++++-
 00_nucleo/prompts/core/entities/image_format.md                               |  26 ++++-
 00_nucleo/prompts/core/entities/image_sizer.md                                |  26 ++++-
 00_nucleo/prompts/core/entities/layout_types.md                               |  31 +++++-
 00_nucleo/prompts/core/entities/mod.md                                        |  24 ++++-
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 +++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 ++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  31 +++++-
 00_nucleo/prompts/infra/export/render.md                                      |  36 ++++++-
 00_nucleo/prompts/infra/export/stream/draw.md                                 |  10 +-
 00_nucleo/prompts/infra/export/svg/images.md                                  |  19 +++-
 00_nucleo/prompts/infra/export/svg/tests.md                                   |  10 +-
 00_nucleo/prompts/infra/export/tests/mod.md                                   |  27 +++++-
 00_nucleo/prompts/infra/lib.md                                                |  17 +++-
 00_nucleo/prompts/infra/pipeline/context_stabilization.md                     |  19 +++-
 00_nucleo/prompts/infra/pipeline/paged.md                                     |  17 +++-
 00_nucleo/prompts/infra/pipeline/tests.md                                     | 125 +++++++++++++++++++++++-
 00_nucleo/prompts/infra/shaper/bidi.md                                        |  37 +++++++-
 00_nucleo/prompts/infra/shaper/shaping.md                                     |  35 ++++++-
 00_nucleo/prompts/infra/tests/svg_composite_graph.md                          |  10 +-
 00_nucleo/prompts/shell/cli.md                                                |  19 +++-
 01_core/src/compiler/eval/bindings/field_access.rs                            |   4 +-
 01_core/src/compiler/eval/bindings/field_access/content_methods.rs            |  21 ++++-
 01_core/src/compiler/eval/bindings/field_access/fields.rs                     |   6 +-
 01_core/src/compiler/eval/observation/elements.rs                             |  34 ++++---
 01_core/src/compiler/eval/repr/content.rs                                     |  12 ++-
 01_core/src/compiler/eval/rules/set.rs                                        |  54 ++++++++++-
 01_core/src/compiler/eval/rules/set/text.rs                                   |  42 ++++++++-
 01_core/src/compiler/eval/tests.rs                                            |   4 +-
 01_core/src/compiler/introspect.rs                                            |   4 +-
 01_core/src/compiler/introspect/convergence.rs                                |   4 +-
 01_core/src/compiler/introspect/figure_demands.rs                             |   4 +-
 01_core/src/compiler/introspect/materialize_time.rs                           |  34 ++++++-
 01_core/src/compiler/introspect/page_running.rs                               |  68 +++++++++++--
 01_core/src/compiler/introspect/walk.rs                                       |  69 +++++++++++++-
 01_core/src/compiler/layout/bidi_origin.rs                                    |   7 +-
 01_core/src/compiler/layout/block.rs                                          |   9 +-
 01_core/src/compiler/layout/checkpoint.rs                                     |  17 +++-
 01_core/src/compiler/layout/columns.rs                                        |  32 ++++---
 01_core/src/compiler/layout/columns/flow.rs                                   |  58 +++++++++++-
 01_core/src/compiler/layout/columns/segmented.rs                              |  30 ++----
 01_core/src/compiler/layout/columns/transitions.rs                            |  72 +++++++++++++-
 01_core/src/compiler/layout/cursor.rs                                         |  75 ++++++++++++++-
 01_core/src/compiler/layout/equation.rs                                       |   9 +-
 01_core/src/compiler/layout/figure.rs                                         | 240 ++++++++++++++++++++++++++++++++++++++++++++--
 01_core/src/compiler/layout/flush.rs                                          |   3 +-
 01_core/src/compiler/layout/footnote.rs                                       | 171 ++++++++++++++++++++++++++++++++-
 01_core/src/compiler/layout/footnote_flush.rs                                 |  71 +++++++++++++-
 01_core/src/compiler/layout/footnote_fragments.rs                             |  99 +++++++++++++++++--
 01_core/src/compiler/layout/grid/cell_emission.rs                             | 167 +++++++++++++++++++++++++++++---
 01_core/src/compiler/layout/grid/tracks.rs                                    | 114 ++++++++++++++++++++--
 01_core/src/compiler/layout/helpers.rs                                        |   5 +-
 01_core/src/compiler/layout/image.rs                                          |  80 +++++++++++++---
 01_core/src/compiler/layout/measurement.rs                                    |   4 +-
 01_core/src/compiler/layout/mod.rs                                            |   6 +-
 01_core/src/compiler/layout/page_run.rs                                       |  16 +---
 01_core/src/compiler/layout/page_running_requests.rs                          |  79 +++++++++++++++-
 01_core/src/compiler/layout/paragraph.rs                                      |  78 +++++++++++++--
 01_core/src/compiler/layout/paragraph_indent.rs                               |   9 +-
 01_core/src/compiler/layout/place.rs                                          |  67 +++++++++++--
 01_core/src/compiler/layout/placement.rs                                      |   9 +-
 01_core/src/compiler/layout/references.rs                                     | 141 ++++++++++++++++++++++++++-
 01_core/src/compiler/layout/region_progress.rs                                |  86 ++++++++++++++++-
 01_core/src/compiler/layout/repeat.rs                                         |   7 +-
 01_core/src/compiler/layout/set_page.rs                                       |  16 +---
 01_core/src/compiler/layout/shape.rs                                          |   7 +-
 01_core/src/compiler/layout/slicing.rs                                        |   4 +-
 01_core/src/compiler/layout/stack.rs                                          |  17 +++-
 01_core/src/compiler/layout/sub_frame.rs                                      |  57 ++++++++++-
 01_core/src/compiler/layout/text/style.rs                                     |  11 ++-
 01_core/src/compiler/layout/tiling.rs                                         |  12 ++-
 01_core/src/compiler/math/layout/delimited.rs                                 | 211 ++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/math/layout/external_bridge.rs                           |  10 +-
 01_core/src/compiler/math/layout/mod.rs                                       |  10 +-
 01_core/src/compiler/math/layout/op.rs                                        |  91 +++++++++++++++++-
 01_core/src/compiler/realize/figure.rs                                        |  76 ++++++++++++++-
 01_core/src/compiler/realize/figure_kind.rs                                   |   6 +-
 01_core/src/compiler/stdlib/figure_image.rs                                   | 114 +++++++++++++++++-----
 01_core/src/compiler/stdlib/tests/figure_image.rs                             | 282 +++++++++++++++++++++++++++++++++++++++++++++++++++++-
 01_core/src/compiler/stdlib/text/constructor.rs                               |  20 +++-
 01_core/src/compiler/stdlib/visualize.rs                                      |   7 +-
 01_core/src/entities/content/construction.rs                                  |   9 +-
 01_core/src/entities/content/fields.rs                                        |   4 +-
 01_core/src/entities/content/tests/traversal.rs                               |   4 +-
 01_core/src/entities/content/traversal.rs                                     |   4 +-
 01_core/src/entities/elements/figure.rs                                       |  79 +++++++++++++++-
 01_core/src/entities/elements/image.rs                                        |  21 ++++-
 01_core/src/entities/image_format.rs                                          |   9 +-
 01_core/src/entities/image_sizer.rs                                           |  13 ++-
 01_core/src/entities/layout_types.rs                                          |   4 +-
 01_core/src/entities/mod.rs                                                   |   4 +-
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            |  27 +++++-
 03_infra/src/export/render.rs                                                 |  58 +++++++++++-
 03_infra/src/export/stream/draw.rs                                            |   3 +-
 03_infra/src/export/svg/images.rs                                             |   8 +-
 03_infra/src/export/svg/tests.rs                                              |   4 +-
 03_infra/src/export/tests/mod.rs                                              |  15 ++-
 03_infra/src/lib.rs                                                           |   4 +-
 03_infra/src/pipeline/context_stabilization.rs                                |   4 +-
 03_infra/src/pipeline/paged.rs                                                |   4 +-
 03_infra/src/pipeline/tests.rs                                                | 593 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++-
 03_infra/src/shaper/bidi.rs                                                   |  78 ++++++++++++++-
 03_infra/src/shaper/shaping.rs                                                |  14 +--
 03_infra/tests/svg_composite_graph.rs                                         |   3 +-
 Cargo.lock                                                                    | 275 +++++++++++++++++++++++++++++++++++++++++++++++++++--
 180 files changed, 7181 insertions(+), 437 deletions(-)
```

</details>

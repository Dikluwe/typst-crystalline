# Fontes: consultas de filesystem e candidato statx

**Resultado:** a prova vigente executa 3.803 consultas nos wrappers e outras 116 consultas de metadata internas à configuração. O custo da prova completa isolada ficou próximo de12ms CPU, dos quais cerca de9ms em kernel. A variante preservadora statx passou nos controles, mas economizou somente0,100ms CPU por prova na confirmação longa. Nenhuma otimização foi integrada ao produto.

## Captura da prova atual

O medidor usa os corpos atuais de generation.rs (ProbeKey, configuração, identity e GenerationKey), extraídos sem alterações no baseline. Setup usa fontdb0.23.0 e o parser original para obter os paths/índices na mesma ordem de prepare_from; os stamps e a prova são validados antes do relógio. Não constrói todo FontBook nem executa eval/layout/export. A prova cronometra o equivalente ao hit font_session: config_key fresco, roots e stamps da geração, nesta ordem. A captura por query usa um módulo instrumentado separado, fora dos relógios; timers não executam logging por stat.

|Área|Nofollow|Follow|Total|
|---|---:|---:|---:|
|Guards explícitos de configuração|2|0|2|
|ProbeKey: raízes/diretórios/candidatos/links|1345|1344|2689|
|GenerationKey: arquivo de cada face|0|1112|1112|
|Wrappers totais|1347|2456|3803|

Foram1.345 stamps de descoberta e1.112 stamps de faces, 1.371 paths distintos e26 repetições de mesma operação/path dentro da mesma área. Repetições entre áreas são barreiras distintas, não economia demonstrada. As26 repetições de coleção foram mantidas. Nofollow observou65 diretórios,193 symlinks,1.086 arquivos e3 paths ausentes/outros (incluindo os dois guards de configuração). A classe de path foi conferida após a captura; não define um snapshot atômico.

Comprimento dos paths: mínimo16, mediana64 e máximo98 bytes; número de barras:3/7/9. São descritores do trabalho de resolução, não uma medição isolada de seu tempo. Nenhum parent-fd foi retido e nenhum path foi canonicalizado pelo novo backend para evitar resolução fresca.

O trace de uma verificação completa delimitada por marcadores contou3.919 consultas de metadata:3.803 nos wrappers acima e116 dentro do parser/leitura de configuração. Também contou436 readlink,111 openat,220 read,111 close e2 getdents64. A sequência lógica de4.799 solicitações, incluindo path/follow/flags/status/tamanhos de read, coincidiu entre baseline e variantes. Strace é controle de contagem/ordem, não relógio da performance.

## Primeira variante: statat

Linuxx86_64/rustix1.0.8 safe statat usa newfstatat. Os oito campos e os controles ordinários de Btrfs/tmpfs coincidiram, e14 mutantes foram rejeitados. Foram substituídos3.801 statx por newfstatat, mantendo as118 consultas da configuração e todos os outros pedidos. O ganho pareado na prova completa foi só0,072ms CPU; dispersões8,11% e10,03%, sem evidência de vantagem material.

A revisão de contrato encontrou uma diferença de lookup: fstatat trata o componente terminal como NO_AUTOMOUNT, enquanto o statx observado do std não pede essa flag. Portanto essa troca não foi aceita como substituição transparente geral. A diferença está documentada em [statx(2)](https://man7.org/linux/man-pages/man2/statx.2.html) e [stat(2)](https://man7.org/linux/man-pages/man2/stat.2.html). Não foram criadas montagens/automounts para testar essa classe; o candidato foi refinado antes de qualquer integração.

## Candidato preservador: statx com os campos utilizados

Mantém a syscall statx e flags exatas do std: AS_STAT (default) com ou sem SYMLINK_NOFOLLOW. Pede TYPE|MODE|INO|SIZE|MTIME|CTIME (mask0x3c3): dev reconstruído de major/minor, inode, tamanho, mode completo, mtime/ctime com segundos assinados convertidos aos mesmos bits u64 e nanossegundos integrais. Não pede btime/atime/uid/gid/nlink/blocks, que não pertencem à identidade atual.

decode_statx exige a presença dos seis bits em stx_mask; ausência devolve Unsupported e a prova recusa a otimização, em vez de usar zeros. Bits extras não impedem aceitação. São mantidos dois requests independentes nofollow→follow mesmo em arquivo regular, ausência só nos lugares permitidos, follow único da geração, short-circuit, regular-file check e ordem dos stamps. Nenhum cache, dedup, retry, fallback oculto, fd retido ou unsafe próprio.

A feature linux_4_11 de rustix foi necessária para remover seu wrapper de compatibilidade, que possui estado global de disponibilidade e pode executar um probe adicional após o primeiro erro. A bancada qualificou Linuxx86_64/kernel≥4.11. Isso não aprova automaticamente essa feature para o produto: impacto de unificação de features e compatibilidade precisaria de auditoria antes de integração.

No trace candidato, as3.919 chamadas continuaram statx:3.801 com mask mínima e118 com STATX_ALL da configuração. Flags de lookup/sincronização não mudaram; NO_AUTOMOUNT/DONT_SYNC/FORCE_SYNC não foram introduzidos. Erros EINTR/EACCES/EIO/ENOSYS injetados na primeira chamada produziram exatamente uma syscall e recusa, sem retry/probe/fallback. NUL pré-kernel preserva ErrorKind de recusa; rawerrno pode diferir do std nesse caso e não é parte da decisão da prova.

Um exemplo real em /usr/share/fonts devolveu stx_mask=STATX_ALL|STATX_MNT_ID|0x8000 apesar do pedido mínimo. A API permite campos extras e pode omitir campos solicitados, razão para verificar a máscara de resposta. Isso não demonstra redução proporcional de trabalho interno pelo pedido menor. Ver [semântica de stx_mask](https://man7.org/linux/man-pages/man2/statx.2.html).

## Medição por componente

Sete pares alternados AB/BA no mesmo processo/binário,100 verificações por bloco, afinidadeCPU0, sem builds/trace concorrentes nem limpeza de page cache. Setup e captura ficam fora do relógio. CPU por CLOCK_PROCESS_CPUTIME_ID e wall por Instant. Usuário/kernel por /proc/self/stat a100Hz (10ms/tick): valores aproximados, resolução de0,1ms por validação nos blocos de100; não usar a soma de medianas como total preciso. Configuração é controle com o mesmo código nos dois rótulos, sem cache XML.

Valores em ms por verificação; ganho é a mediana dos deltas pareados, não necessariamente diferença das medianas. Componentes foram medidos separadamente e não devem ser somados.

|Escopo|CPU std|CPU candidato|Ganho CPU pareado|Ganho wall pareado|CPU menor|Dispersão CPU std/candidato|
|---|---:|---:|---:|---:|---:|---:|
|probe|6.366706|6.284556|0.086226|0.087013|6/7|3.16%/2.35%|
|generation|2.554177|2.498169|0.052839|0.053187|7/7|1.59%/2.85%|
|config|2.313483|2.319567|0.006236|0.006297|4/7|1.98%/1.35%|
|full|11.585849|11.280386|0.305464|0.305582|7/7|3.64%/2.74%|

O primeiro full teve baseline com dispersão3,64%, acima do critério3%. Para conferir o pequeno efeito, repetiu-se somente full em outro processo, com sete pares de300 verificações por bloco, sem trocar código nem descartar observações.

Confirmação: CPU 11.778674 → 11.703289 ms; ganho pareado 0.099510 ms. Wall 11.780457 → 11.704962 ms; ganho pareado 0.099678 ms. CPU menor em7/7 pares; dispersões 1.62%/2.32%.

|Confirmação full|Usuário aproximado ms|Kernel aproximado ms|
|---|---:|---:|
|std|2.800|8.967|
|candidato|2.667|9.067|

Na confirmação, a resolução de ticks equivale a aproximadamente0,033ms por verificação; estimativas de usuário/kernel e medianas independentes não fecham necessariamente a CPU total. Kernel inclui resolução de paths/metadata e demais I/O da configuração; não foi perfilado internamente para atribuir todo esse tempo a uma única função. Não se infere uma aceleração exata universal de0,100ms: o primeiro lote teve0,305ms e a direção se manteve, mas a magnitude é pequena e dependente da bancada.

## Correção e limite da conclusão

Skill tekt-materializacao-segregada: A escreveu código, B escreveu testes sem consultar A e C integrou/executou. Passaram22 testes da primeira variante; a preservadora passou nos mesmos22 mais13 de máscara/decoder, total35. Quatorze mutantes da primeira variante e18 da preservadora foram rejeitados após compilar, sem sobreviventes. Casos incluem ordem, duas observações em regular, mutações entre chamadas/faces/rodadas, ENOENT/ENOTDIR/EACCES/EINTR/EIO/ELOOP, nonregular, links/dangling/ciclos/retarget, chmod, substituição, mesmo tamanho com mtime restaurado, signed seconds/nsec, nonUTF8/NUL/path vazio/longo/relativo/trailing slash, device/proc e encoding dev/inode/size altos. EACCES foi exercitado sob UID1000; fixture em tmpfs e corpus do host em Btrfs, kernel 6.17.0-42-generic.

A preservação demonstrada é da decisão e dos campos/requests da prova isolada nesses controles, incluindo mudanças agendadas. O contrato continua best-effort por demanda, sem snapshot atômico de todo filesystem. Não foi conectado ao pipeline completo CLI/watch e não se mediu render/latência final/RSS por variante; não se executou a suíte do workspace. O resultado não é certificação de plataformas, filesystems remotos ou montagens não exercitadas.

## Decisão

Não integrar nenhuma variante. statat não é substituição universal do lookup atual. statx mínimo é o candidato mais delimitado para preservar a prova, mas o ganho confirmado é de ordem0,1ms por prova isolada. Isso não sustenta tratar troca de syscall/máscara/conversão Rust como solução para o custo principal de fontes. O tempo aproximado de kernel ficou perto de9ms e o número de observações foi integralmente mantido.

A próxima frente precisa mirar trabalho maior: quantificar duplicação entre descoberta e aquisição inicial da CLI, ou formular e demonstrar uma prova equivalente que permita menos observações. Não deduplicar as26 consultas TTC nem remover follow/stamps com base apenas na contagem. O L0 atual e o Nucleus continuam autoridade; qualquer candidato futuro requer bancada adversarial e medição líquida, e qualquer mudança de contrato de frescor exige o gate ADR-0127.

## Proveniência e amostras

HEAD 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, uncommitted working tree. Estado inicial 2026-11-05T01:29:37.203507+00:00. Captura de consultas e variantes pertencem a essa cópia de código. Medição statx inicial de 2026-11-05T02:02:27.462068+00:00 até 2026-11-05T02:02:59.519228+00:00; confirmação de 2026-11-05T02:07:51.112186+00:00 até 2026-11-05T02:08:40.438735+00:00. Nenhum arquivo recebido de código/L0/configuração/diagnóstico mudou entre essas medições; somente o experimento RAM variou, conforme os deltas acima. A lista rastreada exata do estado recebido está ao final.

Binários de bancada fs-bench SHA-256: statat `bb3b19c05386676dc6603163750e4fcedcdb10fdffac0abb2cc819a860209ec1`; statx mínimo `9de26b896b0a2219c1ec757b79356063cfe98f9b2aac51826e41d58065bb65ac`. Eles identificam os medidores, não um release produtivo do Tekt. Rust1.92.0, rustix1.0.8, fontdb0.23.0, fontconfig-parser0.5.8; lock local derivado do lock recebido, versões relevantes fixadas. Cargo offline/locked após resolução inicial, release opt-level=s/lto=thin/codegen-units=1, CARGO_INCREMENTAL=0, jobs1; unidades de build/mutação MemoryHigh8G/MemoryMax12G/MemorySwapMax0. Targets/cache/font fixtures/código/logs/mutantes descartáveis em /dev/shm/tekt-font-fs-20261104, removidos após verificação. Produto, Cargo.toml/Cargo.lock, L0 e selos preservados; sem commit.

[Amostras de CPU/wall/ticks](../../lab/results/construction/diagnostics/font-filesystem-2026-11-04/amostras.csv) e [sequência das consultas nos wrappers](../../lab/results/construction/diagnostics/font-filesystem-2026-11-04/consultas.csv). CPU/wall no CSV são totais do bloco; dividir por iterations. Ticks são10ms. Consultas guardam ordem, área, follow e path; resultados de strace incluem também operações internas à configuração. Os CSVs são dados locais de domínio sob a regra de ignore já existente de lab, sem force-add; não são logs/recibos de processo.

<details><summary>git diff HEAD --stat=240 --stat-name-width=200 do estado medido</summary>

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
 00_nucleo/prompts/core/entities/tiling.md                                     |  37 +++++++++-
 00_nucleo/prompts/infra/export/builder.md                                     |  15 ++++
 00_nucleo/prompts/infra/export/builder/resources.md                           |  15 +++-
 00_nucleo/prompts/infra/export/images.md                                      |  31 +++++++-
 00_nucleo/prompts/infra/export/images/png.md                                  |  20 +++++-
 00_nucleo/prompts/infra/export/images/scan.md                                 |  31 +++++++-
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
 01_core/src/entities/tiling.rs                                                |   7 +-
 02_shell/src/cli.rs                                                           |   4 +-
 03_infra/Cargo.toml                                                           |   7 ++
 03_infra/src/export/builder.rs                                                |   2 +-
 03_infra/src/export/builder/resources.rs                                      |   7 +-
 03_infra/src/export/images.rs                                                 |   7 +-
 03_infra/src/export/images/png.rs                                             |  93 +++++++++++++++++++++++-
 03_infra/src/export/images/scan.rs                                            |  27 ++++++-
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
 182 files changed, 7719 insertions(+), 443 deletions(-)
```

</details>

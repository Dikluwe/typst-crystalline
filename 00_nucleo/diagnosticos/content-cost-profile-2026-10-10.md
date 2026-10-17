# Perfil do custo restante após travessia com reuso — 2026-10-10

## Resultado e próximo candidato

O perfil foi refeito no release aceito após os itens 1–3. **O próximo experimento deve atacar a resolução repetida da cadeia de fontes nas métricas**, especialmente `text_edges` e `vertical_metrics`. A seleção no catálogo aparece em aproximadamente 25–29% das amostras de CPU de processo novo e foi chamada **48.019 vezes** em cada tabela de 1.600 células, com e sem preenchimento local. O corpo de `resolve_primary` foi chamado **6.401 vezes**; não há cache dessa cadeia no carrier atual de métricas.

A intervenção ainda não foi implementada. Estas medidas justificam testar reuso da resolução; não demonstram quanto tempo um cache economizaria. Trocar apenas o buffer temporário de `select_pattern` já foi uma hipótese rejeitada no diagnóstico anterior; não se deve confundir esse experimento com evitar a resolução inteira repetida.

## Referência atual contra vanilla

Três rodadas, quatro pares medidos por fixture e rodada, warmup excluído, mesma CPU, fontes, ambiente e stack nos dois motores. Comparações em processos novos, sem interpositor ou debugger nesta coleta.

| Caso | Tekt (ms) | Vanilla (ms) | Diferença pareada | RSS Tekt / vanilla (KiB) |
|---|---:|---:|---:|---:|
| local-set-100 | 108,05 | 129,91 | −16,7% | 29.760 / 31.674 |
| local-set-400 | 128,76 | 141,26 | −8,5% | 30.980 / 34.766 |
| local-set-1600 | 211,65 | 174,34 | **+21,3%** | 42.192 / 46.332 |
| no-fill-100 | 107,46 | 129,36 | −16,9% | 29.808 / 31.472 |
| no-fill-400 | 124,90 | 137,97 | −9,9% | 30.556 / 33.868 |
| no-fill-1600 | 193,26 | 169,01 | **+14,4%** | 38.000 / 43.414 |
| inline-set-1600 | 240,05 | 181,78 | **+31,1%** | 47.654 / 48.776 |
| alias-1600 | 193,54 | 169,86 | +14,3% | 37.792 / 44.204 |

As três rodadas repetem o sinal das diferenças nos casos grandes. O Tekt está mais rápido nos documentos menores e usa menos RSS nos casos medidos, mas seu custo cresce mais com a quantidade de células. De 100 para 1.600 células, local-set cresce aproximadamente 104 ms no Tekt e 44 ms no vanilla; no-fill cresce 86 ms e 40 ms. Isso favorece investigar trabalho repetido por célula. Não se inferiu daí uma função linear universal.

O acréscimo de estilo local em 1.600 células é aproximadamente **18,39 ms no Tekt** e **5,33 ms no vanilla**, tomando a diferença das medianas local-set/no-fill na mesma janela. Esse acréscimo é distinto do custo comum das métricas de texto.

## Tempo por fases: confirmação necessária

A primeira coleta de fases, agrupada por fixture, registrou tempos maiores e variáveis: em local-set-1600, eval 153,05 ms e layout 113,74 ms. Esses números não reproduzem o custo do benchmark principal e não fundamentam a decisão sobre a contribuição de cada fase. Foram preservados em `phases-current.json.gz`.

A inspeção de `04_wiring/src/compile.rs:415–445` e `03_infra/src/pipeline/pdf.rs:375–391,668–695` confirma que os caminhos com/sem `--timings-json` convergem ao mesmo `compile_to_pdf_bytes_impl`, com `oracle=false`; não se encontrou uma implementação distinta de compilação por essa flag. Uma coleta adicional alternou flag/sem flag e ordem dos dois documentos, com cinco pares medidos após warmup:

| Fase Tekt (ms) | local-set-1600 | no-fill-1600 |
|---|---:|---:|
| eval | 108,83 | 100,77 |
| introspect | 0,017 | 0,016 |
| expand_context | 0,353 | 0,107 |
| layout | 66,12 | 56,82 |
| shape | 18,88 | 19,33 |
| render | 6,39 | 6,26 |

As diferenças de medianas por fase sugerem **+8,06 ms de eval e +9,30 ms de layout** no caso local; não são tempo exclusivo de `StyleChain` ou de outro helper. A chamada `eval_to_module_with_sink_target_features` é cronometrada em `pipeline/paged.rs:316–325`; o campo não equivale somente a executar a expressão `table`.

Os tempos externos com flag foram 209,90/193,02 ms; sem flag, 224,37/197,02 ms. A mediana pareada flag/sem flag foi −3,46% no caso local e +0,40% no controle. Essa variação impede tratar pequenas diferenças por fase como atribuição exata. A medição externa principal permanece a referência bilateral.

## CPU: nova atribuição

`cpu-style-current.json.gz` registra 12 processos de cada um dos quatro controles grandes, amostrando CPU com ITIMER_PROF de 1 ms. A sonda de diagnóstico foi corrigida para conservar a identidade do executável: endereços de outras bibliotecas são marcados `x` e não enviados ao simbolizador do Tekt. As sondas antigas normalizavam offsets de todos os DSOs sem essa distinção; seus números não foram reutilizados como prova deste perfil.

| Caso | Amostras totais | Com select_pattern | Com resolve_primary |
|---|---:|---:|---:|
| block-no-set-1600 | 2.432 | 646 (26,6%) | 541 (22,2%) |
| no-fill-1600 | 2.311 | 679 (29,4%) | 572 (24,8%) |
| local-set-1600 | 2.629 | 704 (26,8%) | 580 (22,1%) |
| inline-set-1600 | 2.967 | 729 (24,6%) | 588 (19,8%) |

Estas são **presenças inclusivas**, sobrepostas; as porcentagens não se somam nem equivalem a tempo exclusivo. Frames desconhecidos e inlining limitam a atribuição. A função monomorfizada `Vec::from_iter` frequentemente aparece junto de `select_pattern`, coerente com o scan e construção de candidatos em `entities/font_book.rs:451–475`.

A maior parte das amostras com seleção e resolução primária está sob layout da tabela e da grid. Em local-set, 560 amostras com seleção aparecem sob layout da tabela; em no-fill, 555. O trabalho de descoberta/carregamento do catálogo também tem presença alta, mas é comum aos casos e não explica sozinho o crescimento por célula. A vantagem Tekt nos documentos curtos torna inadequado priorizar somente o startup para fechar a diferença dos casos grandes.

A travessia genérica não aparece como dominante neste perfil. Ausência de um símbolo em uma amostra **não prova custo zero**, sobretudo com monomorfização/inlining; a conclusão é prioridade relativa, não autorização para eliminar passagens semânticas.

## Contagem direta de chamadas

GDB contou entradas nos símbolos do **mesmo release**, sem usar o tempo sob debugger como medida. O driver de contagens não afixa a CPU; essa exceção não altera o significado de contagens de entrada. A primeira execução foi bloqueada pelo sandbox por ptrace; o rerun autorizado a depurar somente seus próprios processos produziu os recibos válidos em `resolution-calls.json.gz`.

| Entrada | local-set-1600 | no-fill-1600 |
|---|---:|---:|
| FontBook::select_pattern | 48.019 | 48.019 |
| resolve_primary | 6.401 | 6.401 |
| text_edges | 3.200 | 3.200 |
| vertical_metrics | 3.201 | 3.201 |
| layout_styled | 1.602 | 2 |

A repetição comum está ancorada no código: `font_metrics/fallback.rs:184–189,272–273` resolve primárias em cada chamada; `font_metrics/resolution.rs:27–85` procura famílias e acrescenta fallback textual; `font_metrics.rs:49–86` já mantém caches de faces e larguras, mas não dessa resolução.

## Alocações: custo comum e resíduo local

Duas repetições com amostragem determinística de uma solicitação glibc em cada 64 deram os mesmos resumos. A sonda completa conta aproximadamente **342.822 pedidos em no-fill**, **418.093 em local-set** e **549.176 em inline-set**, sem transformar esses números em heap vivo. A diferença de um pedido frente ao interpositor simples do item 3 pertence à instrumentação/caminho de execução, não a uma mudança de produto.

| Presença na pilha de alocação amostrada | no-fill | local-set | inline-set |
|---|---:|---:|---:|
| resolve_primary | 999 | 1.000 | 1.003 |
| StyleChain::collapse | 2 | 201 | 626 |
| collect_context_blocks_into | 0 | 50 | 50 |
| realize_document | 333 | 783 | 783 |
| layout_styled | 1.471 | 1.896 | 1.896 |

Os resumos completos e o frame de projeto mais próximo de cada alocação estão em `analysis.json`. Em local-set, `with_source_span` é esse frame em 300 amostras; em inline-set, em 375. Isso deixa origem/carriers como investigação posterior, preservando o núcleo de primeira origem.

A amostragem é periódica, não aleatória: a mesma posição de alocação dentro de um ciclo pode ser escolhida repetidamente. **Não multiplicar diferenças de grupos por 64 para provar alocações exclusivas**, nem comparar 685 contra 598 amostras de seleção como se isso contradissesse a contagem direta igual de chamadas. Pilhas de owners ancestrais se sobrepõem. Os métodos change-aware de Table/Cell ainda aparecem em alguns controles, embora o antigo anchor de trait-map não apareça; nenhuma ausência de reconstrução global foi certificada.

## Ordem das próximas hipóteses

1. **Reutilizar a resolução de fontes nas métricas.** Medir a repetição das identidades efetivas e experimentar reuso por documento entre `text_edges`/`vertical_metrics`, mantendo o contrato de famílias, variantes, variações, regex, ordem de fallback, MATH e legibilidade da face. Owners vigentes lidos: `prompts/infra/font_metrics.md`, `prompts/infra/font_metrics/resolution.md` e `prompts/entities/font_book.md`. Antes de código, atualizar os owners necessários e escrever controles de invalidação/paridade; retirar se tempo ou retenção piorarem. A hipótese será refutada se as identidades não se repetirem ou o custo de acesso/retenção anular o ganho.
2. **Separar o resíduo de estilo local.** A mesma contagem de seleção nos dois casos não explica as 1.600 entradas adicionais de escopo. Medir `StyleChain::collapse`/reconstrução de estilo e realização da célula, sobretudo em inline-set, onde o trabalho ainda se repete. Não remover a herança léxica para ganhar tempo.
3. **Depois investigar alocações de origem/carriers e catálogo.** Há evidência para ambos, mas menor ligação com o tempo incremental do corpus principal. Origem, disponibilidade de fontes e identidade semântica continuam requisitos.

Este passo não escolhe nova representação, não acrescenta vtable, não altera L0 nem código produtivo e não promete ganho antecipado.

## Equivalência medida e limites

Os 14 documentos comparados entre Tekt e vanilla têm o mesmo texto, palavras, páginas e tamanhos de página. A maior diferença de caixa de palavra foi **0,000024 pt**, registrada sem arredondá-la a zero. O raster de 36 dpi difere em todos: a paridade de raster **continua aberta**. Esses dados sustentam proximidade do trabalho de layout nesta matriz, não conformidade visual total nem equivalência de todos os documentos Typst.

Nenhum teste funcional ou rebuild produtivo foi necessário para a coleta: a árvore produtiva e os dois binários foram preservados. `crystalline-lint .` retornou zero, mantendo os avisos anteriores; o receipt final verifica identidade de todas as fontes Rust produtivas e dos binários.

## Proveniência e reprodução

Receipts em `00_nucleo/diagnosticos/content-cost-profile-2026-10-10/`:

- HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, **working tree não commitado**. `before.json.gz` guarda status/diffstat completos e SHA-256 de todas as fontes Rust L1–L4; os recibos individuais conservam estado, UTC, argumentos, fixtures e hashes de scripts/binários. O receipt final verifica que as fontes produtivas não mudaram.
- Tekt **9ae0a3d9285b8d0b4bae0da1db058483e7f5543003fabff46bb7d93b17e474f2**, `target/release/typst`, mesmo release aceito no item 3.
- Oracle commit **9f2b6e8715237cb086899a42873660fe744622e8**, árvore/receita P1489, `lab/typst-original/target/release/typst`, binário **7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea**.
- Tempo bilateral: **2026-10-10 02:15:18.086726 → 02:16:12.762086 UTC** (`measurements.json.gz`).
- CPU: **02:16:30.353426 → 02:16:46.331189 UTC** (`cpu-style-current.json.gz`).
- Alocações: **02:16:46.582612 → 02:16:53.911824 UTC** (`stacks-current.json.gz`).
- Contagens GDB: **02:19:12.911612 → 02:19:53.439104 UTC** (`resolution-calls.json.gz`).
- Fases confirmadas: **02:20:31.302028 → 02:20:37.417738 UTC** (`checked-phases.json.gz`).
- Qualificação: **02:17:29.785017 → 02:17:37.673865 UTC** (`qualification.json.gz`). Todas as janelas são de 2026-10-10.
- CPU fixada no menor ID permitido, `RAYON_NUM_THREADS=1`, stack 64 MiB, fontes explicitadas e ambiente limpo dos overrides TYPST/CRYSTALLINE/CARGO_PROFILE. A medição bilateral inclui hashes das fontes tipográficas.

Reproduzir com `python3 .../measure.py`, `phases_current.py`, `profile_cpu.py`, `stack_profile_current.py`, `qualify.py`, `count_resolution.py`, `check_phases.py` e `analyze.py`. GDB exige permissão ptrace; sondas C são compiladas por gcc no diretório de diagnóstico; `eu-addr2line` e `nm` usam o executável registrado. Coletas com sondas/debugger servem à atribuição e **não** ao benchmark de tempo/RSS. Os scripts sobrescrevem os respectivos recibos numa nova execução.

Continuação implementada: [reuso limitado da resolução primária](font-primary-reuse-2026-10-10.md), com métricas e preservação contra o release deste perfil.

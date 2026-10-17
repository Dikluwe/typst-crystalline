# Reaproveitamento local dos produtos de medição de células

A tabela simples de 320 células reduziu as entradas de subframe de 640 para 320. Nesta rodada, a mediana da CLI caiu de 215,84 para 210,50 ms (2,47%); a fase de layout caiu de 24,42 para 19,40 ms. A tabela numerada manteve as duas passagens do documento, mas reduziu os subframes de 1.280 para 640 e a CLI de 232,50 para 221,65 ms (4,67%). O ganho é limitado às células qualificadas, sem novo unsafe ou contrato público.

## Evidência antes da decisão

O [estudo anterior](reuso-sem-transformacao-2026-10-05.md) registrou diferenças de região, alinhamento e efeitos entre medir e emitir, além da remoção histórica de um cache amplo. A inspeção anterior à implementação está no L0 proprietário: `00_nucleo/prompts/compiler/layout/grid/measurement_cache.md`. `grid/tracks.rs` construía itens para medir a altura Auto e os descartava; `grid/cell_body.rs` reconstruía o corpo. `sub_frame.rs` restaura o pai, mas conserva largura e bottom como resultados laterais. A ausência de callbacks/identidade em um token simples permitiu investigar esse subconjunto primeiro.

O baseline desta etapa já contém avaliação única, índice semântico reutilizado, reuso de carriers sem transformação e reuso do primeiro layout independente. Portanto os ganhos desta rodada não devem ser somados às medianas de rodadas anteriores. O snapshot anterior ao novo código/L0 está em before-state; os testes e fontes RED estão em red-state.

O teste RED com duas células Auto Alpha/Beta compilou, mas registrou quatro layouts de corpo quando exigia dois. Não se usou falha de compilação como RED. Os testes adicionais de fallback/guardas e as suítes completas verificaram a implementação final.

## Implementação e limites

A nova unidade `01_core/src/compiler/layout/grid/measurement_cache.rs` possui um Prompt L0 exclusivo. Os owners grid, tracks, cell_emission e cell_body delegam a regra para essa unidade. Os L0 foram atualizados antes das mudanças produtivas; o ajuste é interno na mesma fase, seguindo o fluxo contínuo da ADR-0127. Sub_frame/tests possuem apenas instrumentação cfg(test) adicional.

A pool pertence a uma invocação do grid. Tracks guarda o produto completo da medição Auto; cell_body tenta consumi-lo antes de instalar a região da célula. Mantém a emissão existente para insets, rebase vertical, decoração, preenchimento, bordas, clipping e caudas. Também repõe last_sub_frame_width/bottom. A entrada retirada é consumida no máximo uma vez; uma falha posterior de compatibilidade não a recoloca na pool. Não há retenção entre layouts, campos novos em entidades/Layouter, comemo adicional, globals, I/O em L1 ou geração de identidades.

A prova aceita um token ASCII alfanumérico não vazio, uma sequência de um token com Empty neutro e wrappers simples de célula. Exige placement sequencial de todas as células; origem, largura, row/col e spans compatíveis; estilo efetivo igual ao emitido; ambiente/margens/decoração estáveis; produto de uma única folha Text e ausência de alinhamentos órfãos. A igualdade empregada é de dados mecânicos privados de render, não identidade/morfologia de Content.

Styled, Realized, Located, conteúdo contextual ou locatable, múltiplos tokens, matemática, smallcaps, floats pendentes, colunas, transações de fluxo, alinhamento especial, posições explícitas e spans especiais não recebem esse reuso. Fixed/Fraction não possuem produto Auto capturado. Qualquer divergência segue o layout anterior. A tolerância classificatória de 1e-9 pt na largura lateral apenas evita rejeição por cancelamento de f64; origem/largura de entrada e geometria emitida continuam exatamente iguais.

A sonda CLI wrapped_tokens conservou quatro subframes: o controle de inset/wrapper preservou a saída, mas não demonstrou cache. Não foi isolada nesta etapa a guarda específica responsável por esse miss; não extrapolamos o resultado de wrappers construídos manualmente para todos os carriers da linguagem. A tabela show_grid320 também ficou fora do cache e não demonstrou ganho.

## Contagem de produção

GDB contou todas as especializações locais encontradas por nm de layout_sub_frame_with_origin nos binários release, após o benchmark. Sem instrumentação produtiva ou GDB concorrente nos tempos.

| Fixture | Antes | Depois |
| --- | ---: | ---: |
| grid320 | 640 | 320 |
| numbered_grid320 | 1.280 | 640 |
| show_grid320 | 640 | 640 |
| plain_multi_table | 80 | 40 |
| multi_table | 80 | 80 |
| wrapped_tokens | 4 | 4 |
| rtl_tokens | 8 | 4 |

São entradas de subframe dessas fixtures, incluindo eventuais chamadas aninhadas, não uma contagem universal de células/layouts. Plain_multi_table contém um token por célula e quatro páginas; multi_table contém texto mais rico e sete páginas. RTL aqui mantém tokens ASCII com direção herdada, não comprova reuso de texto árabe/hebraico.

## Desempenho

Uma rodada de aquecimento e nove amostras intercaladas por motor/fixture na mesma CPU; RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, pretty=false e PDF sem tags. Tempos da CLI inteira incluem fontes, avaliação, layout, exportação e escrita do PDF. Não houve build, testes ou debugger concorrentes.

| Fixture | CLI antes, ms | CLI depois, ms | Variação | Layout antes → depois, ms |
| --- | ---: | ---: | ---: | ---: |
| hello | 176,85 | 176,65 | -0,11% | 0,51 → 0,53 |
| paragraphs | 243,20 | 242,18 | -0,42% | 27,87 → 27,06 |
| grid320 | 215,84 | 210,50 | -2,47% | 24,42 → 19,40 |
| eval30000 | 26,68 | 27,27 | +2,20% | 0,20 → 0,20 |
| show_headings500 | 249,04 | 249,85 | +0,33% | 30,58 → 30,52 |
| show_eval30000 | 195,78 | 196,64 | +0,44% | 165,97 → 166,90 |
| show_grid320 | 231,90 | 233,84 | +0,83% | 33,85 → 33,92 |
| multi_table | 187,14 | 186,86 | -0,15% | 4,56 → 4,53 |
| numbered_grid320 | 232,50 | 221,65 | -4,67% | 40,53 → 30,28 |
| plain_multi_table | 182,96 | 182,63 | -0,18% | 3,48 → 2,82 |
| wrapped_tokens | 178,44 | 178,54 | +0,05% | 0,57 → 0,59 |
| rtl_tokens | 178,50 | 178,86 | +0,20% | 0,69 → 0,66 |

Medianas, sem teste estatístico formal. Grid320 teve faixas CLI sobrepostas (213,95–221,11 antes; 207,72–222,95 depois); seu ganho de parede é modesto nesta rodada. A queda de aproximadamente 20,6% da fase de layout e a contagem corroboram a remoção do trabalho. Numbered_grid320 teve faixas separadas (228,85–234,64; 218,73–223,42) e aproximadamente 25,3% de redução de layout. Não somar medianas de fases para explicar o tempo total.

Os outros controles tiveram pequenas variações e sobreposição; inclusive eval30000 com +2,20% e show_grid320 com +0,83%. Não demonstram ganho geral ou regressão causal isolada. RSS mediano de grid320 foi 42,32 → 42,51 MiB e do numerado 42,46 → 42,66 MiB; a pool retém temporariamente produtos, mas estes números não são um perfil causal de alocações. Não houve nova comparação de desempenho com vanilla, logo não afirmamos que Tekt o superou.

## Verificação

Núcleo: 6.515 testes passaram, cinco ignorados. Infraestrutura: 1.148 passaram. Total: 7.663 testes aprovados. Release build concluído. A suíte cobre reuso Auto, fallback Fixed/alinhamento/posição explícita, invalidação de origem e consumo único, e mudança de tracking efetivo na chain sem alteração do estilo pai. A contagem do teste de alinhamento inclui dois subframes aninhados adicionais; seu valor esperado foi corrigido após inspeção do caminho existente.

Textos, queries, HTML, returncodes e diagnósticos baseline/candidato preservados nas sondas, incluindo contexto/total de páginas, rodapés, camadas, numbering callback, outlines, colunas, include/import e identidades capturadas. Bundle PDF/HTML preservado. Controles com vanilla pinado registram os limites históricos; não são declaração de paridade total.

Os 34 controles PDF compiláveis mantiveram caixas de palavras, dimensões de páginas e rasters a 36 dpi exatamente iguais. Incluem multipágina, inset/wrapper, RTL e casos que exigem fallback (alinhamento, posição, span, estilo, heading/contexto e palavra longa). Plain_multi_table foi renderizada a 144 dpi e inspecionada visualmente: texto e bordas preservados. Erros foram comparados por diagnósticos/returncodes, não tratados como PDFs equivalentes.

Lint global sem erros. Permanecem avisos herdados: V5=171, V16=315, V17=22, V12=19, V18=6; e notas V20=858, V19=693, V16=18. Não é zero violations global. Nenhum V5/V6/V15/V26 nos 62 consumers do conjunto pendente. Gate V15/V26 e git diff --check limpos. O V2 inicial do novo helper foi resolvido com testes locais das guardas; não se ressellaram owners externos para esconder dívida.

## Proveniência e reprodução

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, incluindo etapas anteriores. before-state, red-state, bench-paired e execution guardam lista exata de arquivos alterados, diff stat e hashes das fontes/L0. Cada recibo registra comandos, fixtures e horários do ensaio correspondente.

- Baseline release SHA-256: `80c6187d45cd5186f0e4a376fd6b629ea3367e1b5efbc5bc5b049265c800dd0d`.
- Candidato release SHA-256: `52a6f2ef049a7fd84ac7afd7b7a02c2977eb447cde94e5b2993d5de4afe5a667`.
- Benchmark UTC: `2026-10-05T17:20:01.071936+00:00` a `2026-10-05T17:20:51.861732+00:00`.
- Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`; binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Após os ensaios foram acrescentados/corrigidos testes cfg(test) do helper e esclarecida a proveniência da largura no L0, com resselo dos hashes. O algoritmo produtivo permaneceu igual. A suíte core completa foi repetida. O release final foi reconstruído e tem SHA-256 idêntico ao candidato medido; a infraestrutura não foi repetida após esses ajustes exclusivamente de teste. Os hashes de benchmark e de execução final preservam essa diferença de fontes, sem apresentá-los como o mesmo snapshot textual.

Recibos JSON gzip e harnesses em [frames-celulas-2026-10-05](frames-celulas-2026-10-05/): before-state, red-state, bench-paired, count, geometry, verification, bundle execution e width-probe. A sonda width-probe registra as larguras de Alpha/Beta no algoritmo final com snapshot simultâneo completo: 33/33 e 26,4/26,400000000000006 pt, respectivamente. A instrumentação temporária cfg(test) foi restaurada byte a byte, seguida da suíte core e build finais. O log exploratório discovery pertence a uma versão intermediária da guarda, não é medição do algoritmo final nem fundamenta ganho de tempo.

## Próximas frentes

O reaproveitamento é real, mas ainda estreito. A próxima ampliação deve medir quais carriers/guardas explicam misses de células com show/Styled e texto composto, preservando identidade e efeitos da região. Outra frente independente é inicialização/fontes: seu custo limita a CLI mesmo quando o layout cai. Ampliar a pool sem prova, ou usar igualdade visual como chave geral, repetiria o problema histórico.

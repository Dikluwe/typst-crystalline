# Custo fixo da CLI e compressão de cobertura Unicode

A primeira aquisição do catálogo de fontes dominou o documento de texto simples no estado atual. Uma alteração interna em Coverage::from_codepoints reduziu a mediana da CLI de **173,61 para 150,34 ms (13,41%)**. A tabela de 320 células caiu de **203,95 para 181,37 ms (11,07%)**. O vanilla pinado continuou mais rápido: 126,09 ms no documento simples e 146,63 ms nessa tabela. Nenhum unsafe, contrato público, política de aquisição ou fase do pipeline foi alterado.

## Medir antes de decidir

O [estudo anterior](infra-performance-followup.md) apontava aquisição do catálogo como dominante, com dispersão importante e demanda até em shapes. Ele pertence a outro snapshot. Nesta etapa revalidamos o código atual, sem transplantar suas porcentagens: **empty, shapes e image não adquiriram fontes**, e a CLI levou aproximadamente 7–10 ms. Não foi isolado nesta etapa qual alteração histórica explica a diferença desse caminho em relação ao estudo de setembro.

O diagnóstico inicial utilizou o CLI anterior intacto, o vanilla ratificado e uma sonda independente ligada às APIs públicas atuais. O decorator World mede a primeira e as demais chamadas de book, font e candidates_for_char. A sonda de catálogo reproduz a composição da função privada de descoberta e confere slots, índices, metadados e cobertura contra o loader público real. Isso evita adicionar APIs produtivas de benchmark. A composição diagnóstica tem instrumentação/alocações adicionais e não é uma instrumentação interna do CLI.

Primeira rodada de auditoria, antes de decidir o algoritmo:

| Controle | CLI Tekt, ms | Pipeline fresco da sonda, ms | Primeira chamada book, ms | Acessos book |
| --- | ---: | ---: | ---: | ---: |
| empty | 7,41 | 2,03 | 0,00 | 0 |
| shapes | 9,79 | 3,96 | 0,00 | 0 |
| hello | 172,77 | 164,01 | 160,80 | 56 |
| grid320 | 202,66 | 196,37 | 162,86 | 12.495 |
| unicode | 217,80 | 205,83 | 160,33 | 360 |

Medianas de populações diferentes: não subtrair uma coluna da outra para atribuir custo. O setup de SystemWorld, incluindo primeira leitura/parsing da source e instalação de fontes/host, ficou aproximadamente em 0,1–0,4 ms na sonda; a escrita do PDF ficou próxima de 0,1–0,15 ms. Startup do processo, parser da CLI, outros trabalhos do coordenador e teardown permanecem no residual da medição de parede; esse residual não foi atribuído todo à inicialização. A sonda possui outro executável/grafo de ligação e não substitui o timer completo da CLI.

A sonda pré-carregada fez hello em aproximadamente 2,43 ms e grid320 em 32,76 ms. **Pré-carregar apenas desloca o custo para fora do intervalo**: não é ganho produtivo, nem prova de reuso entre processos. O catálogo OnceLock é adquirido uma vez por World e depois retido.

Contagem independente no release anterior, com GDB e sem fornecer tempos de parede:

| Controle | Catálogo | Descoberta de sistema | Fontes embutidas | Metadados | Extração de cobertura |
| --- | ---: | ---: | ---: | ---: | ---: |
| empty | 0 | 0 | 0 | 0 | 0 |
| shapes | 0 | 0 | 0 | 0 | 0 |
| hello | 1 | 1 | 1 | 1.129 | 1.112 |
| unicode | 1 | 1 | 1 | 1.129 | 1.129 |

Assim, os milhares de acessos book não são milhares de reconstruções. A cobertura adicional no controle Unicode corresponde também ao caminho lazy das entradas sem cobertura pré-calculada; não foi classificada como reconstrução do catálogo. Os recibos guardam contagens de chamadas e custos de fontes individuais no decorator; não chamamos toda consulta font de cache miss.

## Decomposição diagnóstica do catálogo

1112 faces de sistema, 1086 arquivos distintos e 17 faces embutidas neste ambiente. O ambiente não define a quantidade universal de fontes de um usuário. A enumeração entregou **28.689.553 ocorrências de codepoints**, incluindo sobreposição de subtables.

| Intervalo | Mediana, ms |
| --- | ---: |
| Catálogo diagnóstico total | 163,15 |
| Fontes embutidas | 7,46 |
| Database::load_system_fonts | 36,58 |
| Processamento das faces, inclusive callback/composição diagnóstica | 119,24 |
| Drop da Database | 0,10 |
| Metadados, soma por face, dentro do processamento | 18,69 |
| Segundo parse da face, soma por face | 0,45 |
| Enumeração cmap, soma por face | 29,67 |
| Coverage::from_codepoints, soma por face | 46,23 |

Não somar as linhas aninhadas novamente ao total. As medianas tampouco precisam fechar aritmeticamente. O residual do processamento inclui callbacks, mapeamento, inserção e instrumentação diagnóstica; não foi convertido numa afirmação de I/O puro. O parse repetido existe em fontdb.rs, mas seu custo medido é pequeno nesta população, por isso não foi a primeira correção.

A compressão foi escolhida por evidência. No código anterior, cada ponto consultava/atualizava intervals.last_mut(). Um experimento isolado manteve o intervalo ativo em dois escalares locais e só o inseriu no vetor quando terminou. Três processos com nove lotes cada, mantendo clones de entrada e drop final fora do timer, deram medianas **44,66 → 24,53 ms**. A codificação canônica foi igual para todas as 1.112 faces em todos os lotes. Esse controle é mecânico do constructor, não uma prova completa de paridade da linguagem, nem uma decomposição transplantável da CLI.

## Materialização

Owner exclusivo: `00_nucleo/prompts/entities/font_book.md`, atualizado antes de código. Consumer: `01_core/src/entities/font_book.rs`. Fluxo contínuo ADR-0127: ajuste de RAM/algoritmo privado, com as assinaturas e a representação pública preservadas. A implementação continua pura no L1.

O intervalo ativo conserva start/end locais. Adjacência crescente é testada primeiro, seguida por pertencimento e adjacência decrescente. Somente um ponto separado fecha o intervalo anterior; o fim fecha o último. Sorting, merge de intervalos e codificação gap/count continuam. Entradas vazias, desordenadas, repetidas, ranges invertidos, sobreposição, zero e máximo Unicode preservam a cobertura exata. O vetor intermediário de codepoints do L3 não foi removido. A aquisição eager/lazy, a ordem do catálogo, a escolha de fontes e os caches existentes não foram alterados.

As sete provas de cobertura existentes foram executadas antes da implementação e depois; já passavam no baseline. Não houve defeito semântico RED inventado, nem threshold de relógio introduzido nos testes ordinários. O experimento isolado precedeu a decisão e o ganho produtivo foi decidido pelo ensaio pareado, com controles de saída. Isso distingue validação funcional de uma hipótese de custo.

## CLI completa antes/depois/vanilla

Três rodadas; um aquecimento por fixture/motor/rodada e cinco amostras medidas por rodada, totalizando 15 por população. Motores intercalados com rotação de ordem, fixtures invertidas na segunda rodada, CPU fixa, Rayon um, epoch zero, PDF sem tags e pretty=false. Fontes de sistema habilitadas, sem paths extras. OS page cache aquecido; nenhum cold-disk claim. Sem builds, testes, sondas ou GDB concorrentes nos tempos. Hardware não reservado; não há teste estatístico formal.

| Fixture | Antes, ms | Depois, ms | Variação Tekt | Vanilla, ms |
| --- | ---: | ---: | ---: | ---: |
| hello | 173,61 | 150,34 | -13,41% | 126,09 |
| context | 175,08 | 151,82 | -13,28% | 127,55 |
| grid320 | 203,95 | 181,37 | -11,07% | 146,63 |
| eval30000 | 25,56 | 25,93 | 1,46% | 17,16 |
| empty | 7,50 | 7,67 | 2,34% | 8,81 |
| shapes | 9,78 | 9,84 | 0,55% | 10,58 |
| image | 8,03 | 8,11 | 0,95% | 9,79 |
| unicode | 219,40 | 197,04 | -10,19% | 160,56 |
| default_hello | 173,75 | 150,84 | -13,18% | 126,67 |
| show_grid320 | 223,92 | 200,45 | -10,48% | 151,93 |
| numbered_grid320 | 214,66 | 190,55 | -11,23% | 147,81 |

As faixas de hello ficaram separadas: 170,96–179,34 ms antes e 149,05–154,82 ms depois. Para grid320: 201,48–206,38 e 178,19–186,09 ms. As três medianas de rodada de hello foram 172,97/173,65/175,13 antes e 149,79/150,57/150,78 depois. O ganho não depende de selecionar somente uma rodada favorável.

No hello, a diferença para o vanilla nesta mesma comparação caiu de 47,53 para 24,25 ms, aproximadamente metade. Isso não significa superar o vanilla. Os tempos vanilla das fixtures com divergências de saída são referências exploratórias, não provas de ganho sob paridade já estabelecida.

Empty, shapes, image e eval30000 conservaram variações pequenas, com faixas sobrepostas. Não há ganho geral em inicialização ou documentos que não adquirem fontes. Também não afirmamos regressão causal nesses controles. RSS mediano de hello ficou praticamente igual: 41,46 → 41,47 MiB; não foi feito perfil causal de alocações.

## Em qual fase o ganho aparece

| Controle/fase | Antes, ms | Depois, ms |
| --- | ---: | ---: |
| hello, eval | 163,95 | 140,76 |
| hello, layout | 0,49 | 0,49 |
| default_hello, eval | 1,74 | 1,75 |
| default_hello, layout | 162,57 | 139,63 |
| grid320, eval | 165,07 | 142,78 |
| grid320, layout | 18,62 | 18,57 |
| grid320, shaping | 9,00 | 9,09 |
| grid320, subset | 0,11 | 0,12 |
| grid320, render PDF sem subset | 1,52 | 1,55 |

O catálogo lazy entra no envelope da fase que o demanda. `set text(font: ...)` pode adquirir o catálogo na avaliação; o texto padrão o adquire no layout. Esses dois controles refutam atribuir toda a antiga diferença de eval ao algoritmo de avaliação, ou toda a diferença de layout à paginação. Não somar essas medianas para fechar a mediana da parede. Render exporta recursos/builder/serialização completos; não é timer isolado do stream. O subset é contabilizado separadamente por Timings.

## Verificação e limites de paridade

Core: **6.515 passaram, cinco ignorados**. Infraestrutura: **1.148 passaram**. Total aprovado: **7.663**. Build release concluído. Cobertura exata/ordenação continuaram verificadas por oráculo independente de conjunto, além dos controles de seleção/fallback, parsing, exportação e pipeline.

Nos 11 PDFs pareados, baseline e candidato mantiveram texto extraído, caixas de palavras, dimensões de página, rasters a 36 dpi e inventário de fontes PDF exatamente iguais. O inventário CLI `fonts --variants` também permaneceu idêntico. O controle Unicode foi renderizado a 144 dpi e inspecionado; conservou a saída do baseline, sem declaração de tipografia multilíngue perfeita.

A comparação com vanilla registrou limites **já presentes no baseline desta etapa**: context devolve contador de página 0 no Tekt e 1 no vanilla; Unicode tem separação extra na extração devanágari; show_grid320 segmenta/ordena o texto extraído diferentemente. Grid320 e numbered_grid320 diferem na disposição/whitespace do texto extraído, mesmo preservando sua sequência normalizada. Esses casos ficam abertos e não foram corrigidos silenciosamente numa etapa de desempenho. Os recibos conservam as saídas dos três motores; baseline=candidato não é prova de paridade total com vanilla.

Lint global sem erros, mas não zero violations: avisos V5=171, V16=315, V17=22, V12=19, V18=6; notas V20=858, V19=694, V16=18. Nenhum V5/V6/V15/V26 nos 63 consumers do conjunto pendente. Gate V15/V26 e git diff --check limpos. Owners externos não foram ressellados para esconder dívida.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado, incluindo as etapas anteriores. before-state preserva bytes/L0 anteriores; audit e paired registram status, diff stat, fontes/fixtures/hashes e timestamps; execution preserva o estado final e logs. audit-helper-source preserva a versão inicial da sonda; compression preserva a versão seguinte e seus comandos/fontes. Os perfis não usam binários debug.

- Baseline CLI SHA-256: `52a6f2ef049a7fd84ac7afd7b7a02c2977eb447cde94e5b2993d5de4afe5a667`.
- Candidato CLI SHA-256: `f8fc2599bc599b39678575df1eea3fd71b807f1ffd257abbb0ce3413b2344915`.
- Vanilla pin: `9f2b6e8715237cb086899a42873660fe744622e8`; binário ratificado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
- Auditoria inicial UTC: `2026-10-05T18:04:48.480467+00:00` a `2026-10-05T18:05:23.416820+00:00`.
- Comparação produtiva UTC: `2026-10-05T18:12:36.015644+00:00` a `2026-10-05T18:13:44.659935+00:00`.

Receitas e recibos em [custo-fixo-2026-10-05](custo-fixo-2026-10-05/): before-state, audit, audit-helper-source, count, compression, paired, verification e execution, JSON gzip. Incluem fontes de fixtures/helpers, manifestações/lock da sonda, binários identificados por SHA, hashes dos arquivos de fonte de sistema, amostras individuais, clocks, comandos e logs. Os executáveis locais de apoio ficam em `/tmp/tekt-fixed-cost-2026-10-05`; os harnesses/recibos permanentes permitem reconstruí-los com as dependências e arquivos identificados. Não houve commit/PR nesta etapa.

## Frentes seguintes

Ainda há custo na enumeração cmap e no vetor intermediário do L3, na extração de metadados e na aquisição de arquivos. Eliminá-los exige nova medição/equivalência; uma nova API de construção de ranges exigiria o gate de contrato público. O segundo parse observado não é a maior parcela. Não voltar a pré-carregar tudo e chamar deslocamento de custo de ganho.

A cobertura conservou sua semântica. As diferenças explícitas de linguagem/saída nos controles context/show/Unicode merecem uma frente própria de paridade; os 24 ms restantes do hello frente ao vanilla não justificam ignorá-las.

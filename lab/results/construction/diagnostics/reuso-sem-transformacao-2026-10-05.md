# Reuso sem transformação e âncoras já preparadas

Implementadas três frentes internas: preservar o conteúdo quando a realização de outlines não transforma nada; evitar a substituição de contextos sem produtores; registrar diretamente as identidades já preparadas dos originais consumidos por `show`. O PDF de controle passou de cinco reconstruções do índice puro para uma. Na fixture de 500 headings com show, o layout caiu aproximadamente 25%, mas a CLI inteira melhorou apenas 2,45% nesta rodada. A tabela simples não demonstra ganho convincente.

## Evidência antes da implementação

O backtrace do baseline registrou cinco entradas em `introspect_with_introspector`: uma na realização inicial, duas na estabilização de outlines e duas no layout dos originais de Realized. A fonte da realização de outlines reconstruía o carrier mesmo sem transformação; a sessão contextual mapeava a árvore mesmo sem roots. Essas evidências motivaram o reuso. Não provavam antecipadamente seu ganho.

Os dois testes de preservação de carrier falharam antes da implementação, assim como o teste que exige registrar uma origem preparada sem reconstruir seu índice. Os logs RED e o estado exato estão nos recibos; os testes completos finais servem como GREEN. Preservar Arc aqui comprova ausência de transformação da mesma árvore imutável; não define identidade de linguagem nem substitui Location.

## Implementação e limites

- `01_core/src/compiler/eval/rules/outline.rs:20`: marca transformação somente ao realizar um outline pertinente e devolve o conteúdo original quando nada mudou. Ainda percorre e constrói um resultado temporário pelo mapper existente.
- `03_infra/src/pipeline/paged.rs:95`: os carriers Sequence/Styled/Located/Realized idênticos permitem conservar o índice da tentativa e evitar a reconstrução de prefixos. Outlines realmente transformados conservam a estabilização anterior. Outros carriers seguem o caminho conservador.
- `03_infra/src/pipeline/context_stabilization.rs:286`: sem produtores descobertos, retorna o conteúdo original. A descoberta continua a acontecer; contextos efetivos seguem a substituição existente.
- `01_core/src/compiler/layout/realized.rs`: unidade estática descendente do layout, delegada pelo hub. Coleta Locations explícitas somente quando todo locatable está preparado. Origens incompletas, erro de travessia e tipos com síntese/conversão/exclusão conservam a reconstrução anterior. Inclui descendentes e origens aninhadas; preserva âncoras existentes e observa IDs no Locator sem gerar novos. O mapper ainda aloca: eliminamos os stores redundantes, não toda travessia/alocação.

L0s proprietários foram atualizados antes do código; a unidade nova tem owner exclusivo. São alterações internas na mesma fase, sem contrato público novo, em fluxo contínuo ADR-0127. Nenhum unsafe ou cache global foi introduzido. Não foi removida a segunda passagem de layout do documento nem a introspecção de runtime.

## Contagem estrutural

GDB no próprio processo filho, sem probes produtivas e fora dos ensaios de tempo; fixture `#show heading: it => upper(it.body)` seguida de `= Original`.

| Target | Baseline | Candidato |
| --- | ---: | ---: |
| PDF | 5 | 1 |
| HTML | 1 | 1 |
| Query | 1 | 1 |

Conta somente entradas na função pura `introspect_with_introspector`, não toda introspecção do compilador. O backtrace final mostra a única entrada desse controle em `realize_document`. A contagem não extrapola para outlines efetivos ou origens que exigem fallback.

## Tempos da CLI e memória

Uma rodada de aquecimento e nove amostras por fixture e motor, intercaladas, afinidade em uma CPU e RAYON_NUM_THREADS=1. Compilação inclui fontes, avaliação, realização, layout e exportação; usa `--pdf-tagged=false --pretty=false --timings-json`. Não houve build, suítes ou GDB concorrentes durante o benchmark.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS antes → depois, MiB |
| --- | ---: | ---: | ---: | ---: |
| hello | 177.96 | 176.99 | -0.54% | 41.29 → 41.31 |
| paragraphs | 270.74 | 271.74 | +0.37% | 52.83 → 52.89 |
| grid320 | 230.87 | 228.62 | -0.98% | 42.55 → 42.55 |
| eval30000 | 26.62 | 26.63 | +0.03% | 20.76 → 20.95 |
| show_headings500 | 258.76 | 252.41 | -2.45% | 56.60 → 53.32 |
| show_eval30000 | 196.83 | 198.13 | +0.66% | 41.14 → 41.27 |
| show_grid320 | 262.06 | 257.55 | -1.72% | 42.96 → 43.31 |

Medianas, sem teste estatístico formal. As faixas da CLI se sobrepõem: headings500 foi 257,57–262,86 ms antes e 246,92–263,92 depois; grid320 foi 227,34–234,44 e 226,12–233,13 ms. Isso não sustenta uma promessa de ganho geral. Headings500 reduziu o pico RSS mediano em 3,28 MiB, compatível com evitar stores repetidos; não é um perfil isolado de alocações.

A instrumentação existente reporta estas medianas da etapa de layout:

| Fixture | Antes, ms | Depois, ms | Variação |
| --- | ---: | ---: | ---: |
| grid320 | 40.70 | 39.11 | -3.91% |
| show_headings500 | 40.91 | 30.54 | -25.36% |
| show_grid320 | 61.88 | 58.95 | -4.73% |

Não somar medianas das etapas para explicar a CLI: as amostras, envelopes e custos de inicialização diferem. As mudanças foram medidas como conjunto; ainda não isolamos a contribuição individual de cada uma. O grande custo restante de fontes/inicialização limita o ganho total.

## Verificação de linguagem e geometria

6.511 testes do núcleo passaram, cinco ignorados; 1.145 da infraestrutura passaram. Build release concluído. Os controles CLI mantiveram os resultados do baseline em aliases, originais e produtos gerados, labels, contextos, numeração, colunas, include/import, outlines transformados, prefixos e contexto dentro de entries, HTML e bundle misto. Os três novos controles de outline também coincidiram com o vanilla pinado no texto extraído. Não é declaração de paridade completa: `show table.cell` continua sem suporte e outros desvios históricos permanecem registrados nos recibos.

As sete fixtures pareadas e doze controles PDF compiláveis mantiveram dimensões de páginas, caixas de palavras e rasterização a 36 dpi iguais entre baseline/candidato. A tabela com show foi renderizada a 144 dpi e inspecionada em recorte legível. Isso valida esses controles, não todos os documentos possíveis.

## Auditoria da próxima frente: frames das células

`01_core/src/compiler/layout/grid/tracks.rs:119` mede a linha Auto com `layout_sub_frame` e descarta itens/decoração. Agrupa a entrada em chunks e usa largura e origem da coluna de entrada. `01_core/src/compiler/layout/grid.rs:218–239` resolve essas alturas antes de executar placement. Colspan, rowspan e coordenadas explícitas podem mudar a geometria e a ordem efetivas.

Na emissão, `grid/cell_body.rs:90–166` configura a região final da célula, a altura disponível, o parent_bbox, as origens e o alinhamento efetivo; só então diagrama novamente. `sub_frame.rs:106–146,314–346` restaura numerosos campos de estado, mas devolve itens, segmentos de decoração, alinhamentos órfãos e atualiza métricas laterais. Não encapsula todo o runtime como um produto puro. Identidades/posições e efeitos contextuais atravessam o layout e precisam ser auditados antes de um cache de frame.

Os comentários históricos P234 dizem que o antigo cache foi removido por reordenação de placed_cells e aceitam o custo duplicado no MVP. São pistas da origem da dívida, não justificativa atual para manter ou restaurar aquele desenho.

**Ainda não implementado:** reaproveitar indiscriminadamente um frame pelo índice da célula ou por igualdade visual. Uma próxima experiência precisa provar equivalência de largura, região, estilo, alinhamento e contexto, além de decidir como conservar/reproduzir efeitos e reposicionar âncoras/decorações. Começar por um subconjunto demonstravelmente puro pode reduzir o risco, mas exige medição e especificação antes de código. A segunda passagem completa do documento também continua uma frente aberta.

## Proveniência e validação arquitetural

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado; inclui as alterações anteriores de avaliação única e memoização do índice. Lista exata de alterações, diff stat, fontes e hashes estão nos recibos. O baseline já contém essas melhorias anteriores.

- Baseline SHA-256: `70bb22e90f7b0fa1a660de1c8f0df0a89f8ebb6ddd82a77fd4794f753ebb07f5`.
- Candidato SHA-256: `a97804a4dac8a9f6780010a0d46da8d1f61ec25a0c72e9054301ea3edab098c0`.
- Benchmark UTC: `2026-10-05T16:35:29.026951+00:00` até `2026-10-05T16:36:00.258366+00:00`.

- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Linter global sem erros, com 172 avisos V5 herdados fora dos owners tocados, além de V12/V16/V17/V18 e notas V19/V20. A nova unidade acrescenta avisos V16/V17 sobre classificação conservadora e braços neutros. Não alegamos zero violations globais. Nenhum V5/V6/V15/V26 nos 56 consumers do conjunto pendente; gate dedicado V15/V26 limpo. Diff check limpo.

Recibos e receitas ficam em [reuso-sem-transformacao-2026-10-05](reuso-sem-transformacao-2026-10-05/): `bench-paired.json.gz`, `count.json.gz`, `geometry.json.gz`, `verification.json.gz`, `bundle.json.gz`, `red-state.json.gz` e `execution.json.gz`, acompanhados dos harnesses. Execution preserva RED, suites, build, lint e backtraces; o benchmark preserva fontes exatas, hashes, horários, comando e todas as amostras.

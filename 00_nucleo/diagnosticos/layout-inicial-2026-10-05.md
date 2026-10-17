# Reuso do primeiro layout independente de páginas

A tabela simples de 320 células passou de duas chamadas completas de layout para uma. A CLI inteira caiu de 230,40 para 216,28 ms (6,13%) nesta rodada; a tabela com show caiu de 261,84 para 233,61 ms (10,78%). A implementação mantém a construção do PageStore e só reutiliza um candidato cuja independência foi comprovada por um conjunto estreito de carriers. Documentos com numeração ou contexto conservam o ciclo completo.

## Evidência anterior à decisão

`03_infra/src/pipeline/paged.rs` repetia expansão contextual, introspecção runtime, realização de outlines e layout após o primeiro candidato, mesmo para conteúdo simples sem dependência de páginas. O [estudo anterior](grid-pagination-cost-2026-10-05.md) quantificou aproximadamente 15 ms de CPU na segunda passagem de 320 células naquele snapshot; esse número histórico motivou a investigação e não foi tratado como ganho causal atual.

O baseline desta etapa, preservado antes de editar L0/código, já inclui avaliação única, índice no Module e as três melhorias de reuso sem transformação. O teste RED de tabela multipágina compilou e validou conteúdo/números lógicos, mas falhou na contagem: duas entradas no wrapper, quando exigíamos uma. O log e as fontes da sonda estão nos recibos. As fontes RED foram reconstruídas a partir do snapshot preservado e do patch isolado da sonda; isso está explicitado no recibo, sem apresentá-las como uma captura simultânea inexistente.

A qualificação inicial era estreita demais: os carriers reais transportam geometria em customs de Styled e normalizam fontes para Array. A inspeção de `eval/rules/set/page.rs`, `eval/markup_stream.rs:218` e `eval/rules/set/text.rs:269` precedeu a ampliação explícita dessa lista no L0. Não generalizamos arbitrary Value/Func como independente.

## Implementação

O owner L0 é `00_nucleo/prompts/infra/pipeline/paged.md`, atualizado antes do código. A suíte tem owner próprio `00_nucleo/prompts/infra/pipeline/tests.md`. Fluxo contínuo ADR-0127: reuso interno na mesma fase, sem API externa nova, flag, semântica de produto ou incompatibilidade. Nenhum unsafe, cache global ou estado produtivo adicional.

`initial_layout_independent` usa travessia emprestada e prova positiva, cobrindo conteúdo contextual original e conteúdo final realizado. Aceita texto e folhas simples, sequências/parágrafos, strong/emph, Located/Realized (original e visual), tabelas/grids/células e todos os seus corpos. Styled aceita somente fontes literais, tamanho, linebreaks simples, as seis propriedades físicas de dimensões/margens e o marcador de escopo de fluxo; outras chaves/valores invalidam a prova. SetPage rejeita numeração, supplement explícito, marginais de conteúdo e camadas visuais. Carriers desconhecidos continuam pelo caminho anterior.

Também exige numbering ausente em todas as páginas do primeiro documento. ContextBlock, produtores/marcadores, heading, outlines, referências, counters/state, matemática/callbacks, bibliografia, PageRun e receitas show residuais ficam fora do atalho. O primeiro candidato já completou o wrapper de callbacks antes da prova, mas carriers de callbacks não são aceitos. prepared.settled e bundles permanecem nos coordenadores existentes.

No ciclo vigente, o coordenador ainda realiza PageStore, conserva warnings e injeta páginas/posições. Somente após isso encerra o ciclo quando a prova permite. Binding lógico, opções do documento, bidi, shaping, reconciliação e exportação permanecem. Não usamos igualdade visual nem quantidade de páginas como critério.

Esta etapa reutiliza um resultado completo; não adiciona comemo nem cache persistente de frames. É um passo necessário para reduzir trabalho redundante antes de memoizar unidades menores.

## Contagem no binário de produção

GDB conta entradas no wrapper paginado, localizando seu símbolo no próprio binário por nm. Nenhuma sonda cfg(test) entra no release. A contagem ocorreu depois do benchmark e não forneceu seus tempos.

| Fixture | Baseline | Candidato |
| --- | ---: | ---: |
| grid320 | 2 | 1 |
| show_grid320 | 2 | 1 |
| multi_table | 2 | 1 |
| numbered_grid320 | 2 | 2 |
| show_headings500 | 2 | 2 |
| footer_context | 2 | 2 |

A fixture multi_table produziu sete páginas em ambos os motores. Isso refuta a hipótese de que o atalho só funciona por haver uma página. O wrapper pode executar tentativas internas de callbacks em outros conteúdos; esta contagem não equivale ao número universal de chamadas internas ao layouter.

## Desempenho

Uma rodada de aquecimento e nove amostras por fixture/motor, intercaladas na mesma CPU; RAYON_NUM_THREADS=1, SOURCE_DATE_EPOCH=0, PDF sem tags e pretty=false. Inclui fontes, avaliação, layout e geração/escrita do PDF. Não houve builds, testes ou GDB concorrentes no benchmark.

| Fixture | Antes, ms | Depois, ms | Variação | Pico RSS antes → depois, MiB |
| --- | ---: | ---: | ---: | ---: |
| hello | 179.99 | 179.07 | -0.52% | 41.46 → 41.34 |
| paragraphs | 271.18 | 246.08 | -9.26% | 52.89 → 51.79 |
| grid320 | 230.40 | 216.28 | -6.13% | 42.50 → 42.33 |
| eval30000 | 27.23 | 26.74 | -1.82% | 20.91 → 20.96 |
| show_headings500 | 250.08 | 249.87 | -0.09% | 53.26 → 53.33 |
| show_eval30000 | 197.36 | 197.78 | +0.21% | 41.28 → 41.14 |
| show_grid320 | 261.84 | 233.61 | -10.78% | 43.23 → 42.64 |
| multi_table | 194.94 | 190.67 | -2.19% | 42.21 → 41.99 |
| numbered_grid320 | 238.62 | 239.88 | +0.53% | 42.68 → 42.46 |

Medianas, sem teste estatístico formal. Para grid320, as faixas de parede foram 228,41–240,50 ms antes e 214,20–218,06 depois, sem sobreposição nesta rodada. Paragraphs também teve faixas separadas: 265,67–280,55 e 242,45–250,31 ms. Show_grid320 teve pequena sobreposição: 255,38–269,45 e 231,43–255,83 ms. Esses ganhos pertencem às fixtures qualificadas; não extrapolam para documentos complexos.

Hello, eval30000, headings com show, show_eval30000 e controle com numbering apresentaram variação pequena e faixas sobrepostas; não demonstram ganho geral. O controle multipágina também teve ruído grande na CLI, apesar da redução estrutural comprovada.

A instrumentação existente reportou estas medianas de layout:

| Fixture | Antes, ms | Depois, ms | Variação |
| --- | ---: | ---: | ---: |
| paragraphs | 52.48 | 28.88 | -44.97% |
| grid320 | 39.05 | 24.26 | -37.88% |
| show_grid320 | 58.57 | 33.67 | -42.51% |
| numbered_grid320 | 39.60 | 41.54 | +4.91% |

A redução de layout de grid320 foi 14,79 ms nesta rodada. Não somar medianas das fases para explicar a CLI, nem comparar esse ganho causal diretamente com CPU medida sob GDB em outro snapshot. O vanilla pinado foi utilizado para controles de linguagem nesta etapa, sem novo ensaio de desempenho vanilla; não afirmamos que o Tekt já o alcançou.

## Verificação

1.148 testes completos da infraestrutura passaram. O núcleo não foi alterado nesta etapa; sua suíte havia passado com 6.511 testes e cinco ignorados na etapa anterior, não foi repetida nem contada como uma execução atual. Build release concluído. Testes específicos cobrem tabela multipágina com fontes literais, vinculação lógica, segundo layout com numbering e rejeição de conteúdo escondido/estilos não qualificados/origem Realized com heading.

As sondas CLI mantiveram texto, queries, HTML e diagnósticos do baseline, inclusive contextos de total de páginas, rodapé contextual, camadas com contexto, erro de callback de numeração, outlines, prefixos, referências, colunas, aliases e include/import. Bundle misto preservado. Controles relevantes foram comparados com vanilla pinado, sem declaração de paridade total: show table.cell e outros limites históricos continuam registrados.

As nove fixtures pareadas e quinze controles PDF compiláveis mantiveram caixas de palavras, dimensões e rasters a 36 dpi iguais (24 controles). A tabela multipágina foi renderizada a 144 dpi e inspecionada. Erros não são considerados PDFs equivalentes: seus returncodes e diagnósticos foram comparados separadamente.

Linter global sem erros, com avisos herdados e avisos conservadores de V16 no predicado; não está globalmente em zero violations. Nenhum V5/V6/V15/V26 nos 56 consumers do conjunto pendente. Gate V15/V26 limpo e git diff --check limpo. Não ressellamos owners externos para ocultar dívida.

## Frentes abertas

Medição de altura Auto e emissão da célula ainda constroem seu corpo separadamente no layout restante. O [diagnóstico anterior](reuso-sem-transformacao-2026-10-05.md) documenta diferenças de região/alinhamento e efeitos que impedem um cache indiscriminado de frames. Não contamos novamente essas chamadas nesta etapa; a eliminação de um layout inteiro reduz sua repetição por inferência do fluxo, não por uma nova contagem de células.

Ampliar a prova para headings, mais estilos, page runs ou contextos exige demonstrar suas dependências; não basta adicionar variantes à lista. A frente de inicialização/fontes continua relevante, pois limita o ganho da CLI mesmo quando o layout cai bastante.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-state preserva fontes, lista dirty e diff stat anteriores; benchmark e execution registram a lista exata final e hashes de código/L0. A árvore inclui todo o trabalho anterior ainda não commitado.

- Baseline SHA-256 `a97804a4dac8a9f6780010a0d46da8d1f61ec25a0c72e9054301ea3edab098c0`.
- Candidato SHA-256 `80c6187d45cd5186f0e4a376fd6b629ea3367e1b5efbc5bc5b049265c800dd0d`.
- Benchmark UTC: `2026-10-05T16:54:05.612957+00:00` a `2026-10-05T16:54:45.572473+00:00`.

- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.

Receitas/recibos em [layout-inicial-2026-10-05](layout-inicial-2026-10-05/): before-state, red-state, bench-paired, count, geometry, verification, bundle e execution (JSON gzip), com os harnesses Python. Preservam fontes das fixtures, comandos, amostras individuais, timestamps, hashes, RED/GREEN, testes, build e lint.

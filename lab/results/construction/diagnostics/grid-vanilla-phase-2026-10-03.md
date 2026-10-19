# Grid: fases reais no vanilla pinado, Tekt atual e triagem Bateia

**Data:** 2026-10-03. Estudo diagnóstico; nenhum L0 ou código produtivo foi
alterado. O oráculo é o vanilla pinado
`9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O Tekt foi recompilado em release (`cargo build --release --locked --offline
-p typst-wiring --bin typst`) no HEAD
`9914187fdac5b975da00102e556db59e974085bb`, binário SHA-256
`b0207af549b16e85001f1e2b571fd8d620d07406ccdb3a6cdd850474694ffa10`.

## Tempo bilateral atual

Às `2026-10-03 03:32:33 UTC`, árvore limpa, o
[runner pareado](grid-time-2026-10-02/run.py) fez 12 pares alternados por
fixture, com duas warmups, CPU fixo, `SOURCE_DATE_EPOCH=0`,
`RAYON_NUM_THREADS=1` e `--jobs 1` no vanilla. O tempo inclui inicialização,
fontes, compilação e escrita do PDF. Fontes e resultados integrais gerados em
`/tmp/tekt-grid-measure/results.json`; SHA-256 das fontes de 1/20/80 linhas:
`d5739642bf8c6da1967d8665bb9b45f8b0c522ca4e1a463eb59725447e63e9d2`,
`de5ce30a77ca2c3bab2c92661ed924175d1b4fb366638faef0ae35a345bf1753`,
`902986bd28ace4b39144ce567592229c5a5621453cbb661a408414c803a58d6d`.

| Linhas / células | Tekt mediana | Vanilla mediana | Delta pareado mediano |
| --- | ---: | ---: | ---: |
| 1 / 4 | 176,30 ms | 127,43 ms | +48,56 ms |
| 20 / 80 | 188,84 ms | 132,45 ms | +56,45 ms |
| 80 / 320 | 228,66 ms | 148,83 ms | +79,55 ms |

O crescimento do delta de 4 para 320 células é **30,99 ms** nesta coleta.
Texto bruto, ordem, tamanho de página e contagem de páginas coincidem, mas
larguras de coluna `Auto` não coincidem. Este delta não é atribuível inteiro
ao grid: o cronômetro envolve o pipeline completo.

## Chamadas e custo de fase

O [probe reproduzível](grid-vanilla-phase-2026-10-03/probe.py) fez cinco
compilações de cada escala e motor com trace vanilla `--timings` e timing
Tekt `--timings-json`. Às `2026-10-03 03:35:59 UTC`, o HEAD era o mesmo;
`git diff HEAD --stat` vazio e o único untracked era o próprio `probe.py`.
Resultados individuais e traces ficam em `/tmp/tekt-grid-vanilla-phase/`.

| Linhas | `layout_table` vanilla | `layout_pad` vanilla | Tempo inclusivo mediano de `layout_table` | `layout_ms` mediano Tekt |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1 | 12 = 3×4 | 1,11 ms | 1,86 ms |
| 20 | 1 | 240 = 3×80 | 10,65 ms | 14,13 ms |
| 80 | 1 | 960 = 3×320 | 23,32 ms | 43,08 ms |

Os 960 eventos `layout_pad` de 80 linhas aparecem em três grupos contíguos
de 320. Pela ordem da fonte, são **inferidos** como medição de colunas, medição
de linhas e emissão. Medianas da soma dos spans `layout_pad` nos três grupos:
4,46, 4,17 e 4,80 ms. Esses spans não abrangem todo o custo das fases.
`layout_table` e `layout_ms` usam instrumentos e escopos diferentes; **não**
subtrair seus valores para atribuir um ganho possível.

Fonte vanilla pinada: `grid/layouter.rs:967-1100` mede cada coluna `Auto`
diagramando sua célula via `layout_cell`; `:1243-1364` mede a linha `Auto`
por novo `layout_cell`; `:1467-1524` emite a linha com outra chamada. Em
`grid/mod.rs:30-66`, `layout_cell` chama `layout_fragment`, cuja implementação
interior tem `#[comemo::memoize]` em `flow/mod.rs:117-119`. A contagem de
eventos não mede hits dessa memoização; repetir uma chamada não equivale a
reexecutar todo o trabalho interno.

No Tekt, `grid/tracks.rs:11-100` mede largura com
`measure_content_constrained`; `:116-189` mede altura via `layout_sub_frame`;
`grid/cell_body.rs:139-166` emite via outro `layout_sub_frame`. O caminho comum
de `pipeline/paged.rs:318-414` executa layout inicial e sempre pelo menos uma
iteração de layout do candidato após construir PageStore. O GDB anterior
(`grid-time-2026-10-02.md`) observou 640 chamadas para medição de altura e
640 para emissão em 320 células, coerentes com duas passagens completas nessa
fixture. O trace vanilla atual mostra um único evento de `layout_table` por
compilação; isso pode incluir reúso por memoização em outra passagem, e não
prova que todo o pipeline vanilla tenha só uma iteração.

## Diferença geométrica de `Auto`

`pdftotext -bbox` da tabela de 80 linhas encontrou 320 palavras nos dois
motores. Com colunas `Auto`, 240 diferem em x por mais de 0,001 pt; o maior
desvio é 7,5 pt. `C000` inicia em x=35,00 nos dois motores; `C003` inicia em
x=133,73 no Tekt e 126,23 no vanilla. As quatro colunas acumulam 0, 2,5,
5,0 e 7,5 pt de diferença. Com a mesma fonte e colunas fixas de 100pt,
**nenhuma** das 320 palavras difere em x por mais de 0,001 pt; o maior desvio
em y nas duas variantes é 0,0002 pt. Controle: fonte fixa
`grid-phase-2026-10-02/grid80-fixed.typ`, compilada bilateralmente com as
mesmas opções de CLI.

Em `measurement.rs:70-96`, o braço de `Content::Text` soma `word_w + space_w`
para **toda** palavra, inclusive a última, ao determinar a largura da coluna.
Uma sonda de linguagem com `measure("a a").width - 2 * measure("a").width`
sob Libertinus Serif 10pt devolveu `2.5pt` em ambos os motores. **Inferência
forte:** a adição do espaço final em cada célula de uma palavra explica
exatamente os 2,5 pt extras por coluna `Auto`. A fonte vanilla usa a largura
do frame diagramado, sem essa fórmula manual. Uma ablação estreita no Tekt
com teste de geometria é o gate para converter a inferência em correção
comprovada; esta análise não presume equivalência para células complexas.

## Bateia depois da medição

Foi usado o binário Bateia SHA-256
`4d5b7ffc4152eeb4fe58a0094ab9fa47bad923eb3286e2dab48c7cef9e21ae9d`
de `/tmp/bateia-grid-cells`, construído anteriormente do HEAD Bateia
`9d00e2f58661289a12d67bb9bd40f0027dc6fe2e` com working tree não
commitado (seis arquivos modificados, 111 adições e 13 remoções no diff;
novos arquivos em `02_shell/cli/`, `03_infra/bitbucket/` e
`03_infra/research/`). Nenhuma edição foi feita nesse projeto.

Uma `IdeaSpec` sobre evitar o segundo layout sem dependências de página e
medição intrínseca de célula foi passada ao modo local e, com a autorização
prévia do dono para Sourcegraph, ao modo live sem filtro de linguagem. O modo
local retornou `INCONCLUSIVE` (run `702fb42112fd9145`): encontrou o vanilla,
mas o catálogo não tem regras para verificar as cinco capacidades.
`analyze` localizou símbolos, sem verificar comportamento. O modo live retornou
`INCONCLUSIVE` (run `46fd82b7330ad2e0`): 16 candidatos descobertos, dez
coletados, nenhum com evidência validada de layout paginado aplicável. A busca
multilinguagem devolveu majoritariamente caches genéricos; não promovemos
essas coincidências lexicais a algoritmos. Recibos gerados em
`/tmp/bateia-grid-phase-local/` e `/tmp/bateia-grid-phase-live/`; SHA-256 de
`run.json` live `a70daebd597a505266eb2ec184d833fcd0ff94fa49cde7e11565575f9ea4cfe9`
e do snapshot de descoberta
`9c1b5c04546e4c8c4b8d174acc6da869b6df195b0613f36b1e4793fa7c2795b4`.
O estado do Bateia limita a cobertura da busca, não refuta as hipóteses de
otimização. As conclusões acima derivam das fontes e medições locais.

## Decisão para o próximo protótipo

1. Corrigir/validar primeiro a largura `Auto` de célula simples: a diferença
   geométrica é observável e a causa candidata está em uma fórmula localizada.
   A aceitação deve cobrir texto de uma palavra, várias palavras, espaços,
   estilo, inset, spans e quebra de página; medir tempo junto da geometria.
2. Depois testar um *fast path* da segunda passagem paginada apenas quando a
   primeira geração provar ausência de conteúdo dependente de páginas ou de
   mudanças na realização. Preservar PageStore, warnings, callbacks, posições
   e diagnostics. Medir contagem dinâmica de layouts e A/B bilateral. Sem esse
   predicado causal, pular a passagem por haver apenas uma página seria errado.
3. Uma medição de altura só de tamanho ou cache de frame de célula fica para
   depois: o vanilla também chama layout de célula em largura, altura e emissão,
   e a chave correta de região/estilo/posição ainda não foi demonstrada.

Qualquer protótipo produtivo deve começar por auditar/atualizar o L0 owner e
seguir RED→GREEN, conforme ADR-0127 e o protocolo do repositório.

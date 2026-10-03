# Grid: medição dos protótipos e controle no compilador real

## Proveniência e método

Medição em 2026-10-03, aproximadamente 02:05–02:07 UTC, no HEAD
`fb48692805c72c98ef76220e8408ac486b126b69`. A árvore **não estava
commitada**: no momento da medição do laboratório, `git status --short` listava
`M lab/grid-prototypes/Cargo.toml` e
`?? lab/grid-prototypes/src/measure.rs`; `git diff HEAD --stat` era
`lab/grid-prototypes/Cargo.toml | 4 ++++`. O binário de medição tinha SHA-256
`434120ec509f316b22c580b3f12228cfd5b93c9c3817018ea8e8e0dee6fc8450`.
Antes da medição de produção, o README do laboratório também mudou; o estado
exato desse segundo teste está nos campos `status` e `diff_stat` dos
[resultados brutos](grid-prototypes-2026-10-03/production-results.json),
com hora `2026-10-03 02:06:55 UTC`.

O binário `grid-prototypes-measure`, compilado com `cargo build --release
--manifest-path lab/grid-prototypes/Cargo.toml --bin grid-prototypes-measure`,
foi executado em CPU 0 com `taskset -c 0`, 31 blocos e 200 execuções por modo
e bloco. Os 16 modos foram aquecidos e tiveram a ordem rotacionada por bloco.
Cada número abaixo é a mediana dos 31 tempos por execução; os arquivos CSV
guardam também percentis 10/90 e contadores. Os percentis descrevem a variação
entre blocos, não um intervalo de confiança. O custo da construção da fixture
está fora do cronômetro; `run` inteiro, inclusive suas alocações, está dentro.
Todos os 16 modos produziram a mesma saída **dentro do modelo**.

| Fixture e saída | Baseline | `height` | `window` | `reuse+window` | `reuse+height+window` | `width` |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 80 linhas, 320 células repetidas, documento inteiro | 35,16 µs | 29,95 µs (−14,8%) | 36,19 µs (+2,9%) | 26,55 µs (−24,5%) | **25,34 µs (−27,9%)** | 45,14 µs (+28,4%) |
| 80 linhas, 320 células únicas, documento inteiro | 34,47 µs | 28,63 µs (−16,9%) | 34,87 µs (+1,1%) | 26,38 µs (−23,5%) | **25,24 µs (−26,8%)** | 60,17 µs (+74,5%) |
| 80 linhas, conteúdo repetido, só primeira página | 17,98 µs | 12,57 µs (−30,1%) | 11,18 µs (−37,8%) | 9,74 µs (−45,8%) | **9,52 µs (−47,0%)** | 22,93 µs (+27,5%) |
| 320 linhas, 1280 células repetidas, documento inteiro | 103,86 µs | 81,81 µs (−21,2%) | 104,58 µs (+0,7%) | 83,31 µs (−19,8%) | **79,97 µs (−23,0%)** | 124,11 µs (+19,5%) |

Dados completos: [80 repetidas](grid-prototypes-2026-10-03/grid-prototype-repeated80.csv),
[80 únicas](grid-prototypes-2026-10-03/grid-prototype-unique80.csv),
[primeira página](grid-prototypes-2026-10-03/grid-prototype-first80.csv) e
[320 repetidas](grid-prototypes-2026-10-03/grid-prototype-repeated320.csv).

## O que os contadores explicam

- Com 320 células repetidas, o baseline faz 640 layouts completos; `height`
  faz 320 completos e 320 somente de altura. Ele ainda percorre as palavras
  e decide as quebras, mas deixa de construir as linhas descartadas na medição.
- `reuse+height+window` faz 320 layouts completos, 48 somente de altura e
  reaproveita 272 frames na emissão. Sem a janela, `reuse` alcança pico de
  408 linhas de frame retidas; com a janela, o pico é 6. A janela sozinha
  praticamente não acelera o documento inteiro, pois as 80 linhas são medidas.
- Na saída só da primeira página, `window` mede 22 linhas em vez de 80. Por
  isso seu ganho é de latência parcial no modelo, e não prova de redução de
  tempo para compilar um PDF inteiro.
- `width` reduz as avaliações intrínsecas de 160 para 2 no conteúdo repetido,
  porém clonar a chave e consultar o `HashMap` custa mais do que a pequena
  conta de larguras do modelo. No conteúdo único, não há acertos. É um
  exemplo concreto de contagem de operações menor com tempo pior.

## Controle de produção e decisão

O [runner de produção](grid-time-2026-10-02/run.py) foi repetido com 12 pares
por fixture, duas warmups, CPU 0 e saídas de subprocessos novos. O binário Tekt
tem SHA-256 `a12d75ded40944932b20dbd751d7969d3f846ac04de10c817cfffee707175cc4`;
o vanilla pinado em `9f2b6e8715237cb086899a42873660fe744622e8`
tem SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
O binário Tekt é idêntico ao da [medição anterior](grid-time-2026-10-02.md):
os protótipos não estão integrados no L1.

| Tabela | Tekt | Vanilla | Delta pareado |
| --- | ---: | ---: | ---: |
| 4 células | 175,90 ms | 127,21 ms | +48,37 ms |
| 80 células | 192,17 ms | 131,95 ms | +60,29 ms |
| 320 células | 246,22 ms | 147,42 ms | +98,64 ms |

Logo, **há ganhos apenas no laboratório sintético; ainda não há ganho medido
no compilador**. A direção mais promissora para o próximo experimento causal é
`height` e, se a identidade de contexto puder ser demonstrada, `reuse+window`.
O cache `width` desta forma deve ser descartado: inclusive com acertos, foi
mais lento. A janela deve ser avaliada por memória e latência parcial além do
tempo total. O laboratório omite estilo, decoração, fragmentação e caches reais
do Typst; seus percentuais não devem ser projetados no Tekt. A fixture de
produção ainda tem a diferença geométrica já registrada no relatório anterior,
embora texto, ordem e paginação coincidam.

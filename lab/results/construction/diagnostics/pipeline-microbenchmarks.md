# Pipeline: baseline exploratório de microbenchmarks

Medição em 2026-09-26T08:59:01-03:00, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado. Estado completo, hashes de arquivos, identidade do executável, amostras e intervalos de confiança estão em [pipeline-microbenchmarks-baseline.json](pipeline-microbenchmarks-baseline.json).

## Escopo e limites

A suite `benches/pipeline_bench.rs` mede entrypoints de produção completos, não helpers privados. Fonte, world e fontes embutidas são preparados fora do loop; parse, carregamento inicial de fontes e I/O de entrada não fazem parte dos tempos. São execuções com processo aquecido. O resultado e seus warnings são consumidos com `black_box`. Cada fixture é validada antes da amostragem.

Foram feitas três rodadas sequenciais do mesmo executável release, com 10 amostras, aquecimento de 1 s e medição solicitada de 2 s por caso. Criterion pode prolongar a coleta. A tabela apresenta a mediana entre as três estimativas de regressão (slope) e seu mínimo/máximo; essa faixa não é um intervalo de confiança. Os ICs de 95% de cada rodada estão no JSON.

Houve compilação concorrente observada na primeira rodada. CPU Ryzen 7 5800H, boost ativo, sem afinidade nem governor fixados. Estes números servem para iniciar investigação, não para fechar ganho/regressão. Fixtures diferentes não isolam o custo incremental de uma fase. Não há comparação contra revisão anterior nem alegação de speedup.

## Resultados

| Caso | Mediana das rodadas (ms) | Mín.–máx. (ms) | Amplitude relativa |
|---|---:|---:|---:|
| `pipeline/bundle/mixed` | 35.138 | 31.854–35.734 | 11.0% |
| `pipeline/bundle/one` | 1.786 | 1.706–1.815 | 6.1% |
| `pipeline/bundle/several` | 1.803 | 1.783–1.805 | 1.2% |
| `pipeline/html/frame` | 1.884 | 1.850–1.979 | 6.9% |
| `pipeline/html/semantic` | 1.732 | 1.713–1.850 | 7.9% |
| `pipeline/pdf/context_stable` | 2.183 | 2.003–2.544 | 24.8% |
| `pipeline/pdf/fallback` | 2.276 | 2.043–2.291 | 10.9% |
| `pipeline/pdf/line_callback` | 2.797 | 2.710–2.851 | 5.0% |
| `pipeline/pdf/multifont` | 3.560 | 3.500–3.603 | 2.9% |
| `pipeline/pdf/single_font` | 2.502 | 2.500–2.759 | 10.3% |
| `pipeline/pdf/tagged` | 3.364 | 3.326–5.836 | 74.6% |
| `pipeline/raster/document_ppi` | 2.071 | 1.949–2.095 | 7.1% |
| `pipeline/raster/override_144` | 2.083 | 2.032–2.085 | 2.5% |
| `pipeline/svg/destination` | 2.398 | 2.298–3.974 | 69.9% |
| `pipeline/svg/glyphs` | 3.874 | 2.229–4.139 | 49.3% |
| `pipeline/svg/with_fonts` | 2.019 | 1.949–2.081 | 6.6% |
| `pipeline/svg/without_fonts` | 1.876 | 1.766–1.902 | 7.3% |

## Próximo ciclo de refinamento

1. Repetir em máquina sem compilação concorrente antes de estabelecer um gate de regressão.
2. Investigar primeiro fixtures PDF com fontes e bundle misto; decompor pelos `Timings` existentes antes de atribuir o custo a eval, layout ou export. Não expor helpers privados apenas para medir.
3. Usar os benchmarks de layout e builder para isolar uma hipótese, alterar uma única responsabilidade com L0 vigente e testes de comportamento, e repetir a mesma fixture/configuração.

Comparação posterior com o baseline salvo localmente (ele não substitui o JSON versionável):

```sh
cargo bench -p typst-benches --bench pipeline_bench --offline -- --sample-size 10 --warm-up-time 1 --measurement-time 2 --baseline pipeline-initial-20260926-r3
```

Artefatos Criterion ficam em `target/criterion/pipeline_*`. A medição não modificou código produtivo nem a arquitetura L0.


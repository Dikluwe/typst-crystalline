# Ablação do limite superior dinâmico — 2026-10-02

## Proveniência

- Working tree não commitado sobre `d9c3b0ab41a75166f5828c20630902bc638f5cf9`. Captura `2026-10-02 19:22:05 UTC`: `git diff HEAD --stat` indicava `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md` (4 linhas adicionadas) e `01_core/src/compiler/layout/paragraph_select_forward.rs` (59 linhas alteradas), 54 inserções e 9 remoções. Havia também o relatório não rastreado `00_nucleo/diagnosticos/microbench-paragraph-select-incremental-2026-10-02.md`. Nenhum desses arquivos mudou até o fim da medição.
- Comando: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`. Criterion: 30 amostras, 1 s de aquecimento e 2 s de medição por variante. Log bruto local: `/tmp/tekt-paragraph-dynamic.log`.
- Os ranges da variante dinâmica coincidiram com os do seletor exato na matriz diferencial e nos seis cenários medidos.

## Resultado

Medianas em µs; menor é melhor. `avaliados` conta candidatos relaxados pelo forward.

| Caso | Forward prefixado | Limite dinâmico | Avaliados antes → depois |
|---|---:|---:|---:|
| 40 palavras, 76pt | **2,307** | 2,367 | 94 → 93 |
| 160 palavras, 240pt | 7,733 | **7,676** | 343 → 343 |
| 160 palavras, 76pt | 11,227 | **11,125** | 648 → 647 |
| 1.000 palavras, 240pt | 139,18 | **137,33** | 10.300 → 10.300 |
| 1.000 palavras, 1.200pt, justificado | **56,031** | 58,274 | 3.139 → 3.139 |
| 1.000 palavras, 1.200pt, sem justificação | **42,460** | 43,124 | 2.061 → 2.061 |

O limite final melhorou o custo inicial em alguns casos, mas quase nenhum início ou candidato restante foi eliminado. O pequeno ganho de tempo em algumas linhas sem redução de trabalho pode ser ruído de medição ou efeito de compilação; não prova vantagem algorítmica.

## Decisão

Rejeitar o aperto dinâmico nesta forma e removê-lo do consumer experimental. Investigar uma partição inicial melhor, que possa podar antes de percorrer os candidatos, em vez de esperar por um caminho completo no fim da DP.

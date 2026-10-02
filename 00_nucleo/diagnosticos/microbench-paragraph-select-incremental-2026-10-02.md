# Ablação da medida incremental — 2026-10-02

## Proveniência

- Working tree não commitado sobre `d9c3b0ab41a75166f5828c20630902bc638f5cf9`. Captura `2026-10-02 19:13:44 UTC`: `git diff HEAD --stat` indicava somente `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md` (6 linhas adicionadas) e `01_core/src/compiler/layout/paragraph_select_forward.rs` (93 linhas alteradas), 94 inserções e 5 remoções no total. Este relatório foi escrito depois.
- Comando: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`. Criterion: 30 amostras, 1 s de aquecimento, 2 s de medição por variante. Log bruto local: `/tmp/tekt-paragraph-incremental.log`.
- A variante acumulava a medida enquanto avançava o fim. Perto dos limiares numéricos de admissibilidade e snap, usava a medida prefixada para evitar diferenças de quebra. A matriz diferencial e os seis casos do benchmark passaram após essa proteção.

## Resultado

Medianas em µs; menor é melhor. Valores das variantes medidos na mesma rodada.

| Caso | Exato | Forward prefixado | Forward incremental |
|---|---:|---:|---:|
| 40 palavras, 76pt | 2,220 | **2,200** | 3,873 |
| 160 palavras, 240pt | 19,370 | **7,180** | 13,784 |
| 160 palavras, 76pt | **8,407** | 10,932 | 23,755 |
| 1.000 palavras, 240pt | **118,88** | 139,12 | 343,54 |
| 1.000 palavras, 1.200pt, justificado | 562,72 | **69,123** | 125,04 |
| 1.000 palavras, 1.200pt, sem justificação | 527,56 | **44,498** | 87,601 |

O teste RED encontrou um contraexemplo de arredondamento: com 27 palavras, gap 6pt, tracking 0,2pt, largura 120pt e justificação, a largura 7–10 foi `123.99999999999999` pelo prefixo e `124.00000000000001` pelo incremento. A diferença atravessou o ratio `-1` e mudou fortemente o custo. A proteção recuperou as quebras, mas a variante ficou mais lenta em todos os casos medidos. A operação adicional por candidato e a checagem de proximidade do limiar superaram a economia de subtrações de prefixos. Esta última atribuição é inferência, não perfil de CPU.

## Decisão

Rejeitar esta variante e removê-la do consumer experimental. Manter a medida prefixada para a busca forward. Procurar redução de trabalho algorítmico, como apertar o limite superior durante a DP, em vez de trocar a aritmética de cada medida.

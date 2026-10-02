# Reparação local da semente forward — 2026-10-02

## Proveniência

- HEAD comum às rodadas: `d9c3b0ab41a75166f5828c20630902bc638f5cf9`, working tree não commitado. O consumer `01_core/src/compiler/layout/paragraph_select_forward.rs` e seu L0 foram os únicos arquivos modificados rastreados; os diagnósticos incremental e dynamic-bound estavam não rastreados e não influenciaram o build.
- Busca completa em cada par: captura `2026-10-02 19:29:56 UTC`, `git diff HEAD --stat`: L0 4 linhas adicionadas, consumer 90 linhas alteradas, 83 inserções e 11 remoções. Log `/tmp/tekt-paragraph-local-seed.log`.
- Deslocamento de um passo, duas passagens: captura `2026-10-02 19:34:58 UTC`, L0 6 linhas adicionadas, consumer 93 linhas alteradas, 88 inserções e 11 remoções. Log `/tmp/tekt-paragraph-local-step.log`.
- Escala 500/1.000/2.000 palavras: captura `2026-10-02 19:39:36 UTC`, L0 8 linhas adicionadas, consumer 95 linhas alteradas, 92 inserções e 11 remoções. Log `/tmp/tekt-paragraph-local-scale.log`.
- Comparação de uma e duas passagens: captura `2026-10-02 19:58:42 UTC`, L0 14 linhas adicionadas, consumer 159 linhas alteradas, 161 inserções e 12 remoções. Log `/tmp/tekt-paragraph-onepass.log`.
- Versão final de uma passagem com escolha de três caminhos: captura `2026-10-02 20:05:58 UTC`, L0 16 linhas adicionadas, consumer 122 linhas alteradas, 126 inserções e 12 remoções. Log `/tmp/tekt-paragraph-algorithm-final.log`. Este relatório e o resselo foram feitos depois da rodada.
- Comando em cada rodada: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`. Criterion: 30 amostras, aquecimento de 1 s e medição de 2 s por variante. Os testes exigiram ranges iguais ao seletor exato antes de cada grupo.

## Mecanismo

A busca forward usa uma partição gulosa como limite superior inicial. A reparação examina cada par de linhas vizinhas uma vez, da esquerda para a direita, e tenta mover o corte uma palavra para cada lado. Aceita somente o deslocamento de menor soma de custos se houver melhoria estrita e ambas as linhas permanecerem não vazias. A DP final continua exata no subconjunto preparado; a semente muda apenas a poda por limite superior.

A busca de todos os cortes entre cada par reduziu candidatos, mas custou mais que a economia (1.000 palavras/240pt: 174,24 µs contra 131,98 µs do forward simples). Duas passagens de um passo baixaram para 118,80 µs. Uma passagem obteve o mesmo limite e a mesma quantidade de candidatos que duas nos oito casos medidos, com menos tempo (1.000 palavras/240pt: 113,70 contra 118,79 µs na mesma rodada).

## Rodada final

Medianas em µs; menor é melhor. A coluna adaptativo inclui o custo da escolha. Intervalos de confiança completos estão no log final.

| Caso | Exato | Forward simples | Forward reparado | Escolha | Adaptativo |
|---|---:|---:|---:|---|---:|
| 40 palavras, 76pt | **2,076** | 2,128 | 3,035 | exato | 2,091 |
| 160 palavras, 240pt | 19,094 | **7,385** | 7,712 | forward simples | 7,570 |
| 160 palavras, 76pt | **8,309** | 10,681 | 13,661 | exato | 8,537 |
| 500 palavras, 240pt | 59,402 | 43,289 | **39,529** | reparado | 40,460 |
| 1.000 palavras, 240pt | 117,37 | 134,19 | **114,82** | reparado | 115,66 |
| 2.000 palavras, 240pt | **235,60** | 303,76 | 280,18 | exato | 239,24 |
| 1.000 palavras, 1.200pt, justificado | 507,43 | **52,712** | 54,832 | forward simples | 55,132 |
| 1.000 palavras, 1.200pt, sem justificação | 490,92 | **40,641** | 42,758 | forward simples | 42,481 |

Em 500 palavras/240pt, a reparação reduziu candidatos de 2.932 para 2.305; em 1.000, de 10.300 para 7.895. Em 2.000, reduziu de 23.723 para 20.577, mas o custo de preparar e percorrer a variante permaneceu maior que o exato. Nos casos largos, não reduziu candidatos; a escolha adaptativa evita a reparação nesses casos.

## Decisão e limite

Manter a variante e a escolha somente sob `cfg(test)`. Os limiares de 64, 256 e 1.200 palavras e de avanços médios foram calibrados com a mesma matriz sintética; os ganhos medidos não estimam desempenho geral. Medir um corpus real, fontes, tracking, gaps e hifenização no pipeline completo antes de propor alteração do default de produção. Uma mudança de caminho padrão requer o gate da ADR-0127.

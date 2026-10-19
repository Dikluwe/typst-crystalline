# Escolha adaptativa experimental do seletor de linhas — 2026-10-02

## Proveniência

- Rodada refinada: working tree não commitado sobre `39a74d47778f8685d54fe16ed1f56112e4332af0`. Captura `2026-10-02 18:56:32 UTC`, antes da execução do teste: `git diff HEAD --stat` mostrava somente `00_nucleo/prompts/compiler/layout/paragraph_select_forward.md` (6 linhas adicionadas) e `01_core/src/compiler/layout/paragraph_select_forward.rs` (79 linhas alteradas), total de 69 inserções e 16 remoções. O relatório e o resselo vieram após a medição.
- Comando: `cargo test --release -p typst-core microbench_forward_vs_exact --lib -- --ignored --nocapture`. Criterion: 30 amostras por variante, 1 s de aquecimento e 2 s de medição. Log bruto local: `/tmp/tekt-paragraph-adaptive-refined.log`.
- O benchmark confirmou ranges idênticos ao seletor exato antes de medir cada caso. A variante adaptativa cronometra a decisão e o algoritmo escolhido; não executa ambos na mesma seleção.
- A primeira rodada, com regra baseada só em contagem para 64–256 palavras, foi capturada `2026-10-02 18:50:57 UTC` sobre o mesmo HEAD com apenas os dois arquivos acima alterados (`git diff HEAD --stat`: 63 inserções, 16 remoções). Log: `/tmp/tekt-paragraph-adaptive.log`. Ela motivou a correção para largura relativa ao avanço médio.

## Resultado da regra refinada

Medianas em µs; menor é melhor. Entradas sintéticas de palavras preparadas com gap de 3pt. Intervalos de confiança completos nos logs.

| Caso | Exato | Forward prefixado | Escolha | Adaptativo |
|---|---:|---:|---|---:|
| 40 palavras, 76pt | **2,100** | 2,164 | exato | 2,162 |
| 160 palavras, 240pt | 19,973 | **14,175** | forward | 13,747 |
| 160 palavras, 76pt | **9,294** | 11,092 | exato | 8,783 |
| 1.000 palavras, 240pt | **119,58** | 133,47 | exato | 124,97 |
| 1.000 palavras, 1.200pt, justificado | 518,37 | **54,307** | forward | 55,839 |
| 1.000 palavras, 1.200pt, sem justificação | 499,36 | **42,352** | forward | 43,821 |

A escolha acertou o menor dos dois caminhos isolados nestes seis casos. A primeira regra errou no caso de 160 palavras/76pt: adaptativo 10,744 µs contra exato 8,338 µs e forward 10,794 µs. Usar a largura relativa ao avanço médio corrigiu a seleção.

O custo de escolha não é nulo. No caso de 1.000 palavras/240pt, a variante adaptativa ficou cerca de 4,5% acima do exato isolado nesta rodada. A medição de 160 palavras/240pt apresentou variação ampla entre amostras; a rodada anterior encontrou exato 19,545 µs e forward 7,407 µs. O sinal de escolha permaneceu o mesmo, mas a magnitude do ganho nesse caso exige repetição em ambiente controlado antes de qualquer decisão de produção.

## Decisão

Manter a escolha apenas sob `cfg(test)`. Os limiares foram calibrados com esta matriz sintética; seis acertos aqui não estimam a taxa de acerto em textos reais. Para promover a ideia, medir corpus representativo com diferentes fontes, gaps, tracking, hifenização e justificação, além de validar a equivalência das quebras no pipeline completo. Qualquer alteração do caminho padrão exige o gate de L0 da ADR-0127.

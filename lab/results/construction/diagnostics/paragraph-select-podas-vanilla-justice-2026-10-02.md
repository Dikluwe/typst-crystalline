# Podas por linha no seletor de parágrafos — 2026-10-02

## Proveniência

- HEAD das duas rodadas: `902bc8f2ddd0da2e69bce1156329e8a8b136d2cd`; working tree não commitado.
- Primeira captura: `2026-10-02 20:37:51 UTC`, `git diff HEAD --stat`: dois arquivos, 167 inserções e 26 remoções (`paragraph_select_bounded.md`: 6 linhas; `paragraph_select_bounded.rs`: 187 linhas no stat). Comando: `cargo test --release -p typst-core microbench_exact_vs_bounded --lib --no-default-features -- --ignored --nocapture`. Log: `/tmp/tekt-pruning-benchmark.log`.
- Segunda captura: `2026-10-02 20:44:57 UTC`, `git diff HEAD --stat`: dois arquivos, 223 inserções e 30 remoções (`paragraph_select_bounded.md`: 6 linhas; `paragraph_select_bounded.rs`: 247 linhas no stat). Comando: `cargo test --release -p typst-core microbench_justice_without_seed --lib --no-default-features -- --ignored --nocapture`. Log: `/tmp/tekt-justice-no-seed-benchmark.log`.
- CPU: AMD Ryzen 7 5800H, Linux x86_64. Cada variante usa Criterion com 30 amostras, aquecimento de 1 s e medição de 2 s; `Prepared` é reutilizado. Os logs contêm intervalos de confiança e outliers. Os arquivos foram formatados e os hashes L0/L1 selados **depois** das medições, sem alteração algorítmica.

## Hipóteses e implementação experimental

O vanilla pinado em `lab/typst-original/crates/typst-layout/src/inline/linebreak.rs:264-344` mantém limite superior e um limite inferior para tentativas sucessivas de linha subcheia. O [Justice no commit observado](https://github.com/kitlangton/justice/blob/86dbc6da8cc2059cabb2a67c0aa42d773d5023e2/src/engine.ts) percorre predecessores em ordem inversa e interrompe depois de transbordamento sob condição de monotonicidade. São mecanismos de busca, não requisitos de paridade de linguagem.

O protótipo privado `paragraph_select_bounded.rs` mede quatro caminhos: DP exata produtiva; limite superior Simple; limite superior com limite inferior de linha no estilo vanilla; e limite superior com interrupção por transbordamento no estilo Justice. Uma segunda rodada mede Justice sem preparar o limite superior. Todas as variantes usam o mesmo custo e medidas prefixadas. O teste diferencial confere ranges completos com a DP exata em tamanhos de 0 a 28 palavras, gaps e tracking positivos/negativos, larguras diversas, justificação e runt negativo/positivo. As podas dependentes de monotonicidade são desligadas com contribuições negativas ou não finitas. Resultado: 3 testes verdes, 2 microbenchmarks ignorados na suíte normal.

## Primeira rodada

Mediana em µs da seleção completa; menor é melhor. `Vanilla` e `Justice` incluem a preparação do limite superior Simple.

| Caso | Exata | Limite superior | Vanilla | Justice |
|---|---:|---:|---:|---:|
| 40 palavras, 76 pt | **2,560** | 3,923 | 3,911 | 3,599 |
| 160 palavras, 240 pt | **19,622** | 25,097 | 24,352 | 29,471 |
| 160 palavras, 1.200 pt, justificado | **70,604** | 90,273 | 89,501 | 102,790 |
| 1.000 palavras, 76 pt | **51,923** | 85,667 | 88,984 | 91,147 |
| 500 palavras, 240 pt | **59,677** | 84,055 | 81,507 | 98,297 |
| 1.000 palavras, 240 pt | **120,770** | 192,410 | 195,470 | 210,360 |
| 2.000 palavras, 240 pt | **233,340** | 403,600 | 411,340 | 424,800 |
| 1.000 palavras, 1.200 pt, justificado | **509,520** | 645,540 | 641,110 | 744,790 |

Em 500 palavras/240 pt, a variante Vanilla reduziu avaliações de custo de 2.932 para 2.063, mas ainda gastou 81,507 µs contra 59,677 µs da DP exata. Em 1.000 palavras/240 pt, reduziu de 10.300 para 9.695 avaliações e gastou 195,470 µs contra 120,770 µs. Em 2.000 palavras/240 pt, reduziu apenas de 23.723 para 23.564. O custo extra de verificar os limites e preparar o teto não foi compensado pela poda nessa matriz.

## Ablação Justice sem semente

Medianas em µs numa segunda rodada, com a DP exata medida novamente ao lado; não comparar os tempos absolutos de rodadas diferentes como se fossem amostras emparelhadas.

| Caso | Exata | Justice sem semente | Avaliações / podas Justice |
|---|---:|---:|---:|
| 40 palavras, 76 pt | **2,080** | 3,059 | 156 / 7 |
| 160 palavras, 240 pt | **19,464** | 29,979 | 1.852 / 23 |
| 500 palavras, 240 pt | **60,321** | 96,141 | 5.932 / 77 |
| 1.000 palavras, 240 pt | **122,730** | 196,900 | 11.933 / 157 |
| 2.000 palavras, 240 pt | **257,260** | 393,200 | 23.932 / 314 |
| 1.000 palavras, 1.200 pt, justificado | **664,490** | 1.002,700 | 58.626 / 198 |

## Decisão

Manter todas as variantes sob `cfg(test)`. Nesta função de custo simplificada, o descarte adicional é pequeno ou custa mais que a avaliação que evita. O mecanismo Vanilla pode ainda ser útil no pipeline completo, onde construir uma linha é mais caro; este microbenchmark mede apenas seleção sobre `Prepared` sintético. O Justice observado usa uma função de custo e estados diferentes, e a nossa variante é somente um decalque da regra de interrupção, não um porte integral. Nenhuma mudança de default é justificada por estes resultados. Para nova investigação, medir o custo de construção de linhas e testar em corpus real antes de tentar promover uma poda.

Verificações finais: `cargo test -p typst-core paragraph_select_bounded --lib --no-default-features` (3 passados, 2 ignorados), `cargo build` verde, `git diff --check` verde. `crystalline-lint .` saiu com código 0 e não reportou o consumer focal; há warnings globais preexistentes no repositório. `crystalline-lint --fix-hashes .` falhou no preflight global (`canonical hash metadata must occur exactly once`) por outros consumers; o par L0/L1 focal foi selado manualmente com SHA-256 após remover as respectivas linhas de metadata, conforme o algoritmo de `tekt-linter/03_infra/hash_writer.rs`.

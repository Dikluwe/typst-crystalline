# Microbenchmark: seleção exata versus protótipo com poda

## Proveniência da medição

- Data de conclusão da saída: 2026-09-30 23:35:06.988634183 -0300 (`stat` de `/tmp/tekt-paragraph-microbench.log`).
- Estado do código durante a medição: HEAD `453237f59b58aa47aca5fb5342e6ea3c484cc8d5`, working tree não commitado. Alterações exatas: `00_nucleo/prompts/compiler/layout/paragraph_select_bounded.md` (6 linhas no diff; adição do microbenchmark ao L0 e resselo) e `01_core/src/compiler/layout/paragraph_select_bounded.rs` (65 linhas no diff; benchmark ignorado e resselo). `git diff HEAD --stat` registrou `2 files changed, 69 insertions(+), 2 deletions(-)`. Não havia arquivos não rastreados. A árvore não mudou durante a execução.
- Comando: `cargo test --release -p typst-core microbench_exact_vs_bounded --lib -- --ignored --nocapture`.
- Ambiente: Linux x86_64, AMD Ryzen 7 5800H, 16 CPUs lógicas; rustc 1.92.0, cargo 1.92.0. Perfil `release` do workspace; Criterion 0.5, 30 amostras, 1 s de aquecimento e 2 s de medição por variante.
- Entrada: `Prepared` determinístico reaproveitado pelos dois seletores em cada caso; custo default, fonte 12pt, gaps 3pt, tracking zero. A equivalência dos ranges é verificada antes de medir. A medição inclui alocações, cálculo do limite guloso e retrace; exclui preparação do `Prepared`.

## Resultado local

| Caso | Palavras | Largura | Justificado | Exato (µs, intervalo Criterion) | Com poda (µs, intervalo Criterion) | Relação exato/com poda | Candidatos avaliados/podados no protótipo |
|---|---:|---:|:---:|---:|---:|---:|---:|
| short_narrow | 40 | 76pt | não | 2.2596 [2.2481, 2.2745] | 2.8802 [2.8706, 2.8911] | 0.78× | 94 / 69 |
| medium | 160 | 240pt | não | 21.118 [21.047, 21.223] | 17.533 [17.380, 17.706] | 1.20× | 343 / 1532 |
| wide_justified | 160 | 1200pt | sim | 75.628 [75.252, 76.029] | 49.704 [49.511, 49.986] | 1.52× | 271 / 7620 |

## Leitura e limite

O protótipo é mais lento na entrada curta e mais rápido nas duas entradas maiores deste ensaio. A passada gulosa adicional custa mais do que a poda economiza quando há poucos candidatos. O resultado só cobre seleção de finais de palavra com largura fixa e sem hifenização; não mede compilação completa, layout paginado, nem o algoritmo do vanilla. Não autoriza troca do default de produção. Outras distribuições de palavra/largura e execuções em máquinas diferentes precisam de medição antes de concluir ganho geral.

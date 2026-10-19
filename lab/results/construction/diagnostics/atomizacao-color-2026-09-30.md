# Atomização de `stdlib/color` — 2026-09-30

## Proveniência

O estado inicial foi o commit `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` com árvore limpa. O manifesto [atomizacao-color-2026-09-30-before.json.gz](atomizacao-color-2026-09-30-before.json.gz) registra SHA-256 e número de linhas do arquivo de código e do Prompt L0 antes da alteração. A medição final foi feita em 2026-09-30T20:29:35.635050+00:00 sobre a working tree não commitada do mesmo HEAD; [atomizacao-color-2026-09-30-after.json.gz](atomizacao-color-2026-09-30-after.json.gz) contém o estado exato da árvore, caminhos, linhas e SHA-256 de cada arquivo resultante. Essa medição é estrutural e não indica desempenho.

## Separação por comportamento

O módulo `01_core/src/compiler/stdlib/color.rs` conserva registro, interface e testes históricos; chamadas e operações foram extraídas para arquivos da própria camada L1. O despacho continua estático. Cada novo consumer tem um Prompt L0 proprietário em `00_nucleo/prompts/compiler/stdlib/color/`.

| Unidade | Consumer |
|---|---|
| `palette` | `palette.rs` |
| `arguments` | `arguments.rs` |
| `light` | `light.rs` |
| `mix` | `mix.rs` |
| `transform` | `transform.rs` |
| `inspect` | `inspect.rs` |
| `alpha` | `alpha.rs` |
| `spot` | `spot.rs` |

O arquivo original tinha 1386 linhas. A fachada final tem 638 linhas e os arquivos resultantes somam 1637 linhas, incluindo headers e testes novos. O número de linhas não é uma métrica de performance.

## Verificação

- Testes direcionados de `color`: passaram.
- Suíte `cargo test -p typst-core --lib --offline --locked --quiet`: **6447 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A extração preserva contratos públicos, defaults e fase do pipeline. Não há afirmação de ganho de tempo ou memória; o objetivo é localizar algoritmos candidatos e permitir comparações posteriores por unidade.

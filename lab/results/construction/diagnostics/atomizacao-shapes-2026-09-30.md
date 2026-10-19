# Atomização de `stdlib/shapes` — 2026-09-30

## Proveniência

O estado inicial foi o commit `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` com árvore limpa. O manifesto [atomizacao-shapes-2026-09-30-before.json.gz](atomizacao-shapes-2026-09-30-before.json.gz) registra SHA-256 e número de linhas do arquivo de código e do Prompt L0 antes da alteração. A medição final foi feita em 2026-09-30T20:29:35.633541+00:00 sobre a working tree não commitada do mesmo HEAD; [atomizacao-shapes-2026-09-30-after.json.gz](atomizacao-shapes-2026-09-30-after.json.gz) contém o estado exato da árvore, caminhos, linhas e SHA-256 de cada arquivo resultante. Essa medição é estrutural e não indica desempenho.

## Separação por comportamento

O módulo `01_core/src/compiler/stdlib/shapes.rs` conserva registro, interface e testes históricos; chamadas e operações foram extraídas para arquivos da própria camada L1. O despacho continua estático. Cada novo consumer tem um Prompt L0 proprietário em `00_nucleo/prompts/compiler/stdlib/shapes/`.

| Unidade | Consumer |
|---|---|
| `paint` | `paint.rs` |
| `rect` | `rect.rs` |
| `ellipse` | `ellipse.rs` |
| `line` | `line.rs` |
| `coordinates` | `coordinates.rs` |
| `polygon` | `polygon.rs` |
| `curve` | `curve.rs` |

O arquivo original tinha 1676 linhas. A fachada final tem 442 linhas e os arquivos resultantes somam 1916 linhas, incluindo headers e testes novos. O número de linhas não é uma métrica de performance.

## Verificação

- Testes direcionados de `shapes`: passaram.
- Suíte `cargo test -p typst-core --lib --offline --locked --quiet`: **6447 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A extração preserva contratos públicos, defaults e fase do pipeline. Não há afirmação de ganho de tempo ou memória; o objetivo é localizar algoritmos candidatos e permitir comparações posteriores por unidade.

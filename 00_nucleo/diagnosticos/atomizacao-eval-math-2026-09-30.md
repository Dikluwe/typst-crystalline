# Atomização de `eval/math` — 2026-09-30

## Proveniência

Estado inicial: commit `e754ab364a221dbe344852178e84bd42b02c463e`, árvore limpa, arquivo com 1671 linhas e SHA-256 `8af872e299721875c6f27184944dc64eb86b415a6d7c15b22fa1a289eec7630e`; [atomizacao-eval-math-2026-09-30-before.json.gz](atomizacao-eval-math-2026-09-30-before.json.gz) registra hash L0 e horário UTC. Estado final medido em 2026-09-30T21:32:26.271486+00:00 sobre working tree não commitada do mesmo HEAD; [atomizacao-eval-math-2026-09-30-after.json.gz](atomizacao-eval-math-2026-09-30-after.json.gz) contém alterações exatas, caminhos, linhas e SHA-256. Linhas medem estrutura, não desempenho.

## Separação

A fachada mantém imports e o ponto de entrada. Os consumers L1 de lookup, entrada, callee, argumentos, construção, avaliação de expressão e delimitadores possuem Prompts L0 1:1 em `00_nucleo/prompts/compiler/eval/math/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 47 |
| `lookup` | 105 |
| `entry` | 45 |
| `callee` | 94 |
| `arguments` | 341 |
| `construct` | 30 |
| `expression` | 963 |
| `delimiters` | 125 |

`expression.rs` permanece com 963 linhas porque o `match` exaustivo da expressão é um algoritmo central único. Uma subdivisão posterior das famílias de arms exige preservar ordem de avaliação, spans e morfologia; esta extração não alterou o match.

## Verificação

- Teste novo de delimitador: RED antes da extração, GREEN depois.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6458 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

Não houve benchmark nem alegação de ganho de tempo ou memória. A extração conserva contratos públicos, defaults e fase do pipeline.

# Atomização de `eval/bindings/field_access` — 2026-09-30

## Proveniência

Estado inicial: commit `ba5a4c7d305ed1444660d9eb80862f51b417efb6`, árvore limpa, arquivo com 2869 linhas e SHA-256 `38d68e90b2aa0216151627711fffe8e18bf600db3b363e33d23f6e0ed62fee55`; [atomizacao-field-access-2026-09-30-before.json.gz](atomizacao-field-access-2026-09-30-before.json.gz) registra o hash L0 e horário UTC. Estado final medido em 2026-09-30T21:12:49.270790+00:00 sobre a working tree não commitada do mesmo HEAD; [atomizacao-field-access-2026-09-30-after.json.gz](atomizacao-field-access-2026-09-30-after.json.gz) contém a lista exata de alterações, caminhos, linhas e SHA-256 dos arquivos. Linhas medem estrutura, não desempenho.

## Separação

A fachada mantém interface e testes históricos. Os consumers L1 de identidade, resolução de campos, diagnóstico de callee e métodos de Content possuem Prompts L0 individuais em `00_nucleo/prompts/compiler/eval/bindings/field_access/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 1739 |
| `identity` | 132 |
| `fields` | 502 |
| `callee_error` | 120 |
| `content_methods` | 442 |

## Verificação

- Teste novo de métodos de Content: RED antes da extração, GREEN depois.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6451 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A extração preserva assinaturas públicas, defaults e fase do pipeline. Não houve benchmark nem alegação de ganho de tempo ou memória.

# Atomização de `entities/gradient` — 2026-09-30

## Proveniência

Estado inicial: commit `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229`, árvore limpa, arquivo com 2216 linhas e SHA-256 `3093f3c9453f9e20a533bda3242280523ee913bf87a22f962ee71046400986e6`; [atomizacao-gradient-entity-2026-09-30-before.json.gz](atomizacao-gradient-entity-2026-09-30-before.json.gz) registra hash L0 e horário UTC. Estado final medido em 2026-09-30T21:20:39.813907+00:00 sobre working tree não commitada do mesmo HEAD; [atomizacao-gradient-entity-2026-09-30-after.json.gz](atomizacao-gradient-entity-2026-09-30-after.json.gz) contém alterações exatas, caminhos, linhas e SHA-256 dos arquivos. Linhas medem estrutura, não desempenho.

## Separação

A fachada preserva `GradientStop`, `RelativeTo`, a superfície pública por reexports e os testes históricos. Os consumers L1 de offsets, Linear, interpolação, Radial, Conic e enum Gradient possuem Prompts L0 individuais em `00_nucleo/prompts/entities/gradient/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 1242 |
| `offsets` | 48 |
| `linear` | 198 |
| `interpolation` | 348 |
| `radial` | 205 |
| `conic` | 146 |
| `kind` | 188 |

## Verificação

- Teste novo de offset implícito: RED antes da extração, GREEN depois; testes locais das quatro variantes passaram.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6456 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

Não houve benchmark nem alegação de ganho de tempo ou memória. A extração conserva contratos públicos, defaults e fase do pipeline.

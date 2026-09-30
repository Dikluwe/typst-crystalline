# Atomização de `stdlib/gradients` — 2026-09-30

## Proveniência

Estado inicial: commit `b2705f23fe5bfb53e623fd3b65f8b4ea50815560`, árvore limpa, arquivo com 1058 linhas e SHA-256 `38c38dc27e1051ab37737a905ae283d7e8d09cab68378e99e543f72edea5cff2`; [atomizacao-gradients-stdlib-2026-09-30-before.json.gz](atomizacao-gradients-stdlib-2026-09-30-before.json.gz) registra hash L0 e horário UTC. Estado final medido em 2026-09-30T21:26:36.328764+00:00 sobre working tree não commitada do mesmo HEAD; [atomizacao-gradients-stdlib-2026-09-30-after.json.gz](atomizacao-gradients-stdlib-2026-09-30-after.json.gz) guarda alterações exatas, caminhos, linhas e SHA-256. Linhas medem estrutura, não desempenho.

## Separação

A fachada preserva a interface pública. Os consumers L1 de despacho, acessores, amostragem, modificadores, parsing e construtores por variante possuem Prompts L0 1:1 em `00_nucleo/prompts/compiler/stdlib/gradients/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 65 |
| `dispatch` | 107 |
| `accessors` | 157 |
| `sampling` | 99 |
| `modifiers` | 164 |
| `linear` | 80 |
| `parsing` | 225 |
| `radial` | 166 |
| `conic` | 92 |

## Verificação

- Teste novo de dispatch: RED antes da extração, GREEN depois.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6457 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A extração preserva assinaturas públicas, defaults e fase do pipeline. Não houve benchmark nem alegação de ganho de tempo ou memória.

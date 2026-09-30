# Segunda atomização de `eval/call_dispatch/call` — 2026-09-30

## Proveniência

Estado inicial: commit `1492976302212e007282875aef066f84a3cf13aa`, árvore limpa, `call.rs` com 828 linhas e SHA-256 `67dc387ebf34821670f0e2a0bf2f1cdc80e91e679ad32fc739b5291b316d06b9`; [atomizacao-call-core-2026-09-30-before.json.gz](atomizacao-call-core-2026-09-30-before.json.gz) registra hash L0 e horário UTC. Estado final medido em 2026-09-30T21:38:09.170837+00:00 sobre working tree não commitada do mesmo HEAD; [atomizacao-call-core-2026-09-30-after.json.gz](atomizacao-call-core-2026-09-30-after.json.gz) contém alterações exatas, caminhos, linhas e SHA-256. Linhas medem estrutura, não desempenho.

## Separação

A sequência de métodos de Selector foi movida para `selector_intercepts.rs` e a fase de invocação do callee para `invoke.rs`. `call.rs` conserva a ordem dos demais interceptores e delega à invocação final; os três consumers têm Prompts L0 1:1.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 576 |
| `call` | 594 |
| `selector_intercepts` | 71 |
| `invoke` | 206 |

## Verificação

- Teste novo de seleção de métodos lógicos: RED antes da extração, GREEN depois.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6459 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

O despacho permanece estático e sem mudança intencional de avaliação, diagnósticos, defaults ou fase do pipeline. Não houve benchmark nem alegação de ganho de tempo ou memória.

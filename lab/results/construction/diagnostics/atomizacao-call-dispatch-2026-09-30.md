# Atomização de `eval/call_dispatch` — 2026-09-30

## Proveniência

Estado inicial: commit `d5c563dfab39f789e8e2f72c9a9364f5a761e3ca`, árvore limpa, código com 2674 linhas e SHA-256 `343b94c5999791d4dcca3897940e7fce684ffefb5a3c384e8895be6b1edc706e`. [atomizacao-call-dispatch-2026-09-30-before.json.gz](atomizacao-call-dispatch-2026-09-30-before.json.gz) registra também hash L0 e horário UTC. A medição final foi feita em 2026-09-30T21:04:17.515939+00:00 sobre a working tree não commitada do mesmo HEAD; [atomizacao-call-dispatch-2026-09-30-after.json.gz](atomizacao-call-dispatch-2026-09-30-after.json.gz) contém a lista exata de alterações, caminhos, linhas e SHA-256 dos arquivos resultantes. Estes números descrevem organização, não desempenho.

## Separação

A fachada conserva os pontos de entrada de outros módulos e os testes históricos. A lógica foi agrupada por âncoras de chamada, avaliação/propagação de argumentos, aplicação de funções, campos estáticos de tipos, métodos de valores, traces/plugins, resolução de chamadas e métodos de Location. Cada consumer possui Prompt L0 1:1 em `00_nucleo/prompts/compiler/eval/call_dispatch/`.

| Unidade | Linhas finais |
|---|---:|
| `fachada` | 567 |
| `call_spans` | 351 |
| `arguments` | 124 |
| `apply` | 94 |
| `type_fields` | 125 |
| `value_methods` | 496 |
| `trace_plugin` | 111 |
| `call` | 828 |
| `location` | 89 |

O arquivo resultante `call.rs` mantém a ordem do algoritmo central de intercepção e fallback em uma função longa de 828 linhas. Uma divisão posterior desse algoritmo exige explicitar os estados de retorno/receiver para provar avaliação única e ordem de prioridade; não foi alterada nesta extração. Os arquivos resultantes somam 2785 linhas com headers, delegações e testes novos.

## Verificação

- Teste de tipo/ordem de `with`: RED antes da extração, GREEN depois.
- Teste direto de escolha de span: passou.
- `cargo test -p typst-core --lib --offline --locked --quiet`: **6450 passaram, 2 ignorados**.
- `cargo build --offline --locked --quiet`: passou.
- `crystalline-lint .`: passou com zero violações.
- `git diff --check`: passou.

A extração é interna: preserva assinaturas públicas, defaults e fase do pipeline. Não foi feito benchmark nem afirmado ganho de tempo ou memória.

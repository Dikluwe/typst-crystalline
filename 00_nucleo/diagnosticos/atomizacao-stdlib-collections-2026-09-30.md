# Atomização comportamental de `stdlib/collections.rs` — 2026-09-30

## Proveniência

Medições na working tree não commitada sobre HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O estado anterior está em `atomizacao-stdlib-collections-2026-09-30-before.json.gz`; o estado final, com `git status`, `git diff HEAD --stat` e SHA-256 dos owners, está em `atomizacao-stdlib-collections-2026-09-30-after.json.gz`. Ambos identificam árvores não commitadas, não commits novos.

## Inventário

A medição anterior registra 3542 linhas em `collections.rs`. O hub final contém 1495 linhas, sobretudo despacho exaustivo, catálogo estático, adaptadores de chamadas e testes. Foram movidas 93 funções para 11 módulos com Prompt L0 exclusivo. O despacho continua estático e os corpos dos algoritmos foram preservados, sem intenção de alterar a linguagem.

| Owner L1 | Comportamento |
|---|---|
| `collections/validation.rs` | casts e validação de argumentos compartilhada |
| `collections/bytes.rs` | len, at e slice de Bytes |
| `collections/array_access.rs` | acesso e limites de Array |
| `collections/array_aggregate.rs` | sum, product, fold e reduce |
| `collections/array_predicate.rs` | predicados, callbacks, dedup e sorted |
| `collections/array_sequence.rs` | transformações e partições de sequência |
| `collections/dictionary.rs` | métodos de Dictionary |
| `collections/str_access.rs` | índices, escalares e grafemas de String |
| `collections/str_search.rs` | busca literal/regex e capturas |
| `collections/str_transform.rs` | normalização e transformações de String |
| `collections/arguments.rs` | sequência causal de Arguments |

`collections.md` permanece owner exclusivo de `collections.rs` e referencia os 11 novos owners. A superfície pública, as assinaturas e os defaults não foram alterados. Os testes históricos continuam no hub.

## Verificação

- Antes da extração: `cargo test -p typst-core --lib --offline --locked --quiet` — 6401 passaram, 2 ignorados (`/tmp/collections-baseline.log`).
- Depois do resselo: mesmo comando — 6401 passaram, 2 ignorados (`/tmp/collections-final-test.log`).
- `cargo build --offline --locked` — exit 0 (`/tmp/collections-final-build.log`).
- `crystalline-lint .` — exit 0 (`/tmp/collections-final-lint.log`).
- `git diff --check` — exit 0.

A suíte verifica ausência de regressões no comportamento coberto. Isto não é benchmark nem uma nova medição bilateral com o vanilla pinado.

# Atomização do restante de `compiler/stdlib/structural/math.rs` — 2026-09-30

## Proveniência

Medição final na working tree não commitada sobre o HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`. O estado completo dos ficheiros alterados, `git diff HEAD --stat` e SHA-256 dos owners deste recorte está em `atomizacao-stdlib-math-restante-2026-09-30-after.json.gz`. O estado anterior à extração está em `atomizacao-stdlib-math-restante-2026-09-30-before.json.gz`. Os dois snapshots identificam estados não commitados, não commits novos.

## Medição estrutural

O `math.rs` atual tem 1463 linhas e uma única função produtiva, `make_math_module` (`math.rs:63`). O seu corpo continua a montar o catálogo estático. As 48 funções produtivas que antes coexistiam no ficheiro foram deslocadas para 16 owners comportamentais, preservando os corpos e o despacho estático. Não se mediu ganho de tempo de execução; o objetivo deste recorte é isolar alternativas algorítmicas para pesquisa.

| Owner L1 | Funções | Comportamento isolado |
|---|---:|---|
| `math/casts.rs` | 4 | casts e conversões compartilhadas |
| `math/underline.rs` | 1 | sublinhado |
| `math/lr.rs` | 2 | delimitadores esquerda/direita |
| `math/mid.rs` | 2 | delimitador central |
| `math/annotations.rs` | 16 | oito famílias de anotações com validação comum |
| `math/primes.rs` | 1 | primos |
| `math/overline.rs` | 2 | linha superior |
| `math/norm.rs` | 2 | norma |
| `math/round.rs` | 2 | arredondamento |
| `math/abs.rs` | 2 | valor absoluto |
| `math/stretch.rs` | 2 | esticamento |
| `math/attach.rs` | 4 | slots de scripts e presença triestatal |
| `math/binom.rs` | 3 | binomial |
| `math/underover.rs` | 1 | posições sob/sobre |
| `math/op.rs` | 2 | operador matemático e valor do catálogo |
| `math/limits.rs` | 2 | limites e scripts |

Cada owner tem um Prompt L0 exclusivo em `00_nucleo/prompts/compiler/stdlib/structural/math/`. O núcleo `math-attach-slot-presence.toml` passou a ser pinado pelo owner `attach.md`; o parent `math.md` retém somente o binding estático. `structural/mod.rs` reexporta diretamente os owners. Os testes históricos continuam em `math.rs` e nos módulos de teste existentes, sem alterar o comportamento observado.

## Verificação

- Antes da extração: `cargo test -p typst-core --lib --offline --locked --quiet` — 6401 passaram, 2 ignorados; `/tmp/math-restante-baseline.log`.
- Depois do resselo: mesmo comando — 6401 passaram, 2 ignorados; `/tmp/math-restante-final-test.log`.
- `cargo build --offline --locked` — exit 0; `/tmp/math-restante-build.log`.
- `crystalline-lint .` — exit 0, sem violações; `/tmp/math-restante-lint.log`.
- `git diff --check` — exit 0.

Estas verificações mostram preservação do comportamento coberto pela suíte existente e validade da linhagem. Não constituem benchmark nem nova medição bilateral contra o vanilla pinado.

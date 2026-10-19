# Atomização da família `math.accent` no stdlib estrutural

## Proveniência e medição

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado anterior, com `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos quatro arquivos inspecionados e hora UTC, está em `atomizacao-stdlib-math-accent-2026-09-30-before.json.gz` (2026-09-30 17:07:48.445184 UTC). O estado final, com o mesmo inventário e SHA-256 dos seis arquivos L0/L1 afetados, está em `atomizacao-stdlib-math-accent-2026-09-30-final.json.gz`. O filtro `cargo test -p typst-core p1429 --lib --offline --locked --quiet` passou 9 testes antes do movimento.

## Mudança

`01_core/src/compiler/stdlib/structural/math/accent.rs` agora contém a tabela de aliases de combining marks, a normalização, casts de base/accent/size/dotless, construção de `Content::MathAccent` e a nativa. `math.rs` conserva o registro estático do binding. `structural/mod.rs` reexporta diretamente do filho `math_accent_content` e `native_accent` com os mesmos nomes e visibilidades para eval e registries. O Prompt L0 novo é owner exclusivo do consumer; os L0 do pai e do hub foram atualizados antes do código. Os três pares foram ressellados individualmente, com hashes em `atomizacao-stdlib-math-accent-2026-09-30-seals.json`.

Uma regressão local cobre a convergência de alias e combining codepoint direto. Não houve alteração de contrato público, defaults, fase, tabela de aliases, spans, diagnósticos ou dispatch.

## Verificação

No estado final inventariado, o filtro P1429 passou 9 testes; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6396 e ignorou 2; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `git diff --check` e `rustfmt --check` passaram nos arquivos afetados. Avisos não bloqueantes da árvore permanecem. O recorte permite pesquisar normalização e casts de accent em um único owner; nenhum ganho de CPU foi medido.

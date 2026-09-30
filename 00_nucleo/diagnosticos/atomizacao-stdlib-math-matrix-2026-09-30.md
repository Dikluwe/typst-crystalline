# Atomização da família `math.mat` no stdlib estrutural

## Proveniência e medição

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado antes da intervenção, com `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos quatro arquivos L0/L1 inspecionados e hora UTC, está em `atomizacao-stdlib-math-matrix-2026-09-30-before.json.gz` (2026-09-30 16:32:54.954634 UTC). O estado final com o mesmo inventário e hashes dos seis arquivos afetados está em `atomizacao-stdlib-math-matrix-2026-09-30-final.json.gz`. O filtro `cargo test -p typst-core p1431 --lib --offline --locked --quiet` passou 29 testes antes do movimento.

## Mudança

`01_core/src/compiler/stdlib/structural/math/matrix.rs` agora reúne `MathMatrixOptions`, casts de comprimentos relativos e `augment`, aplicação de opções nomeadas, interpretação das linhas, preenchimento retangular e construção de `Content::MathMatrix`. `math.rs` conserva apenas o registro estático do binding `math.mat`. `structural/mod.rs` reexporta diretamente do filho os três bridges já consumidos por eval/layout, preservando os nomes no stdlib. O novo módulo tem Prompt L0 exclusivo; os L0 do pai e do hub foram atualizados antes do código. Os três pares foram ressellados individualmente, com hashes em `atomizacao-stdlib-math-matrix-2026-09-30-seals.json`.

O movimento preserva ordem, defaults, spans, diagnósticos, bits explícitos e identidade semântica da matriz. Uma regressão local exercita o preenchimento de linha curta e a precedência de `row-gap` sobre `gap`. Não há alteração de fase, contrato público ou dispatch dinâmico.

## Verificação

No estado final inventariado, o filtro P1431 passou 29 testes; a regressão local passou; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6394 testes e ignorou 2; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `git diff --check` passou nos arquivos rastreados afetados e `rustfmt --check` passou no novo módulo e no hub estrutural. O lint mantém avisos não bloqueantes presentes na árvore. O recorte facilita comparar alternativas de parsing de linhas e aplicação de opções; nenhum ganho de CPU foi medido.

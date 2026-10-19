# Atomização da família `math.cases` no stdlib estrutural

## Proveniência e medição

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado anterior, incluindo `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 dos seis arquivos inspecionados e hora UTC, está em `atomizacao-stdlib-math-cases-2026-09-30-before.json.gz` (2026-09-30 16:53:35.299546 UTC). O estado final, com o mesmo inventário e SHA-256 dos oito arquivos L0/L1 afetados, está em `atomizacao-stdlib-math-cases-2026-09-30-final.json.gz`. O filtro `cargo test -p typst-core p1430 --lib --offline --locked --quiet` passou 22 testes antes do movimento.

## Mudança

`01_core/src/compiler/stdlib/structural/math/cases.rs` agora contém opções, cast e pareamento de delimitadores, conversão dos filhos e construção de `Content::MathCases`. `math.rs` conserva o registro estático do binding `math.cases`. `math/matrix.rs` chama diretamente o conversor de delimitadores desse novo owner, sem tabela duplicada. `structural/mod.rs` continua a expor os mesmos três bridges de cases a eval/layout, agora diretamente do filho. Os quatro Prompts L0 foram atualizados antes do código; cada consumer tem owner exclusivo e seus hashes estão em `atomizacao-stdlib-math-cases-2026-09-30-seals.json`.

A regressão local verifica que um delimitador de fechamento mantém seu lado e infere o parceiro de abertura. Não houve alteração de assinatura pública, fase, defaults, mensagens, spans ou dispatch.

## Verificação

No estado final inventariado, o filtro P1430 passou 22 testes, o filtro P1431 do consumidor matrix passou 29, e `cargo test -p typst-core --lib --offline --locked --quiet` passou 6395 com 2 ignorados. `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `git diff --check` e `rustfmt --check` passaram nos arquivos afetados. Avisos não bloqueantes existentes permanecem. O recorte facilita estudar alternativas de conversão de delimitadores e construção de cases; nenhum ganho de CPU foi medido.

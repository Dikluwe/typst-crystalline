# Atomização da família `math.vec` no stdlib estrutural

## Proveniência e medição

HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado. O estado anterior com `git status --porcelain=v1 --untracked-files=all`, `git diff HEAD --stat`, SHA-256 do L0 e consumer inspecionados e hora UTC está em `atomizacao-stdlib-math-vector-2026-09-30-before.json.gz` (2026-09-30 16:22:35.485875 UTC). O estado final com o mesmo inventário e hashes dos quatro arquivos L0/L1 afetados está em `atomizacao-stdlib-math-vector-2026-09-30-final.json.gz`. O filtro `cargo test -p typst-core math_vec --lib --offline --locked --quiet` passou 3 testes antes do movimento.

## Mudança

`01_core/src/compiler/stdlib/structural/math/vector.rs` agora contém `MathVecOptions`, conversão de delimitadores, cast de `align`/`gap`, construção de `Content::MathVec`, nativa `math.vec` e seu teste de razão percentual. O pai `math.rs` conserva somente o registro estático do binding `math.vec` e suas outras famílias. O Prompt L0 `math/vector.md` é o owner exclusivo do novo consumer; `math.md` cedeu o contrato detalhado de `vec` e mantém o catálogo. Os dois pares L0–consumer foram ressellados individualmente, com hashes em `atomizacao-stdlib-math-vector-2026-09-30-seals.json`.

O recorte não altera assinatura pública, ordem de argumentos, spans, diagnósticos, bits de presença explícita, default nem fase de pipeline. O algoritmo de layout de vetores permanece em seu próprio owner; esta etapa organiza apenas construção e casts de linguagem.

## Verificação

No estado final inventariado, o filtro `math_vec` passou 3 testes; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6393 testes e ignorou 2; `cargo build --offline --locked` concluiu; `crystalline-lint .` retornou código 0; `git diff --check` passou nos arquivos rastreados afetados e `rustfmt --check` passou no novo arquivo. O lint mantém avisos não bloqueantes presentes na árvore. Esta extração facilita localizar e comparar alternativas de cast/construção para vetores; nenhum ganho de CPU foi medido.

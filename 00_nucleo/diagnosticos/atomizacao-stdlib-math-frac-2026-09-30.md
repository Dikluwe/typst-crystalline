# Atomização de `math.frac` — 2026-09-30

## Medição anterior à decisão

[`before.json.gz`](atomizacao-stdlib-math-frac-2026-09-30-before.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, o working tree não commitado completo, diff stat e SHA-256 dos arquivos afetados. Em `math.rs:291-391` estavam o cast de estilo, cast dos operandos, validação, constructor e nativa de `math.frac`; o registro estático também estava no parent. O filtro P1427 passou 15 testes antes do movimento. Isto é medição estrutural e de regressão, não benchmark nem nova sonda bilateral do vanilla.

## Decisão e execução

O L0 filho `compiler/stdlib/structural/math/frac.md` foi criado antes do consumer `math/frac.rs`. Ele passou a ser o owner exclusivo de casts, diagnósticos, ordem de validação e construção de `Content::MathFrac`. O L0 parent foi atualizado e `math.rs` reteve o binding estático, agora apontando para `frac::native_math_frac`. A assinatura nativa, os argumentos, as flags semânticas e os diagnósticos foram preservados; não foi introduzido despacho dinâmico.

O teste local para `style: "vertical"` explícito versus omitido foi executado RED contra o stub e GREEN após o movimento. Os testes P1427 externos permaneceram no owner próprio de testes.

## Verificação e proveniência

[`after.json.gz`](atomizacao-stdlib-math-frac-2026-09-30-after.json.gz) registra o mesmo HEAD, estado completo do working tree, diff stat e SHA-256 após a mudança. Nesse estado, o filtro P1427 passou 15 testes; o teste local passou; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6.398 testes, com 2 ignorados; `cargo build --offline --locked` passou; `crystalline-lint .` saiu com código 0. Os logs da sessão estão em `/tmp/math-frac-*.log`.

Esta separação melhora a localidade do comportamento para pesquisa de algoritmos alternativos. Não há alegação de ganho de desempenho.

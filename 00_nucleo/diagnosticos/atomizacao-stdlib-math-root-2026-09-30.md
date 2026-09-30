# Atomização de `math.sqrt` e `math.root` — 2026-09-30

## Medição anterior à decisão

[`before.json.gz`](atomizacao-stdlib-math-root-2026-09-30-before.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, estado completo do working tree não commitado, diff stat e SHA-256 dos arquivos afetados. `sqrt_content` estava em `math.rs:46-48`, as duas nativas em `math.rs:216-289`, os registros no parent e o bridge `sqrt_content` no hub. O filtro P1426 passou 12 testes antes do movimento. Isto mede estrutura e regressão local; não mede tempo de execução nem nova paridade bilateral com vanilla.

## Decisão e execução

O L0 filho `compiler/stdlib/structural/math/root.md` foi redigido antes do código e passou a ser owner exclusivo de `math/root.rs`. `sqrt_content`, `native_math_sqrt` e `native_math_root` foram transferidos com os mesmos casts, ordem de validação, diagnósticos e carriers. `math.rs` retém os bindings estáticos; o hub reexporta `sqrt_content` diretamente do filho. Nenhum despacho dinâmico foi introduzido.

O teste local distingue índice ausente em `sqrt` de índice `none` explicitamente presente em `root`; falhou RED contra o stub e passou GREEN depois do movimento.

## Verificação e proveniência

[`after.json.gz`](atomizacao-stdlib-math-root-2026-09-30-after.json.gz) registra o mesmo HEAD, estado completo do working tree, diff stat e SHA-256 após a mudança. Neste estado, o filtro P1426 passou 12 testes, o teste local passou, `cargo test -p typst-core --lib --offline --locked --quiet` passou 6.399 testes com 2 ignorados, `cargo build --offline --locked` passou, `crystalline-lint .` saiu com código 0 e `git diff --check` não encontrou whitespace inválido. O novo `root.rs` recebeu somente um aviso informativo V20 por um padrão do corpo movido. Os logs temporários estão em `/tmp/math-root-*.log` nesta sessão.

A separação concentra o comportamento de raiz para pesquisa de algoritmos alternativos. Não há alegação de ganho de desempenho.

# Atomização de `math.equation` — 2026-09-30

## Medição anterior à decisão

[`before.json.gz`](atomizacao-stdlib-math-equation-2026-09-30-before.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, estado completo do working tree não commitado, diff stat e SHA-256 dos arquivos afetados. `math.rs:46-210` continha constructor, casts de opções e nativa; o hub reexportava os quatro helpers via parent. O filtro P1140 passou 85 testes antes da mudança. Isto mede estrutura e regressão local; não mede tempo de execução nem nova paridade bilateral com vanilla.

## Decisão e execução

O L0 filho `compiler/stdlib/structural/math/equation.md` foi redigido antes do consumer `math/equation.rs` e tornou-se seu owner exclusivo. Foram transferidos `equation_content`, os casts de `number-align`, `supplement` e `alt`, e `native_math_equation`. O parent mantém o binding estático; o hub reexporta os quatro helpers diretamente do filho. Ordem de validação, mensagens, style chain, assinaturas e visibilidade foram preservadas, sem despacho dinâmico.

O teste local distingue string convertida para conteúdo em `supplement` de conteúdo rejeitado por `alt`. Falhou RED contra o stub e passou GREEN após o movimento.

## Verificação e proveniência

[`after.json.gz`](atomizacao-stdlib-math-equation-2026-09-30-after.json.gz) registra o mesmo HEAD, estado completo do working tree, diff stat e SHA-256 após a mudança. Nesse estado, o filtro P1140 passou 85 testes; o teste local passou; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6.400 testes, com 2 ignorados; `cargo build --offline --locked` passou; `crystalline-lint .` saiu com código 0; `git diff --check` não encontrou whitespace inválido. O novo arquivo só recebeu avisos informativos V19/V20 por padrões já presentes no corpo movido. Logs temporários: `/tmp/math-equation-*.log`.

A separação concentra o comportamento para pesquisa de algoritmos alternativos. Não há alegação de ganho de desempenho.

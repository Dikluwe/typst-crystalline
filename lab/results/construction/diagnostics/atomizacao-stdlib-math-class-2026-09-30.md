# Atomização de `math.class` — 2026-09-30

## Medição anterior à decisão

[`before.json.gz`](atomizacao-stdlib-math-class-2026-09-30-before.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree não commitado completo, diff stat e SHA-256 dos arquivos afetados. `native_math_class` estava em `math.rs:1069-1152`, o parent registrava o binding e o hub reexportava a nativa via parent. O helper `vanilla_type_name_class` era compartilhado por outras nativas. Antes do movimento, os filtros `math_class` e P825 passaram 29 e 15 testes. Isto é medição estrutural e de regressão, não benchmark nem nova sonda bilateral com vanilla.

## Decisão e execução

O L0 filho `compiler/stdlib/structural/math/class.md` foi redigido antes do consumer `math/class.rs` e tornou-se seu owner exclusivo. O cast público de dez classes, validação do body, diagnósticos e carrier foram transferidos; `vanilla_type_name_class` ficou no parent como helper compartilhado. O parent mantém o binding estático e o hub reexporta diretamente do filho, com o mesmo caminho público. Não foi adicionado despacho dinâmico.

O teste local verifica que uma classe interna (`alphabetic`) é rejeitada pelo cast público. Falhou RED contra o stub e passou GREEN após o movimento.

## Verificação e proveniência

[`after.json.gz`](atomizacao-stdlib-math-class-2026-09-30-after.json.gz) registra o mesmo HEAD, estado completo do working tree, diff stat e SHA-256 após a mudança. Nesse estado, os filtros `math_class` e P825 passaram 29 e 15 testes; o teste local passou; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6.401 testes, com 2 ignorados; `cargo build --offline --locked` passou; `crystalline-lint .` saiu com código 0; `git diff --check` não encontrou whitespace inválido. O novo arquivo recebeu apenas avisos informativos V20 de padrões existentes. Logs temporários: `/tmp/math-class-*.log`.

A separação concentra o comportamento para pesquisar algoritmos alternativos. Não há alegação de ganho de desempenho.

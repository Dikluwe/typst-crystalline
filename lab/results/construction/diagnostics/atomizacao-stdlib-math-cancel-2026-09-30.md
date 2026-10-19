# Atomização de `math.cancel` — 2026-09-30

## Medição anterior à decisão

A captura [`before.json.gz`](atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, a lista completa dos arquivos alterados, o diff stat e os SHA-256 dos arquivos afetados no working tree não commitado. Em `math.rs:395-529`, `native_cancel` continha casts, defaults, bits de presença morfológica e construção do carrier. O hub reexportava a nativa via parent. O filtro `native_cancel` passou 4 testes antes do movimento. Isto mede estrutura e regressão local; não mede tempo de execução nem nova paridade bilateral com o vanilla.

## Decisão e execução

O L0 filho `compiler/stdlib/structural/math/cancel.md` passou a ser dono exclusivo da implementação em `math/cancel.rs`. O L0 parent e o L0 do hub foram atualizados primeiro. A função foi transferida preservando sua assinatura, corpo de validação, ordem dos named, mensagens e construção de `MathCancelElem`. O parent mantém o binding estático `math.cancel`; o hub reexporta diretamente o filho. Nenhum despacho dinâmico foi adicionado.

Um teste local de `cross: false` foi executado RED contra o stub e GREEN após a extração, verificando que a presença explícita permanece distinta do valor default.

## Verificação e proveniência

A captura [`after.json.gz`](atomizacao-stdlib-math-cancel-2026-09-30-after.json.gz) registra HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, estado completo do working tree, diff stat e SHA-256 após a mudança. Com esse working tree não commitado: o filtro `native_cancel` passou 4 testes; o teste local passou; `cargo test -p typst-core --lib --offline --locked --quiet` passou 6.397 testes, com 2 ignorados; `cargo build --offline --locked` passou. `git diff --check` não apontou whitespace inválido. Os logs temporários estão em `/tmp/math-cancel-*.log` durante esta sessão. `crystalline-lint .` terminou com código 0. Os únicos achados no novo `cancel.rs` foram dois avisos informativos V20 sobre padrões aninhados já presentes no corpo movido; não houve violação bloqueante de linhagem, ownership ou núcleo.

Esta refatoração facilita procurar algoritmos alternativos de `math.cancel` no arquivo da feature. Não há alegação de ganho de desempenho.

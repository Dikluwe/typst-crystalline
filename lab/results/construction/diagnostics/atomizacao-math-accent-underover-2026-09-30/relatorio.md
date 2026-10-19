# Atomização interna de accent e underover

Em 2026-09-30, HEAD `17f13b1df79b87ea065ee62e78164b18e477da3d`, working tree compartilhada não commitada (arquivos em `working-tree-status.txt` e `working-tree-stat.txt`). L0 de `accent.md` e `underover.md` foi atualizado antes do código. `scoped-seals.json` documenta os hashes de cada par L0↔consumer, sem resselo global.

`accent.rs` agora separa a resolução de size/dotless da colocação da marca e aplica a identidade textual em uma função privada após a composição visual. `underover.rs` separa a classificação de suportes, o estilo das anotações, a leitura de attachment e o envelope textual dos glifos de suporte. Ambos permanecem nos seus owners existentes; não foram criados contratos públicos ou novos modos.

A regressão `cargo test -q -p typst-core compiler::math::layout --lib` passou com 373/373 testes. `cargo build -q`, `crystalline-lint .` (zero violations) e `git diff --check` passaram. Após o build, `comparison.json` comparou os PDFs novos dos quatro fixtures diretamente relacionados (`sec_09`, `sec_10`, `sec_12`, `sec_22`) com os PDFs Tekt da matriz integrada anterior: em todos os quatro, `pdftotext -raw`, `pdftotext -layout`, dimensões e raster a 120 dpi permaneceram iguais. Os hashes dos arquivos PDF diferem, mas igualdade de bytes não é requisito de paridade da linguagem.

Comparação: 2026-09-30T14:09:31.849857+00:00, binário Tekt SHA-256 `89d2b89ef715ab52ec349e0cf96a68fb9f9012232cdd37b8b8ee3cdac82fbde4`. `comparison.json` registra hashes das entradas, PDFs novos e anteriores e cada veredito.

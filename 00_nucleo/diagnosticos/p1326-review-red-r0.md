# P1326 — RED inicial e objeção de controle

Manifesto `800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`.
Recibo `p1326-unit-red.json` SHA-256
`baea1f72f4458289621a3628c8d5231a226146c8a9c316ac607f2129dded439a`:
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado,
inventários before/after idênticos, diff/stat completos no recibo; execução
`2026-09-09T01:30:02.816809+00:00`–`01:31:55.946577+00:00`.

Comando `cargo test --release --locked -p typst-core
compiler::eval::bindings::field_access:: -- --nocapture` compilou com sucesso,
executou testes e saiu 101. Resultado 14 passaram / 4 falharam. Três falhas
são RED real de mensagem Closure prevista no L0, nos dois sucessores legados
e no novo teste público AST. Não são erro de compilação ou harness.

A quarta falha é controle positivo JSON com expectativa de formato compactado
incompatível com o baseline. Não pode ser aceita como RED do fragmento nem
motivar modificação de produto. Autor independente notificado para medir o
controle contra o vanilla, conservar o snippet r0 e fornecer sucessor test-only
restrito antes de C. Necessário re-freeze e focal RED após a correção.

Hashes iniciais de snippets verificados: novos testes
`38bd68a1cd8d32f321420cda78e8aa5d22a262c9482feae5be037cc13d7c4067`;
sucessor legado `765e4e40f8203c33d42aa267b6ff763ae38d871337033a3183636c2bf4d87ee5`.
Diff do legado contém somente as duas sucessões Closure autorizadas e newline
terminal; Element/Plugin e asserções restantes intactos.

Gate de entrada de C temporariamente pendente da revisão do controle; regime
A/B sem atestação de isolamento. Não há selo nem declaração de paridade geral.

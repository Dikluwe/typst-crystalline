# Atomização de repr — 2026-09-30

## Proveniência

Base: `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb`, árvore limpa. Os estados completos, SHA-256, horário e contagem por arquivo estão em `atomizacao-repr-2026-09-30-before.json.gz` e `atomizacao-repr-2026-09-30-after.json.gz`. O estado final foi medido antes do commit; o manifesto identifica cada arquivo.

## Medição e decisão

Na base, `01_core/src/compiler/eval/repr.rs` tinha 2.760 linhas. O código de produção concentrava Value, medidas, pintura, matemática, Content, Selector e formatação de listas. Foram criados sete módulos L1 com sete Prompts L0 proprietários. A fachada pública e os testes históricos permanecem no pai, agora com 1174 linhas, majoritariamente testes. O dispatcher de Content continua exaustivo e estático, com os helpers delegados por função.

As assinaturas públicas de `repr_value`, `repr_content`, `repr_selector` e a visibilidade de `repr_stroke` foram preservadas. O recorte não altera morfologia, ordem de campos, arredondamento, truncamento de arrays nem saída de seletores.

## Verificação

Baseline: `cargo test -p typst-core --lib --offline --locked --quiet` — 6.413 testes passaram, 2 ignorados. Depois: 6.420 passaram, 2 ignorados, 0 falhas; sete novos testes cobrem as unidades extraídas. `cargo build --offline --locked --quiet` passou. `crystalline-lint .` não emitiu erros. `git diff --check` passou.

A separação facilita investigar algoritmos de representação por comportamento. Não mede tempo/memória nem comprova nova paridade bilateral contra o vanilla pinado.

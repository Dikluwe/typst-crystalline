# L0 — compiler/eval/math/native_attachments
Hash do Código: 181f8045

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/native_attachments.rs`. O comportamento original estava em `01_core/src/compiler/eval/math/native_dispatch.rs:183-299` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Despacha as chamadas matemáticas diretas de scripts, limits e attach. Preserva a ordem de validação dos argumentos nomeados, a sintaxe booleana, o bit inline explícito, a morfologia dos anexos, os spans e a delegação ao construtor. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

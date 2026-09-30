# L0 — compiler/eval/math/native_dispatch
Hash do Código: 3a801d86

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/native_dispatch.rs`. A lógica original estava em `01_core/src/compiler/eval/math/expression_call.rs:181-766` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own exhaustive static dispatch for recognized bare math function names. Preserve constructor validation, argument order, span and morphology. Delegate unrecognized/global/symbol behavior to call_fallback without changing precedence. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

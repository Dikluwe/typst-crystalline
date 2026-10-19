# L0 — compiler/eval/math/call_fallback
Hash do Código: a489b966

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/call_fallback.rs`. A lógica original estava em `01_core/src/compiler/eval/math/expression_call.rs:650-766` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own unknown/global/symbol mathematical call fallback. Preserve lexical/global function lookup, symbol deprecation warning, delimited argument morphology and error priority. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

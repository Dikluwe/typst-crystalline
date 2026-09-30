# L0 — compiler/introspect/from_tags/state_updates
Hash do Código: 00e9f59f

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/from_tags/state_updates.rs`. A lógica original estava em `01_core/src/compiler/introspect/from_tags.rs:108-155` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own post-walk StateUpdate::Func replay. Preserve location order, closure capture, contextual evaluation, uninitialized-value behavior and callback diagnostics. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

# L0 — compiler/introspect/from_tags/counter_updates
Hash do Código: c0cb6dd1

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/from_tags/counter_updates.rs`. A lógica original estava em `01_core/src/compiler/introspect/from_tags.rs:34-103,157-287` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own filtered counter replay, ordinary counter functions and bundle Page replay. Preserve causal order, key equality, source position, callback inputs/results, diagnostic priority and event history. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

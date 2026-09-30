# L0 — compiler/introspect/from_tags/displays
Hash do Código: c5745f60

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/introspect/from_tags/displays.rs`. A lógica original estava em `01_core/src/compiler/introspect/from_tags.rs:288-461` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own state and counter display realization after update passes. Preserve callback argument shape, defensive conversion/error behavior and typed location-indexed storage. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

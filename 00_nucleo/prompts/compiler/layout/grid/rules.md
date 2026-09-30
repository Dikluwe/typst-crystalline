# L0 — compiler/layout/grid/rules
Hash do Código: 71e15208

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/rules.rs`. A lógica original estava em `01_core/src/compiler/layout/grid.rs:1175-1250` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Emit explicit horizontal and vertical grid/table rules over the rendered cells using actual emitted row starts and resolved tracks. Preserve stroke-none suppression, clipping, positions, dimensions, ordering and pagination geometry. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

# L0 — compiler/layout/grid/row_groups
Hash do Código: bea50653

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/row_groups.rs`. A lógica original estava em `01_core/src/compiler/layout/grid.rs:185-282` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Normalize Grid/Table header and footer groups into whole rows, detect explicit body-cell conflicts with header rows, preserve diagnostic and hint, and assemble cells in header/body/footer order. Return the conflict through the existing caller error channel without altering cell_align restoration. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

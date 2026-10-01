# L0 — compiler/layout/grid/tracks
Hash do Código: 74ba0d73

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/tracks.rs`. Comportamento original em `01_core/src/compiler/layout/grid.rs:212-451` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own Grid/Table column and row track resolution. Keep Fixed/Auto/Fraction order, constrained cell measurement, insets, page break before fractional row allocation, auto-height finite fallback, and current cursor/style effects. Return widths, starts and row heights without changing placement or emission. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.

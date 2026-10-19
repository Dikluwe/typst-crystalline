# L0 — compiler/math/layout/grid_composition
Hash do Código: fc4a512b

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/grid_composition.rs`. Comportamento original em `01_core/src/compiler/math/layout/mod.rs:1835-2153` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own two-pass math-grid measurement and placement, row and column gaps, alignment-boundary spacing, baseline sequence, synthetic-parenthesis floor and item order. Preserve matrix/cases/multiline callers and static dispatch. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.

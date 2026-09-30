# L0 — compiler/layout/grid/borders
Hash do Código: b9e19479

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/borders.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/layout/grid.rs:1262-1499` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own cell bounds and effective strokes, horizontal and vertical segment emission, open vertical segment flushing, and row-border coalescing. Preserve stroke priority, geometry, item ordering and pagination boundaries. Do not perform placement or cell layout. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

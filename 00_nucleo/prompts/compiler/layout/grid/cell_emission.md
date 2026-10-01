# L0 — compiler/layout/grid/cell_emission
Hash do Código: 37c8d9f9

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/layout/grid/cell_emission.rs`. O comportamento original estava em `01_core/src/compiler/layout/grid.rs:276-882` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Emite as células posicionadas de Grid/Table linha a linha, incluindo quebras de página, alinhamento e inset herdados, subframes, caudas de overflow, ordem de fill/stroke e descarregamento dos segmentos de borda. Devolve os inícios das linhas emitidas para as regras visuais explícitas. Preserva o restauro do estado do Layouter, a ordem dos itens, o ajuste dos caminhos e o suporte a regiões limitadas. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

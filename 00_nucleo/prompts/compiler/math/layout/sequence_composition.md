# L0 — compiler/math/layout/sequence_composition
Hash do Código: 7f17f62f

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/sequence_composition.rs`. O comportamento original estava em `01_core/src/compiler/math/layout/mod.rs:1729-1832,1834-1906` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Prepara sequências lineares de math, resolve espaços fracos e gaps de classe e concatena MathBox horizontalmente. Preserva a seleção do ramo de grelha, a ordem dos filhos, a translação dos itens relativa à baseline e o ajuste de texto. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

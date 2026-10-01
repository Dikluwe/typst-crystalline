# L0 — compiler/math/layout/external_bridge
Hash do Código: e69888f1

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/external_bridge.rs`. O comportamento original estava em `01_core/src/compiler/math/layout/mod.rs:1325-1580` no HEAD `6fc546566`, árvore limpa, 2026-10-01 01:04:22 UTC. Esta é extração interna, não nova medição de paridade ou desempenho.

## Contrato

Dispõe conteúdo externo dentro de math e percorre recursivamente a tinta vertical. Preserva as métricas injetadas, a cadeia local de estilos, a âncora de baseline, os itens de topo achatados, o fallback textual quando o frame está vazio e a travessia de Group/Link. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

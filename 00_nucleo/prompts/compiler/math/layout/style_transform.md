# L0 — compiler/math/layout/style_transform
Hash do Código: d9c07801

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/math/layout/style_transform.rs`. A lógica original estava em `01_core/src/compiler/math/layout/mod.rs:2263-2732` no HEAD `4e2a321ce`, árvore limpa, 2026-09-30 23:49:20 UTC. Trata-se de extração interna; a leitura não é prova nova de paridade nem medição de desempenho.

## Contrato

Own recursive application of default mathematical italics and explicit glyph styles. Preserve MathStyled axis precedence, size wrappers, attachment slot presence, content morphology and immutable input. This unit performs no geometric layout or callback execution. Não alterar contratos públicos, fase do pipeline, defaults nem compatibilidade. Preservar despacho estático e pureza L1.

## Aceitação

Testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho.

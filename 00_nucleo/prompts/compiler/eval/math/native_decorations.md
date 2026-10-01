# L0 — compiler/eval/math/native_decorations
Hash do Código: 9fd42d0c

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/native_decorations.rs`. Comportamento original em `01_core/src/compiler/eval/math/native_dispatch.rs:338-452` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own the recognized over/under decoration and vec mathematical call family. Preserve name-specific constructor, argument order, style defaults, spans, output morphology and exact fallback boundary. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.

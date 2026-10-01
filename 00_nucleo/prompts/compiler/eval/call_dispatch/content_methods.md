# L0 — compiler/eval/call_dispatch/content_methods
Hash do Código: 879a6053

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/content_methods.rs`. Comportamento original em `01_core/src/compiler/eval/call_dispatch/call.rs:323-421` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own Location, Content/LocatedContent and OutlineEntry method interception in the existing order. Preserve method gates, receiver and argument evaluation, query morphology, spans and fallthrough before field-callee errors. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.

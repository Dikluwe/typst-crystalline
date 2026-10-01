# L0 — compiler/eval/call_dispatch/collection_methods
Hash do Código: 436886ee

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/call_dispatch/collection_methods.rs`. Comportamento original em `01_core/src/compiler/eval/call_dispatch/call.rs:265-315` no HEAD `51a027f99`, árvore limpa, 2026-10-01 00:44:23 UTC. A classificação é extração interna, não nova prova bilateral de paridade ou desempenho.

## Contrato

Own collection method interception and Func.with partial application in the existing order. Preserve argument spans, single receiver/argument evaluation, trace handling and fallthrough. Preservar contrato público, fase do pipeline, defaults, compatibilidade e pureza L1.

## Aceitação

Suíte L1, `cargo build` e `crystalline-lint .` passam; a extração não implica ganho de desempenho.

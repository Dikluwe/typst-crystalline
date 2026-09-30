# L0 — compiler/eval/bindings/value_methods/selector
Hash do Código: 2ffa0fbc

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/value_methods/selector.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/bindings/value_methods.rs:1217-1335` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own Selector conversion and logical/contextual method adapters. Preserve ordered composition, argument evaluation, diagnostics and static dispatch. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

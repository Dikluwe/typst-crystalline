# L0 — compiler/eval/bindings/value_methods/state
Hash do Código: cff3d847

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/bindings/value_methods/state.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/bindings/value_methods.rs:42-176` and `:655-682` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own State method evaluation, at-location/label resolution and context observations. Preserve validation order, arguments, spans and contextual behavior. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

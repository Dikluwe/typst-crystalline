# L0 — compiler/eval/math/expression_call
Hash do Código: 7685b804

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/expression_call.rs`. Extração interna medida no HEAD `ce27af795` e árvore limpa em 2026-09-30 23:12:44 UTC. A fonte é o consumer pai desta unidade; a extração não é medição bilateral de paridade nem prova de desempenho.

A lógica extraída estava em `01_core/src/compiler/eval/math/expression.rs:138-913` no HEAD indicado. A classificação é extração interna de comportamento, não nova decisão de paridade.

## Contrato

Own the `Expr::FuncCall` arm in mathematical evaluation, including lexical shadowing, qualified callees, native math functions, fallback symbol calls and their errors. Preserve argument evaluation order, spans, content morphology and the static dispatch order defined by the parent math prompt. Não alterar contrato público, fase do pipeline, comportamento por defeito ou compatibilidade.

## Aceitação

Os testes L1, `cargo build` e `crystalline-lint .` passam. Não inferir ganho de desempenho da divisão estrutural.

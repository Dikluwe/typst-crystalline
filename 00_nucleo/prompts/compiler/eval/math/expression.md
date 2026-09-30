# L0 — compiler/eval/math/expression
Hash do Código: 9517fdb3

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/expression.rs`. No HEAD `e754ab364a221dbe344852178e84bd42b02c463e` e árvore limpa, `01_core/src/compiler/eval/math.rs:599-1554` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contrato

Avaliar o match exaustivo de expressões matemáticas mantendo a morfologia e a ordem de avaliação. Delegar o braço `Expr::FuncCall` ao owner `00_nucleo/prompts/compiler/eval/math/expression_call.md`, sem mudar a posição do braço. Preservar assinaturas, semântica da linguagem e diagnósticos de `00_nucleo/prompts/compiler/eval/math.md`; não alterar fase do pipeline, defaults ou compatibilidade. A extração é interna a L1, sem I/O nem estado global mutável.

## Aceitação

A suíte L1 e os testes de math passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

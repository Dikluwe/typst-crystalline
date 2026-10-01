# L0 — compiler/eval/expression_dispatch
Hash do Código: 21150b90

## Medição anterior à decisão

No HEAD f7195cdf6 (árvore limpa, 2026-09-30), o bloco está em `01_core/src/compiler/eval/mod.rs:1651-2237`. Esta observação estabelece o limite da extração; não mede desempenho ou nova paridade bilateral.

## Owner e contrato

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/expression_dispatch.rs`. Dispatcher exaustivo e estático de Expr e helpers de expressão; preserva ordem, escopos, spans e diagnósticos. Manter contrato público, comportamento por defeito, fase do pipeline, compatibilidade, pureza e despacho estático.

## Aceitação

A suíte L1, o build e crystalline-lint passam. A separação não constitui prova de ganho de desempenho.

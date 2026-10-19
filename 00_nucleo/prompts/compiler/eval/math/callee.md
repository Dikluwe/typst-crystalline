# L0 — compiler/eval/math/callee
Hash do Código: cdbfbf94

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/callee.rs`. No HEAD `e754ab364a221dbe344852178e84bd42b02c463e` e árvore limpa, `01_core/src/compiler/eval/math.rs:164-250` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contrato

Resolver callee de namespace matemático sem perder o alvo e sua identidade. Preservar assinaturas, semântica da linguagem e diagnósticos de `00_nucleo/prompts/compiler/eval/math.md`; não alterar fase do pipeline, defaults ou compatibilidade. A extração é interna a L1, sem I/O nem estado global mutável.

## Aceitação

A suíte L1 e os testes de math passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

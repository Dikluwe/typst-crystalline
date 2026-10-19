# L0 — compiler/eval/math/lookup
Hash do Código: a2fc5e51

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/eval/math/lookup.rs`. No HEAD `e754ab364a221dbe344852178e84bd42b02c463e` e árvore limpa, `01_core/src/compiler/eval/math.rs:35-125` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-eval-math-2026-09-30-before.json.gz`.

## Contrato

Resolver nomes matemáticos nos scopes, catálogo padrão e diagnósticos de identificadores desconhecidos. Preservar assinaturas, semântica da linguagem e diagnósticos de `00_nucleo/prompts/core/compiler/eval/math.md`; não alterar fase do pipeline, defaults ou compatibilidade. A extração é interna a L1, sem I/O nem estado global mutável.

## Aceitação

A suíte L1 e os testes de math passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

# L0 — compiler/stdlib/gradients/accessors
Hash do Código: 75f084e4

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/gradients/accessors.rs`. No HEAD `b2705f23fe5bfb53e623fd3b65f8b4ea50815560` e árvore limpa, `01_core/src/compiler/stdlib/gradients.rs:132-278` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-gradients-stdlib-2026-09-30-before.json.gz`.

## Contrato

Validar o receiver e os argumentos e consultar atributos de gradientes via wrappers nativos. Preservar as assinaturas, defaults, diagnósticos e semântica de `00_nucleo/prompts/core/compiler/stdlib/gradients.md`. A extração é interna a L1, com despacho estático e sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e testes de gradiente; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

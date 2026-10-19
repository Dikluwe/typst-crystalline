# L0 — compiler/stdlib/gradients/radial
Hash do Código: 7d3386f2

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/gradients/radial.rs`. No HEAD `b2705f23fe5bfb53e623fd3b65f8b4ea50815560` e árvore limpa, `01_core/src/compiler/stdlib/gradients.rs:803-951` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-gradients-stdlib-2026-09-30-before.json.gz`.

## Contrato

Construir gradiente radial com center, radius e focal. Preservar as assinaturas, defaults, diagnósticos e semântica de `00_nucleo/prompts/compiler/stdlib/gradients.md`. A extração é interna a L1, com despacho estático e sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e testes de gradiente; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

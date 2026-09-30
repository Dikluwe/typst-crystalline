# L0 — compiler/stdlib/gradients/parsing
Hash do Código: 6c17accb

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/gradients/parsing.rs`. No HEAD `b2705f23fe5bfb53e623fd3b65f8b4ea50815560` e árvore limpa, `01_core/src/compiler/stdlib/gradients.rs:606-802,952-970` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-gradients-stdlib-2026-09-30-before.json.gz`.

## Contrato

Interpretar argumentos comuns, stops, espaço de cor, relative e ratio. Preservar as assinaturas, defaults, diagnósticos e semântica de `00_nucleo/prompts/compiler/stdlib/gradients.md`. A extração é interna a L1, com despacho estático e sem I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e testes de gradiente; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

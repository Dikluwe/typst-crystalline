# L0 — entities/gradient/kind
Hash do Código: 07521c7c

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/entities/gradient/kind.rs`. No HEAD `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229` e árvore limpa, `01_core/src/entities/gradient.rs:912-1074` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.

## Contrato

Definir o enum fechado Gradient e os construtores, acessos e delegações comuns às variantes. Preservar contratos públicos, defaults, semântica de amostragem e os critérios de paridade em `00_nucleo/prompts/entities/gradient.md`. A extração permanece em L1, sem I/O nem estado global mutável.

## Aceitação

Os testes de gradiente e a suíte L1 passam; `crystalline-lint .` valida ownership e hashes. Não inferir ganho de desempenho.

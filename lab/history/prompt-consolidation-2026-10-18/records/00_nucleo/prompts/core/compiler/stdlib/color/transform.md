# L0 — stdlib/color/transform
Hash do Código: adb5e7d3

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/color/transform.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/color.rs:455-626` contém o comportamento extraído. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-color-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Implementar negate, saturate, desaturate e rotate, respeitando espaços válidos, hue, Spot e domínios de ângulo.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/core/compiler/stdlib/color.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de color, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

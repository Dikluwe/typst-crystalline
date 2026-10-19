# L0 — stdlib/calc/integer
Hash do Código: ca95c863

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/integer.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:1052-1293` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

Implementar even/odd, rem, rem-euclid, div-euclid, quo, gcd e lcm. Preservar operações inteiras checadas, sinais, divisão por zero, quociente floored e diferença entre resto comum e Euclidiano.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/core/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.

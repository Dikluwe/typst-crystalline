# L0 — stdlib/calc/abs
Hash do Código: d68856b5

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/calc/abs.rs`. No HEAD `d9e851a850c66905bdb0c3ea4b155a6f36881106` e árvore limpa, `01_core/src/compiler/stdlib/calc.rs:133-255` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-calc-2026-09-30-before.json.gz`. Esta decisão apenas separa algoritmos por comportamento.

## Contrato

Implementar calc.abs com prioridade causal de argumentos e spans. Int usa checked_abs, Float/Decimal conservam tipo e valores dimensionais respeitam suas condições de magnitude. Preservar erros e hints de tipos/posicionais.

Os detalhes de paridade, tipos, diagnósticos e bindings permanecem em `00_nucleo/prompts/compiler/stdlib/calc.md`. A extração não altera assinatura pública, função de módulo, fase, resultado ou política de erro. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de calc, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho desta separação.

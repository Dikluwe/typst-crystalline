# L0 — stdlib/html/global_attrs
Hash do Código: 8e70e7ce

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/html/global_attrs.rs`. No HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b`, a fonte estava em `01_core/src/compiler/stdlib/html.rs:475-1398`; o manifesto `00_nucleo/diagnosticos/atomizacao-html-2026-09-30-before.json.gz` registra a proveniência exata.

## Contrato

Tabelas de atributos globais, ARIA, roles e enums aplicáveis a múltiplas tags. Preservar cada nome, categoria, valor permitido e ordem de especificação medidos no L0 principal `00_nucleo/prompts/compiler/stdlib/html.md`. A separação é interna, na mesma camada L1; não altera contratos públicos nem defaults.

## Aceitação

A suíte L1, os testes de html e `crystalline-lint .` passam. Não inferir ganho de desempenho.

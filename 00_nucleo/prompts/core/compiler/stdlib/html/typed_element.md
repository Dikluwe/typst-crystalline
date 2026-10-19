# L0 — stdlib/html/typed_element
Hash do Código: e9df24a9

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/html/typed_element.rs`. No HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b` e árvore limpa, `01_core/src/compiler/stdlib/html.rs:1399-1489` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `lab/results/construction/diagnostics/atomizacao-html-2026-09-30-before.json.gz`.

## Contrato

Construção de elementos tipados, body void/raw/content e coleta de atributos. Preservar as assinaturas públicas, defaults, classes de diagnóstico, morfologia dos elementos e ordem de validação atualmente especificadas em `00_nucleo/prompts/core/compiler/stdlib/html.md`. Esta extração só separa responsabilidade dentro de L1; o dispatch permanece estático, sem I/O nem estado global mutável.

## Aceitação

Preservar os testes de `html`, a suíte L1 e a validade de `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho a partir da separação.

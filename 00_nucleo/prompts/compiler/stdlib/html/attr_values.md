# L0 — stdlib/html/attr_values
Hash do Código: 59362bce

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/html/attr_values.rs`. No HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b` e árvore limpa, `01_core/src/compiler/stdlib/html.rs:1693-2033` contém o comportamento extraído. O SHA-256, horário e estado inicial estão em `00_nucleo/diagnosticos/atomizacao-html-2026-09-30-before.json.gz`.

## Contrato

Coerções auxiliares de enum, lista, data, duração, tamanho, imagem e diagnósticos. Preservar as assinaturas públicas, defaults, classes de diagnóstico, morfologia dos elementos e ordem de validação atualmente especificadas em `00_nucleo/prompts/compiler/stdlib/html.md`. Esta extração só separa responsabilidade dentro de L1; o dispatch permanece estático, sem I/O nem estado global mutável.

## Aceitação

Preservar os testes de `html`, a suíte L1 e a validade de `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho a partir da separação.

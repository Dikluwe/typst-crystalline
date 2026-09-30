# L0 — stdlib/color/arguments
Hash do Código: 69933aa0

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/color/arguments.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/color.rs:246-324` contém o comportamento extraído. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-color-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Validar argumentos Color, Ratio e ColorSpace e mapear ColorSpaceError para diagnósticos existentes. Não converter tipos não admitidos.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/compiler/stdlib/color.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de color, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

# L0 — stdlib/shapes/rect
Hash do Código: af30504d

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/rect.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:77-228` contém o comportamento extraído. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Construir rect e square com width/height, fill, stroke e radius, preservando defaults e o Rect comum sem tipo Square novo.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/compiler/stdlib/shapes.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de shapes, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

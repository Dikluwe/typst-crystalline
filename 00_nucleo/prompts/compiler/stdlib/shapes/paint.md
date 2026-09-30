# L0 — stdlib/shapes/paint
Hash do Código: a1f5fb78

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/shapes/paint.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/shapes.rs:26-76` contém o comportamento extraído. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-shapes-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Converter cores tipadas, Paint e Stroke para shapes, preservando tipo, espessura, diagnostics e o fallback somente quando contratado.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/compiler/stdlib/shapes.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de shapes, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

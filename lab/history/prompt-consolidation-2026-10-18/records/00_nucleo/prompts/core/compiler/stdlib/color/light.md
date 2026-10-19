# L0 — stdlib/color/light
Hash do Código: c5e872a1

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/color/light.rs`. No HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` e árvore limpa, `01_core/src/compiler/stdlib/color.rs:325-374` contém o comportamento extraído. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-color-2026-09-30-before.json.gz`. Esta é separação interna por comportamento.

## Contrato

Implementar lighten/darken no espaço próprio da cor com amount Ratio e preservação da identidade Spot quando contratada.

As regras detalhadas, evidências históricas, limites de paridade e diagnósticos continuam em `00_nucleo/prompts/core/compiler/stdlib/color.md`. A extração não altera assinatura pública, defaults, fase ou compatibilidade. Nenhum I/O ou estado global mutável.

## Aceitação

Preservar a suíte L1 e os testes de color, compilar e passar `crystalline-lint .` com ownership e hashes corretos. Não inferir ganho de desempenho.

# L0 — stdlib/loading/json
Hash do Código: 1f515750

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/json.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:651-681` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Decodificar bytes JSON em Value preservando árvore, ordem de objeto, inteiros i64 quando representáveis e números não inteiros como Float. Erros de parsing permanecem diagnósticos de JSON. A função nativa é composta pelo parent com a fonte já resolvida.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/core/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

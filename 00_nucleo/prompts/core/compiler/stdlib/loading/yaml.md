# L0 — stdlib/loading/yaml
Hash do Código: 7183d19f

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/yaml.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:687-741` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Decodificar UTF-8 YAML via saphyr. Um documento produz um Value; vários documentos produzem Array ordenado; escalares, sequências e mapas preservam a projeção vigente. UTF-8 inválido e parser inválido produzem os diagnósticos contratados.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/core/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

# L0 — stdlib/loading/toml
Hash do Código: 54d20f15

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/toml.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:746-773` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Decodificar UTF-8 TOML em Value com ordem de tabela. Datetime mantém a representação textual graded definida no L0 vigente; não criar tipo novo. Erros de UTF-8 ou parser continuam diagnósticos.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

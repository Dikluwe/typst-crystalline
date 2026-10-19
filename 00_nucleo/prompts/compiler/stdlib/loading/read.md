# L0 — stdlib/loading/read
Hash do Código: 0e6167de

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/read.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:1155-1296` contém o comportamento correspondente. SHA-256, horário e estado estão em `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Validar e resolver a fonte de read e dos cinco loaders. Path/Str usam o World injetado; Bytes entram diretamente nos loaders. read distingue encoding utf8 e none e reporta posição do primeiro byte UTF-8 inválido. Preservar spans, nomes de tipos, mensagens e ordem de validação vigentes; nenhum acesso direto a filesystem em L1.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

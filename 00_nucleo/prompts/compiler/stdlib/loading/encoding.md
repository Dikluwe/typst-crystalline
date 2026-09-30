# L0 — stdlib/loading/encoding
Hash do Código: c0298050

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/encoding.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:40-646; encoders em 360-645` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Compartilhar a projeção textual de Content/LocatedContent com func e campos públicos ordenados, a leitura causal de Args para encoders e a normalização comum do token numérico. Snapshot presente é autoritativo; ausente usa somente a projeção legada contratada. A unidade não emite formato final nem executa callbacks.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

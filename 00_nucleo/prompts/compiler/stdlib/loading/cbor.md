# L0 — stdlib/loading/cbor
Hash do Código: d85d2d20

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/cbor.rs`.
No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` e árvore limpa, `01_core/src/compiler/stdlib/loading.rs:778-897 e 1317-1333` contém o comportamento correspondente. SHA-256, horário e estado estão em `00_nucleo/diagnosticos/atomizacao-loading-2026-09-30-before.json.gz`. É separação interna conforme ADR-0109/0127/0129; não há novo contrato público ou desempenho medido.

## Contrato

Decodificar e codificar CBOR em memória, incluindo byte strings, arrays e mapas; converter os tipos pela partição Value vigente. A nativa cbor distingue fonte path de Bytes para o sufixo diagnóstico de erro e a nativa cbor.encode conserva sua validação e saída Bytes. Não reusar cegamente a projeção estruturada dos encoders textuais.

As regras detalhadas e evidências históricas do comportamento permanecem em `00_nucleo/prompts/compiler/stdlib/loading.md`; esta unidade as herda sem alterar paridade nem fases do pipeline. O consumer opera em L1, recebe dados/World por argumento e não introduz I/O direto ou estado global mutável.

## Aceitação

Preservar o resultado linguístico e os diagnósticos existentes. Compilar, passar a suíte L1 e `crystalline-lint .` com ownership e hashes válidos.

# L0 — stdlib/loading/json_encode
Hash do Código: a0d97c1b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

## Ownership e medição

Camada L1. Consumer produtivo único: `01_core/src/compiler/stdlib/loading/json_encode.rs`. No HEAD `4ac3e2082e6e708dd815ab01b47ac8ed281a14fa` com árvore limpa, os encoders estavam em `01_core/src/compiler/stdlib/loading.rs:40-646`; estado completo em `lab/results/construction/diagnostics/atomizacao-loading-2026-09-30-before.json.gz`. A separação é interna e não mede desempenho.

## Contrato

Converter Value em árvore JSON com Content/LocatedContent projetado pelos campos públicos, emitir pretty ou compacto conforme Args, preservar expoentes numéricos e strings sem normalizar conteúdo de string. Diagnósticos preservam span do valor.

A especificação detalhada e a evidência histórica estão em `00_nucleo/prompts/compiler/stdlib/loading.md`. Snapshot presente é autoritativo conforme o núcleo pinado; sem reconstrução por estilo atual.

## Aceitação

Preservar saída e diagnósticos existentes, compilar e passar a suíte L1 e `crystalline-lint .`.

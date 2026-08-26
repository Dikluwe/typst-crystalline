# Prompt L0 — regressões do motor de layout
Hash do Código: 0718ad3e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1 test-only
**Ficheiro alvo:** `01_core/src/compiler/layout/tests.rs`

## Contrato e aceitação

Concentrar Worlds/métricas test-only e regressões de geometria e morfologia do
layout. Testes cobrem referencial absoluto, rebase, regiões, paginação, texto,
containers e aninhamento; números decisórios registram estado e vanilla.

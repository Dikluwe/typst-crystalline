# Prompt L0 — align e place
Hash do Código: 40ec5a92

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/placement.rs`

## Contrato e aceitação

Align reposiciona dentro da região; Place aplica escopo, alinhamento, offsets e
float sem entrar no fluxo quando definido. Sublayout usa origem absoluta e
recomposição soma apenas delta. Fixups sob dimensões auto preservam paths e
referenciais, inclusive em aninhamento.

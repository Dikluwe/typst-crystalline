# Prompt L0 — helpers geométricos de layout
Hash do Código: 8690aabb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/helpers.rs`

## Contrato e aceitação

Manipular, medir e transladar FrameItems de forma pura, incluindo limites de
conteúdo e descida por Group/Link. Helpers não introduzem semântica específica
de elemento e preservam o referencial declarado pelo caller.

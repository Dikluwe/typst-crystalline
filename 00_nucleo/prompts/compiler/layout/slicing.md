# Prompt L0 — slicing e tails de células
Hash do Código: 4f9752ca

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/slicing.rs`

## Contrato e aceitação

Fatiar FrameItems por altura/região sem perder grupos, links, semântica ou
fixups associados. Tails diferidos conservam coordenadas relativas ao próprio
buffer e são rebaseados uma única vez na emissão final.

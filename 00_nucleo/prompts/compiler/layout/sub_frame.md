# Prompt L0 — sublayout isolado
Hash do Código: 5aaff0a3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/sub_frame.rs`

## Contrato e aceitação

Executar conteúdo numa SubLayoutRegion salvando/restaurando cursor, região,
linha, collectors e pending state em LIFO. Devolver items, altura, decorações e
fixups no mesmo referencial local. Variante inline isola somente items do body
e deixa o caller decidir emissão.

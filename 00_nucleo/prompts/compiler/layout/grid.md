# Prompt L0 — layout de grid e table
Hash do Código: 63cf2ae9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/grid.rs`

## Contrato

Resolve tracks, spans, gutters, insets, alinhamento por eixo, header/footer e
células em duas fases. Sublayouts usam origem absoluta da célula; overflow,
tails diferidos, decoração e fixups são rebaseados no mesmo referencial.

## Aceitação

Grid/table, spans, alinhamento, paginação, overflow e grupos de linhas possuem
testes geométricos e não usam Place como substituto de Align.

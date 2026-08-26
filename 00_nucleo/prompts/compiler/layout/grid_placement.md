# Prompt L0 — placement algorítmico de grid
Hash do Código: bb0a3bd0

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/grid_placement.rs`

## Contrato e aceitação

Posicionar células explícitas e automáticas respeitando row/column, colspan,
rowspan e ocupação, com ordem determinística e erros para ranges inválidos.
Este owner decide índices lógicos; emissão geométrica pertence a `grid.rs`.

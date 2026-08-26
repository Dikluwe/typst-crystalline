# Prompt L0 — resolução de geometria de página
Hash do Código: 03d54729

**Camada:** L1
**Ficheiro alvo:** `01_core/src/compiler/layout/page_geometry.rs`

## Contrato e aceitação

Resolver paper, overrides, flip, margens auto/lógicas e binding em PageConfig.
Inside/outside depende de direção raiz e paridade física somente no fechamento;
eval não resolve geometria e dimensões auto permanecem explícitas.

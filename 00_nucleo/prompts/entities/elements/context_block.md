# Prompt L0 — `entities/elements/context_block`
Hash do Código: 173b59ad


**Camada:** L1
**Ficheiro proprietário:** `01_core/src/entities/elements/context_block.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/context/block-semantics.toml sha256:0ede5ade7908670f0440672becaba2aa356acc51346fd590dc426e4788dc6dd0

## Contrato

`ContextBlockElem` conserva `id` e `closure`. Clone preserva ambos; igualdade e
hash usam o id. O elemento produz `ElementKind::ContextBlock` e payload com o
id, preserva-se em map_content/map_text e não possui plain text próprio.
Construção nativa e expansão pertencem a outros owners.

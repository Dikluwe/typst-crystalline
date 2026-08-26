# Prompt L0 — `compiler/layout/page_canvas` — layers decorativos

Hash do Código: 3e82f5a4

**Camada:** L1
**Ficheiro proprietário:** `01_core/src/compiler/layout/page_canvas.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-canvas.toml sha256:442dcd9884551a8bfc99600298aea9d1f6591a5d0ccaf61a811de075c5b435d6

## Contrato

`layout_layer` devolve vazio quando não há conteúdo. Caso exista, materializa-o
num sub-frame com dimensão trim+bleed e envolve os items em Group posicionado
em `(-bleed.left, -bleed.top)`, matriz identidade e sem clip. Percentuais do
layer resolvem contra o canvas; o body continua fora deste owner.

## Aceitação

Background/foreground cobrem o canvas físico sem mudar a região trimada nem a
semântica acessível do body. Estrutura dos tipos de bleed/fill pertence ao
owner `entities/page_canvas.md`.

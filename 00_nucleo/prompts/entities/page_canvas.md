# Prompt L0 — `entities/page_canvas`
Hash do Código: fa746e95

**Camada:** L1  
**Ficheiro proprietário:** `01_core/src/entities/page_canvas.rs`
**Origem:** P1140.20–P1140.20.2  
**Estado:** especificado; implementação condicionada ao gate P1140.20.2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-canvas.toml sha256:442dcd9884551a8bfc99600298aea9d1f6591a5d0ccaf61a811de075c5b435d6

## Medição anterior à decisão

Vanilla: bleed aceita comprimento relativo/dict, inside/outside e zero default
(`layout/page.rs:170-219`), resolvido contra tamanho trimado
(`typst-layout/pages/run.rs:132-138`). Layers resolvem contra
`inner + margin + bleed` e cercam o body (`run.rs:218-244`). Fill é
`Smart<Option<Paint>>`: auto transparente em PDF e branco em SVG/raster
(`page.rs:257-278`, `document.rs:88-121`). PDF aumenta MediaBox e cria TrimBox
com bleed não zero (`typst-pdf/convert.rs:108-123`).

## Tipos

`PageBleedSpec`: top/right/bottom/left `Option<Rel<Length>>` e
`two_sided: Option<bool>`, fold como margem mas sem auto; omitido herda no
delta e default final é zero. Percentual horizontal usa width trimado e
vertical height. `PageBleed` é snapshot físico em pontos por binding/paridade.

`PageFill` é enum `Auto | None | Paint(Paint)`. Auto nunca colapsa em L1; L3
resolve transparente para PDF e branco para SVG/raster. None é transparente.

Este owner define apenas `PageBleedSpec`, `PageBleed` e `PageFill`. A composição
de layers pertence a `compiler/layout/page_canvas.md`.

## Geometria

Page width/height continuam TrimBox. Canvas mede
`width+left+right × height+top+bottom`; origem trimada é `(left,top)`. Layers
resolvem 100% contra canvas; body continua contra região trimada menos margens.

## Incompletude e aceitação

Running matter/supplement ficam P1140.20.3/.4; `page` em P1140.21. Aceitar só
com bleed escalar/dict/lógico e restauração, MediaBox/TrimBox, fill ternário,
ordem de layers, percentuais contra canvas e decoração fora da acessibilidade.

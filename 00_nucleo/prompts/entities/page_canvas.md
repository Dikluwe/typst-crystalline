# Prompt L0 — `entities/page_canvas`
Hash do Código: a36f4aa0

**Camada:** L1  
**Alvos:** `01_core/src/entities/page_canvas.rs`, `layout_types.rs`  
**Origem:** P1140.20–P1140.20.2  
**Estado:** especificado; implementação condicionada ao gate P1140.20.2

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

`Page` preserva `background: Vec<FrameItem>`, `items` body e
`foreground: Vec<FrameItem>`, bleed e fill. Ordem é
fill→background→body→foreground. Layers são artefatos decorativos: ausentes de
plain text, query, estrutura acessível e MCID.

## Geometria

Page width/height continuam TrimBox. Canvas mede
`width+left+right × height+top+bottom`; origem trimada é `(left,top)`. Layers
resolvem 100% contra canvas; body continua contra região trimada menos margens.

## Incompletude e aceitação

Running matter/supplement ficam P1140.20.3/.4; `page` em P1140.21. Aceitar só
com bleed escalar/dict/lógico e restauração, MediaBox/TrimBox, fill ternário,
ordem de layers, percentuais contra canvas e decoração fora da acessibilidade.

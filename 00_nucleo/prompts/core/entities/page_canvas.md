# L0 — Page canvas domain
Hash do Código: 4d341037

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/page_canvas.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/page-canvas.toml sha256:2aa7464200337a5915eec7850b8cf4488b9a471877220342546661f9823f19f1

## Evidence

Retained upstream evidence: `layout/page.rs:170-219` for bleed,
`typst-layout/pages/run.rs:132-138` for trim-relative resolution,
`run.rs:218-244` for layers, `page.rs:257-278` and `document.rs:88-121`
for fill, and `typst-pdf/convert.rs:108-123` for MediaBox/TrimBox.
These coordinates are historical evidence, not fresh current-pin measurements.

## Owned domain

`PageBleedSpec` stores four optional Rel<Length> sides and optional two_sided.
Its fold keeps inner supplied components and inherits omitted components.
No Auto state is introduced for bleed; final omitted sides resolve to zero.
`PageBleed` stores physical point-valued sides, supports zero/horizontal/vertical
queries, and resolves horizontal percentages against trim width and vertical
percentages against trim height, adding the resolved absolute/em component.
Logical sides swap only according to resolved binding and physical page parity.

`PageFill` remains `Auto | None | Paint(Paint)` with Auto default. Auto must not
collapse into None in L1; downstream PDF resolves it transparently and SVG/raster
white, while None remains transparent. Target policy is not implemented here.

The pinned canvas nucleus supplies shared geometry/decorative obligations.
Layer composition belongs to `00_nucleo/prompts/core/compiler/layout/page_canvas.md`.
Verify scalar/per-side bleed, fold, percentages, binding/parity and distinct fill
states without adding running matter, public page construction or exporter logic.

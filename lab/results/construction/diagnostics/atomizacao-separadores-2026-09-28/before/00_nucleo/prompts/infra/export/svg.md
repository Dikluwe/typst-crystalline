# L0 — svg
Hash do Código: 1fc03204

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

## Consumer evidence

`03_infra/src/export/svg.rs:40` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own SvgOptions, default pretty=false/render_bleed=false, FontKey/GlyphFontIdentity
aliases, compatibility reexports and all existing standalone/context/font/HTML-frame
entrypoints and methods. Preserve signatures and private context fields. The pinned
context nuclei define the shared identity/transport invariants; specialized context
implementation belongs to glyphs and navigation.

Own the root XmlWriter, page canvas/viewBox and bleed translation. Standalone without
render_bleed crops to the logical page; with it use the physical canvas. Auto page
fill is white and None transparent. Paint background/body/foreground in order and
emit definitions through child writers. Keep exhaustive static FrameItem dispatch;
Semantic is visually transparent and never paints alt or invents accessibility.

The HTML-frame variant consumes the supplied Page with the same canvas, fill and
layer rules; it does not independently remove backgrounds or convert dimensions to
em. Its root style is `display: block; overflow: visible`. Intrinsic transparent
frame construction belongs to the HTML layout caller, not this wrapper. Reuse child
writers, including the HTML textual path, without textual paint/palette substitution
or duplicating SVG in HTML.
Pretty selects writer indentation only; do not alter the graph or child algorithms.
Bundle/file routing stays outside this page exporter. No filesystem or new layout.

## Delegated owners

- `00_nucleo/prompts/infra/export/svg/geometry.md` owns `03_infra/src/export/svg/geometry.rs`.
- `00_nucleo/prompts/infra/export/svg/glyphs.md` owns `03_infra/src/export/svg/glyphs.rs`.
- `00_nucleo/prompts/infra/export/svg/groups.md` owns `03_infra/src/export/svg/groups.rs`.
- `00_nucleo/prompts/infra/export/svg/images.md` owns `03_infra/src/export/svg/images.rs`.
- `00_nucleo/prompts/infra/export/svg/navigation.md` owns `03_infra/src/export/svg/navigation.rs`.
- `00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md` owns `03_infra/src/export/svg/p1360_ab_tests.rs`.
- `00_nucleo/prompts/infra/export/svg/paints.md` owns `03_infra/src/export/svg/paints.rs`.
- `00_nucleo/prompts/infra/export/svg/serialization.md` owns `03_infra/src/export/svg/serialization.rs`.
- `00_nucleo/prompts/infra/export/svg/tests.md` owns `03_infra/src/export/svg/tests.rs`.

# L0 — svg
Hash do Código: 8dfcc406

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-destination-context.toml sha256:13cad5ab1322bad4c569eec2aaf544452530cb972c3421130d0b9cb127170ef1
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

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

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/svg.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator não possui contorno visual; no frame HTML emitir U+0020 no mesmo referencial textual.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

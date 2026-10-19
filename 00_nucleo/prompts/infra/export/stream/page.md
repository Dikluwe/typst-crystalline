# L0 — stream/page
Hash do Código: 32f519a3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:afd5a809b6e3f2ac1f9a752fb1f6c6d704139eb1f71b085d430d9417f826b04e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/page.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/page.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page assembly

Own build_page_stream and emit_page_items. Emit canvas fill and background, body,
foreground in that order, using the physical canvas and bleed translation with a
balanced q/cm/Q envelope. Borrow PageContext and return page-operator bytes.
Delegate prosaic run coalescing to text and individual item dispatch to draw;
UA1 leaves keep their marked-content boundaries: this owner bypasses prose run
coalescing for UA1. Auto and None canvas fill emit no explicit fill operators;
Paint uses the existing solid/spot or to_color projection, not a canvas Pattern.
When UA1 is enabled, explicit canvas fill is an Artifact Layout. Do not allocate resources, objects,
ParentTree entries or fonts, and do not compress the returned operators.

## Verification

Check blank pages, fill, bleed, background/foreground order and unchanged top-level
coordinates with both graphic modes and tagging settings.

## Translucent solid canvas composition

### Evidence

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz`
retains sources/owners/HEAD/UTC/working tree. tiling-nativo-2026-10-14/final-renders
and remaining-analysis.md record cell8×8/pitch4×4 alpha50%: opaque-red PDF, one SVG
contribution, four PNG contributions (255,15,15). pdf_tiling.rs:184 emits BBox=size;
svg/paints.rs:148 uses overflow visible; stream/shape.rs:155/text.rs:275 discard
alpha. These are observable composition defects; vanilla also clips overlap and
is no requirement to copy that defect. Inference: explicit period composition
preserves nonoverlap controls and matches four analytic source-over contributions;
controls can refute it.

### Contract and acceptance

For translucent Solid canvas, wrap only page fill in q/Q with opacity. Later items
start opaque. Do not change old PageFill Gradient/Tiling projection or claim it
fixed by this slice. Internal ADR-0127 correction remains inside previously approved
T1–T3, no API/phase/flag/default addition. L0 first, RED→GREEN, analytic composition
including distinct-color order, simple/nested/rotated/text/image and page extraction/
selection controls; build/workspace suite/fmt/Rustdoc/lineage. Variable-gradient
alpha and PNG quality remain incomplete; do not certify a production package
while these issues remain open.

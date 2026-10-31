# L0 — stream/page
Hash do Código: 45b3a5aa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/word-joiner-unicode.toml sha256:3fbe26365aae1904cecc40eb2cb61bd6c73ea0f6a4d84ca9ba7a8820f028075f
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


## Readiness: source joiner prefix beside a causal separator (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/fraction-{candidate,extras}/comparison.json retain
identical original outline PDF/SVG paint after the allocation correction but
failed exact word strings. The immutable list-body-extras zero-inset PDF and
its source-bound build are retained independently. outline-scope-experiment/
receipt.json binds exact QDF/PDF/source/tool/worker hashes, HEAD/dirty stat,
UTC, commands, frozen compiler inputs and independent rasters. Repacked control
and a single wide ActualText remain negative. Two separate scopes pass exact
words and0.001pt boxes (maximum0.00001pt), with identical paint: the existing
space carries space+U+2060 and the original shaped run carries only its suffix.
Both inside-BT and deferred-restoration variants pass; copying a vanilla BT
arrangement is not required. This is observable text identity/bounds, not byte
or operator parity.

wj-before/receipt.json separately binds the new joined/split DejaVu dot/suffix
sources, compiler/oracle bytes and all source hashes. Tekt extracts separate
"." and "U+2060+1" words; pinned oracle extracts ". SPACE U+2060+1" as one word,
then TAIL. Bundled/system DejaVuSans.ttf bytes both hash
 ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280.
Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, binary
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The earlier glyph-global collision and pure-joiner scale corrections remain
intact. No original source or expected word is normalized to grant a pass.

### Exclusive responsibility and acceptance

Before the existing prose-coalescing check, delegate only a draw-certified adjacent separator/joiner-prefix pair, retaining both ORIGINAL leaves. Advance the original slice index by two only after drawing both once. Apply this to the existing background/body/foreground traversal; do not allocate a resource, clone a leaf, assign MCIDs or alter canvas/bleed/UA1/prose behavior. Other items and refusals use the identical existing run/coalescing/dispatch path. Root and nested/group cases must agree on source words/bounds and painting.

ADR-0127 continuous internal parity work: no public field/signature/trait,
phase/default/mode/compatibility change. L0 and meaningful real-font PDF RED
precede implementation. Preserve actual source concatenation, all earlier
Unicode/ALT/ligature/space/bidi/UA1 controls and independent PDF/SVG painting.
Require V15/V26 before resealing, forward/reverse lineage, full technical gates
and final historical/JPEG resource validation. This does not certify general
Unicode extraction, PDF-UA, HTML, bibliography or overall product readiness.

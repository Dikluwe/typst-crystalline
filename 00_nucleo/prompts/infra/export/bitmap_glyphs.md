# L0 — infra/export/bitmap_glyphs
Hash do Código: 05d221c2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/bitmap_glyphs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:eb0ad1967bbbd9ee9ded17582451b09de2820855ba38c8a554bbe7b2db7b1848

## Consumer evidence

`03_infra/src/export/bitmap_glyphs.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Bitmap glyph collection

Own BitmapGlyph/BitmapGlyphRef carriers and the per-face used-ID/raster collection
adapter under the pinned bitmap-embedding nucleus. BitmapGlyph carries the processed
PDF payload, bearing_x, bearing_y and pixels_per_em; width/height remain in payload.
BitmapGlyphRef carries the emitted XObject name, dimensions, bearings and strike
scale. This owner does not allocate names/IDs or emit streams.

used_glyph_ids_for_face returns an ordered union of collected direct/shaped IDs
within face.number_of_glyphs() and IDs resolved from both plain/shaped codepoints
through that same face. Calls/maps are per-face; identical GID numbers in different
fonts are not a shared identity.

collect_bitmap_glyphs_for_ids queries face.glyph_raster_image(GlyphId(gid), u16::MAX)
for the largest strike. Accept only RasterImageFormat::PNG, use the existing
process_png_for_pdf adapter, and skip absent, unsupported or invalid raster data.
Keep at most one map entry per GID. Preserve x/y bearings and pixels_per_em signs
and units. Absence selects the caller's normal font fallback; never synthesize pixels.

## Boundaries and verification

`00_nucleo/prompts/infra/export/builder/fonts.md` owns font/GID XObject allocation,
`00_nucleo/prompts/infra/export/builder/resources.md` owns resource merging,
`00_nucleo/prompts/infra/export/stream/text.md` owns placement/Do operators and
`00_nucleo/prompts/infra/export/images/png.md` owns decoding/compression.
Check supported PNG payload/metrics, duplicate IDs, out-of-range IDs and unsupported/
invalid raster omission. No font selection, public format extension or I/O belongs here.


## R10 — source-preserving raster glyph fallback

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,bitmap-evidence.json,font-tables.json}` binds received R9 code/dirty stat/UTC and fixed bd5ebec2 binaries. Nimbus/Liberation mixed123☃ selects Noto Color Emoji. The received PDF raster route emits only image Do, losing the source character in extraction; SVG glyphs.rs:100-105 requires an outline and emits no raster. Pinned paired PDFs/SVGs preserve source text and the PNG. Font-family names are evidence, never runtime branches; supported actual raster data authenticates the correction. Source character, physical position/advance, PNG pixels and geometry are language observations. Object numbering, Type3 versus CID structures and byte encoding are mechanics.

Classification: private font/export parity correction under ADR-0127 continuous flow, with no new public method/entity/phase/mode/dependency. The previously approved FontMetrics method remains separately bounded. No invisible duplicate source text, fake outline, font-name coefficient, or oracle-byte copying. Build a1016-byte self-authored portable bitmap font fixture from the preserved fontTools generator; it is test data, not a font copied from the oracle or a productive dependency. Authenticate its bytes explicitly in the receipt.

### Exclusive owning obligation

The internal BitmapGlyphRef carrier may additionally hold an optional single-byte Type3 code, assigned only by the builder for an authenticated complete <=256-glyph bitmap font; None preserves the existing image-only path. This crate-private carrier is not an external entity/API change. Preserve all actual raster metrics and dedup keys.
Require module RED→GREEN before implementation, fixed-pin actual fallback PDF/SVG paint/text/geometry controls, previous A–H/R1–R9, full workspace/lineage/resource gates and byte preservation of received unrelated inputs.

# L0 — infra/export/bitmap_glyphs
Hash do Código: 275c1560

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

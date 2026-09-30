# L0 — svg/glyphs
Hash do Código: 26406d33

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/glyphs.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/svg-glyph-font-context.toml sha256:4d185c303f0262799e475ff76459118a64fdbd406dfd704d95a836b4b254985c

## Consumer evidence

`03_infra/src/export/svg/glyphs.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font context and glyph definitions

Own GlyphFontRequest/SvgGlyphFontContext, immutable request normalization/lookup,
GlyphDefs, outline extraction/cache/path builder, direct/shaped glyph painting and
the existing HTML textual writer. Follow the pinned glyph-font-context nucleus;
its shared resolution invariants are not redeclared by the façade.

Use the resolved FontKey's supplied bytes for glyph_id, verifying the key exists
in the collection. Deduplicate definitions by font identity and GID. Apply size /
units_per_em, position and Y inversion once; preserve shaped advances/offsets,
fill/alpha and supplied styles without reshaping or moving the layout cursor.
Keep OutlineBuilder methods as trait methods and current visibility.
Absent/ambiguous mapping, invalid face, zero units_per_em or missing outline remains
Unknown with the existing fallback, not a fabricated glyph. Fontless wrappers do
not synthesize text/outlines or probe incidental/system fonts or network.

## Verification

Exercise reordered fonts, same-GID distinct fonts, mathematical assemblies,
fill/alpha, transforms and missing/invalid/fontless contexts.

# L0 — builder/fonts
Hash do Código: 74702211

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:016908bda7174f00ff63a909070553e9728d3d44c7ebf17b9616b4f676ad4def

## Consumer evidence

`03_infra/src/export/builder/fonts.rs:51` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font preparation and construction

Own CIDFont/multifont build paths, font usage collection, subsetting and variable
instancing, descriptors, widths, ToUnicode, embedded font streams and CIDSet.
Font identity includes FontList, FontVariant and FontVariations. Merge derived and
explicit axes using the existing precedence; do not extend the single-font variable
instancing scope. Accumulate only the existing subsetting timing into subset_ms.

Collect shaped cluster text once before the per-font loop. Include codepoints and
glyphs from fallback Text, including spaces, in each applicable subset. ToUnicode
must retain shaped clusters and ligatures even when subsetting falls back to full
embedding; remap GIDs only when a subset mapping exists. Prepare the per-font reverse
map for direct mathematical glyphs; do not hardcode their resource to F1.

Compute glyph_to_nominal from the same effective face and axes as /W. Use variable
advances only when instancing succeeded; a default-face fallback uses default-face
advances consistently on both sides of the TJ boundary.

Emit Type0 Identity-H fonts with the correct descendant and stream representation:
TrueType uses CIDFontType2 and FontFile2/SFNT; CFF1 uses CIDFontType0 and
FontFile3/CIDFontType0C with the extracted CFF table; CFF2 uses CIDFontType0 and
FontFile3/OpenType with the complete container. Preserve descriptor metrics and
mapping semantics. Resolve BaseFont/FontName from PostScript name, then full name,
then family, then fallback; remove spaces and use the deterministic AAAAAA subset
prefix without renaming internal Fn resources.

Retain the 256 KiB font-stream compression cutoff and existing raw-data fallback
on compression failure. Every embedded CID font has a CIDSet: set MSB-first bits
for the effective CIDs after subset remapping, or used original CIDs for full
embedding, and allocate its stream outside reserved IDs. Bitmap-only fonts do not
emit an outline font or CIDSet; mixed fonts retain outline embedding for those glyphs
that need it. Prepare bitmap references under the pinned bitmap-embedding nucleus;
resource dictionary merging belongs to the resources child, and painting to the
text child. Unsupported bitmap formats retain the normal font path.

## Verification

Exercise TrueType, CFF1/CFF2, variable fallback, multifont identity, sparse/remapped
CIDSet, ligatures, spaces and bitmap-only/mixed fonts. Verify extracted text and
glyph morphology, not equivalence to an external object-allocation algorithm.

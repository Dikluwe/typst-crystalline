# L0 — infra/export/font_subset
Hash do Código: 7c2d5724

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/subset.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/subset.rs:19` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Subset adapter

Own only FontSubset and subset_font_with_mapping, subset_font and remap_glyph_id
in the exclusive consumer. Preserve public signatures and FontSubset fields:
data contains subset font bytes; mapping is old GID to new GID.
L1 continues carrying original glyph IDs; it does not know this adapter.

Use subsetter::GlyphRemapper to include .notdef explicitly (0 → 0), every GID
from char_to_old_gid, and additional_gids such as ligatures. Call subsetter::subset
at face index zero on the already extracted individual-face bytes. Build mapping
from that remapper, not from a subset cmap; removal of cmap is intentional.
Return None on subsetter failure, retaining the existing test-only failure message
"subsetter::subset falhou: {:?}". Return valid TrueType or CID-keyed CFF subsets
when supported by the library. No new variation instancing or OpenType processing
policy is introduced here.

The compatibility subset_font wrapper returns bytes and keeps its current conversion
of nonzero used GIDs into the mapping input. remap_glyph_id returns the mapped ID
or zero for an absent key. Whether an empty mapping means full embedding is a
caller decision, not a different meaning of that helper.

## Ownership boundaries

- `00_nucleo/prompts/infra/export/builder/fonts.md` owns fallback to full
  embedding, variable instancing, font names, descriptors, widths/ToUnicode
  orchestration, CIDSet and the font compression threshold.
- `00_nucleo/prompts/infra/export/fonts.md` owns Unicode/cluster helpers.
- `00_nucleo/prompts/infra/export/stream/text.md` owns mapped text operators.
- `00_nucleo/prompts/infra/export/bitmap_glyphs.md` owns raster collection.

Do not duplicate those contracts or claim that bitmap-only fonts must be embedded.
Unsupported CBDT/CBLC subsetting retains None/fallback rather than invented support.
Shaping positions already supplied by L1 do not depend on preserving shaping tables
in the embedded subset.

## Verification

Check .notdef for empty input, char and additional-GID mappings, absent remap,
parseable TrueType/CFF fixtures and text extraction through the caller integration.
Exact subset size, object IDs or equality to an upstream subset algorithm are not
language-parity criteria.

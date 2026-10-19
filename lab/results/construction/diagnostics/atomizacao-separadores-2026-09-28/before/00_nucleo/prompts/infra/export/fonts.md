# L0 — infra/export/fonts
Hash do Código: b189de68

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/fonts.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF font and Unicode helpers

Own escape_pdf_string, document collectors, char/GID mapping, widths, ToUnicode
CMap, UTF-16BE hexadecimal encoding and shaped cluster reconstruction.
Keep existing sibling visibility and no filesystem or network lookup.

The Helvetica literal helper escapes parentheses and backslash, retains printable
ASCII (including DEL under the current predicate), and replaces non-ASCII or
characters below ASCII space with "?". This is the actual restricted helper,
not a claim of full Latin-1 support. The Identity-H helper emits four-hex-digit
GIDs and maps an absent character to 0000.

Collect codepoints from plain Text and shaped char_code; collect_text_codepoints
reads only plain Text; collect_glyph_ids includes direct Glyph and shaped GIDs.
Traverse background/body/foreground and Group/Link/Semantic descendants through
FrameVisitor. Nonvisual alt never contributes glyphs or text. Ordered sets keep
unique output deterministic. Shaped mapping uses the first char_code per GID and
ignores zero-x_advance marks; shaped cluster-text collection preserves run order.

cluster_text derives boundaries from sorted unique UTF-8 byte offsets plus text
length, independently of visual LTR/RTL glyph order. Invalid/non-boundary ranges
yield an empty substring. Only the first occurrence of a cluster receives its
complete UTF-16BE text; later glyphs in the cluster receive empty hex. Preserve
ligatures and non-BMP surrogate pairs, not truncated u16 codepoints.

widths_array accepts current (gid, hex) pairs and converts the effective face's
advance to rounded 1/1000 text units, retaining missing-advance fallback 500.
to_unicode_cmap accepts GID-to-UTF-16BE strings, Identity-UCS codespace 0000–FFFF,
and batches bfchar entries at no more than 100. Empty input produces a valid
empty map without an empty bfchar batch.
These helpers do not select effective font axes, embed streams or decide remapping;
those belong to `00_nucleo/prompts/infra/export/builder/fonts.md`.

## Verification

Check escaping, ordered recursive collection, marks, ligatures, RTL clusters,
surrogate pairs, notdef and CMap validity. Extracted Unicode and positioned glyph
morphology are observables; subset sizes and exact GID assignment are not parity.

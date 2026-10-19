# L0 — stream/text
Hash do Código: 70b95db2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/text.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/text.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Text and glyph operators

Own font lookup by family/variant/variations and Text/TextShaped/Glyph emitters in
Compact/Verbose. Type1 retains WinAnsi fallback. CIDFont/Multifont use supplied
old-to-new GID mappings; direct mathematical glyphs use their per-font reverse map.
Paint bitmap glyphs through prepared XObjects and placement metadata, not outline
fonts. Do not perform shaping, font instancing or resource allocation.

TJ uses (nominal - x_advance) / units_per_em * 1000 and compensates x_offset with
the existing paired adjustments, rounding and units. Preserve the current y_offset
scope without introducing new operators. Verbose uses balanced q/cm/color/BT/Tr/Tf/
Tc/Tm/TJ-or-Tj/ET/Q envelopes, explicit rendering mode, faux bold, tracking and spot
colors. Compact retains its current operator form. These helpers serve both top
and local dispatchers, whose coordinate composition is owned by draw.

Coalesce only consecutive shaped prose with equal baseline and complete envelope:
font, size, fill, bold/italic, tracking, direction and units_per_em. Never cross
nontext, group/link scopes or envelope changes. Math and bitmap leaves remain
separate. UA1 noncoalescing is the page owner's call-site policy, not a separate
UA1 decision in these run helpers. Compute each boundary TJ from absolute x against the effectively rounded
cursor, preserving glyph positions rather than replacing them by nominal advances.

## Verification

Exercise kerning, offsets, remapping, same-GID multifont identity, bitmap placement,
mode envelopes, rounded cursor boundaries and mathematical/UA1 noncoalescing.

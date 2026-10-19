# L0 — math/layout/assembly
Hash do Código: 0512cdd8


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/assembly.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/assembly.rs:26` locates `fn resolve_assembly_repeat<'p>(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Glyph assembly geometry

Own vertical and horizontal part assembly plus the local repeat/overlap
helper. Consume injected GlyphAssembly; do not load fonts.
Non-extender parts occur once, extenders repeat sufficiently to reach the
requested advance, bounded by the existing MAX_REPEATS=1024 safety limit.
For each joint, maximum overlap is the smaller adjacent connector length;
the growable interval is `max(0, max_overlap-min_overlap)`.
Choose repeat/ratio and position parts using
`advance - max_overlap + ratio*(max_overlap-min_overlap)`.
Honor the font's minConnectorOverlap, not an implicit zero.

Full advance/connectors measure the assembly axis. FrameItem glyph x_advance
always uses the part's horizontal advance; vertical box width is the widest
horizontal part advance, never a height-derived width.

Vertical parts follow the font's bottom-to-top order. Place each baseline at
the bottom of its accumulated slot, then center on the math axis using
`shift_y = -axis-total_height/2`; publish
`ascent=axis+total_height/2`, `descent=total_height/2-axis`.
Horizontal parts follow left-to-right order without vertical inversion;
advance x by full advance minus adjusted overlap, keep baseline y=0 and
derive signed ascent/descent from the union of actual part ink bounds.

Acceptance uses synthetic unequal advances/connectors to distinguish axis
advance from horizontal width, repeated extenders and min overlap; lower-ink
horizontal assemblies must not inherit global font ascent.
The historical lack of per-part negative-yMin compensation is a limitation
to investigate, not a requirement that other fonts reproduce that error.

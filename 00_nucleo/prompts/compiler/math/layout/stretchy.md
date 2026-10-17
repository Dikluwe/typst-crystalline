# L0 — math/layout/stretchy
Hash do Código: c2394720


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/stretchy.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/stretchy.rs:21` locates `fn apply_delim_short_fall(target_du: f64, upem: f64) -> f64`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Font-derived variant selection

Own vertical and horizontal stretch selection/emission, not part assembly.
Consume the active style's GlyphVariants, GlyphAssembly and FontMetrics.
Keep axis-specific implementations: vertical geometry centers ink on the
math axis; horizontal geometry remains baseline-anchored.

Ordinary vertical delimiters use target minus 0.1em, floored at zero.
Radicals and explicit stretch use zero shortfall. Horizontal callers supply
their own shortfall: accents 0.5em, spreaders and explicit stretch zero.
Do not replace these distinct policies with one shared constant.

Horizontal selection keeps the base if the short target fits its shaped/hmtx
advance, not its MATH variant AdvanceMeasurement. Otherwise choose a sufficient
variant; if none, assemble when possible; without assembly retain the largest
available variant, then fall back to the base if none exists.
Preserve the vertical selection/fallback route separately rather than
assuming horizontal symmetry.

Glyph horizontal width/x_advance uses hor_advance, never the stretch-axis
advance. An unmapped vertical variant uses actual ink bounds for centering:
`shift_y=(ink_up-ink_down)/2-axis`, with extents around that axis.
Retain the base glyph's applicable italics correction in its logical advance.
Horizontal glyph extents use signed ink bounds, not global font ascent or
clamped positive-only values; lower spreader ink may have negative ascent.
A mapped-character path remains distinct from explicit glyph emission.

Delegate repeated parts to `00_nucleo/prompts/compiler/math/layout/assembly.md`.
Acceptance discriminates keep-base, sufficient/largest variant, assembly,
axis advance versus horizontal advance, signed ink and shortfall policies.
Do not treat fallback approximations as proof of universal font parity.

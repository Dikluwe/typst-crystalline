# L0 — math/layout/underline

Hash do Código: c5d0b118

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underline.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/underline.rs:18` locates `pub(super) fn layout<M: FontMetrics>(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Mathematical underline and overline

Own free functions for the distinct MathUnderline/MathOverline identities.
The core only delegates; textual decorations have separate owners.
Use font-derived MATH constants, current fill and baseline-relative geometry,
without changing the public body's morphology.

Underline lays out the body without forcing cramped. Line thickness, gap and
extra descender use underbar_* constants. The line center is
`body.descent+gap+thickness/2`.
Preserve logical body width; line width alone is
`max(0,body.width-italics_correction(body))`. Positive correction shortens,
zero preserves and negative correction extends outside the box without
enlarging it. Do not clamp protruding ink to a root frame or use ink width as
the logical width. Descender includes gap, thickness and extra descender.

Overline lays out the body cramped and uses overbar_* constants. Its rule
center is `-(body.ascent+gap+thickness/2)`; width equals body logical width,
without underline's italics subtraction. Include the extra ascender.
Keep body baseline coordinates consistent; no additional fixture-derived
translation or textual decoration is required.

Both retain body's mathematical boundaries/properties; the line does not
manufacture class, text-like state or italics correction.
Acceptance covers above/below placement, width versus rule width, positive/
zero/negative correction, cramped behavior and full vertical extents.

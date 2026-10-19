# L0 — math/layout/vec
Hash do Código: 79a5ad13


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/vec.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/vec.rs:15` locates `pub(super) fn layout<M: FontMetrics>(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Column-vector identity and geometry

Own `vec::layout(layouter, elem, style)`, consuming MathVecElem without
converting it into MathMatrix. Each child forms one row in denominator style;
alignment and delimiters preserve their public values. Alignment points can
subdivide a child without creating extra rows. Reuse common grid/delimiter
mechanics; vector decisions remain here.

Apply Left/Center/Right. The existing direction-limited projection uses
Start→Left and End→Right while retaining the original alignment in the
carrier; this does not establish complete direction-aware parity.
Zero children returns a zero-size box before adding delimiters, not a
present empty row.

Gap applies only between rows. Its percentage base is the effective available
region height supplied by the contextual MathLayouter, never final page height,
grid height, font size or an arbitrary zero. For finite height:
`gap=rel*region_height+abs.resolve_pt(font_size)`.
For nonfinite height, including height:auto, relative contribution is zero
and the absolute contribution remains active. Do not propagate infinity
or discard mixed absolute length.

Center the grid on the math axis, stretch optional delimiters with the shared
policy and retain full extents. Acceptance varies region independently from
page/grid/font dimensions, including percent-only, absolute and mixed gaps
under finite/auto height, alignment and empty/nonempty vectors.

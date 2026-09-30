# L0 — math/layout/p1431-tests

Hash do Código: 1b6caa22

**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/p1431_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/p1431_tests.rs:27` locates `struct TestWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Independent matrix tests

Own only `01_core/src/compiler/math/layout/p1431_tests.rs`.
Use deterministic metrics and observe horizontal alignment, late MathAlignPoint
splitting, axis-specific relative gaps, every hline/vline offset including
borders/negatives, auto/custom stroke, delimiters, denominator style and extents.
Out-of-dimension offsets are diagnostic, not clamp/ignore.
Preserve the established P825/P912/P919/P923/P945/P1042 controls and
compatibility wrapper. Unattributable frame/font/stroke/geometry is Unknown
and blocks acceptance.

P26 includes unequal alignment-point counts and mixed classes. For asymmetric
`(1 & 2), (33 & 4)`, require
`x(1)+width(1)==x(33)+width(33)` and `x(2)==x(4)`.
Widths come independently from FixedMetrics::text_width at observed item size,
never from grid-produced positions. Centering subcolumns must fail the first
equation.

P27 compares default, 1em and 150% gaps in both axes.
P29 observes thickness, color and cap: auto=0.05em Square; custom/fold
preserves all three. P30 includes optional left/right delimiters in a tall
grid. P31 covers Display, Text/Inline, Script and ScriptScript with denominator
descent. P34 observes automatic Square, custom cap and compatibility Butt
in final FrameItem::Line.

M23 requires duplicate/out-of-order offsets, retaining coordinate quantity and
order; compute interior midpoints independently from cell metrics, while
borders 0/n coincide with complete box bounds.
M25 uses valid/invalid 2x3 and 3x2 matrices: hline validates rows, vline columns.
M26 calls Content::math_matrix directly, requires the complete mat carrier
and compares geometry with the equivalent common route. None of these
assertions promotes a private implementation shape or bytes to a requirement.

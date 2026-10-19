# L0 — math/layout/p1427-tests
Hash do Código: 68b6cbfd


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/p1427_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/p1427_tests.rs:20` locates `fn style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent fraction-layout tests

Own `01_core/src/compiler/math/layout/p1427_tests.rs` only.
Exercise the approved P18–P23 cases independently of the candidate algorithm,
using deterministic metrics: reconstruction, three layouts, independent
constants and the barless binomial control.

Both transformations preserve optional style, both deparenthesization bits
and line without normalization. None without cascade resolves Vertical;
a Horizontal cascade affects layout, but explicit Vertical wins.
Vertical stacks operands with exactly one bar when line=true.
Skewed positions numerator above/left and denominator below/right of the
inclined slash, uses independent gaps and has no horizontal bar.
Changing vertical gap affects only relevant vertical separation; horizontal
gap affects only horizontal separation. Swapping 96/350 must be detectable;
fallbacks are exactly 96du vertical and 350du horizontal.
Horizontal orders numerator/slash/denominator and restores parentheses only
on marked operands. Line=false always remains barless/slashless vertical
binomial, including under Horizontal cascade.

Assert typographic relationships and approved carrier preservation, not
serialized bytes, private frame decomposition or operation order.

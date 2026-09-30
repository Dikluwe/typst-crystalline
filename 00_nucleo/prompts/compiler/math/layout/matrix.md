# L0 — math/layout/matrix

Hash do Código: 6c80f717

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/matrix.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/matrix.rs:21` locates `pub(super) fn layout_matrix(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Matrix carrier, alignment and augmentation

Consume complete MathMatrixElem and preserve rows/cells, optional delimiter
sides, alignment, augment, relative gaps and presence metadata.
Derive effective fields by explicit > math.mat StyleChain > defaults;
resolve gaps/stroke against outer style, while cells use shared denominator
style. Default row gap is 0.2em, column gap 0.5em and automatic augment
thickness 0.05em; these are language defaults, not measured PDF adjustments.

Without ampersands, honor start/left/center/right/end alignment. With
MathAlignPoint, split cells late into alternating right/left subcolumns.
Transfer natural class spacing at ampersand boundaries into the left cell's
width, not a normal comma-separated column gap. Preserve unequal split
counts, row order and empty cell morphology.
Physical direction limitations are not a new definition of start/end.

Use shared grid floors, row baselines and 1.1 delimiter target. Axis-center
only grid content before merging already-centered delimiters, with no
redundant horizontal padding. Final ascent/descent includes delimiters;
horizontal composition preserves all component baselines.

Augment accepts multiple horizontal/vertical offsets, preserves duplicates
and order, normalizes negative offsets from the end and accepts borders 0/n.
Horizontal offsets validate against row count, vertical against column count;
out-of-dimension offsets are diagnostic, not silently clamped or ignored.
Interior strokes lie at the corresponding gap midpoint; borders use the
full box boundary. Auto stroke uses Square cap; explicit/custom folded stroke
preserves thickness, paint and cap. The compatibility wrapper retains its
established Butt behavior and does not redefine public mat defaults.
Do not bake raster/PDF behavior into this owner.

Acceptance checks axis-specific relative gaps, asymmetric alignment, every
offset including negatives/borders/duplicates, invalid dimensions, automatic/
custom/folded strokes, optional delimiters and the complete Content::math_matrix
route. Existing consumer deviations remain pending diagnostics, not exceptions
to these contracts.

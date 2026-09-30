# L0 — math/layout/cancel

Hash do Código: c51ac5ba

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cancel.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cancel.rs:21` locates `pub(super) fn layout_cancel(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Complete cancellation geometry

Consume MathCancelElem, measuring its body once. Preserve body width/ascent/
descent; strokes overlay the box and never expand its logical dimensions.
Resolve length against the body's diagonal, plus absolute length against
causal font size. Default length is 100% + 0.3em; auto direction derives
from the box. Center the stroke at
`(width/2, (descent-ascent)/2)` in baseline-relative coordinates.
Default thickness is 0.05em with current fill; explicit stroke preserves its
resolved paint, thickness and cap.

Inverted reflects direction; cross draws both directions and wins over
inverted. Background inserts strokes before body items, otherwise after.
For functional angles, construct/consume requests through
`00_nucleo/prompts/compiler/math/layout/callbacks.md`, using the occurrence
reserved by the dispatcher. Both cross requests receive the same measured
default; reflection follows each resolved result. No second execution runtime
or locator identity is introduced here.

For the 100% diagonal component, endpoints are (0, descent) and
(width, -ascent); additional length extends symmetrically. Preserve Auto,
explicit Angle and Func as distinct carrier states; a missing resolution is
provisional and cannot escape as successful compilation.

Acceptance covers baseline-relative endpoints, length, ordering, explicit
stroke, cross/inverted precedence and unchanged body dimensions.

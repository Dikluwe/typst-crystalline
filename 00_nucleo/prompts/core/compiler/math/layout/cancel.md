# L0 — math/layout/cancel
Hash do Código: 8185f0f5


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cancel.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cancel.rs:21` locates `pub(super) fn layout_cancel(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

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
`00_nucleo/prompts/core/compiler/math/layout/callbacks.md`, using the occurrence
reserved by the dispatcher. Both cross requests receive the same measured
default; reflection follows each resolved result. No second execution runtime
or locator identity is introduced here.

For the 100% diagonal component, endpoints are (0, descent) and
(width, -ascent); additional length extends symmetrically. Preserve Auto,
explicit Angle and Func as distinct carrier states; a missing resolution is
provisional and cannot escape as successful compilation.

Acceptance covers baseline-relative endpoints, length, ordering, explicit
stroke, cross/inverted precedence and unchanged body dimensions.

## Test-only legacy wrapper boundary

### Evidence before decision

compiler/math/layout/cancel.rs:21 contains the historical Clippy inventory
item. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains sources/owners/references/UTC/status/diffstat. Call-site and productive-path
research precedes classification, without new parity/performance evidence.

### Contract and acceptance

Compile internal layout_cancel only under `#[cfg(test)]`, preserving signature/body
for existing tests. Productive dispatch uses layout_cancel_elem with the complete carrier and occurrence; the wrapper preserves its historical fixture without a new runtime. MathCancelExplicit is wrapper-only and also cfg(test); MathCancelAngle and MathCancelElem remain productive.

Preserve productive routes, test names/bodies, external APIs, defaults and phase.
This ADR-0127 internal correction requires all workspace-target build/check,
core suite, test inventory, Clippy and lineage; remove no expectation to pass.

## Pure numerical backend

### Evidence before decision

cancel.rs:60,63,96,97 contains the disallowed_methods calls. HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz`
retains source/owner/config/libm/compiler/UTC/status/diffstat. host-math compares
backends before choosing: 4968 cases, 69 trig/atan2 differences up to 1 ULP,
without numeric-class/signed-zero change; powers/hypot agree in the sample.
This is host evidence, not universal proof or new vanilla parity.

### Contract and acceptance

Use libm::hypot/atan2/sin/cos for diagonal, auto angle and endpoint projection.
Preserve atan2(width,height), signs, baseline-relative center, length,
cross/inverted/background, stroke/callback and body logical dimensions. Do not
alter nonfinite policy while changing the numeric backend. This ADR-0127 internal
correction adds no public contract/default/phase. Preserve geometry/language
tests and compare measured SVGs before/after, reporting differences; geometry
does not certify all cases/platforms. Validate build/all targets/core/Clippy/
lineage. Do not suppress the lint or substitute compiler-byte identity for
acceptance.

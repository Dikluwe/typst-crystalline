# L0 — compiler/eval/operators/ordering
Hash do Código: 7e791199

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/ordering.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Contract

Own the single language ordering: `compare`, crate-private, used by the four
operators, by sorted and by calc.min/max, with cmp_arrays, length_partial_cmp and
rel_partial_cmp. The operators have no separate numeric path: an undefined order,
NaN included, is always an error. Preserve numeric Int/Float comparison and exact Int/Decimal
comparison, homogeneous Decimal, and Fraction's existing own comparison.
Compare boolean, string, version (zero-padded), duration, angle and ratio in
their current public domains. Length/Relative comparisons require the existing
component compatibility; do not resolve em/percentage against an invented
reference. Datetime requires compatible date/time/datetime kinds.

Arrays compare lexicographically and recurse; preserve the first inner error
rather than replacing it with an outer array error. Known comparable families
with an undefined relation (including incompatible components/nonfinite cases
as implemented) use canonical repr operands in the existing 'cannot compare'
form; unsupported families use canonical long type names. Datetime kind mismatch
uses its distinct nominal diagnostic. Preserve dedicated-before-generic branch
precedence and comparison-derived boolean results.

A failing Int with Float pair shows the Int as Float when the conversion is exact,
and as Int otherwise, so the message never names another number. A failing Length
with Relative pair shows the Relative's length part. NaN ratios, angles and
fractions fail; the reference's internal total order for them is not reproduced.

Do not invent Decimal/Float or cross-Fraction coercion, a numeric tower or
document-dependent ordering. Unmeasured negative-fraction CLI cases remain
Unknown rather than granting a general parity claim.

## Acceptance

RED→GREEN for NaN with Int and Float in the four operators, in conditions and
nested arrays, large Int text, and every comparable family through `compare`.
Preserve short-circuiting and the incomparable controls.

## Evidence

Package approved on 2026-11-24 (“Aprovo A–F”), refined by the owner's instruction
not to copy proven reference errors: `00_nucleo/diagnostics/value-comparison-package-2026-11-24.pt-BR.md`. Earlier
source anchors of this owner are at commit `f2c737125`.

# L0 — compiler/eval/operators/ordering
Hash do Código: 62f07e39

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/ordering.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/ordering.rs:18` — `enum CompareFailure {`.
- `01_core/src/compiler/eval/operators/ordering.rs:26` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/ordering.rs:107` — `fn value_cmp`.
- `01_core/src/compiler/eval/operators/ordering.rs:167` — `fn datetime_kind`.
- `01_core/src/compiler/eval/operators/ordering.rs:176` — `fn datetime_cmp`.
- `01_core/src/compiler/eval/operators/ordering.rs:197` — `fn partial_or_values`.
- `01_core/src/compiler/eval/operators/ordering.rs:208` — `fn cmp_arrays`.
- `01_core/src/compiler/eval/operators/ordering.rs:221` — `fn length_partial_cmp`.
- `01_core/src/compiler/eval/operators/ordering.rs:237` — `fn rel_partial_cmp`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/ops.rs:471-500`.
- `foundations/ops.rs:514-530`.
- `layout/length.rs:195-204`.
- `layout/rel.rs:193-202`.

## Contract

Own comparison operators, value_cmp, cmp_arrays, length_partial_cmp and
rel_partial_cmp. Preserve numeric Int/Float comparison and exact Int/Decimal
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

Do not invent Decimal/Float or cross-Fraction coercion, a numeric tower or
document-dependent ordering. Unmeasured negative-fraction CLI cases remain
Unknown rather than granting a general parity claim.

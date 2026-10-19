# L0 — Independent mathematical position formula oracle
Hash do Código: b8fef30a

**Layer:** L1, `#[cfg(test)]` only
**Exclusive consumer:** `01_core/src/testing/math_oracle.rs`

## Evidence and qualification

The historical formula readings and confirmations are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/testing/math_oracle.md`.
The source coordinates below refer to those historical vanilla readings, not a
fresh measurement against the current upstream pin. Each oracle function must
retain its literal formula and confirmed upstream `file:line` in its doc-comment,
with a reference to the record that confirmed it.

## Contract

This is a verification module, never a production execution path. Provide pure
functions, one per already read and confirmed position formula, for geometry
tests to compare production behavior with an independent reference instead of
unattributed hardcoded numbers.

Functions take primitives (`f64`, points); MATH constants and dimensions arrive
as arguments. Do not import MathLayouter, FontMetrics, Content or other
production code: the oracle must not share the engine's failures. Expose it only
through the `#[cfg(test)]` module tree and never compile it in production.

Do not admit a formula without a confirmed upstream `file:line`; reread the
source first under ADR-0108. Formulas read but deliberately not implemented in
Crystalline, including the fraction-like binom form in the archived record, stay
excluded: the oracle mirrors intended engine behavior, not all of vanilla.

## Confirmed formula obligations

| Function | Formula | Historical source |
|---|---|---|
| `large_operator_upper_shift(base_ascent, t_descent, gap_min, rise_min)` | `base.ascent + max(rise_min, gap_min + t.descent)` | `typst-layout/src/math/scripts.rs:290-313` |
| `large_operator_lower_shift(base_descent, b_ascent, gap_min, drop_min)` | `base.descent + max(drop_min, gap_min + b.ascent)` | `scripts.rs:290-313` |
| `grid_total_descent(rows, gap)` | `d_1 + Σ_{r≥2}(a_r + d_r) + gap × (n−1)` | `typst-layout/src/math/table.rs:103-106` |
| `grid_axis_baseline(height, axis)` | `height/2 + axis` | `table.rs:188` |
| `radical_gap_redistribution(gap, sqrt_height, thickness, radicand_height)` | `max(gap, (sqrt_height − thickness − radicand_height + gap)/2)` | `radical.rs:76`, TeXbook p443 item 11 |

## Revoked formula and pending scope

The owner-confirmed drawing review revoked
`grid_cell_dy(baseline_offset, row_ascent, cell_ascent) = baseline_offset + row_ascent − cell_ascent`
(the earlier translation of `run.rs:137`). Baseline-relative items require simply
`baseline_offset`; the historical review is in the archived source and
`layout_grid_boxes` record. Do not encode the revoked formula as an oracle.
A future nontrivial cell-anchor formula requires independent confirmation first.

The local consumer also contains later radical-degree, radical-offset and
post-subscript helpers. Their presence does not by itself close missing
specification evidence: retain their independently confirmed doc-comments and
review their formula provenance before extending this table. This documentary
consolidation does not derive new mathematical requirements from code.

## Acceptance and use

Geometry tests call the independent function with the production calculation's
inputs and compare with the specified tolerance (typically 0.01pt). Migrate
unattributed expected values to oracle calls as each formula is admitted.
Retain the initial migration obligations for four limit-shift assertions and the
grid-total-descent assertion identified in the archived record, including their
existing test identities. Do not use this oracle to claim new bilateral parity,
performance or production-phase behavior.

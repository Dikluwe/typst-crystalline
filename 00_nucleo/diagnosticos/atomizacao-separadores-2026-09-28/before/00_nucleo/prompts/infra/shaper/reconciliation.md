# L0 — infra/shaper/reconciliation
Hash do Código: 6146b980

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/reconciliation.rs`

## Current evidence

- `03_infra/src/shaper/reconciliation.rs:28` — `fix_line_positions`.
- `03_infra/src/shaper/reconciliation.rs:98` — `reconcile_rtl_positions`.
- `03_infra/src/shaper/reconciliation.rs:149` — `fix_line_positions_page`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/reconciliation.rs`.

## Owned contract

- Own fix_line_positions and helpers. Adjust only top-level page.items under the existing nonrecursive policy, grouping baselines within 0.5pt and ordering by physical x; preserve y.
- Skip the entire line if it contains math TextShaped or Glyph.
- LTR anchors its first item and accumulates real-minus-estimated advances while preserving layout gaps. Nontext widths remain stable under the existing supported cases.
- RTL captures `gap[i]=x[i+1]-(x[i]+w_est[i])` and `right_edge=x[last]+w_est[last]`; reconstruct `x_real[last]=right_edge-w_real[last]`, then `x_real[i]=x_real[i+1]-gap[i]-w_real[i]`. Preserve normal/justified gaps and the right edge; do not replace them with fixture constants or recompute layout decisions.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

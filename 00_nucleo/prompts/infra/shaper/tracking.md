# L0 — tracking
Hash do Código: 3dd060a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-tracking.toml sha256:4223de1e6a3e82254979134ff4c35c82f22c052e7231c0b058b8954271329fbe

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/tracking.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before decision

Inspection of text_measure.rs:85, word_layout.rs:89 and space.rs:162 found text
boundaries, baseline admission and leading-space integration tests in other owners.
Exact state/sources/prior prompts are in
`lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json`
and before/. The measured correction is in
`lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; this
separation preserves its results/limits. Position/text/style are language
observables; function location is mechanics. No historical intent or new bilateral
parity follows from this inspection.

## Current contract

Own only crate-private residual(glyphs,index,style,upem) and derived width.
residual returns zero if current/next glyph is absent, clusters equal, size is
nonpositive/nonfinite or tracking absent/zero/nonfinite. Between distinct clusters,
resolve tracking in points and subtract its rounded i32 representation converted
back to points; upem is at least 1. Preserve operation/cast order and semantics.

width sums each glyph's advance converted to points plus that index's residual.
Do not acquire fonts, mutate glyphs, cache, select FrameItem boundaries or emit
PDF. This same calculation feeds measurement/reconciliation/PDF positions without
extra Tc or a new Glyph contract. Keep positive/negative residual, internal-cluster
and final-glyph tests.

## Acceptance and limits

Preserve existing causal tests and pre-separation text/positions/styles/pages/
diagnostics. Require 1:1 ownership, valid pins, build/lint without errors. Internal
ADR-0109/0127 movement uses descendant static functions without new state. Mixed-
style/RTL/global-selection/cluster-count tracking limitations remain exactly as
recorded in the correction diagnostic.

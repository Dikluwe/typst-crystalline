# L0 — infra/export/p1250_svg_integrated
Hash do Código: d64d3985

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/tests/p1250_svg_integrated.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/tests/p1250_svg_integrated.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## SVG integration harness

Own the existing small integration corpus combining tiling, clips, destinations,
images, glyphs, gradients and strokes. Preserve logical_number:1 in synthetic
pages under `00_nucleo/prompts/entities/layout_types.md`; SVG visual oracles do
not derive their geometry from that PDF logical number.

Check unique local IDs and resolved references, supplied geometry/transforms/
clips, visible alpha/payload and unknown-destination fallback. Exercise coexistence
of glyph/paint/pattern/clip/destination graphs without identity capture.
Productive behavior is specified by the individual
`00_nucleo/prompts/infra/export/svg/` owners, not a second contract here.

## Verification

Use the existing independent small visual/graph corpus. Do not validate mutant
catalogs, historical percentages, approval records or frozen benchmark bytes as
language parity.

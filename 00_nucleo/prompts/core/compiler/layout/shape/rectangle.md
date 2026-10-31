# L0 — resolved rectangle decoration
Hash do Código: a2650d54

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/rectangle-decoration.toml sha256:dbc5dd8b047bdebcd6adf912663a2d5e50b50ad46d4af6c6cf7beff23097ded8


## Evidence and exclusive ownership

Own only `01_core/src/compiler/layout/shape/rectangle.rs`. The 8583 before/combined
snapshots and baseline.json bind exact source/UTC/diffstat and the blue square
without its requested side strokes. rect.rs:44–56 discards sides and the radius
caster truncates ratios. The approved entity contract is retained in
`00_nucleo/prompts/core/entities/elements/shape.md`, approval.json binds the user.
Language geometry and paint are acceptance, not upstream Rust steps or bytes.

## Contract

Own pure layout free functions below shape.rs. None carriers decline to the
legacy emitter. Resolve relative corners using the actual shorter frame axis
and occurrence font, clamped to half the shorter axis. Preserve original logical
frame, position and paint bbox. Uniform sides retain one prepared stroked Shape.
For unequal default solid-contour styles, derive outer/inner elliptical offsets
from the shared fill corner centers; take half corners per side and merge equal
paints into compound paths so translucent adjacency is painted once. Zero
radii retain mitered straight corners; collapsed inner radii stay nonnegative.
Nondefault dash/cap/join/miter styles retain original side centerline/stroke
fields through prepared FrameStroke instead of flattening them to a solid.
Never move geometry into entities or sample Paint to Color. No I/O/global state.

## Acceptance

Before implementation require the actual source-level pipeline RED; after code,
require both carrier failures GREEN, official PDF/SVG/raster without white gaps,
independent radius/side tests, aliases/fonts/dimensions, omitted/None/empty,
alpha/rich paint/styles, unchanged legacy controls, suites/build/all-target,
zero lint errors/warnings and scoped forward/reverse seals with full provenance.
The finite literal does not certify global Auto/Block/Box/square geometry.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Apply the approved rectangle contour construction above, preserving all
coefficients until owning layout and all prepared paint/style fields.

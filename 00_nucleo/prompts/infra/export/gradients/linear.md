# L0 — infra/export/gradients/linear
Hash do Código: 7237adb4

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/linear.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/linear.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Axial coordinates and RGB samples

Own compute_axial_coords and multispace_sample_stops for PDF axial shading.
Endpoints are symmetric around (x0+w/2, y0+h/2), using dx=cos(angle), dy=sin(angle)
and half offsets (w*dx/2, h*dy/2). Preserve horizontal/vertical cases and bbox units.

Sample max(n_samples,2) points uniformly from 0 to 1 in f32. Use Linear::sample
in its declared color space, then to_rgba_f32; discard alpha in this PDF RGB
sampler and clamp each RGB channel to [0,1]. Do not force Oklab or special-case
two original stops as two samples. PDF shading/function objects belong to
`00_nucleo/prompts/infra/export/builder/resources.md`; native interpolation
belongs to the gradient entity, not this adapter.

## Verification

Check centered endpoints, sample endpoints/cardinality, each declared interpolation
space and bounded RGB channels. The sampler's alpha omission is a limitation;
SVG's adaptive alpha-preserving path has a different owner.

## Private import maintenance

### Evidence

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`
retains sources/owners/spans/original compiler output, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
Selected warnings occur at linear.rs:12. Inspection distinguishes productive
references, descendant-test imports and comments; no new parity measurement.

### Contract and acceptance

Remove only private imports proven unused here; keep public reexports and imports
required by calls/contracts. Exact pre-edit spans remain in the diagnostic.
Internal continuous ADR-0127 maintenance changes no public signature/default/
phase/formula/acceptance. No lint suppressions. Revalidate all-target compile,
infra test names/execution, build/workspace Clippy/Rustdoc/lineage with scoped
resealing. No import-removal/comment-format mirror tests.

## Platform-independent numeric components

### Evidence

`lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`
retains library/consumer/owner/manifest/lock/spans/HEAD/uncommitted tree/UTC/status/
diffstat. linear.rs:21–22 uses Clippy-flagged methods; clippy.toml requires
platform-independent math and libm =0.2.11 is already pinned in core/Cargo.lock.
before-renders.json records 24 sources rendered to PDF/SVG by Tekt and historical
vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, binary hashes/commands;
before-comparison.json measures whole canvases without error-adjusted masks.
Pre-existing divergences prohibit complete parity claims. The initial invalid
hsv-outside-color fixture failed parsing before productive edits and remains in
initial-fixture-error.log; cases.json contains valid inputs.

### Contract and acceptance

Use libm::cos/libm::sin from 0.2.11 for f64 axis components. Preserve center/
dimensions/signs/arithmetic order/all samplers; do not move formulas to L1 or add
public signatures. Pin direct L3 dependency =0.2.11, reuse the locked version,
without package upgrades/I/O. Continuous ADR-0127 changes no public/default/phase/
strategy; L0 first, involved-owner resealing.

Validate existing formula tests, before/after exports of those same 24 documents
and raster/structural PDF/SVG observations. Separate local preservation from
historical-vanilla divergences; analyze any new render difference. Do not expand
tolerances/limits, change fixtures or suppressions to hide failure. All-target
compile/infra suite/build/Clippy/Rustdoc/fmt/lineage; counts/file identity certify
no language parity. Historical measurements do not certify the current oracle.

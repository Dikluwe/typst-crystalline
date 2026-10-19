# L0 — infra/export/gradients/radial
Hash do Código: 7ddce23d

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/radial.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/radial.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Radial coordinates and RGB samples

Own compute_radial_coords and multispace_sample_stops_radial for PDF radial
shading. Preserve the current coordinate-helper signature: center, radius,
focal_center, focal_radius, width and height.
Resolve center/focal ratios against width/height; both radii scale by min(width,height).
Return (fx,fy,fr,cx,cy,r). Default focal=center and focal_radius=0 are supplied
by the domain/caller, not a second option policy here.

Sample max(n_samples,2) uniform f32 points through Radial::sample, convert to
RGBA, discard alpha and clamp RGB to [0,1]. Function/shading dictionaries and
bbox placement are owned by
`00_nucleo/prompts/infra/export/builder/resources.md`.

## Verification

Check independent focal circles, unequal bbox dimensions, centered defaults and
bounded multispace RGB samples. Do not claim alpha-preserving SVG support from
this PDF sampler or exact upstream algorithm parity.

## Private import maintenance

### Evidence

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz`
retains sources/owners/spans/original compiler output, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
Selected warnings occur at radial.rs:12,13,14. Inspection distinguishes productive
references, descendant-test imports and comments; no new parity measurement.

### Contract and acceptance

Remove only private imports proven unused here; keep public reexports and imports
required by calls/contracts. Exact pre-edit spans remain in the diagnostic.
Internal continuous ADR-0127 maintenance changes no public signature/default/
phase/formula/acceptance. No lint suppressions. Revalidate all-target compile,
infra test names/execution, build/workspace Clippy/Rustdoc/lineage with scoped
resealing. No import-removal/comment-format mirror tests.

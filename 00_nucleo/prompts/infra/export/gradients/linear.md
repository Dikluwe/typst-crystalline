# L0 — infra/export/gradients/linear
Hash do Código: 1e59d7bf

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/linear.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/linear.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Axial coordinates and RGB samples

Own compute_axial_coords and multispace_sample_stops for PDF axial shading.
Endpoints are symmetric around (x0+w/2,y0+h/2), using dx=cos(angle), dy=sin(angle)
and half offsets (w*dx/2,h*dy/2). Preserve horizontal/vertical cases and bbox units.

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

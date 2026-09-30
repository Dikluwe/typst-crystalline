# L0 — infra/export/gradients/conic
Hash do Código: e581e3e0

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/conic.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/conic.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF conic Coons mesh

Own conic RGB sampling, circular-arc control points, patch-count helpers and
binary Type 6 Coons mesh emitters for RGB, CMYK and spot. Preserve this mesh
strategy without claiming upstream tessellation identity.

The base patch-count helper returns stops.len(); the extended helper returns
stops.len()*4. Preserve each emitter's existing minimum/degenerate handling,
angular origin and continuous progression. Arc controls use tangent offset
radius*(4/3)*tan((end_angle-start_angle)/4), not an extra cos factor.
Mesh coordinates use the current unit circle centered at (0.5,0.5), radius 0.5;
the builder resources owner applies physical bbox placement.

RGB samples through Conic::sample and converts/clamps process RGB; CMYK retains
native CMYK or the existing conversion helper fallback. Preserve the current
binary flag/control-point/component encoding in Vec<u8>, never UTF-8 strings.
Spot retains the same geometry and writes four one-byte corner tints (clamped
to [0,1], quantized by round(tint*255)) instead of RGB triples.
The Separation and one-component Decode dictionary belong to
`00_nucleo/prompts/infra/export/builder/resources.md`.

## Verification

Check geometric continuity, angular direction, process/CMYK/spot component widths
and valid Type 6 decoding. Binary encoding validity is a format contract; patch
counts and equality to an upstream algorithm are not language parity.

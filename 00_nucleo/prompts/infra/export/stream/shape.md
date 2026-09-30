# L0 — stream/shape
Hash do Código: 9042e78a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/shape.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/shape.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Paint and geometry primitives

Own process/spot paint operators, paths, rounded rectangles, fill/stroke and line
caps together. Consume ShapeKind<Pt>/Corners<Pt> resolved in L1; clamp radii to the
available geometry without resolving Length or discarding relative context.
Preserve path segments, curves, ellipse and fill rules. Butt omits 0 J, Round emits
1 J and Square 2 J; a missing line color retains the empty prefix. Isolate state
with the existing graphics envelopes.

Process colors preserve rg/RG. Spot references use complete colorant identity,
deterministic resource names and the existing u8-quantized tint. Gradient paint
finds the prepared Pattern using its DedupKey/bounding box; spot gradients use the
shape box consistently with resource collection. Preserve process-gradient keys,
tiling and missing-resource fallbacks. Do not create resources, choose subsetting
or compress streams.

## Verification

Check four resolved corners, paths/fill rules, cap variants, spot tint/resource
closure and gradient bounding-box lookup in top and local painting.

# L0 — svg/groups
Hash do Código: dfc5d2fb

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/groups.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/groups.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Groups and local clips

Own render_group, geometric clip validation/writing and matrix-identity checks.
Recurse through façade dispatch. Position/transform apply once; local clips and
children share the same spatial basis. Nested clips intersect with ancestors.
None adds no clip. Finite representable Rect/RoundedRect/Ellipse/Path creates a
local clipPath and resolved reference: dimensions use inner_width/height, corners
remain distinct and paths preserve subpaths/curves. Share collision-free deterministic
clip IDs with images. Allocate a new definition for each accepted clip; this
consumer does not deduplicate equivalent clip definitions.

clip_mask carries no fill-rule. Paths requiring evenodd, open lines without area,
nonfinite/unrepresentable or absent information remain Unknown/CONTRACT-GAP;
do not claim missing clipping is equivalent or silently drop the content. Alpha masks,
filters, opacity and rasterization are not substitutes for geometric clipping.

## Verification

Check supported shapes, transformed/nested clips, unique clip IDs, reference
closure and opaque/nonfinite/fill-rule gaps.

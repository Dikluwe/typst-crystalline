# L0 — svg/geometry
Hash do Código: 7dca7ec8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/geometry.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/geometry.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Shapes and stroke geometry

Own line/shape rendering, generic path and rounded-rectangle path helpers, fill/
stroke/cap/join/dash/fill-rule attribute writers. Consume PaintReference from paints;
do not reconstruct paint, layout or Length. RoundedRect receives four resolved Pt
radii. With stroke, use max(radius - thickness/2, 0) per corner on the stroke-center
path, separating fill and stroke so the external radius remains the declared radius.
Without stroke use resolved radii directly. Fill closes its path; rounded stroke
returns to its start without Z, preserving cap/join morphology.

For Shape stroke attributes, preserve cap/join/miter limit, dash array order and
phase including negative values;
resolve DashLength::LineWidth using effective thickness. Omit attributes only when
semantically identical to SVG defaults and supplied carriers; do not bake transforms
into stroke lengths. Keep alpha and operation order. Use supplied evenodd/nonzero
fill rule without inferring winding. The separate FrameItem::Line writer currently
emits butt regardless of the line-cap carrier; do not claim cap parity for that path.

An exactly open MoveTo+LineTo path uses translate(first point) and a relative line
delta, without changing geometry/paint. Closed, cubic and other paths retain generic
serialization. Rounded-path helpers also serve groups' geometric clips without
taking ownership of clip transforms or support validation.

## Verification

Check four radii, stroke-center inset, open paths, complex dash/cap/join, alpha,
fill-rule and resolved relative-radius inputs.

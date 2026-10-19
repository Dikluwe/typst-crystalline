# L0 — infra/export/gradients/mod
Hash do Código: 6ec5e88f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/mod.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Gradient discovery façade

Own PatternRef, GradientObjectKind/GradientObject, RectKey/DedupKey, bbox/key helpers,
scan_all_gradients, pattern_resources_for_page and restricted child reexports.
Keep private static child modules; neither formatting functions nor sampling/
mesh algorithms are duplicated here.

Traverse page background, body and foreground recursively through Group/Link/
Semantic. Register Shape.fill and Shape.stroke Gradient paints in encounter order
with the same effective key; reuse matching fill/stroke identity once.
DedupKey combines gradient Arc pointer with effective bbox quantized by the existing
RectKey representation. Group passes its local bbox context; the shape's own
parent_bbox_at_emit wins over the inherited override.

Spot gradients additionally retain the painted shape rect for placement and lookup,
separate from a container's relative-parent bbox. The same spot gradient reused
at different shape locations must retain those placements; process keys preserve
their existing policy. Keep sequential allocated pattern/shading/function IDs and
recursive page resource lookup synchronized with discovery, without collisions
or unresolved references. Do not include nonvisual alt as paint.

## Delegated owners

- `00_nucleo/prompts/infra/export/gradients/linear.md`: axial coordinates/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/radial.md`: focal circles/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/cmyk.md`: CMYK sampling/conversion.
- `00_nucleo/prompts/infra/export/gradients/relative.md`: relative tuple transforms.
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`: PDF counts/SVG adaptive stops.
- `00_nucleo/prompts/infra/export/gradients/conic.md`: Type 6 mesh payloads.
- `00_nucleo/prompts/infra/export/gradients/function_dict.md`: interpolation functions.

`00_nucleo/prompts/infra/export/builder/resources.md` owns emitted resource objects
and shading placement; `00_nucleo/prompts/infra/export/stream/shape.md` owns paint
operators. This façade performs no I/O or entity interpolation.

## Verification

Check recursive three-layer discovery, fill/stroke reuse, distinct effective boxes,
spot placements and page-local reference closure. Pointer/key mechanics preserve
the current implementation; they do not define language equality.

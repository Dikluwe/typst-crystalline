# L0 — infra/export/pdf_tiling
Hash do Código: 780d8ec5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/pdf_tiling.rs`

## Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains sources/UTC/HEAD/working tree: stream/shape.rs:156/204 reduces Tiling to
color, builder.rs:277 reserves only gradients. Approved manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
requires native prepared cells. Historical vanilla 9f2b6e871 paint.rs:161–195
prepares a local-root/corrected-transform cell stream; this is supporting source,
not a requirement to copy every upstream mechanical choice.

native-v1/dense-open-tekt.pdf used Matrix [1 0 0 1 0 139] with Y-up paths; 144-DPI
pdftoppm shifted lines/merged 0.5pt columns. The oracle's [1 0 0 -1 20 120] uses
Y-down. Continuous geometry equivalence did not preserve raster pattern coverage.
The measured refinement therefore uses Y-down local items and to_pdf*placement;
this supersedes the earlier double-flip Y-up expression only for this cell frame.

Inspection found global Pattern names/first Pattern.page unsafe for shared Arc
cells reused after page selection. The refuter clones pages with nested tiling
and selects only the second. verified-v3/followup-clippy-inventory.json and
followup-architecture.log (candidate-final/workspace-pre) record four forbidden
f32::hypot, an extra argument, two productive items after test modules and two
new neutral wildcards.

`tiling-overlap-2026-10-14/before.json.gz` retains sources/owners/HEAD/UTC/tree.
Final renders/remaining-analysis.md in tiling-nativo-2026-10-14 observe 8×8 cell,
4×4 pitch, 50% alpha: opaque-red PDF, one-contribution SVG, four-contribution PNG
(255,15,15). pdf_tiling.rs:184 BBox=size, svg/paints.rs:148 overflow visible and
shape/text alpha loss explain observable composition defects. Vanilla also clips
overlap, which is no requirement to reproduce the defect. Explicit period
composition is a hypothesis refuted by changed nonoverlap controls or disagreement
with analytic source-over, including distinct-color order. Full chronology:
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/export/pdf_tiling.md`.

## Current private contract

Register patterns by borrowed Shape occurrence and cell Arc identity without
serializing addresses; names/IDs are local deterministic mechanics. Traverse
Groups with local matrix and logical paint_root; Link/Semantic contain visual
items only. Each nested cell owns its canvas and identity root. The frame graph
is finite with no mutable backreferences.

The affine tiling helper chooses Self/Parent and normalizes periods. Reject invalid
placement explicitly, never substitute solid. Emit borrowed cell items without
cloning occurrences or invalidating gradient keys, in local Y-down; matrix is
`to_pdf * placement`, nested cells start to_pdf identity and their gradients use
the same Y-down frame. The original direct-native-overlap/BBox=size Pattern
proposal is superseded by period composition below, not a fallback.

Emit the cell vector body once into a Form XObject with BBox=size. Reserve two IDs
per occurrence, pattern and form. PatternType 1 / PaintType 1 / TilingType 1 uses
BBox=pitch, positive XStep/YStep=pitch=size+spacing, and translated Form references
for each helper-supplied contribution. Form resources retain fonts/images/
ExtGState/gradients and only descendant tiling; Pattern resources reference only
its Form. Do not add an isolated transparency group changing leaf composition.
Preserve placement and semantic isolation. Nested pattern default user space
remains the cell's; test before promotion. Native PDF overlap is no longer relied
on for negative spacing.

During cell emission disable tagging/MCIDs/UA1 and restore context before page
emission. No cell annotation/destination. Clip cell content/overflow to its BBox.
Share font/image/spot/gradient/nested-pattern resources without cycles: each pattern
references only descendants. Page resources include every used pattern/descendant.
Names are per page; every descendant is emitted with the selected page's resources,
never requiring an excluded page to have emitted them. Assembly returns (ID,bytes)
objects to the owning builder's existing private storage API.

Explicitly enumerate FrameItem leaves without paints; no neutral wildcard, raised
limit or allow. Future variants require an explicit decision. Preserve suite/
renders before/after plus fmt/doc/lineage.

## Acceptance and unresolved scope

Within the approved native-tiling package (historical T1–T3), no new API/phase/
flag/default or publication authorization. L0 first, RED→GREEN, analytic composition
including color order, PDF/qpdf structure, bounds/cell/spacing/offset/rotation/
affine Parent/stroke/density, nested text/images and semantic isolation. Check
simple/nested/rotated/text/image controls and page extraction/selection, especially
second-page-only nested reuse. Render/diagnostics are language acceptance; IDs/
bytes certify no parity. Full build/workspace suite/fmt/Rustdoc/lineage green.
Variable-gradient alpha and PNG quality remain incomplete; do not certify a
production package with these open issues. Later publication remains out of scope.

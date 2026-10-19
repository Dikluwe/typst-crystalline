# L0 — infra/export/tiling
Hash do Código: b99cdfdd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:d6e2fba3d2143daec23bcf7a8d959ce59600f3b8ccc5fa35a853c4f6831e51e0

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/tiling.rs`

## Evidence and approved boundary

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/UTC/tree/diffstat/sources. Legacy layout/tiling.rs only cancels shape
translation; SVG ignores paint_root and paints.rs accepts only size+Color. Approved
manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
requires full affine transformation, T(offset)*R and local root. Historical vanilla
9f2b6e871 visualize/tiling.rs:375–377 composes translate/rotate, PDF paint.rs:171–173
corrects transform before placement and SVG lib.rs:346–358 separates self/parent.
Parent/large-offset probes belong to that approved contract. Observable geometry,
not upstream structure, determines acceptance.

images/scan.rs:57, 139, 273, fonts.rs:80 and font_binding.rs:124 use the semantic
visitor, while layout_types.rs:1370 visits Shape without paint descent. Prepared
transport would otherwise hide resources previously in Group.
verified-v3/followup-clippy-inventory.json/followup-architecture.log (candidate-final/
workspace-pre) records four forbidden f32::hypot, an extra argument, two productive
items after test modules and two new neutral wildcards.

`tiling-overlap-2026-10-14/before.json.gz` retains sources/owners/HEAD/UTC/tree;
final-renders/remaining-analysis.md for tiling-nativo-2026-10-14 observes 8×8 cell,
4×4 pitch, 50% alpha: opaque PDF, one SVG contribution, four PNG (255,15,15).
BBox=size at pdf_tiling.rs:184, SVG overflow visible at paints.rs:148 and discarded
alpha at shape.rs:155/text.rs:275 identify observable defects. Vanilla also clips
overlap, no requirement to copy it. Explicit period composition must preserve
nonoverlap controls and match analytic source-over; that is a refutable hypothesis.

## Private geometry and resource contracts

Own shared exporter helpers for finite affine inverse, dimensions/pitch/placement
validation and cell→Shape-containing-frame transformation. No layout, resource
loading, style decision or format emission here.
Self composes T(shape.pos)*T(offset)*R(angle). Parent composes full inverse
local_to_root with T(offset)*R(angle); shape.pos already belongs to the Shape path,
never subtract twice. local_to_root is identity at Page/cell/Group paint_root;
technical groups concatenate T(pos)*matrix. Normalize final translation in grid
basis: invert linear part, reduce each axis by pitch, reconstruct. Whole-period
offsets retain phase under rotation/scale/reflection/shear, including large offsets.

Reject singular/nonfinite matrices and nonpositive/nonfinite cell/pitch before
integer conversion/repetition. Return private Option; caller cannot replace None
with solid. Invalid SVG keeps explicit unrepresentable-transform marker and paint
none, never successful-render credit. L1 preparer normally diagnoses invalid domain;
singular Group/external Rust frames still require this local guard.

Private visual-resource walker delegates leaves to visitor and descends Group/
Link/Semantic/TextReplacement and fill/stroke TilingFrame. No semantic/link callbacks
for cell, public FrameVisitor/navigation/MCID change or I/O; existing owners retain
decoding/font logic. Enumerate every FrameItem leaf explicitly so new variants
cause compile errors; productive helpers precede tests, no changed behavior,
raised limits or allow.

Private contribution iterator emits intersections with period [0,pitch), ordered
y increasing then x increasing. Per axis indices are 1-ceil(size/pitch) through
zero inclusive, translated index*pitch. Cell clips at size, period at pitch;
no viewport-proportional clone vector. Finite positive cell/pitch are transport
preconditions; check representable count before conversion. Extreme-limit diagnostic
integration remains open: never truncate repeats or claim success.

## Acceptance and unresolved scope

Analytic tests distinguish Self/Parent/full composition/grid-basis normalization/
rotation pivot/guards. Source→native-target controls must observe actual visual
geometry; green helpers do not complete prepared transport/native integration/
parity slices (historical T1c/T2/T3). Retain public APIs/defaults/language, no silent
fallback/N-clone expansion/universal byte criterion. L0 first, RED→GREEN, analytic
composition including distinct-color order, simple/nested/rotated/text/image/page
extraction-selection controls, suite/build/doc/fmt/lineage and full-contract
refuters. Internal continuous ADR-0127 adds no API/phase/flag/default and grants no
publication. Variable-gradient alpha/PNG quality remain incomplete; no production
certification with these open. Original provenance:
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/export/tiling.md`.

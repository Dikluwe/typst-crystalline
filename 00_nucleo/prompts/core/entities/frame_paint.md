# L0 — entities/frame_paint
Hash do Código: 5e19d16f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/frame_paint.rs`

## Evidence before transport contract

`lab/results/construction/diagnostics/tiling-transporte-2026-10-14/{parent-before,solid-controls}/results.json.gz`
records HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, uncommitted tree/UTC/
diffstat/status/sources/owners/argv/binary hashes. Historical upstream pin
9f2b6e8715237cb086899a42873660fe744622e8, tree6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4,
oracle SHA2567fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea,
uses the archived P1489 recipe, not the current ratified oracle.

layout/tiling.rs:124-127 returned None above 10000 cells; shape.rs:151-185
materialized fill only, leaving declarative stroke; export/stream/shape.rs:151/201
reduced Tiling to color; layout_types.rs:505/531 carried neither cell nor paint
root; render.rs:217 reduced fill to color and 579 ignored clip_mask. These are
transport limits, not only offset formulas. The prior tiling-cobertura record
observed solid output over-limit 200×100pitch1 and unexplained stroke; unchanged
sources do not turn it into a fresh binary measurement.

At 240×192 white/librsvg, SVG place-parent differed in 1194 pixels by>16 and
box-fixed/auto-parent in 1192, while Solid controls matched exactly. block-parent
had 2 pixels>16,max 23. Upstream SVG/source typst-svg/lib.rs:346-358 and
inline/box.rs:35-49 show T(offset)*R, inverse inherited transforms and local
roots before final inset; frame.rs:459-474 distinguishes Hard/Soft.
This is geometry semantics, not required FrameKind/Packed/Rust equality.
Solid controls also refuted general box-place/rotate geometry (3200/5296/3645
pixels>16), requiring their own owners rather than automatic transport closure.
The older quarter control matched Tekt PDF/SVG with vanilla SVG but not vanilla
PDF; do not copy that known upstream discrepancy.

## Ratified pure carriers

Declare pub mod frame_paint in the entity facade. Exact types derive Debug/Clone:

```rust
pub enum FramePaint {
    Solid(Color),
    Gradient(Gradient),
    Tiling(Arc<TilingFrame>),
}
pub struct TilingFrame {
    pub frame: Frame,
    pub spacing: Size,
    pub offset: Point,
    pub angle: Angle,
    pub relative: RelativeTo,
}
pub struct FrameStroke {
    pub paint: FramePaint,
    pub thickness: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Option<DashPattern>,
    pub miter_limit: f64,
    pub specified: StrokeFields,
    pub overhang: bool,
}
```

Use existing Color; Gradient/RelativeTo Self_/Parent (no Auto); Frame/Size/
Point/Angle; and geometry fields. frame.size is physical cell size; signed
spacing is intercell distance; offset resolves against size+spacing; Angle
remains radians. Arc breaks physical recursion. No exporter/I/O/dynamic-runtime
dependency, and no layout/render computation in this unit.
FrameStroke preserves every Stroke field except resolved paint; any Default
must equal Stroke::default. From<Color>/From<Gradient> are allowed; total
From<Paint> is forbidden because it would hide Tiling layout or reduce to color.
Do not derive language equality for these carriers.

The approved integration replaces Shape fill/stroke with Option<FramePaint>/
Option<FrameStroke>, preserving other fields including parent_bbox_at_emit.
Group gains paint_root:bool: false inherits, true sets local origin and
inner_width/inner_height paint root. Page/TilingFrame are implicit roots.
No new FrameItem variant or FrameVisitor signature. External Shape/Group
literals/destructuring/type expectations break and need explicit migration,
including fixtures. Declarative Paint/Stroke/Tiling, Value,TextStyle and export
APIs retain types/signatures/defaults; declarative Tiling never gains Frame.
The earlier resolved-carrier prohibition is superseded only for FramePaint.

## Approval, acceptance and incomplete integration

Human "Aprovar a proposta" approved manifest
d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d;
receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
The archived wait-for-approval is satisfied within this package.
Its contract.md T1a covers pure types/facade; T1b fields/producers/consumers;
T2 PDF/SVG/raster/resources/clipping; T3 geometric integration. T4 remains a
future contract. This entity alone is explicitly incomplete and does not prove
production tiling/render correctness or legitimize legacy fallback.

Test full Solid/Gradient transport, prepared-cell Arc retention without copies
proportional to repetition, complete stroke geometry, default equivalence,
and compile-time boundaries forbidding total Paint/declarative-Tiling conversion.
No PartialEq on carriers changes language equality. Preserve nucleus invariants.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/frame_paint.md` preserves the original evidence and chronology.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's original point-only Stroke test values/assignments
to Length::pt while preserving exact numeric values and all assertions. When
a prepared scalar is compared to its declarative source, explicitly resolve
that source with the fixture font; do not relax expected geometry or tolerance.
This test migration implements only the approved field representation change;
existing non-stroke obligations and prior approvals remain intact.

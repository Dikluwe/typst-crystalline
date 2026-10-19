# Prompt L0 — `entities/tiling` — declarative tiling pattern
Hash do Código: 1c992970

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:d6e2fba3d2143daec23bcf7a8d959ce59600f3b8ccc5fa35a853c4f6831e51e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/tiling.rs`
**Governance:** ADR-0017, ADR-0026, ADR-0029, ADR-0054, ADR-0107.

## Evidence before specification

Historical upstream `e0e8ca4d`, `visualize/tiling.rs:15–24,36–41`, defines repeating arbitrary content for fill or stroke, with size/spacing/offset/angle/relative. Text supports relative:auto (default) or parent, not explicit self. The original Image/Gradient/Color-only model and self default were incomplete and are superseded below.

The public-type probes in `lab/results/construction/diagnostics/superficie-linguagem-p1140*` against historical pin `a51e02804` found repr(type(PATH))=`type` upstream versus `function` locally for seven constructors. After structural extraction, the catalog and 22 probes retained baseline bytes. This is public language semantics, not Rust representation parity.

Historical local `tiling.rs:22–31,61–77` retained only the closed body/size/spacing/self-parent model. Upstream `a51e02804:visualize/tiling.rs:59–76,99–185,278–376` accepts arbitrary content, lays it out into a frame, resolves auto size and retains placement parameters. The subsequently approved declarative-content contract requires language semantics/morphology, not an upstream Frame or public resolved entity.

`lab/results/construction/diagnostics/tiling-transformacao-2026-10-14/before/results.json.gz` preserves sources/owners/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/diffstat/UTC/binaries at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. At angle90, offset(3pt,5pt), vanilla SVG emits matrix(0 1 -1 0 3 5), Tekt (0 1 -1 0 -5 3). Upstream `tiling.rs:173–174,375–377` and `layout/transform.rs:334–343` establish T(offset)*R(angle), rotation around offset rather than rotation of the offset vector. `before/render-comparison.json.gz` finds quarter without offset identical between Tekt PDF/SVG and vanilla SVG, but vanilla PDF differs in 414 pixels above 16; do not copy that defect. Quarter-offset SVG differs in 1764 pixels above 16, using the full 240×192 white canvas/librsvg/Poppler without approval-specific crops/tolerances. This does not certify composed parent space or all PDF output.

## Current declarative carrier contract

Tiling is pure L1 data holding arbitrary closed Content under ADR-0026. Physical recursion may use Arc<Content> or an equivalent already permitted indirection, never dyn/vtable/global state. Preserve existing field types, values, constructor and defaults: size auto, spacing zero, offset zero, finite angle zero, relative auto. Offset retains required relative/absolute components. Relative auto remains contextual: self for shapes, parent for text; explicit self/parent remain selectable. Never collapse auto to self before layout context is known.

Angle documents clockwise rotation about offset without rotating the offset vector. The pinned tiling-placement nucleus owns the shared geometric requirement. No entity method performs layout, resource reads or export; the carrier contains no resolved frame and introduces no public ResolvedTiling. Resolution is private to the existing layout phase and reaches targets through the ordinary pipeline contract.

The global name is `Value::Type(Type::Tiling)`, callable through the static dispatcher and existing L1 constructor: type(tiling)==type, repr(type(tiling))==`type`, while constructed values have type_name()==`tiling`. This does not change internal operations.

## Compatibility, acceptance and incomplete targets

Old Color/Image constructions remain representable. Preserve Debug/Clone/PartialEq behavior and construction/equality/clone tests, shared image-body cloning, auto size and Paint integration. Former self-default tests describe the superseded initial model and must not override approved contextual auto. The original tests `tiling_new_color`, `tiling_new_image`, `tiling_equality`, `tiling_clone`, `tiling_relative_default`, `tiling_size_none`, `paint_tiling_to_color` remain historical acceptance anchors with those qualifications.

Paint::Tiling sits alongside Solid/Gradient. The old Color→same color, Gradient→first-stop color, Image→black projections are at most diagnostic Unknown fallbacks; they never substitute repeated morphology or prove tiling parity. Initial placeholder Gradient/Image consumers, native constructor and PDF pattern rendering were separately scoped; current support requires their own owners/evidence. Uncovered strokes and targets remain explicitly outside certification. This entity cleanup does not close them.

For the placement-text correction use ADR-0127 continuous flow: L0 first, RED→GREEN and restricted resealing, without Rust API, public type, default or phase changes. Preserve every existing carrier field while correcting the old ambiguous offset-before-angle wording.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/tiling.md` preserves the original evidence and chronology.

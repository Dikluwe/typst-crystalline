# Prompt L0 — geometry
Hash do Código: 87e5566a

**Layer:** L1, pure geometry; no I/O or font metrics. **Exclusive consumer:** `01_core/src/entities/geometry.rs`. Use existing domain Color/Paint dependencies;no additional external dependency or separate RgbaColor.

## Evidence and supersession boundaries

The early Stroke(Color,thickness,overhang) and Ellipse-placeholder descriptions are historical. The ratified complex-stroke surface preserves Paint/cap/join/dash/miter below. The later shape audit in diagnostico-visualize-fase-a-passo-259.md established real Ellipse four-cubic rendering,kappa0.552_284_749_831,not rectangle TODO;the unrelated affine-transform debt was already closed. Path analytic-extrema correction supersedes that audit's old control-point-only bbox/open-debt claim. No early inventory may silently reimpose those obsolete limitations.

That historical audit's27entries(10implemented/4promoted/0partial/13absent,51.9%linear/54.8%with bonus)and Boxed6/6/34legacy-literal constructor references are snapshot evidence,not current acceptance totals or universal parity. Its separately proposed Gradient,Paint,Stroke refinement,SVG-image/alt-fit,transform-origin and curve work belongs to corresponding current canonical owners;this geometry unit certifies none of those cross-owner systems by historical absence/completion labels. Preserve their independent validation requirements and any remaining scope in those owners. Earlier administrative proposal thresholds/step revisions are archived,not active architecture authority.

At HEAD781b207b4a5d/uncommitted tree,exporters received Corners<Length> without font size:render/SVG resolve_pt(0.0),PDF only abs.0. radius1em became square versus historical ratified rounded oracle;V23ContextErasure identified all three paths. This supports layout-time per-corner resolution,no source-text/constant-font inference.

Public stroke repr measurements corrected the initial automatic-case expectation against historical9f2b6e8715237cb086899a42873660fe744622e8:resolved values alone do not encode explicit presence. The public type catalog/probes `lab/results/construction/diagnostics/superficie-linguagem-p1140*` at historicala51e02804 found repr(type(stroke))"function" versus"type";postatomization catalog/22probes were byte-identical to structural baseline. This is public language identity,not changed Stroke geometry/equality/render.

A curve with fill-rule:"even-odd" compiled upstream and SVG retained fill-rule="evenodd",while local output forced nonzero. Preserve a semantic FillRule value,not inferred winding/SVG.

geometry.rs:191–194 disallowed_methods and `lab/results/construction/diagnostics/geometria-numerica-ci-2026-10-13/before.json.gz` preserve HEAD06848cdcb5450f2c555626bf116bc7acd82bd82b/tree/source/L0/config/libm/compiler/UTC/status/diffstat. host-math probe4968cases measured69trig/atan2 differences≤1ULP,no numeric-class/signed-zero changes,power/hypot equal in sample. Host evidence is neither universal nor new oracle parity. Bernstein formula:189/invariant:212 is convex on[0,1],so curve is inside control-point bbox;old justification claiming escape was false. y-control10 versus curve max7.5 distinguishes tight/conservative boxes. before/pre-validation.json.gz preserves correction provenance;root algorithm/acceptance unchanged.

## Stroke data, defaults and field presence

Preserve these public data types:

```rust
pub enum LineCap { Butt, Round, Square }
pub enum LineJoin { Miter, Round, Bevel }
pub enum DashLength { Length(f64), LineWidth }
pub struct DashPattern {
    pub array: Vec<DashLength>,
    pub phase: f64,
}
pub struct StrokeFields {
    pub paint: bool,
    pub thickness: bool,
    pub cap: bool,
    pub join: bool,
    pub dash: bool,
    pub miter_limit: bool,
}
pub struct Stroke {
    pub paint: Paint,
    pub thickness: Length,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dash: Option<DashPattern>,
    pub miter_limit: f64,
    pub overhang: bool,
    pub specified: StrokeFields,
}
```

Language defaults:Butt cap,Miter join,no dash,miter_limit4.0. DashLength::LineWidth preserves public"dot" until thickness resolves;explicit lengths/phase use points. Array order is semantic,never sort/dedup. Eval validates accepted public cap/join/dash/miter domains and transports Value::Stroke without loss. Layout/FrameItem::Shape transports geometry;exporters only serialize late. SVG emits stroke-linecap/join/miterlimit/dasharray/dashoffset when observably different from applicable defaults. User constructors cap:"round",join:"bevel",dash:"dashed",miter-limit:2 must preserve dimensions and stop erroneous"argumento nomeado inesperado" rejection;first loss was pre-L3,so entity/tests precede exporter work. This acceptance remains even though its old paragraph called the work future.

Overhang is an internal Crystalline extension,not a replacement for upstream fields. Low-level Rust construction defaults false to preserve legacy literal Shape bounds/zero-impact constructors;user-facing extract_stroke sets true for Length/Color shorthand or Dict lacking explicit overhang. Keep this deliberate distinction. In Block/Boxed Shape emission with true,position subtracts thickness/2 on x/y,width/height add thickness;false preserves original centered-stroke bounds. PDF consumes final bounds without redoing layout. Rounded corners retain radii under expanded bounds. Original activation is Block/Boxed only;Grid/Table borders are Line shapes with separate line-cap semantics,not universal overhang support. Do not infer broader graded parity from the Boxed audit.

Internal constructors set all specified bits false;native_stroke marks each present named argument,even equal to resolved default. Render/layout ignore specified and consume values;repr owns presence projection. Required examples:

```text
stroke()                              -> "stroke()"
stroke(paint: black)                  -> "luma(0%)"
stroke(thickness: 1pt)                -> "1pt"
stroke(paint: black, thickness: 1pt)  -> "1pt + luma(0%)"
stroke(cap: "butt")                   -> "(cap: \"butt\")"
stroke(miter-limit: 4)                -> "(miter-limit: 4.0)"
```

Absent paint/thickness bits together with constructor-resolved defaults identifies wholly automatic stroke() in repr;this correction changes no entity. Explicit paint/thickness combinations remain distinct;four complex-field bits decide parenthesized form. Internal/shorthand strokes without presence carrier/nondefault values retain legacy fallback;complete positional roundtrip remains separate constructor work. Public global stroke is callable Value::Type(Type::Stroke),constructed values remain Value::Stroke with same structure/equality/render. `00_nucleo/prompts/core/compiler/eval/repr.md` and constructor owners govern public formatting/casts,not geometry-derived guesses.

## Shape, paths and fill rule

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeKind<T = Length> {
    Rect,
    Ellipse,
    Line { dx: f64, dy: f64 },
    RoundedRect { radii: Corners<T> },
    Path(Vec<PathItem>),
}
pub enum PathItem {
    MoveTo(Point),
    LineTo(Point),
    CubicTo(Point, Point, Point),
    ClosePath,
}
pub enum FillRule { NonZero, EvenOdd }
```

Default FillRule NonZero;EvenOdd changes overlapping-subpath filled region and must not be discarded. ShapeKind/Stroke preserve Debug/Clone/PartialEq. Line dx/dy uses layout Y-down;Line bbox uses absolute deltas. Path is canonical free/subpath geometry,including polygon MoveTo+NLineTo+ClosePath,no separate Polygon variant. CubicTo stores control1/control2/end;availability does not certify every user-facing producer. Export maps segments to existing m/l/c/h;Ellipse is real four-cubic closed geometry with kappa,not placeholder.

Pure bezier_cubic_bbox(p0,p1,p2,p3)->(min_x,min_y,max_x,max_y) and path_bbox(&[PathItem])->(f64,f64,f64,f64) own tight analytic AABB. Walker maintains current_point;MoveTo/LineTo endpoints contribute. For Cubic P0→P3,candidates are endpoints and derivative roots t∈(0,1) on each axis:

```text
B'(t) = 3 * (a*t² + b*t + c)
a = -P0 + 3P1 - 3P2 + P3
b = 2P0 - 4P1 + 2P2
c = -P0 + P1
```

Bernstein nonnegative weights sum1 over[0,1];derivative extrema tighten the conservative control bbox,not repair nonexistent escape. O(1)/Cubic,≤6candidates,pure f64/no external dependency. polygon uses path_bbox;LineTo-only min/max remains bit-identical. Compute quadratic/cubic Bernstein factors with explicit f64 multiplication,store t²/(1−t)²,cubes multiply once more. Preserve factor3/four-term order/coordinates/root selection;no libm here,root-algorithm change/coefficient reassociation. Existing SVG/geometry-language tests before/after with declared differences/build/all-target/core/Clippy/lineage,no suppression/compiled-byte acceptance/universal-platform claim.

## Rounded corners and layout-resolution boundary

RoundedRect coexists with all variants. Input Content ShapeKind<Length>,Block.radius,Boxed.radius,TextStyle.highlight_radius retain style-dependent Length. At owning Content/TextStyle→ShapeKind layout boundary resolve each corner with Length::resolve_pt(style.size.val()). Preserve ShapeElem/input constructors/default generic T=Length;never invent font size early. FrameItem::Shape.kind and Group.clip_mask explicitly ShapeKind<Pt>,Corners<Pt> resolved.

Use exhaustive Length→Pt conversion:nonlength variants unchanged,RoundedRect all four resolved,no cast/Default/ad-hoc second variant. L3 directly consumes point corners and clamps min(width,height)/2 only;no resolve_pt,abs-only projection,font inference/top_left-only reduction. Uniform-only backend degradation must be explicitly tested,no silent loss of three corners. Canonical boundary owners:`00_nucleo/prompts/core/entities/layout_types.md` and separately owned layout/export units.

All-zero radii is geometrically Rect-equivalent but entity retains structural RoundedRect distinction;layout's existing zero-radius choice is preserved. Historical PDF corner path m+4l+Nc+h and four-control-quarter kappa0.552_284_749_831 are emitter mechanics,not geometry-owned byte parity. Block clip with nonzero radius emits Group rounded clip mask in resolved inner geometry;radius does not enlarge bbox,clamp prevents overflow,measurement treats rectangle equivalently.

Verify1em at20pt→20pt,mixed2pt+0.5em→12pt,four distinct corners survive PDF,zero degeneration,zero V23 resolve_pt(0)/abs radius erasure. No new external/public/default/phase in internal numeric repairs;L0 precedes RED/code,unchanged fixtures/tolerances/build/suites/lint/lineage. Expose pub mod geometry;consumers use explicit paths. Historical chronology/coverage/pending candidates remain archived at `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/geometry.md` without becoming current execution-step structure.


## Issue 8894 — approved font-relative thickness

### Measurement before proposal

`lab/results/construction/diagnostics/issue-8894-2026-10-31/before.json`
binds HEAD, UTC, branch and the exact uncommitted diffstat/status. The official
fixture and original receipt are preserved in
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/`.
`01_core/src/compiler/stdlib/layout/stroke.rs:430–434` discards the em component
of dictionary thickness; `01_core/src/entities/geometry.rs:80–81` has only a
point-valued scalar; `01_core/src/compiler/layout/tiling.rs:45` forwards that
scalar. Thus an exporter-only fix cannot recover the requested 10em. The
existing-release controls in `probe-existing.json` are binary observations,
not certification that the binary was built from the current working tree.

Thickness and cap are language semantics. Carrier representation is mechanics.
Do not infer that vanilla intended disappearance from its observed output.
The hypothesis is refuted if the effective occurrence's font size is already
available in a lossless existing stroke carrier; no such carrier was found.

### Approved exclusive entity change

Owner explicitly approved the saved proposal in this chat; receipt:
`lab/results/construction/diagnostics/issue-8894-2026-10-31/approval.json`.
Change public
`Stroke.thickness: f64` to `Stroke.thickness: Length`, defaulting to
`Length::pt(1.0)`. Retain absolute and em components through evaluation and
reuse. Keep the prepared `FrameStroke.thickness` scalar in points; resolve
the declarative length against the effective font at each layout occurrence.
This approved public entity and resolution-boundary change supersedes the
scalar thickness contract above. No fixed 11pt/12pt font or exporter reconstruction.

### Acceptance

Update every affected owning L0 before its consumer; preserve existing literal
strokes using Length::pt and all field-presence, cap/join/dash/paint behavior.
Test RED then GREEN: dictionary and shorthand 10em; reused strokes under
different effective font sizes; mixed absolute/em; absolute round-cap zero
length versus nondegenerate controls, butt/square controls and PDF/SVG visible
output. Keep dash-length em debt distinct; no blanket closure of that debt.
Preserve full source path and geometry of nondegenerate curves and existing
PDF mode defaults. Build, run proportional suites and require zero crystalline
violations with scoped forward/reverse resealing after approval.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.

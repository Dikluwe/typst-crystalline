# Prompt L0 — `entities/elements/shape` — `ShapeElem`
Hash do Código: fcf408e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/shape.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/rectangle-decoration.toml sha256:dbc5dd8b047bdebcd6adf912663a2d5e50b50ad46d4af6c6cf7beff23097ded8
- 00_nucleo/prompts/_nuclei/layout/relative-line.toml sha256:4d97941169e8f16d10287bb1cb938752181434dab2c76b72cf90f355073c59b9

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1778`, `content.rs:1905`, `content.rs:1907`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ShapeElem {
    pub origin: ShapeOrigin,
    pub kind:   ShapeKind,
    pub width:  Option<Box<Value>>,      // preserve Box
    pub height: Option<Box<Value>>,
    pub fill:   Option<Paint>,
    pub stroke: Option<Stroke>,
    pub fill_rule: FillRule,             // default NonZero
}
```

## Behavior and acceptance

Preserve `Content::Shape(Arc<ShapeElem>)` and the existing
`Content::shape(kind, width, height, fill, stroke)` constructor. width/height
retain Box storage. fill_rule carries the public filling rule to layout;
omission uses FillRule::NonZero, while curve/polygon may supply EvenOdd.
Equality and hash include fill_rule. This public-field addition remains
subject to its ADR-0127 gate; editorial cleanup does not invent an approval.
PartialEq compares every field; Box compares by contents, as did as_deref in
the historical hub. Hash uses Debug because Value, Paint, and Stroke contain
f64; preserve the signed-zero caveat.
Plain text is empty; is_empty remains false. This is a pure geometry leaf:
both maps are terminal, with no content body, and field/kind/payload use
defaults. It is non-locatable. Preserve the combined terminal `|` arm with
other leaves without splitting it. Verify terminal behavior, all fields,
fill-rule default/presence, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/shape.md`. Historical observations are not a new parity certification.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Add public `ShapeOrigin` with exactly `Geometry` and `Circle`, deriving Debug,
Clone, Copy, PartialEq, Eq and Hash. Add required public `origin: ShapeOrigin`
to ShapeElem. Keep kind, width, height, fill, stroke and fill_rule unchanged;
fill is the existing `Option<Paint>`, not the stale documented Option<Color>.
Derived ShapeElem equality and its existing Debug-based structural hash retain
origin; preserve the existing signed-zero caveat and do not promise unique hashes.
Clone and both terminal maps retain origin. Keep this pure nonlocatable leaf,
Content::Shape(Arc<ShapeElem>) and the existing constructor signatures.
Native Circle guarantees Circle origin plus ShapeKind::Ellipse; manually assembled
inconsistent public structs are outside that certified constructor domain.
Rendering continues to dispatch on kind. Generic fixture literals use Geometry.
Require clone/map/identity tests and language equality through normal projections;
Rust equality alone does not certify language. This addition does not decide the
separate historical fill_rule approval or other shape identities.

## R13 — proposed relative-line carrier; awaiting ADR-0127 approval

### Measurement before decision

At HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, with the uncommitted R11/R12
tree preserved, `lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/baseline.json`
records UTC, exact changed-file stat, 2164 input hashes, oracle identity and the
received R12 debug/release binary hashes. `received-source.json.gz` retains those
source bytes. `before/receipt.json` and `before-svg/receipt.json` retain the exact
17 sources, command lines, exit codes, diagnostics and independent geometry.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.

`stdlib/shapes/line.rs:158-167` rejects Ratio before layout;
`line.rs:30-37` projects Length through its absolute component, losing em.
The pinned `typst-layout/src/shapes.rs:29-49` resolves the two axes against
the region after polar projection, preserving start and causal size.
The 260 by 140 pt region yields delta (130,0) for 50%, (112.583302492,35)
for 50% at 30 degrees, and (0,70) at 90 degrees. A 180 pt Box yields
90 pt horizontally. The received binary rejects those percent sources;
its 2em source emits zero rather than the measured 22 pt. Explicit end
continues to override length/angle in the accepted absolute control.
These are observable language acceptance and geometry differences. Upstream
Rust representation and byte identity are not requirements.

### Concrete public contract proposed for approval

Add to the existing exclusive consumer, without creating another source unit:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelativeLine {
    pub start: [crate::entities::rel::Rel<crate::entities::layout_types::Length>; 2],
    pub end: [crate::entities::rel::Rel<crate::entities::layout_types::Length>; 2],
}
```

Add `pub relative_line: Option<RelativeLine>` to `ShapeElem`. Every existing
construction/literal receives None, preserving legacy behavior; the existing
Content::shape signature remains unchanged. Clone, both terminal maps, structural
hash and domain observation must retain the field. This is a public Rust contract
addition, including required migration of struct literals, and is not authorized
for implementation until the owner approves it.

Some carries unresolved endpoint coordinates in x/y order, including ratio,
absolute points and em. Its canonical accompanying kind is Path(empty), with
width and height None: no surrogate resolved coordinates and no encoded meaning
in dx/dy, paint, origin or width. Layout must consume the carrier before final
FrameItem::Shape emission, replacing it with a resolved open MoveTo/LineTo path
and causal dimensions max(0,start.x,end.x), max(0,start.y,end.y). The entity
performs no layout or rendering. None retains all current kind/width/height rules.

### Phase boundary and bounded implementation after approval

Eval owns argument validation, precedence and affine endpoint construction.
Length/Ratio/Relative preserve their coefficients; numeric extensions remain
point values. With explicit end, ignore length/angle; otherwise project each
component of length by libm cos/sin before adding start. Relative/em-bearing
coordinates use the new carrier. Existing absolute paths and legacy dx/dy
remain compatible, including the deliberate bare-line zero default and legacy
Ratio/Relative dx/dy debt. This proposal does not change that debt or add flags.

Layout owns resolution against actual local region width/height and effective
font size. In particular, resolve each projected axis independently; do not
resolve length against width first and then rotate the resulting scalar.
This approved scope would defer relative/em source-line geometry from eval to
layout; approval explicitly covers that phase boundary as well as the public
field/type. Exporters continue to receive resolved geometry only.

The corresponding constructor/layout/observation/migration owners must be
updated before implementation and resealed with valid 1:1 ownership. Any shared
normative invariant must use a pinned Tekt Nucleus under ADR-0129, not duplicated
cross-owner authority. Do not reseal this proposal as implemented before approval.

### Required RED→GREEN and completion evidence

After approval, add assertional failing tests before implementation: coefficient
transport, clone/maps/observation, percent/mixed/em endpoints, angle axes, signed
lengths, start/end precedence, typed supplied-value diagnostics and absolute/
legacy controls. Compare the three unchanged R12 note sources and direct/Box
sources against the pinned oracle in debug and release PDF/SVG at 0.001 pt,
checking frame size, following-content positions and paint as well as endpoints.
Exercise nested regions and footnote separator width. Retain any new refuters
instead of dropping them. Infinite/auto-region and reflection generalization
have no parity certification from the finite probes above; measure before
expanding those claims and never substitute a fabricated zero for unresolved
geometry.

Completion requires the full validation pipeline (workspace build, Rust/Python
tests, Clippy, formatting/docs, strict/reverse lineage, zero full-lint violations),
earlier R8–R12 and A–H/R1–R7 replays and resource gates, source provenance and
preservation receipts. The present proposal and baseline measurements are not
RED→GREEN completion or final validation.



## R13 — approved materialization

Owner approved the saved proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00, including the public carrier and eval-to-layout boundary. Approval and measured baseline are retained in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/. The previously pending section is now ratified within that exact scope. Apply the pinned nucleus before code; RED→GREEN and complete validation remain mandatory.


## Issue 8583 — approved rectangle decoration carriers

### Measurement before proposal

`lab/results/construction/diagnostics/issue-8583-2026-10-31/before.json` and
`before-source.json.gz` preserve HEAD, UTC, the exact uncommitted diffstat/status
and source/owner hashes. Concurrent layout changes and the second build input
are frozen separately in `combined-before.json` and `combined-source.json.gz`.
`baseline.json` binds the rebuilt combined-state binary
to the unchanged official source and independent PDF/SVG/raster controls.
The official case produces only a square-corner blue fill and no red border;
radius-only and side-stroke-only controls isolate each loss, while absolute
10pt radius with uniform 2pt stroke preserves both. These are observed language
geometry/paint failures, not requirements on Rust structure or output bytes.

`01_core/src/compiler/stdlib/layout/casting.rs:31–39` projects Relative to abs
and Ratio to Length::ZERO before corner construction at :388–473.
`01_core/src/compiler/stdlib/shapes/rect.rs:44–56` drops dictionaries containing
side keys. `01_core/src/entities/elements/shape.rs:37–46` has one uniform Stroke;
`01_core/src/entities/geometry.rs:119–132` carries Length-only rounded radii.
Neither preserves the information needed to repair the official source later.
The existing owner `00_nucleo/prompts/core/compiler/stdlib/shapes/rect.md`
explicitly retains per-side stroke as a separate unsupported entity gap.
The prior Stroke Length approval does not authorize these additional fields.

Pinned source `lab/typst-original/crates/typst-layout/src/shapes.rs:495–498,
561–589,617–656,756–848` retains side strokes and relative corners, but the
reproduced oracle has a visible white separation between fill and border.
That observed defect is not adopted as intended language behavior. The
correction must retain the requested decoration and couple border/fill geometry.
The lossless-carrier hypothesis would be refuted by finding an existing public
carrier that retains both coefficients and all four strokes without encoding
metadata in coordinates, width/height, paint, or changing rect into another
language object; the source/owner audit found none.

### Approved exclusive public contract

Add exactly these fields to the existing ShapeElem, in its existing owner unit:

```rust
pub rect_radius: Option<crate::entities::corners::Corners<
    crate::entities::rel::Rel<crate::entities::layout_types::Length>,
>>,
pub rect_stroke: Option<crate::entities::sides::Sides<
    Option<crate::entities::geometry::Stroke>,
>>,
```

None retains the current kind/width/height/fill/uniform-stroke semantics exactly.
Some(rect_radius) is authoritative for all four rectangle corners, retaining
ratio, absolute and em coefficients until actual frame dimensions/effective
font are known. Some(rect_stroke) is authoritative for left/top/right/bottom;
an absent side means no border on that side, including Some(all sides None),
which must not activate the legacy default black stroke. Each side retains
its complete existing Stroke and field presence; no color/thickness flattening.
The certified constructor domain attaches these carriers only to Rect or
RoundedRect kinds and never to relative lines, circles or arbitrary paths.

All legacy constructors/literals initialize both fields to None; keep
Content::shape/shape_with_radius signatures, ShapeOrigin, relative_line,
ShapeKind's Length default, FrameItem and FrameStroke contracts unchanged.
Clone, terminal maps, structural hash and language observation retain the new
data without using structural Rust equality as the parity oracle. No rendering
method or dependency on compiler/layout belongs in this entity.

This is a public entity extension, including literal migration, and explicit
lossless eval-to-layout transport of percentage radii/per-side decoration.
The user explicitly approved this saved proposal with “aprovo”; approval.json
binds its SHA-256 and the exact added public fields. Implementation within this
bounded contract is authorized. Earlier approvals and unrelated scopes remain.

### Bounded implementation and acceptance

Before code, update the separate owning constructor, observation, construction,
layout and test prompts, and migration owners. Shared normative obligations
across owners use pinned Tekt nuclei under ADR-0129; no nucleus owns code.
Do not modify the generic Length caster globally to repair this rectangle.
Preserve the existing absolute-radius/uniform-stroke route and optional-stroke
defaults; materialize the new native rect arguments through the typed carriers.
The square/container families keep their existing contracts outside this scope.

At owning L1 layout, use actual resolved rectangle size and occurrence font to
resolve the new radius data and each stroke. Emit only resolved geometry and
prepared paints through existing FrameItem variants. Keep exhaustive static
ShapeKind dispatch, delegating substantial rectangle logic to its render-layer
owner under ADR-0109. Percentages must not be replaced by zero or a fixed font.
Border bands and fill must derive from a common rectangle contour; asymmetric
thickness must not independently move the corner centers and create the white
separation observed in the oracle. Preserve each Paint, alpha, dash/cap/join
and original paint-coordinate frame; never replace rich paint with a solid.
This narrowly refines the existing single-Shape emission clause only when the
new rectangle carrier requires multiple resolved border/fill geometries. It
authorizes no repetition grid, entity rendering, raster substitute or new mode.

Require module RED before implementation, then GREEN: coefficients and side
presence/precedence/errors, clone/maps/observation, 100% bottom radius and the
original asymmetric em stroke, pure percentage/mixed/absolute/em controls,
None/empty/default, independent dimensions/fonts and reused aliases. Native
PDF/SVG/raster must show the requested curved fill and red side border without
the oracle's separating white wedge; source coordinates/logical frame/following
content and existing absolute/uniform controls must remain unchanged. Include
asymmetric sides/corners, omitted sides, alpha and rich-paint controls and
retain any new refuters. Source checks and XML tag counts alone do not certify
visible geometry. General Block/Box/square materialization and Auto/infinite
frame debt are not closed by this finite explicit-size source.

Completion requires build/all-target compatibility, proportional/full relevant
suites, crystalline-lint with zero errors/warnings, forward/reverse lineage and
exact source/binary/UTC/diffstat receipts. Reseal both directions within the audited consumer scope after valid V15/V26.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Add exactly the approved rect_radius and rect_stroke fields. Preserve clone,
terminal maps, structural hash and pure entity responsibilities. Existing
literals use None. Preserve all other fields, APIs, approvals and defaults.

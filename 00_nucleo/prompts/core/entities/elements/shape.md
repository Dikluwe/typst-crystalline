# Prompt L0 — `entities/elements/shape` — `ShapeElem`
Hash do Código: a94e0899

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/shape.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

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

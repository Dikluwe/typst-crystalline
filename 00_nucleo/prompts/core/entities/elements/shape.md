# Prompt L0 — `entities/elements/shape` — `ShapeElem`
Hash do Código: 182f8864

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
    pub kind:   ShapeKind,
    pub width:  Option<Box<Value>>,      // preserve Box
    pub height: Option<Box<Value>>,
    pub fill:   Option<Color>,
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
the historical hub. Hash uses Debug because Value, Color, and Stroke contain
f64; preserve the signed-zero caveat.
Plain text is empty; is_empty remains false. This is a pure geometry leaf:
both maps are terminal, with no content body, and field/kind/payload use
defaults. It is non-locatable. Preserve the combined terminal `|` arm with
other leaves without splitting it. Verify terminal behavior, all fields,
fill-rule default/presence, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/shape.md`. Historical observations are not a new parity certification.

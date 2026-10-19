# Prompt L0 — `entities/elements/curve` — `CurveElem`
Hash do Código: f7f4de70

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/curve.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct CurveElem {
    pub segments: Vec<CurveSegment>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CurveSegment {
    Move(CurvePoint),
    Line(CurvePoint),
    Cubic(CurvePoint, CurvePoint, CurvePoint), // control1, control2, end
    Quad(CurvePoint, CurvePoint),              // control, end
    Close(CloseMode),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloseMode { #[default] Smooth, Straight }

#[derive(Debug, Clone, PartialEq)]
pub struct CurvePoint {
    pub x: Length,
    pub y: Length,
}
```

## Behavior and acceptance

Preserve `Content::Curve(Arc<CurveElem>)` and constructors
`Content::curve_move(x, y)`, `curve_line(x, y)`,
`curve_cubic(c1x, c1y, c2x, c2y, ex, ey)`, `curve_quad(cx, cy, ex, ey)`,
and `curve_close(mode)`. Public omission resolves to Smooth. Smooth may add
a closing cubic using the control opposite the initial one; Straight closes
with a straight segment. Changes to this public payload require ADR-0127.
Manual Hash uses `format!("{:?}", self)` because Length contains f64.
Plain text is empty, is_empty is false, and both maps are terminal, rebuilding
`Content::Curve(Arc::new(self.clone()))`. Field/kind/payload default to None;
non-locatable. Verify empty text, terminal maps, all segment coordinates,
control-point order, and preserved closing mode. Rendering belongs to layout.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/curve.md`. Historical observations are not a new parity certification.

# Prompt L0 — `entities/elements/place` — `PlaceElem`
Hash do Código: 843101c4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/place.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1824`, `content.rs:1974`, `content.rs:2346`, `content.rs:2612`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceElem {
    pub alignment: Align2D,
    pub dx:        f64,
    pub dy:        f64,
    pub scope:     PlaceScope,
    pub float:     bool,
    pub clearance: Option<Length>,
    pub body:      Content,
}
```

## Behavior and acceptance

Preserve `Content::Place(Arc<PlaceElem>)` and `Content::place(...)` in
existing field order. PartialEq compares all seven struct fields. Hash remains
manual through Debug because dx/dy, Align2D, and Option<Length> lack Hash;
retain the signed-zero caveat. Plain text delegates to body; emptiness stays
the default false and must not delegate to body. Both maps recurse into body
and preserve alignment, dx, dy, scope, float, clearance. Field/kind/payload
retain defaults; non-locatable. Verify all fields, maps and nonempty identity.
The original constructor note said "7 fields + body" although the struct has
six non-body fields plus body; that count is a documentary inconsistency,
not authorization to add an argument or change the signature.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/place.md`. Historical observations are not a new parity certification.

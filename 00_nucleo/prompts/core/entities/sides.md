# Prompt L0 — entities/sides
Hash do Código: 33c29345

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/sides.rs`
**Governance:** ADR-0033, ADR-0037, ADR-0054, ADR-0061.

## Evidence and scope

The original bounded carrier was based on
`lab/typst-original/crates/typst-library/src/layout/sides.rs` for padding;
no separate preimplementation diagnostic was recorded. The historical initial
consumer inventory does not restrict current callers or prove new parity.

## Public contract

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sides<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}
impl<T> Sides<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self;
}
impl<T: Clone> Sides<T> {
    pub fn uniform(value: T) -> Self;
}
impl<T: Default> Default for Sides<T>;
```

Constructor order is left/top/right/bottom, not CSS order. Each side is
independent with no value validation or implicit ordering. uniform clones
into all four sides; default invokes T::default per side (Length::ZERO for
Sides<Length>). Copy applies when T: Copy; Eq when T: Eq, without imposing
Eq on callers using Length/f64. Public fields remain public. The original
consumer convention constructs replacement values instead of mutation; it is
not physical immutability enforced by the type.

## Acceptance and deferred scope

Verify independent construction, uniform/default values, generic Copy/Eq
bounds, and side order. Pad consumes this carrier; margins, block/box insets,
and richer borders are separate owner responsibilities. Helpers relative_to,
sum_by_axis, map_corners and rotations, plus Side and inv/next_cw/axis, were
explicitly outside the bounded original scope and require concrete-consumer
specification before adding interfaces. This cleanup does not certify those
extensions or remove independently established consumers.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/sides.md`. Historical observations are not a new parity certification.

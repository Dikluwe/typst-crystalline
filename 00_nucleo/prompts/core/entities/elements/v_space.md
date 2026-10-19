# Prompt L0 — `entities/elements/v_space` — `VSpaceElem`
Hash do Código: 5a1b0c22

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/v_space.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1611`, `content.rs:1786`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct VSpaceElem {
    pub amount:        Length,
    pub weak:          bool,
    pub weak_explicit: bool,
}
```

## Behavior and acceptance

Preserve `Content::VSpace(Arc<VSpaceElem>)` and
`Content::v_space(amount: Length, weak: bool)`. The existing constructor treats
true as explicit and false as omitted default. Evaluation uses
`v_space_with_weak_presence(amount, weak, weak_explicit)`; layout reads only
weak. weak_explicit retains named-argument presence for repr.
Manual Hash uses Debug because Length contains f64; retain the caveat in the
source comment and relational hash test. PartialEq compares amount, weak,
and weak_explicit because presence changes repr; Debug-based hash includes it.
Plain text is empty; is_empty delegates to amount.is_zero(). Maps are terminal;
field/kind/payload default to None, non-locatable. Verify all these properties,
especially explicit/default presence, relational hash, and zero amount.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/v_space.md`. Historical observations are not a new parity certification.

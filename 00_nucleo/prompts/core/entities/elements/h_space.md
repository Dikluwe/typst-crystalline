# Prompt L0 — `entities/elements/h_space` — `HSpaceElem`
Hash do Código: 9a79efad

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/h_space.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical upstream layout/spacing.rs distinguishes Absolute/Fractional;
the original length-only schema is superseded. The archived Content anchor
1786 records empty text. These are historical anchors, not new parity probes.

## Contract

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Spacing {
    Absolute(Length),   // h(10pt), h(2em)
    Fractional(f64),    // h(1fr); expanded by layout
}

#[derive(Debug, Clone, PartialEq)]
pub struct HSpaceElem {
    pub amount:        Spacing,
    pub weak:          bool,
    pub weak_explicit: bool,
}
```

Preserve `Content::HSpace(Arc<HSpaceElem>)`,
`Content::h_space(amount: Length, weak: bool)` producing Absolute, and
`Content::h_space_fraction(fr: f64, weak: bool)` producing Fractional.
Spacing::is_zero recognizes absolute or fractional zero. Fractional amounts
must reach Layouter's flush_line/finish expansion without premature resolution.
weak_explicit transports supplied named weak, including false, for repr.
Existing constructors keep signatures and treat true as explicit, false as
omitted; h_space_with_weak_presence and its fractional equivalent accept the
evaluator's bit. Layout reads only weak.

Plain text is empty; is_empty calls amount.is_zero. Both maps are terminal;
field/kind/payload default None, non-locatable. PartialEq compares amount,
weak, weak_explicit. Manual Debug Hash includes all three because Length and
fractional f64 lack canonical Hash. Retain the signed-zero caveat in source;
its existing allowance assumes elements are not map keys. Keep relational
hash tests. Verify zero for both amount forms, terminal maps, empty text,
presence-aware repr distinctions/equality/hash, and fractional transport.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/h_space.md`. Historical observations are not a new parity certification.

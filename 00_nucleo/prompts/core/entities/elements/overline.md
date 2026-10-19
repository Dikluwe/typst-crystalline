# Prompt L0 — `entities/elements/overline` — `OverlineElem`
Hash do Código: f5510125

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/overline.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical content.rs:1660,1599,2146,2464,1956 record text, emptiness, maps and
equality. The exhaustive locatable classifier places this element in the
non-locatable group. This is historical extraction evidence, not new parity.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct OverlineElem {
    pub body:   Content,
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}
```

Preserve `Content::Overline(Arc<OverlineElem>)` and
`Content::overline(body, stroke, offset, extent)`. Color/Length use their
existing entity paths. PartialEq compares all four fields.

## Behavior and hash limitation

Plain text and emptiness delegate to body; both maps recurse and preserve
stroke/offset/extent (Copy fields). Field/kind/payload default to None.
Element requires Hash but Length contains f64 and lacks Hash; keep manual
`format!("{self:?}").hash(state)`, matching existing content_hash mechanics.
This is not a hot path. Document in every implementation that Debug hashing
may violate Eq/Hash consistency for -0.0 and 0.0: they compare equal but Debug
and hashes can differ. The existing allowance assumes these elements are not
map keys and follows the pre-existing hash regime. If used in HashMap/HashSet,
revisit it, for example with canonical float bits or signed-zero normalization;
this cleanup does not approve a new hash algorithm.
Verify text/emptiness delegation, recursive maps with metadata preserved,
manual Debug hash and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/overline.md`. Historical observations are not a new parity certification.

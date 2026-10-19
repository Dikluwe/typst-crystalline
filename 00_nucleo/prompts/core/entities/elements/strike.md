# Prompt L0 — `entities/elements/strike` — `StrikeElem`
Hash do Código: ac239a19

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/strike.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1598`, `content.rs:1659`, `content.rs:1954`, `content.rs:2140`, `content.rs:2458`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct StrikeElem {
    pub body:   Content,
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}
```

## Behavior and acceptance

Preserve `Content::Strike(Arc<StrikeElem>)` and
`Content::strike(body, stroke, offset, extent)`. Plain text and emptiness
delegate to body. Both maps recurse into body, preserving stroke, offset, and
extent. Field/kind/payload default to None; non-locatable. Derived PartialEq
compares all four fields.
Manual Hash uses Debug because Length contains f64; its signed-zero caveat
(`-0.0` versus `0.0`) must remain documented in source. This existing compromise
follows content_hash and assumes these elements are not map keys; revisit it
if they become keys. The full rationale is owned by
`00_nucleo/prompts/core/entities/elements/overline.md`.
Verify body transparency, emptiness delegation, maps with metadata preserved,
structural equality and the specified manual hash.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/strike.md`. Historical observations are not a new parity certification.

# Prompt L0 — `entities/elements/underline` — `UnderlineElem`
Hash do Código: d7e41152

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/underline.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1597`, `content.rs:1658`, `content.rs:1950`, `content.rs:2134`, `content.rs:2452`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct UnderlineElem {
    pub body:   Content,
    pub stroke: Option<Color>,
    pub offset: Option<Length>,
    pub extent: Option<Length>,
}
```

## Behavior and acceptance

Preserve `Content::Underline(Arc<UnderlineElem>)` and
`Content::underline(body, stroke, offset, extent)`. Plain text and emptiness
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
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/underline.md`. Historical observations are not a new parity certification.

# Prompt L0 — `entities/elements/repeat` — `RepeatElem`
Hash do Código: 08559690

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/repeat.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1659`, `content.rs:1829`, `content.rs:2016`, `content.rs:2207`, `content.rs:2491`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct RepeatElem {
    pub body:    Content,
    pub gap:     Option<Length>,
    pub justify: bool,
}
```

## Behavior and acceptance

Preserve `Content::Repeat(Arc<RepeatElem>)` and `Content::repeat(body, gap, justify)`.
Manual Hash uses Debug because Length contains f64; PartialEq derives and
compares body, gap, and justify. `plain_text` and `is_empty` delegate to body.
Both maps recurse into body and preserve gap/justify. Field, kind, and payload
access retain the trait defaults. The element is non-locatable.
Verify recursive maps, text/emptiness delegation, and structural equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/repeat.md`. Historical observations are not a new parity certification.

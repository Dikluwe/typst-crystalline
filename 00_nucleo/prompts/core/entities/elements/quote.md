# Prompt L0 — `entities/elements/quote` — `QuoteElem`
Hash do Código: 72fa9794

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/quote.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1632`, `content.rs:1835`, `content.rs:1975`, `content.rs:2039`, `content.rs:2132`, `content.rs:2419`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct QuoteElem {
    pub body:        Content,
    pub attribution: Option<Content>,
    pub block:       bool,
    pub quotes:      bool,
}
```

## Behavior and acceptance

Preserve `Content::Quote(Arc<QuoteElem>)` and
`Content::quote(body, attribution, block, quotes)`. Derive
Debug/Clone/PartialEq/Hash; Content's manual Hash and bool support the hash.
Plain text starts with body's text, wraps it in ASCII double quotes when
quotes is true, then appends ` — ` and attribution's plain text when present.
Emptiness delegates only to body. Both maps recurse into body and optional
attribution, preserving block/quotes. Field/kind/payload use defaults;
non-locatable. Equality compares all four fields. Verify both quote modes,
optional attribution, body-only emptiness, recursive maps, and equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/quote.md`. Historical observations are not a new parity certification.

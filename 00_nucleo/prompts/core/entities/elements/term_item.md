# Prompt L0 — `entities/elements/term_item` — `TermItemElem`
Hash do Código: c5b9fcc2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/term_item.rs`
**Architecture:** ADR-0105. The trait contract is owned by
`00_nucleo/prompts/core/entities/elements/mod.md`.

## Evidence before specification

Original extraction anchors: `content.rs:1571`, `content.rs:1758`, `content.rs:1935`, `content.rs:2119`, `content.rs:2453`.
These historical coordinates support the ownership boundary; they are not
fresh current-pin language-parity measurements.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct TermItemElem {
    pub term:        Content,
    pub description: Content,
}
```

## Behavior and acceptance

Preserve `Content::TermItem(Arc<TermItemElem>)` and
`Content::term_item(term: Content, description: Content)`. Derive
Debug/Clone/PartialEq/Hash. Plain text is
`format!("{}: {}", term.plain_text(), description.plain_text())`.
Emptiness requires both term and description to be empty. Both maps recurse
into both fields, in the existing order. Field/kind/payload access default to
None; the element is non-locatable. Structural equality compares both fields.
Verify `term: description`, joint emptiness, both recursive maps, and equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/term_item.md`. Historical observations are not a new parity certification.

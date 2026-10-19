# Prompt L0 — `entities/elements/list_item` — `ListItemElem`
Hash do Código: e9d2f60c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/list_item.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ListItemElem {
    pub body:         Content,
    pub marker:       Option<ListMarker>,
    pub marker_align: Option<Align2D>,
    pub indent:       Option<Length>,
    pub body_indent:  Option<Length>,
    pub tight:        Option<bool>,
}
```

Preserve Content::ListItem(Arc<ListItemElem>). None marker uses the default
bullet `•`. marker_align is accepted/preserved but its actual layout was
explicitly excluded. indent is marker offset from the left margin; body_indent
is body offset from marker. tight false adds paragraph spacing between items;
true is the default adjoining behavior. Layout resolves these semantics in its
owner; the entity preserves all optional values.

Content::list_item(body: Content) sets every option None.
Content::list_item_with_marker(body: Content, marker: ListMarker) sets only
marker Some. Content::list_item_full(body: Content, marker: Option<ListMarker>,
marker_align: Option<Align2D>, indent: Option<Length>, body_indent: Option<Length>,
tight: Option<bool>) propagates every field unchanged.

Plain text is rendered marker (or `•`) plus one space and body plain text.
is_empty defaults false. Both maps recurse into body, preserving marker,
marker_align, indent, body_indent, tight. Field/kind/payload default None;
non-locatable. Derived equality compares all six fields; keep specified derives.

## Boundary rationale and acceptance

Upstream stores indent/body-indent/tight on a list container; Crystalline
expands list into a Sequence and replicates these values into each item.
That is deliberate representation mechanics under ADR-0107; language argument
semantics and visual output remain acceptance, not identical data structures.
For body `um`, no marker gives `• um`; Custom(`→`) gives `→ um`.
Verify recursive maps with all five options preserved, equality including
indent/body_indent/tight, and full-constructor propagation. The historical
marker-alignment scope-out remains unclosed by this cleanup.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/list_item.md`. Historical observations are not a new parity certification.

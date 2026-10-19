# Prompt L0 — `entities/elements/link` — `LinkElem`
Hash do Código: 04471538

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/link.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations: `content.rs:1637`, `content.rs:1820`, `content.rs:2070`, `content.rs:2444`.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LinkElem {
    pub url:  EcoString,
    pub body: Content,
}
```

## Behavior and acceptance

Preserve `Content::Link(Arc<LinkElem>)` and
`Content::link(url: impl Into<EcoString>, body: Content)`. PartialEq compares
url and body; preserve Debug/Clone/Hash. Plain text delegates to body but
is_empty remains false. Both maps recurse into body and preserve url.
Field/kind/payload default to None; non-locatable.

## Rendering boundary and pending scope

Keep LinkElem a pure url/body struct. Under ADR-0107/0109 the free function
layout_link belongs to `00_nucleo/prompts/core/compiler/layout/link.md`.
The layout result uses FrameItem::Link { url, items }, owned by
`00_nucleo/prompts/core/entities/layout_types.md`, retaining URL metadata and
rendered body. Verify that integration alongside body text/maps and equality.
The original rendering scope excluded blue/underlined text (dependent on
FrameItem::Decoration infrastructure), PDF URI annotations, internal labels/
inter-page links, and renderer-specific hover tooltips. Those are historical
scope boundaries requiring reconciliation with their current owners; this
cleanup neither certifies them complete nor revokes the existing exclusions.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/link.md`. Historical observations are not a new parity certification.

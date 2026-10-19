# Prompt L0 — `entities/elements/label` — `LabelElem`
Hash do Código: 228486e2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/label.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction record preserves the original source observations.
These are historical anchors, not a fresh current-pin parity measurement.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct LabelElem {
    pub name: EcoString,       // destination name: "sec1", "fig1", ...
    pub body: Content,         // content associated with the label
}
```

## Behavior and acceptance

Preserve `Content::Label(Arc<LabelElem>)` and `Content::label(name, body)`.
Derive Hash using EcoString and Content's existing hash. PartialEq compares
name and body. Text and emptiness delegate to body; both maps recurse into
body and preserve name. Field/kind/payload use defaults: non-locatable;
locatable.rs explicitly classifies this variant in the non-locatable group.

The recorded vanilla probe rejects `label("x", [body])`. Public `label(name)`
returns Value::Label, not LabelElem. Content::Label, Content::label, and
Content::label_auto are internal association/introspection/Rust-fixture
mechanisms; none authorizes a public two-argument constructor.

Layout boundary obligation: this wrapper renders body transparently and records
the destination page/position in LayouterRuntimeState. Layouter::finish exposes
it through PagedDocument.extracted_label_pages and extracted_label_positions.
The entity does not implement this rendering/registration. Verify body
transparency, recursive maps, name preservation and equality.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/label.md`. Historical observations are not a new parity certification.

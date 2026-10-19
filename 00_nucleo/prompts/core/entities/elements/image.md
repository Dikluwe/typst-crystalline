# Prompt L0 — `entities/elements/image` — `ImageElem`
Hash do Código: 389d340e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/image.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical extraction anchors content.rs:1740,1896 support text/equality.
The archived image-fit evidence used upstream 0.14.2. Later upstream image
source at `lab/typst-original/crates/typst-library/src/visualize/image/mod.rs:165-181`
provided alt. The archived corpus with alt compiled and passed 106/0 PDF/UA-1
rules; without alt upstream reported `PDF/UA-1 error: missing alt text`.
The old consumer rejected alt as unexpected. These are historical observations,
not a fresh current-pin or general PDF/UA certification.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ImageElem {
    pub path:   String,
    pub data:   PtrEqArc<Vec<u8>>,
    pub width:  Option<Box<Value>>,
    pub height: Option<Box<Value>>,
    pub fit:    EcoString,
    pub alt:    Option<EcoString>,
}
```

The alt field was confirmed on 2026-09-23 under ADR-0127, superseding the
earlier schema without it. Preserve None versus Some("") and include alt in
equality/hash.

## Behavior and acceptance

Preserve `Content::Image(Arc<ImageElem>)` and
`Content::image(path, data, width, height, fit)`; the unchanged constructor sets
alt to None, while stdlib may construct an entity with Some without a new
signature. Fit defaults to cover, with contain/cover/stretch accepted. Its
rendering was an explicit separate scope-out, not certified by this entity.
PtrEqArc and Value belong to entities. PartialEq compares the fields, using
pointer identity for data. Manual Hash remains Debug-based. The historical
claim that PtrEqArc lacks Hash conflicts with its independent current owner
`00_nucleo/prompts/core/entities/ptr_eq_arc.md`; that rationale does not authorize
changing Image's hash. The earlier equality summary omitted fit although the
struct derives equality; preserve all fields, not that incomplete enumeration.
Plain text is empty, is_empty defaults false, maps are terminal, and field/
kind/payload use defaults; non-locatable. Verify terminal carrier preservation,
fit, pointer identity, and distinct absent/empty alt.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/image.md`. Historical observations are not a new parity certification.

# Prompt L0 — `entities/elements/heading` — `HeadingElem`
Hash do Código: 08324c54

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/heading.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The recorded reflection fixtures distinguish heading[H].has("level") false,
heading(level:2)[H].has("level") true, and absent level lookup error:
`field "level" in heading is not known at this point and no default was specified`.
Markup `[= H].fields()` returns depth:1 and body:[H], not level. Historical
Content anchors 1267,1573,2021–2022 and extract_payload.rs:21 support clamp,
text, fields and payload. This is retained evidence, not a fresh current-pin probe.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct HeadingElem {
    pub level: u8,
    pub body: Content,
    pub outlined: bool,
    pub bookmarked: Option<bool>,
    pub set_fields: u8,
}
```

Preserve Content::Heading(Arc<HeadingElem>) and constructor level.clamp(1,6).
outlined defaults true; bookmarked None follows outlined (language auto).
Explicit-field mask bits are HEADING_SET_LEVEL=1 (positional integer or named
level), HEADING_SET_DEPTH=2 (markup via new/new_with_outlined), and
HEADING_SET_OUTLINED=4 (named outlined). bookmarked tracks presence with Option.
Upstream offset remains excluded; exposed depth equals level.
Native heading constructors set bits through heading_native/
heading_numbered_native; markup new/Content::heading use DEPTH. with_body,
both maps and introspection-time reconstruction preserve the whole mask.

## Behavior and acceptance

Plain text delegates to body; is_empty is false for this structural element.
Both maps recurse into body, preserving level, outlined, bookmarked and mask;
the older illustrative map snippets omitted newer fields and are superseded
by the explicit preservation rule. get_field("body") yields Value::Content,
"level" yields Value::Int(level as i64), "outlined" yields Value::Bool;
unknown fields yield None. Show-rule field access returns baked values without
consulting the mask; only has/at/fields read presence, in their evaluator owner.

Locatable kind is Heading. Payload is Heading { depth: level,
body_hash: hash_content(&body), counter_update: CounterUpdate::Step }.
The exhaustive extraction arm delegates to h.to_payload; ElementKind and
ElementPayload remain for other variants. Preserve the existing payload and
`heading_produz_some_payload` expectation. Layout uses level/body under its
own owner without a logic change. Verify text, fields, maps with all metadata,
reflection-presence distinction and identical own payload; if the locatable
boundary fails, revisit this L0 before changing its contract.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/heading.md`. Historical observations are not a new parity certification.

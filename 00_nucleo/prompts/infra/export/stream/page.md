# L0 — stream/page
Hash do Código: fc7f7e39

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/page.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/page.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Page assembly

Own build_page_stream and emit_page_items. Emit canvas fill and background, body,
foreground in that order, using the physical canvas and bleed translation with a
balanced q/cm/Q envelope. Borrow PageContext and return page-operator bytes.
Delegate prosaic run coalescing to text and individual item dispatch to draw;
UA1 leaves keep their marked-content boundaries: this owner bypasses prose run
coalescing for UA1. Auto and None canvas fill emit no explicit fill operators;
Paint uses the existing solid/spot or to_color projection, not a canvas Pattern.
When UA1 is enabled, explicit canvas fill is an Artifact Layout. Do not allocate resources, objects,
ParentTree entries or fonts, and do not compress the returned operators.

## Verification

Check blank pages, fill, bleed, background/foreground order and unchanged top-level
coordinates with both graphic modes and tagging settings.

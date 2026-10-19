# L0 — infra/pipeline/resources
Hash do Código: 4c59900e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/resources.rs`

## Current evidence

- `03_infra/src/pipeline/resources.rs:33` — `resolve_and_instantiate_fonts`.
- `03_infra/src/pipeline/resources.rs:93` — `collect_fonts_from_doc`.
- `03_infra/src/pipeline/resources.rs:123` — `collect_fonts_in_items`.
- `03_infra/src/pipeline/resources.rs:172` — `resolve_fonts`.
- `03_infra/src/pipeline/resources.rs:195` — `first_font_from_doc`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/resources.rs`.

## Owned contract

- Own ordered document font collection, fallback resolution, complete FontKey identity and variable-font instantiation. Keep first-occurrence deduplication and declared-family order.
- Traverse background, items, foreground and Group/Link/Semantic transparently. Preserve semantic kind/placement/alt; never flatten the wrapper or turn alt into visible text.
- Include Text, TextShaped and math Glyph resources. Resolve a Glyph's actual font via FallbackFontMetrics::resolve_font_combo, not just body style.font, so a companion MATH glyph cannot be omitted from export selection.
- Keys include FontList, FontVariant and explicit FontVariations. Derived axes plus explicit per-tag overrides determine variable-instancing need and separate resources, even if only an explicit axis differs.
- Require the Python/fontTools availability probe only when a merged request contains an axis actually present in the resolved variable face. Requests containing only unknown axes follow the instancing owner's unchanged-bytes contract; preserve the missing-tool error for matching axes. A private local predicate owns this resource guard without exposing a new API.
- An empty collected font set resolves to an empty resource set without requesting World::book or World::font. All PDF, bundle PDF and raster/SVG callers use this same private resolution boundary.
- A family wins only after selection and source loading both succeed; stale/missing sources continue to later families. Preserve unresolved-entry filtering and variable-instancing diagnostics. Keep test-only first-font helpers internal; production selection uses the complete resolved font set.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

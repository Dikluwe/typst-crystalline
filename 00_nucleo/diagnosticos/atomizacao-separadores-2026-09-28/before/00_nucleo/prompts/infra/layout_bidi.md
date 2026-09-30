# L0 — Visual bidirectional line ordering
Hash do Código: c8f6933a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/layout_bidi.rs`

## Current evidence

- `03_infra/src/layout_bidi.rs:23`, `:39`, `:49`, `:128` — metrics, document/page/line passes.
- `03_infra/src/layout_bidi.rs:183`, `:305`, `:334`, `:409`, `:485` — visual ordering, space coalescing and suffix separation.
- `03_infra/src/layout_bidi.rs:646`, `:782` — controls preserving L1 wrapping.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Preserve `reorder_bidi_document(doc: PagedDocument, metrics: &dyn FontMetrics) -> PagedDocument`. Own horizontal visual ordering of top-level page items before shaping. The paged pipeline owner places this pass; paragraph direction and line breaking remain owned by L1 layout.

Cluster page items by baseline with `Y_TOLERANCE_PT=0.01`. Semantic envelopes retain their existing first-child coordinate convention. Detect an RTL base paragraph from concatenated plain Text with Unicode bidi auto direction; an LTR-base line remains unchanged. Reordering operates on plain Text only. Nontext items retain their geometry even when vector ordering changes.

For an RTL line, use `FontMetrics::text_width` for word widths and `line_content_right` for the occupied right edge. Compute nonnegative inter-item gap from occupied width minus total word widths, divided by the number of intervals. Resolve visual runs over item text ranges; explicit RTL text style supplies the RTL default level inside this ordering helper. Preserve text/style pairing, unmapped-item handling, empty-paragraph fallback and left-to-right positioning from the existing `x_min`.

Sort the line's items by increasing x. A standalone whitespace item can be coalesced as one trailing space into the preceding nonspace Text in visual order; retain the existing empty-item placeholder behavior rather than remove entries while indices are in use. Separate a terminal LTR suffix from an RTL base using L/EN/AN classes and existing ASCII punctuation handling. Preserve original trailing whitespace on the base, keep its style/y, and place the separate suffix at `x_base - suffix_width`. Pure-suffix text is not split.

L1 baselines are authoritative: never merge lines, recompute wrapping, infer paragraph boundaries from distance, estimate width from byte length or infer the region width from emitted ink. `ExplicitLinebreakBoundary` and `ParbreakBoundary` remain transparent carriers; no cross-baseline fusion consults them. Preserve page dimensions, pagination, leading and nontext positions. This pass neither supplies vertical writing nor guarantees an extractor-specific logical text order.

`00_nucleo/prompts/infra/shaper/bidi.md` owns character-level shaping runs. This owner does not duplicate its run data or shaping policies.

## Verification boundary

Existing tests cover Latin identity, Arabic/mixed visual order, real widths, whitespace and suffixes, empty text, nontext geometry and preserved wrapping/explicit breaks. Sequential extraction must not justify changing glyph order inside the shaper.

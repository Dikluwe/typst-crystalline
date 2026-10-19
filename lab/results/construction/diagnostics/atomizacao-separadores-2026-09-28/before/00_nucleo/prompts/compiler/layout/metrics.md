# L0 — metrics
Hash do Código: cedc0f51

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/metrics.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/metrics.rs:22`, `01_core/src/compiler/layout/metrics.rs:27`, `01_core/src/compiler/layout/metrics.rs:37`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public injected FontMetrics contract and its current defaults: text advance, vertical metrics/cap/text edges, ink, shaping width, math constants, glyph variants/assembly/kern and accent/correction access. Preserve Send + Sync and the existing borrowed-trait forwarding methods without adding public methods or stronger font guarantees.

Text width prefers supplied shaped advance, otherwise plain advance, then adds tracking for Unicode codepoint pairs only. Preserve the existing complex-script predicate. FixedMetrics remains a deterministic approximation (advance 0.6 per character at size, vertical factors 0.8/1.2, default cap edge 0.7; current x-height/ascender/descender/explicit edge handling), not production font parity. line_content_right preserves its current item projections and ignores direct private HTML markers. Shared frame-helper extents belong to `00_nucleo/prompts/compiler/layout/helpers.md`; do not replace this trait method's own existing projection by prose.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

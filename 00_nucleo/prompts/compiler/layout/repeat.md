# L0 — repeat
Hash do Código: f9aa0b9d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/repeat.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/repeat.rs:17`, `01_core/src/compiler/layout/repeat.rs:27`, `01_core/src/compiler/layout/repeat.rs:58`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Render one body occurrence and record a pending repeat only when it remains inline, produces a non-empty range, has finite positive width and finite nonnegative resolved gap. Preserve the range, width, gap and justify flag in hub-owned storage.

Given a valid finite positive fill and range, expand by `floor((fill + gap)/(width + gap))`, clamped to the existing range from one through 1000 copies. With multiple copies and justify enabled distribute only nonnegative residual across the inter-copy gaps. Clone and translate the measured piece, insert copies after the original range and report the inserted item count. Do not lay out the body again per copy. Invalid fill/stride/range and infinite dimensions terminate without expansion. `00_nucleo/prompts/compiler/layout/line_commit.md` supplies fill, associates fractional boundaries, clears buffers and repairs suffix indices; it does not own copy/gap arithmetic. RTL, internally broken pieces and unmeasured contexts remain limited rather than assumed parity.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

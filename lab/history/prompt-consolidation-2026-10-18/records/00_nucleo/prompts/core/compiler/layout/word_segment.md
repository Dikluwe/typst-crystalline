# L0 — word segment
Hash do Código: 6970400c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:7b8a61eaafc8a07df12218f8f0560c33fae5c36f621b822b7350ccefc9774711

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/word_segment.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/word_segment.rs:22`, `01_core/src/compiler/layout/word_segment.rs:28`, `01_core/src/compiler/layout/word_segment.rs:46`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Select Han, Hiragana, Katakana, Thai, Lao, Myanmar and Khmer text for ICU line segmentation. Use embedded LSTM tables and reconstruct the segmenter per invocation. Apply the existing opening/closing CJK quote breakpoint filter when the active language or script supplies CJK context. Forward non-empty UTF-8-valid fragments in breakpoint order to the Cursor chunk hook; do not insert spaces, change shaping, reinterpret language or decide pagination. Word measurement and wrapping remain in `00_nucleo/prompts/core/compiler/layout/cursor.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

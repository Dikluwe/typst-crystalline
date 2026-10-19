# L0 — text
Hash do Código: c285fe7d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/text.rs:20`, `01_core/src/compiler/layout/text.rs:209`, `01_core/src/compiler/layout/text.rs:237`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Build effective TextStyle from the active chain and cached style using the source's field-specific precedence; preserve family/variants/weight/direction/language/variation/edge/leading/tracking/highlight and math/cramped/script provenance. Carry math_text_item independently of math, and semantic_strong/semantic_emph only from their existing morphological provenance, not arbitrary bold/italic appearance.

Preserve current sub/superscript sizing fallback 0.65 and baseline offsets, and the existing SmallCaps lowercase-to-uppercase run rendering at factor 0.8 with temporary size restoration. Split ordinary text by literal spaces, dispatch words/chunks through Cursor and advance/record inter-part space opportunities as currently implemented. Restore the outer style. This unit does not shape through I/O, choose installed fonts, re-evaluate show rules or replace SpaceFromNewline's adapter.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

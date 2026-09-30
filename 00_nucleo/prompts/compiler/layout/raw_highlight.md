# L0 — raw highlight
Hash do Código: 4b333868

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/raw.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/raw.rs:103`, `01_core/src/compiler/layout/raw.rs:117`, `01_core/src/compiler/layout/raw.rs:20`, `01_core/src/compiler/layout/raw.rs:41`, `01_core/src/compiler/layout/raw.rs:47`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Raw rendering in `raw.rs`, not a separate renderer or language parser. Use embedded two_face syntax definitions and the existing Typst Light scope theme; only immutable lazy syntax/theme data is shared. Start from the current raw base style (clear bold/italic/fill, preserve the existing remaining fields), apply supported token foreground/bold/italic and restore the outer style after rendering. Block raw closes surrounding lines and starts token lines at the margin. Unknown syntax or highlighting failure uses the existing plain-token path. The current implementation uses `split_whitespace` for token emission, so the older whitespace-preservation claim is unresolved, not proven. Preserve the existing text/whitespace requirement as an open contract discrepancy; no fix, normalization expansion or weakening of the language requirement is authorized here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

# L0 — heading
Hash do Código: 2d297d4f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/heading.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/heading.rs:28`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Scale the active body size through the helper's level mapping and create a bold Heading TextStyle that retains the other outer fields, including inherited italic. Resolve above spacing against the body size (`1.8em` at level one, `1.44em` otherwise) and below (`0.75em`); collapse entry against the preceding block's physical bottom and pending margin, without empirical offsets. A top-of-region heading starts at margin plus its own top edge. Number only when the chain enables heading numbering: format the causal hierarchical counter with the supplied pattern and append one space, or retain the formatted-counter compatibility prefix with its period. Dispatch and close the body, record its causal baseline and pending below margin, clear stale paragraph/equation collapse state, and restore the outer style. A forced `italic: false` clause in older prose is superseded by the actual inherited-style consumer, not a new behavior change.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

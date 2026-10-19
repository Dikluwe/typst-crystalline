# L0 — vanilla defaults
Hash do Código: f9f664e8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/vanilla_defaults.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `lab/typst-original/crates/typst-library/src/model/par.rs:210`, `lab/typst-original/crates/typst-library/src/model/par.rs:224`, `lab/typst-original/crates/typst-library/src/layout/container.rs:348`, `01_core/src/compiler/layout/vanilla_defaults.rs:14`, `01_core/src/compiler/layout/vanilla_defaults.rs:18`, `01_core/src/compiler/layout/vanilla_defaults.rs:23`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own only the canonical pure numeric defaults: `PAR_SPACING = 1.2`, `PAR_LEADING = 0.65`, `BLOCK_SPACING = 1.2`, expressed as font-size multipliers. These constants are not historical benchmark measurements. Consumers resolve them against their effective font size and own their placement/collapse formulas. Do not infer every blocklike feature's semantics from equality of these values.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

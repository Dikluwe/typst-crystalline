# L0 — h space
Hash do Código: 51c8caf9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/h_space.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/h_space.rs:22`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The spacing-only probe in `logical-before-complete/results.json.gz` preserves explicit space under absolute scaling. `h_space.rs:27–30` only updates the cursor, which is not the causal witness consumed by transform. Changing to that witness must preserve absolute h's contribution, including at the end of a line.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve absolute spacing against the active font size and advance cursor_x. Fractional spacing records the current line-item boundary and its weight, with matching natural width zero; `00_nucleo/prompts/core/compiler/layout/line_commit.md` owns allocation and expansion. Do not allocate vertical fractions or emit an item here.

Record the cursor's endpoint after advance in the existing causal accumulator, without a new item, repeated execution or new state. Fractional spacing remains allocated by `00_nucleo/prompts/core/compiler/layout/line_commit.md`.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Refute the correction with text followed by explicit h under absolute scaling and controls without transformation. This is an internal correction under ADR-0127, with no new public contract.

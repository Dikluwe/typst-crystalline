# L0 — hide
Hash do Código: afc6549a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/hide.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/hide.rs:16`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

Historical scope-tracking evidence is in `lab/results/construction/diagnostics/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` and `summary.json`: 30 sources/90 PDFs, with commands, sources and exact state in `initial.json` and `before-pdf/provenance.json`, against vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`. Repeated equal scopes accumulate approximately 0.4pt and nested scopes 0.8pt; positive wrap changes the line. `text_tracking.rs:50` compares only resolved TextStyle. Vanilla `inline/collect.rs:287-295` preserves segments from distinct chains; `inline/shaping.rs:1269-1295` adds tracking only within each run. This measures observable text boundaries without requiring PartialEq or the chain representation to be copied. Hide/Box are also boundaries: `hide.rs:21-24` and `boxed.rs:157-208` replace/restore buffers without preserving that distinction.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Save and temporarily remove the current emitted-items and pending-line buffers, dispatch the body, then restore those buffers. Preserve the current layout-space and other causal effects of the body; this is not an evaluation suppressor or an all-state rollback. Do not turn hidden content into visible items.

When restoring exterior buffers, interrupt text continuity through `State::interrupt`: hidden content is an object barrier rather than adjacent parent text. Do not restore advances or other causal effects of hiding.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

This internal parity correction follows continuous flow under ADR-0127. Require RED→GREEN, bilateral text/position/line controls, preserved uniform-tracking controls, error-free build/lint and resealing restricted to the affected owners. All other open limits remain open.

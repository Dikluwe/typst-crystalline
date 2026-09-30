# L0 — par line callbacks
Hash do Código: 1255ba9d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/par_line_callbacks.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/par_line_callbacks.rs:18`, `01_core/src/compiler/layout/par_line_callbacks.rs:27`, `01_core/src/compiler/layout/par_line_callbacks.rs:34`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the public request/resolution types, sealed par.line callback store, attempt transcript and observational matching. Match by function, line number and collapsed style-chain observation; consume each matching resolution at most once. Missing resolution records the request and remains Pending rather than fabricating Content. Preserve existing reexports and signatures through the hub, without introducing I/O, Engine or Func execution.

Own the final region-local maximum-number-width fixup over the existing hub buffer, keeping item indices and left/right horizontal alignment consistent. Physical baseline/page/counter selection belongs to `00_nucleo/prompts/compiler/layout/line_commit.md`; segmented candidate consolidation belongs to the column owner. Store/transcript lifetime is coordinated by `00_nucleo/prompts/compiler/layout.md`; this module does not decide rollback, regional progress or the document fixpoint.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

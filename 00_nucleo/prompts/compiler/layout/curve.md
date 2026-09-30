# L0 — curve
Hash do Código: d2bccae0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/curve.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/curve.rs:20`, `01_core/src/compiler/layout/curve.rs:126`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Resolve Move, Line, Cubic and Quad coordinates against the active font size; convert a quadratic segment to its equivalent cubic using the current point. Smooth close reflects endpoint controls within the latest subpath and then emits ClosePath; straight close emits ClosePath without adding the smooth segment. Preserve subpath order. Measure the resolved path bounds, use ordinary fitting/line closure, emit the current unfilled path with the existing black stroke and default fill rule, and advance by its height. Segment construction is owned here; backend path serialization is not. Rich fill/stroke/fill-rule propagation described by older text is not implemented by this consumer and is not authorized by this cleanup.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

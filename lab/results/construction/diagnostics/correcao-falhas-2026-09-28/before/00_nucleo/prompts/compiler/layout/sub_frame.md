# L0 — sub frame
Hash do Código: 35dbb30a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/sub_frame.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/sub_frame.rs:29`, `01_core/src/compiler/layout/sub_frame.rs:65`, `01_core/src/compiler/layout/sub_frame.rs:329`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

The isolated path temporarily installs a SubLayoutRegion and preserves/restores the existing outer cursor, dimensions, line/emitted buffers, page margins, pending baseline/collapse metrics, subframe/unconstrained flags, collectors and validated-prefix one-shot arm in LIFO order. Clear the outer arm locally so a body flush cannot consume it. Return the local items/height/decoration segments plus orphaned horizontal/vertical alignment paths without applying a second parent origin.

The inline path dispatches the body and drains only its pending-line tail; optionally align that tail while preserving the parent's prefix. It is not an all-state isolated path: callers such as Box own additional fraction/repeat isolation. Preserve the existing inline assertion that origin_x and width are zero and the present height fallback for material non-text items.

Direct private HTML markers are retained and returned in order but do not contribute visual had-items, fallback height or decoration metrics. Existing Semantic/Link/Group wrappers retain their old material classification. This module neither extracts ids nor removes carriers; `00_nucleo/prompts/compiler/layout/html_frame.md` owns both. No independent page dispatcher or checkpoint is introduced.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

# L0 — format options
Hash do Código: 8a0ad027

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/format_options.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/format_options.rs:215`, `01_core/src/compiler/layout/format_options.rs:249`, `01_core/src/compiler/layout/format_options.rs:19`, `01_core/src/compiler/layout/format_options.rs:20`, `01_core/src/compiler/layout/format_options.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own pure traversal collecting effective document.format.* options. Keep the canonical keys and defaults for SVG pretty, PNG ppi 144, HTML pretty, PDF pretty/tagged/pages/standards. Preserve independent option folding and lexical precedence: later effective values win; an empty set delta is neutral rather than resetting unrelated fields.

Traverse only the current transparent root Sequence/Styled paths; a document-format set nested in an unsupported container remains diagnosed through the existing error path, not silently applied. Preserve validated typed projections including tagged Auto/bool, positive PDF page selectors/ranges/arrays with empty selection distinct from absence, and accepted standard names. Reconstruct standards through the existing pure validated types and fail closed for impossible input forms.

The hub only reexports this helper. Target-specific export, feature flags, CLI options, PNG raster resizing and format I/O belong elsewhere; this collector does not change layout phase or page geometry.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

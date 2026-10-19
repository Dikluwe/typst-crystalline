# L0 — image
Hash do Código: 0b28f0d8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/image.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/image.rs:18`, `01_core/src/compiler/layout/image.rs:22`, `01_core/src/compiler/layout/image.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the pure image dimension calculator and block-layout adapter. Read intrinsic pixels, DPI and EXIF orientation through injected ImageSizer, not a decoder or filesystem. Use DPI fallback 72 and convert by 72/dpi; metadata precedence is the sizer owner's responsibility. Preserve unknown-size fallback 100pt square. Orientations five through eight swap dimensions used for aspect/layout but retain original intrinsic pixel dimensions for export.

Dimension overrides preserve the current Float/absolute-Length projection. With both axes, retain the target rectangle and fit: stretch uses it directly; cover/contain preserve aspect according to the current wide-image branch. Center the transformed image in the target, clip only cover, and use target height for cursor/fitting. One-axis overrides preserve aspect with no target clip; no overrides use converted intrinsic dimensions.

Close pending text and apply the existing main-flow block spacing/collapse protocol, preserving the source's image spacing constant of 1.2em, matching the canonical default in `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`. Keep the distinct post-text baseline anchor versus first/post-block cap-height anchor; subframes bypass this main-flow protocol. Preserve ordinary pre/post overflow requests and emit original bytes, intrinsic dimensions, orientation and alt without synthesis, keeping None distinct from Some(empty). Geometry follows the shared coordinate convention, not the old prose's upward-Y assertion. No renderer or PDF matrix policy is owned here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

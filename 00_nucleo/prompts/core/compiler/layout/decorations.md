# L0 — decorations
Hash do Código: 8235ec28

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/decorations.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/decorations.rs:21`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The 12 SVG pairs in `bibliography-hanging-2026-10-18/deco-before.json.gz`, with corrected line/path primitive reading in `deco-inspection.json`, use the same binary/freeze as the initial 9026 suite and pinned bd5ebec oracle. In overline h10, second-line text is at x18 while the stroke is at x8; vanilla starts the stroke at x18. h0 already has y/thickness/tail differences, which this horizontal correction neither hides nor closes. Local source: `decorations.rs:43–45,76–78` uses global cursor/line_start_x; `line_commit.rs:615–619` and `sub_frame.rs:350–354` capture global start. This is language geometry, without an obligation to copy vanilla buffers. The negative preview may be less than the real cursor before any admission; therefore end > preview alone does not prove a segment.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Dispatch the body under a temporary decoration collector and restore the outer collector. Preserve wrap segments and the final still-pending segment. Resolve offset and extent against the active font size; defaults are underline `0.10em`, strike `-0.25em`, overline `-0.80em`, thickness `max(0.05em, 0.4pt)`. Explicit stroke color takes precedence over active fill. Emit the existing butt-capped lines with matching segment geometry. Do not create a parallel line-commit or page algorithm.

Capture start_x_initial through the private cursor_start preview. After wraps, the last segment uses the effective left edge from `00_nucleo/prompts/core/compiler/layout/paragraph_indent.md`, not global line_start_x. Do not advance baseline/admission, alter offset/thickness/extent or rerun body; preserve nested collectors and a terminal segment without flush.

Expose only a private read-only applied query in the paragraph-indent owner. Here, when preview differs from the actual initial cursor and no segment has been flushed, the final segment requires an origin actually applied. Do not start a scope/admission to satisfy decoration or create paint/height from an empty body. This private parity correction follows continuous flow under ADR-0127, with no new phase/API.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Refutation cases include duplicated origin, premature compensation consumption, and changes to baseline, end, thickness, text or region outside the prescribed horizontal delta. Add RED tests for empty decoration after a break with negative h, including empty body, metadata and empty Par. This horizontal refinement certifies LTR only; complete decoration geometry and RTL remain unmeasured, without a universal claim.

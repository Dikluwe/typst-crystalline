# L0 — boxed
Hash do Código: ed5effdb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:6dbf8faa56960845c60734f5e047685ec5b3a6f14c0ce3a48d333b8982d0baa5

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/boxed.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/boxed.rs:17`, `01_core/src/compiler/layout/boxed.rs:387`, `01_core/src/compiler/layout/boxed.rs:415`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own inline Box geometry: explicit dimensions, insets/outsets, baseline shift, clipping, radius, fill/stroke and parent bbox. Measure through the existing constrained helper but obtain natural text baseline/inline extents from composition. A body with a text baseline uses inset plus actual ascent; a non-text body retains its box-bottom fallback. The baseline may lie outside the box; aggregate nonnegative ascent/descent without clipping its meaning.

Isolate the parent's pending line, horizontal fractions, natural widths and repeats while composing the body, then restore and append body-local boundaries rebased by the parent prefix once. A fractional box registers its boundary only after body emission and records its provisional natural width; `00_nucleo/prompts/compiler/layout/line_commit.md` replaces that occupancy by the allocation rather than adding both. Child repeats remain associated with their real boundaries.

Preserve the existing paragraph-to-box calibration branch and its size-scaled values at the source evidence anchor. It is an empirical compatibility behavior, not a vanilla-derived general typography formula; replacement requires separate evidence. Ignore only direct private HTML markers when deciding an empty line. Restore outer bbox/width and retain current shape/group/inline emission behavior.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

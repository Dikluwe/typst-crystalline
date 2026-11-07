# L0 — flush
Hash do Código: c5432c81

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/flush.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/flush.rs:14`, `01_core/src/compiler/layout/flush.rs:31`, `01_core/src/compiler/layout/flush.rs:45`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Capture the pending-float prefix boundary by occurrence and call Cursor's `finish_float_prefix_at_marker`. With no pending floats this is a no-op. Do not flush the text line, emit a FrameItem, consume later floats, calculate fitting or force a page break. Stabilization and ordinary progress remain in `00_nucleo/prompts/core/compiler/layout/cursor.md`; fitting and emission remain in `00_nucleo/prompts/core/compiler/layout/place.md`. Only the active Layouter participates.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.


## Approved issue8890 Figure placement — 2026-10-31

### Measurement before decision

`lab/results/construction/diagnostics/issue-8890-2026-10-31/{analysis.json,svg-geometry.json,report.md}` preserves the exact HEAD3c6b0ce7, UTC, clean/dirty state, pinned vanilla64c5e8c0 executable/source/inputs and PDF/SVG measurements. Original Figure placement is rejected in Tekt at stdlib/figure_image.rs:342-360; vanilla loses D after deferring the float. The supported Place control preserves A–E but puts its float on page3 after continuation text, so it does not certify the original trigger. Current source/owner bytes and concurrent dirty state before this amendment are preserved under `lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/before/` and before.json. Geometry/presence/positions are language; queue/carrier mechanics need not copy vanilla.

The user explicitly approved the concrete public-contract proposal with “Aprovo” in this chat; approval.json binds its exact scope. This satisfies ADR-0127 for FigurePlacement and the two public fields only, with existing layout phase/default in-flow behavior retained. The prior placement scope-out is superseded only for this approved subset.

### Exclusive responsibility

Initialize the new private DeferredFloat Figure discriminator false in legacy flush fixtures only, preserving all existing flush-marker behavioral assertions.

### Gates and limits

L0 and scoped reseal precede real RED, code and GREEN; valid V15/V26 precede repairs. Preserve all other ownership, previous clauses/ratifications and concurrent changes. Use only /dev/shm/typst-crystalline-8890-materialization-target for this amendment's compilation/test temporaries, one job, 12GiB virtual ceiling and zero swap. This target supersedes older task-specific target paths only for the present amendment. Record state/hash/UTC around every measurement. Revalidate original and link PDF/SVG, controls and finite progress with all A–E once; page count itself is not an oracle for a known vanilla defect. No new phase/default/compatibility change beyond the approved fields. Do not suppress lint, weaken tests, copy upstream defects, commit or claim global qualification from scoped checks.

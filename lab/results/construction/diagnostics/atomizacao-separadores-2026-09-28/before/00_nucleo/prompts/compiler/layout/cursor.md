# L0 — cursor
Hash do Código: e6571615

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:2ccbb1e5daf5f6806e58cd48272b1463fbc9107300c0bce6ea1c3ccf7b0b8748

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/cursor.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/cursor.rs:27`, `01_core/src/compiler/layout/cursor.rs:33`, `01_core/src/compiler/layout/cursor.rs:41`, `01_core/src/compiler/layout/cursor.rs:94`, `01_core/src/compiler/layout/cursor.rs:195`, `01_core/src/compiler/layout/cursor.rs:314`, `01_core/src/compiler/layout/cursor.rs:429`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own word/chunk emission, initial baseline, wrapping, RTL line placement, regional/page transitions, marker-prefix coordination, pending-float drain and footnote-body emission. The hub retains all state/checkpoints. Line closure and horizontal allocation belong to `00_nucleo/prompts/compiler/layout/line_commit.md`; transactional suffix decisions to `00_nucleo/prompts/compiler/layout/flow_suffix.md`; float fitting/reservation/emission to `00_nucleo/prompts/compiler/layout/place.md`; cell-tail emission to `00_nucleo/prompts/compiler/layout/cell_tail_commit.md`; auto dimensions to `00_nucleo/prompts/compiler/layout/page_geometry.md`; intermediate Page construction to `00_nucleo/prompts/compiler/layout/page_commit.md`; column close/merge/start hooks to `00_nucleo/prompts/compiler/columns.md`.

Use FontMetrics.text_width as the word-width authority, including shaping and tracking; use the active space advance and baseline offset for emission. Establish the first baseline from the active top edge. At a finite full region with committed content, progress before emitting the next first line; do not drain new text beyond the old region then move it. Empty/unconstrained regions do not create no-progress loops. For an already-certified fractional tail only, preserve the existing bottom snap bounded by the outer below-minus-leading boundary and fitting tolerance; ordinary overflow is unchanged.

Maintain inline ascent/descent maxima and translate all current-line items, link/semantic children and matching metadata consistently when a baseline changes. Do not replace child geometry by recognizing Block, Place or future Content.

Word handling retains ordinary measured wrapping, hyphenation fallback and overlong-word progress; `00_nucleo/prompts/compiler/layout/word_segment.md` owns script segmentation and quote tailoring, `00_nucleo/prompts/compiler/layout/hyphenation.md` owns break-point generation. Ordinary singleton closing must not acquire Sequence's validated-prefix exception.

The float marker drains only its captured prefix through the active regional distributor, preserving order and excluding later occurrences. Ask Place for admissible fitting and reservation; finish/advance only by the existing transition rules and require progress. After the prefix stabilizes, mark the hub's suffix checkpoint boundary without flushing pending text or unconditionally breaking. Accept a valid suffix before ordinary transition through `00_nucleo/prompts/compiler/layout/flow_suffix.md`; rejection is handled there, not by migrating only the line.

Regional transition preserves existing float/footnote order, auto-dimension timing, column delegation, page commit and next-region reset. Invoke cell-tail emission after the next top is prepared. Footnote bodies retain the existing separator, usable bottom, split/overflow and bounded first-body fallback; no I/O or callback execution is introduced. Treat only direct private HTML markers as non-material via `00_nucleo/prompts/compiler/layout/html_frame.md`; transport them without inventing line height or an occupied region.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

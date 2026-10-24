# L0 — hide
Hash do Código: 0734c5df

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/element-form-b.toml sha256:7510f3bba27256221d4447db608839af562ac71b4eacc0c042f693b531a95184
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42

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


## Approved physical page occurrence protocol

### Evidence before decision

At baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`, the complete source/owner snapshot and applicable ADRs are retained in `lab/results/set-page-header-2026-10-21/agents/core/source-evidence.json.gz` and its supplement. `baseline-parity/receipt.json` there binds current-pin physical query/state/counter/page observations and inert overwritten templates. `ambient-baseline/receipt.json` refutes freezing creation10pt styles: insertion uses actual page20/30pt ambient style. These are language observations; the source plan, packet transcript and rollback representation are mechanics. The exact five facades, opaque types and Paged discovery/realization/source-construction phase were explicitly approved in `lab/results/set-page-header-2026-10-21/review.md` (SHA-256 `cb4a684b483e7f9f6ac3f7eef52277f7ada405033a24e4e66e55811e93c12039`), recorded in `lab/results/set-page-header-implementation-2026-10-21/approval.json`. Only conflicting no-new-phase clauses for this exact protocol are superseded; legacy signatures/defaults and unrelated limits remain.

### Exclusive protocol responsibility

Route this feature's actual source children through closed typed plan edges `HideBody`, retaining their field/index and original source authority. Generated markers, punctuation and decorations remain explicitly unplanned; do not turn equal-looking generated content into source children. Preserve all existing geometry, style, hiding, count, spacing, diagnostics and restoration formulas. Hidden bodies retain causal admissions even though their paint is removed; no execution is introduced. Source-route coverage is checked independently from paint.


## Refutation: unpainted material and required successors

### Evidence before decision

`lab/results/weak-pagebreak-2026-10-23/edge-after/receipt.json` and `unpainted-before/receipt.json` bind the first candidate executable to UTC/inputs/commands; its build and source state are in `validation.json` and `final-source.json.gz`. These controls refute painting alone as page materiality: hidden text, zero/fractional horizontal spacing and zero/weak/strong vertical spacing can require a page without paint; empty/metadata-only Hide does not. `h_space.rs:27-45` advances or queues spacing without an item, `v_space.rs:20-42` records advancement, and `hide.rs:20-28` removes only painted output. `pagebreak.rs:42-44` also overwrites an empty strong successor's requiredness during weak parity adjustment, dropping a required final page. These are language-visible page obligations, not a request to copy upstream collection mechanics. The earlier painting-only hypothesis was rejected, not qualified by changing the oracle or deleting its cases.

### Exclusive correction

Before restoring saved painted buffers, record whether the actually executed hidden body produced material in its local current items/line. Outside isolated subframes, preserve that fact as unpainted page material. Do not mark an empty or metadata-only hidden body. Keep existing body execution, causal plan edges, geometry, hidden paint removal and nested behavior.

L0 precedes code under continuous-flow ADR-0127. No public field, signature, trait, phase, default or compatibility change. Validate the refuters and retain all previous note/float/Context/column/strong/PageRun controls.

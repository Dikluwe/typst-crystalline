# L0 — text_measure
Hash do Código: e4e15534

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/text_measure.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

Inspection of `01_core/src/compiler/layout/text_measure.rs:85`, `word_layout.rs:89` and `space.rs:162` found, respectively, a textual boundary, baseline admission and an initial-space integration test owned by other responsibilities. Exact state, sources and prior prompts are in `lab/results/construction/diagnostics/separacao-tracking-2026-09-29/initial.json` and `before/`. The previously measured correction is in `lab/results/construction/diagnostics/correcao-tracking-2026-09-29.md`; this separation preserves its results and limits. Positions, text and styles are language observables; function placement is mechanical. Inspection establishes neither historical intent nor new bilateral parity.

## Current contract

Own only `default_text_width`, `word` and `space`. `default_text_width` uses `advance_shaped` when available, otherwise `advance`, preserving the existing formula order: base width plus resolved tracking times the number of Unicode codepoint pairs. `word` delegates to `FontMetrics.text_width` and respects injected overrides; `space` measures `advance(" ")` with the active size/style. Do not consult Layouter, decide boundaries, select breaks, emit items or request vertical metrics. The codepoint formula is a fallback, not proof of complete cluster-based selection.

## Acceptance and limits

Preserve existing causal tests and the text, positions, styles, pages and diagnostics of controls preceding the separation. Require 1:1 ownership, valid pins, and error-free build and lint. This is internal movement under ADR-0109/0127: static descendant functions and no new state. Tracking limits for mixed styles, RTL, global selection and cluster counting remain as recorded in the correction diagnostic.

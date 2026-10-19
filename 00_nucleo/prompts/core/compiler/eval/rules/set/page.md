# L0 — compiler/eval/rules/set/page — Page settings and geometry transport
Hash do Código: eaadc8e6

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set/page.rs`

## Evidence before decision

At the historical post-extraction state, the dispatcher in `01_core/src/compiler/eval/rules/set.rs` retained the page arm as a contiguous block. The L1 baseline in `lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz` and the post-extraction suite passed with 6401 tests and 2 ignored. This measures structure; it is neither a benchmark nor a new bilateral upstream measurement.

## Language contract

Preserve width/height/paper/flipped/binding, margins, numbering, header/footer, bleed/fill/background/foreground, casts, lexical style and the produced SetPageDelta. em sizes resolve against the current eval style; subsequent geometry belongs to layout.

The dispatcher calls this owner statically without changing arm order. Preserve errors, spans, argument evaluation, value identity, absent versus present styles and execution phase. Historical decisions and measurements remain in `00_nucleo/prompts/core/compiler/eval/rules.md`.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Preserve language parity under ADR-0107/0108, 1:1 ownership and pins under ADR-0129/0130.

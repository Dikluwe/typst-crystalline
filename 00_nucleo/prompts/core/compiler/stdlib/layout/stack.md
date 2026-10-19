# L0 — stdlib/layout/stack — stack
Hash do Código: 902a412b

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/stack.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

stack preserves direction, spacing and child order, including validation and content-element construction.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

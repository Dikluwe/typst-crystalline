# L0 — compiler/stdlib/structural/math/binom
Hash do Código: 6224466c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/binom.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `math.binom(upper, ..lower)` and bare bridge. Require upper plus at least one lower; reject named arguments in current order with the existing positional hint. Cast each value through the shared P1293 content cast. Join lower values with the current mathematical comma-space, construct an unlined fraction inside parenthesized MathDelimited, preserve current diagnostics and morphology. No layout.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

# L0 — compiler/stdlib/structural/math/limits
Hash do Código: 0ceb0c16

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/limits.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own `scripts(body)` and `math.limits(body, inline: true)` constructors for MathLimitsOverride. Scripts accepts only one Content body, no named, and constructs lateral attachments. Limits validates named positional body, inline Bool, unknown named, and one Content body in the existing order; preserve explicit inline presence even when true. Keep native signatures and exact diagnostics; no layout.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

# L0 — compiler/stdlib/structural/math/casts
Hash do Código: 272e02c0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/casts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-restante-2026-09-30-before.json.gz` places this behavior in `01_core/src/compiler/stdlib/structural/math.rs`. The full core baseline passed 6,401 tests with 2 ignored before movement. This is a structural regression baseline, not a new bilateral vanilla measurement or benchmark. Historical language measurements and pin qualifications remain in the parent L0.

## Language contract

Own shared argument casts used by more than one mathematical constructor: long vanilla type names (`str`→`string`, `int`→`integer`, `bool`→`boolean`); `math_lr_content_value` accepts Content, LocatedContent, String, Symbol and None with the current distinct conversions; `math_lr_size_from_args` validates only `size`, detects duplicate occurrences, and accepts Relative, Ratio or Length while retaining explicit presence; `p1293_math_content` accepts Content, LocatedContent, String and Symbol and preserves MathText morphology. Preserve exact diagnostics, validation order and spans. Other feature owners call these free functions; they do not duplicate the cast table. No layout or I/O.

The parent `math.rs` retains only static registration and shared catalog orchestration. Existing `compiler::stdlib` bridges retain their names and visibility via direct reexports. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing core regressions remain green. The full core suite, `cargo build` and `crystalline-lint .` pass; header ownership is 1:1. No speedup is claimed without separate measurement.

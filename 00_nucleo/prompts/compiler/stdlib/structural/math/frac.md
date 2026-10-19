# L0 — compiler/stdlib/structural/math/frac
Hash do Código: c8bb280f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/frac.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree recorded in `lab/results/construction/diagnostics/atomizacao-stdlib-math-frac-2026-09-30-before.json.gz` contains the style cast, operand cast, constructor and native in `math.rs:291-391`; the static `math.frac` binding is later in the same parent. The P1427 filter passed 15 tests before movement. This is structural and regression evidence, not a new bilateral vanilla measurement or a timing claim. The historical language measurement remains in the parent L0 P1427 section.

## Language contract

Own the `math.frac(num, denom, style: vertical|skewed|horizontal)` argument validation and semantic `Content::MathFrac` construction. Require exactly two positional operands. Accept Content, LocatedContent, String converted to text, and None converted to empty content. Other operand types retain the current content-cast diagnostic. Validate `num:` and `denom:` as invalid named positionals before other named arguments, then reject unknown named arguments, duplicate `style`, missing/excess positional values, style values, and operand values in the existing order with unchanged spans and diagnostic wording.

`style` accepts only the three literal strings; absent remains absent, while an explicit `vertical` remains explicit in the complete carrier. Preserve the constructor's existing semantic flags and its morphology. No layout, dynamic dispatch or second runtime is introduced.

The parent `math.rs` owns only static binding registration for `math.frac`, calling this child's native function. The child retains the current native signature. The payload remains owned by `entities/elements/math_frac.md`, with layout in `compiler/math/layout/frac.md`. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

P1427 regressions and a local explicit-style test remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.

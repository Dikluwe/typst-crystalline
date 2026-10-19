# L0 — compiler/stdlib/structural/math/class
Hash do Código: b4f8b064

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/class.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree in `lab/results/construction/diagnostics/atomizacao-stdlib-math-class-2026-09-30-before.json.gz` places `native_math_class` at `math.rs:1069-1152`, with static registration in the parent and a public reexport through `structural/mod.rs`. `vanilla_type_name_class` at `math.rs:1060-1067` is shared by other natives. Baseline filters passed 29 `math_class` tests and 15 P825 tests. This is structural and regression evidence, not new bilateral vanilla evidence or a timing claim. The parent L0 retains the historical P772y/P825 language measurements.

## Language contract

Own the native `math.class(class, body)` cast and semantic `Content::MathClassOverride` construction. The first positional must be a String in the exact ten-name public cast domain: normal, punctuation, opening, closing, fence, large, relation, unary, binary or vary. Reject internal enum names such as alphabetic with the current exact cast message. For a nonstring class, retain the same message with long vanilla type names via the shared parent helper. The second positional accepts Content, String converted to text, or Symbol preserving its full value. Keep the present order of class validation, body validation and named rejection, the current detached spans, and the current behavior for extra positional items. Preserve all current diagnostic wording and the same native signature.

The shared `vanilla_type_name_class` remains owned by the parent because other natives use it. The parent retains static `math.class` registration; the structural hub directly reexports the native from this child with the same public `compiler::stdlib` path. No layout, dynamic dispatch, public signature, default, phase or compatibility change is authorized.

## Acceptance

Existing `math_class` and P825 regressions plus a local invalid-internal-class test remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.

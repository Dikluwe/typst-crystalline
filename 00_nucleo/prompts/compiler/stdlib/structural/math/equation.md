# L0 — compiler/stdlib/structural/math/equation
Hash do Código: c259e140

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/equation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree recorded in `lab/results/construction/diagnostics/atomizacao-stdlib-math-equation-2026-09-30-before.json.gz` places `equation_content`, option casts and `native_math_equation` at `math.rs:46-210`. The structural hub reexports four helpers via the parent for eval and set rules, while the parent registers the static native. The P1140 filter passed 85 tests before movement. This measures structure and regression, not a new bilateral vanilla measurement or timing. Historical language measurements and snapshot qualifications remain in the parent L0 P1140 sections.

## Language contract

Own the pure `equation_content(body, block)` semantic constructor, the casts `equation_number_align`, `equation_supplement`, `equation_alt`, and the native `math.equation(body, block: false, numbering:, number-align:, supplement:, alt:)`. Preserve exactly one required Content body and its existing missing, excess and invalid diagnostics and spans. Named `block` accepts Bool; `numbering` accepts String, Func or None; `number-align` accepts Align2D except horizontal Center; `supplement` accepts String converted to Content, Content, Func, None or Auto; `alt` accepts String or None. Unknown named arguments fail. Preserve current cast order and exact diagnostic wording.

Keep the existing style-chain transport: only present named options add entries, under `equation.numbering`, `equation.number-align`, `equation.supplement`, and `equation.alt`; omit them without adding a delta. `block` controls the semantic Equation carrier and defaults to false. The complete Content is wrapped in local Styles only when the custom delta is nonempty. Preserve morphology and the same native signature. Do not execute callbacks, resolve layout defaults or move behavior into eval or entities.

The parent `math.rs` retains static binding registration. The structural hub reexports the four helpers directly from this child with the same visibility and `compiler::stdlib` paths. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

P1140 regressions and a local test of supplement/alt cast distinction remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without measurement.

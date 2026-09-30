# L0 — compiler/stdlib/structural/math/cancel
Hash do Código: ec08107b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/cancel.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `00_nucleo/diagnosticos/atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz` places the complete `native_cancel` body in `01_core/src/compiler/stdlib/structural/math.rs:395-529`. Static `math.cancel` registration is in that parent and the public native reexport is in `structural/mod.rs`. The `native_cancel` baseline filter passed 4 tests. This is an internal structure baseline, not a new bilateral vanilla measurement or runtime benchmark.

## Language contract

Own the native `math.cancel(body, length: 100% + 0.3em, inverted: false, cross: false, angle: auto, stroke: auto, background: false)` cast and semantic carrier construction. One positional body is required; accept Content or String converted to text. Preserve the current validation order, spans and exact diagnostics for missing, invalid and excessive positional arguments, invalid named values and unknown named arguments.

`length` accepts Relative or Length, converting Length into an absolute relative length. `inverted`, `cross` and `background` require Bool. `angle` accepts Auto, Angle or Func. `stroke` accepts None or a value handled by the existing `extract_stroke` cast. Preserve the current defaults and order of named validation. Set the corresponding `MathCancelExplicit` bit whenever a named argument is present, including one equal to its default. Construct `Content::math_cancel_full` with the argument span and unchanged semantic fields. Do not measure the body, execute an angle callback, perform layout or introduce a second runtime.

The parent `math.rs` owns only the static `math.cancel` module registration and its other mathematical behaviors. The structural hub directly reexports `native_cancel` from this child, preserving the public path and signature. The payload remains owned by `entities/elements/math_cancel.md` and geometry by `compiler/math/layout/cancel.md`. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing `native_cancel` regression tests and a local test for explicit named presence remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without separate measurement.

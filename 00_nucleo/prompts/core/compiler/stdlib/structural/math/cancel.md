# L0 — compiler/stdlib/structural/math/cancel
Hash do Código: fa965d16

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/cancel.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz` places the complete `native_cancel` body in `01_core/src/compiler/stdlib/structural/math.rs:395-529`. Static `math.cancel` registration is in that parent and the public native reexport is in `structural/mod.rs`. The `native_cancel` baseline filter passed 4 tests. This is an internal structure baseline, not a new bilateral vanilla measurement or runtime benchmark.

## Language contract

Own the native `math.cancel(body, length: 100% + 0.3em, inverted: false, cross: false, angle: auto, stroke: auto, background: false)` cast and semantic carrier construction. One positional body is required; accept Content or String converted to text. Preserve the current validation order, spans and exact diagnostics for missing, invalid and excessive positional arguments, invalid named values and unknown named arguments.

`length` accepts Relative or Length, converting Length into an absolute relative length. `inverted`, `cross` and `background` require Bool. `angle` accepts Auto, Angle or Func. `stroke` accepts None or a value handled by the existing `extract_stroke` cast. Preserve the current defaults and order of named validation. Set the corresponding `MathCancelExplicit` bit whenever a named argument is present, including one equal to its default. Construct `Content::math_cancel_full` with the argument span and unchanged semantic fields. Do not measure the body, execute an angle callback, perform layout or introduce a second runtime.

The parent `math.rs` owns only the static `math.cancel` module registration and its other mathematical behaviors. The structural hub directly reexports `native_cancel` from this child, preserving the public path and signature. The payload remains owned by `00_nucleo/prompts/core/entities/elements/math_cancel.md` and geometry by `00_nucleo/prompts/core/compiler/math/layout/cancel.md`. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing `native_cancel` regression tests and a local test for explicit named presence remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without separate measurement.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Use casts::math_lr_content_value for body; preserve all named flags and validation.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.

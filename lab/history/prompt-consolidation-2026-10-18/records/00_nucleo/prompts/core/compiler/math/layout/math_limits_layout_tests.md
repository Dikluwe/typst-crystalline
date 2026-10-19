# L0 — core/compiler/math/layout/math_limits_layout_tests
Hash do Código: 0fdb5d11


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_limits_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_limits_layout_tests.rs:14` locates `fn style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Test-only obligation

Own `01_core/src/compiler/math/layout/math_limits_layout_tests.rs` only.
The productive core is owned by
`00_nucleo/prompts/core/compiler/math/layout/mod.md`.

Prove that apply_math_default recurses through the body and exactly preserves
limits, inline and inline_explicit, including subsequent morphology.
Changing only inline_explicit must not alter items or geometry.
Omitted inline and explicit true preserve stacking in inline/display;
inline=false uses side scripts inline and stacked limits in display.
Scripts stays lateral even display.

Position assertions observe stacked-versus-lateral relationships, not render
bytes, helper choice or Rust representation. Coverage is self-contained
and does not depend on external artifacts.

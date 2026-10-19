# L0 — core/compiler/math/layout/math_root_layout_tests
Hash do Código: a9a3f8ec


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_root_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_root_layout_tests.rs:14` locates `fn style() -> TextStyle`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Root-presence reconstruction tests

Own only `01_core/src/compiler/math/layout/math_root_layout_tests.rs` and its cfg(test)
integration, without authorizing productive code.

Verify that apply_math_default and apply_math_style preserve index_explicit
in all three causal states while transforming children as contracted.
Compare carriers with identical index/radicand and opposite presence bit:
layout projection, text, class, baseline and emptiness must agree.
Keep sqrt as the None/false control, with canonical
`root(radicand: ...)` morphology and established square-root geometry.

The bit is opaque to layout. Assertions do not freeze serialized bytes or
a private Rust structure beyond what is needed to observe the contract.

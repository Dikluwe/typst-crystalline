# L0 — math/layout/op
Hash do Código: 7b8881f3


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/op.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/op.rs:21` locates `pub(super) fn layout_op(&self, text: &Content, style: &TextStyle) -> MathBox`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Text operator layout

Own only the thin MathOp text-layout delegate. Return the ordinary math layout
of its text in the supplied style. Do not rewrite textual operators into
glyph variants. The Large spacing class is owned by
`00_nucleo/prompts/compiler/math/layout/spacing.md`; display-only use of
MathOp.limits is owned by `00_nucleo/prompts/compiler/math/layout/attach.md`.
Moving or duplicating those decisions here is not part of this contract.

Acceptance checks unchanged textual morphology/style and transparent geometry;
operator classification and attachment policy are integration checks of the
respective owners, not another implementation in this module.

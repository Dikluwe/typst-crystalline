# L0 — DividerElem
Hash do Código: 547558bc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/divider.rs`

## Evidence

Retained extraction-era anchors are `content.rs:1726` for empty text and
`content.rs:1512` for the nonempty structural singleton.

## Contract

`DividerElem` is a fieldless structural singleton with Debug/Clone/PartialEq/Hash.
Its text is empty but `is_empty` is always false. Both maps are terminal and
return the corresponding Content singleton without visiting children.
get_field, element_kind and to_payload retain default None; it is non-locatable.
All DividerElem instances are structurally equal. The trait is owned by
`00_nucleo/prompts/core/entities/elements/mod.md` and Content construction by its
own canonical child. Rendering does not belong to this entity; the old 0.5pt
line behavior remains a layout-owner handoff recorded in the diagnostic.
Verify empty text, nonempty structural identity, singleton equality and terminal maps.

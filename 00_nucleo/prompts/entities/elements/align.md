# L0 — AlignElem
Hash do Código: f8d8e349

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/align.rs`

## Evidence

Retained extraction-era anchors are `content.rs:1798,2356,2640,1958` for text,
maps and equality. They are historical coordinates, not current-pin parity proof.

## Contract

`AlignElem` has public `alignment: Align2D` and `body: Content`, deriving
Debug/Clone/PartialEq. Structural equality compares both fields. Preserve the
specified manual Debug-based Hash because Align2D does not implement Hash;
this mechanical resource contract is not language observational equality.
`plain_text` delegates to body. `is_empty` remains the trait default false even
when body is empty. Both maps recurse into body and preserve alignment.
`get_field`, `element_kind` and `to_payload` retain their trait defaults.
The element is non-locatable. Content construction belongs to
`00_nucleo/prompts/entities/content/construction.md`, and the trait to
`00_nucleo/prompts/entities/elements/_comum.md`; no layout logic belongs here.
Verify text, both recursive maps, alignment preservation and structural equality.

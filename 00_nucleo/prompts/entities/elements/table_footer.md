# L0 — TableFooterElem
Hash do Código: 88b07686

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table_footer.rs`

## Contract


`TableFooterElem` stores public body: Content and repeat: bool, with
Debug/Clone/PartialEq and the specified hash obligation. Plain text and
emptiness delegate to body. Both maps recurse into body, preserving repeat
and Content::TableFooter identity. Fields/kind/payload default to None:
non-locatable. Structural equality includes body and repeat. Construction
and repetition during layout belong to their own owners.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`00_nucleo/diagnosticos/sanitization-entities-contracts-history.md`.

# L0 — TableHeaderElem
Hash do Código: dfa723e4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table_header.rs`

## Contract


`TableHeaderElem` stores public body: Content and repeat: bool, with
Debug/Clone/PartialEq and the specified hash obligation. Plain text and
emptiness delegate to body. Both maps recurse into body, preserving repeat
and Content::TableHeader identity. Fields/kind/payload default to None:
non-locatable. Structural equality includes body and repeat. Construction
and repetition during layout belong to their own owners.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

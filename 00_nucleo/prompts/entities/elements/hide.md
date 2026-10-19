# L0 — Hidden content carrier
Hash do Código: 86c62663

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/hide.rs`

## Contract


`HideElem` stores public body: Content with Debug/Clone/PartialEq and its
specified hash obligation. Plain text is empty even for a nonempty body.
Emptiness delegates to body; both maps recurse into body and preserve Hide
identity. Field/kind/payload access defaults to None; the element is
non-locatable. Structural equality compares the body.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

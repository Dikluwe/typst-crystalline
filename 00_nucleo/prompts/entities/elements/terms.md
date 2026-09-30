# L0 — Term list carrier
Hash do Código: 667514fc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/terms.rs`

## Contract


`TermsElem` stores public items: Vec<Content>, with Debug/Clone/PartialEq
and the specified hash obligation. Plain text joins item projections with
newline. Emptiness checks whether the item vector is empty, not whether its
items individually project to empty text. Both maps recurse in item order and
preserve Terms identity. Fields/kind/payload default to None: non-locatable.
Structural equality includes item contents and order.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`00_nucleo/diagnosticos/sanitization-entities-contracts-history.md`.

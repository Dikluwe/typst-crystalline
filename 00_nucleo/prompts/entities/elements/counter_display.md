# L0 — Counter display carrier
Hash do Código: db3bf8c7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/counter_display.rs`

## Contract


Preserve the specified `CounterDisplayElem { kind: String }` carrier and
structural equality/hash by kind. The current consumer instead stores CounterKey;
that public-type discrepancy is recorded, not authorized or reversed here.
Plain text is empty; emptiness is false. Both maps are terminal and preserve
the carrier. Fields/kind/payload retain None defaults: this is non-locatable,
distinct from the locatable counter-display callback carrier. Content's ergonomic
counter_display construction belongs to its construction owner.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

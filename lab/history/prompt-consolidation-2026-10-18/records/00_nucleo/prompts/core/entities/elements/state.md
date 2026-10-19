# L0 — State initialization carrier
Hash do Código: a527fcc9

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/state.rs`

## Contract


`StateElem` has public `key: String` and `init: Box<Value>` with
Debug/Clone/PartialEq; hash is manual via Debug. Plain text is empty; emptiness
defaults false; maps are terminal. Kind State and payload State carry key/init.
Registry population belongs to introspection, not the entity.

The prior L0 explicitly required Content::State to remain unequal in the hub,
even though StateElem derives structural equality. That cross-owner obligation
is not revoked by this cleanup; its reconciliation with the content equality
owner is pending in the diagnostic. Do not silently equate entity equality,
hub equality and language equality.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

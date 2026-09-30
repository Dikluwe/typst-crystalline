# L0 — State display carrier
Hash do Código: ced154b4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/state_display.rs`

## Contract


`StateDisplayElem` has public `key: String` and `callback: Option<Func>`.
Debug/Clone/PartialEq retain both fields; callback equality remains Func's
existing identity equality. Hash is manual via Debug. Plain text is empty;
emptiness defaults false; child/text maps are terminal. It is locatable:
kind StateDisplay and payload StateDisplay carry key/callback. Post-fixpoint
rendering is not implemented here. Structural equality compares key and callback.


The trait API/defaults belong to
`00_nucleo/prompts/entities/elements/_comum.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`00_nucleo/diagnosticos/sanitization-entities-contracts-history.md`.

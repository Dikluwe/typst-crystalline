# L0 — Padding carrier
Hash do Código: 221862bf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/pad.rs`

## Contract


`PadElem` stores public body: Content and sides: Sides<Option<Length>>,
with Debug/Clone/PartialEq and manual Debug-based hash. Preserve the known
signed-zero caveat in that hash strategy; cleanup does not approve a new algorithm.
Text and emptiness delegate to body. Maps recurse into body, preserving sides
and Pad identity. Fields/kind/payload default to None: non-locatable.
Structural equality compares body and sides, not just the text projection.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

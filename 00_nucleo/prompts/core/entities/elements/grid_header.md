# L0 — GridHeaderElem
Hash do Código: 7383a3a0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/grid_header.rs`

## Contract


`GridHeaderElem` stores public body: Content and repeat: bool, with
Debug/Clone/PartialEq and the specified hash obligation. Plain text and
emptiness delegate to body. Both maps recurse into body, preserving repeat
and Content::GridHeader identity. Fields/kind/payload default to None:
non-locatable. Structural equality includes body and repeat. Construction
and repetition during layout belong to their own owners.


The trait API/defaults belong to
`00_nucleo/prompts/core/entities/elements/mod.md`. This owner specifies only
this element consumer; language constructors and layout algorithms remain
with their respective owners. Historical inline-arm evidence is retained in
`lab/results/construction/diagnostics/sanitization-entities-contracts-history.md`.

## Grid headers and footers package

A grid header carries its `level`, default 1, like a table header.

Approved on 2026-11-26 (“Aprovo A–D”): `00_nucleo/diagnostics/grid-headers-package-2026-11-26.pt-BR.md`.

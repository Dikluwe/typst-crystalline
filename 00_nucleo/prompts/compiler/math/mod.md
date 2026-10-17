# L0 — math/mod
Hash do Código: 6d1cc5aa


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/mod.rs`

## Consumer evidence

`01_core/src/compiler/math/mod.rs:7` locates `pub mod layout;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Mathematical subsystem hub

Own only `01_core/src/compiler/math/mod.rs`. Declare `pub mod layout` and
`pub mod symbols`; do not add algorithms, behavioral wrappers or reexports.
`00_nucleo/prompts/compiler/math/layout/_comum.md` owns the MathLayouter
core and `00_nucleo/prompts/compiler/math/symbols.md` owns pure symbol
lookup. Mathematical layout remains L1 and receives font metrics through the
existing domain contract, without importing infrastructure or executing I/O.

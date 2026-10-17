# L0 — lang
Hash do Código: 23aa74d0


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/mod.rs`

## Consumer evidence

`01_core/src/compiler/lang/mod.rs:14` locates `pub mod equation_supplement;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Static language hub

Own only `01_core/src/compiler/lang/mod.rs`. Publicly expose
`equation_supplement`, `figure_supplement`, `outline_title` and `quotes`.
Keep the physical `heading_supplement` module private and publicly reexport
`heading_supplement_for_lang`, without a wrapper or duplicate table.

Each child owns its language selection, table, validation and fallback:
`00_nucleo/prompts/compiler/lang/equation_supplement.md`,
`00_nucleo/prompts/compiler/lang/figure_supplement.md`,
`00_nucleo/prompts/compiler/lang/heading_supplement.md`,
`00_nucleo/prompts/compiler/lang/outline_title.md` and
`00_nucleo/prompts/compiler/lang/quotes.md`.
Do not move hyphenation, introspection or layout into this hub.
The public heading helper is additive and remains a compatibility-sensitive API.

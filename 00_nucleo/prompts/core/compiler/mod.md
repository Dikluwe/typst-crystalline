# L0 — mod
Hash do Código: b225e2f9


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/mod.rs`

## Consumer evidence

`01_core/src/compiler/mod.rs:7` locates `pub mod eval;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Ownership and boundary

This owner governs only the static compiler facade in `01_core/src/compiler/mod.rs`.
Declare the public modules `eval`, `introspect`, `lang`, `layout`,
`lexer`, `math`, `parse`, `scopes` and `stdlib`. Do not add execution
logic or behavioral reexports. Each module's own prompt owns its API and behavior.

The compiler remains pure L1: no infrastructure imports, I/O or mutable global
state. Evaluation, paginated layout and semantic HTML are distinct responsibilities;
this facade does not execute, reorder or merge their phases.

Canonical owners include `00_nucleo/prompts/core/compiler/eval/mod.md`,
`00_nucleo/prompts/core/compiler/introspect.md`, `00_nucleo/prompts/core/compiler/lang/mod.md`,
`00_nucleo/prompts/core/compiler/layout/mod.md`, `00_nucleo/prompts/core/compiler/lexer/mod.md`,
`00_nucleo/prompts/core/compiler/math/mod.md`, `00_nucleo/prompts/core/compiler/parse/mod.md`,
`00_nucleo/prompts/core/compiler/scopes.md` and `00_nucleo/prompts/core/compiler/stdlib/mod.md`.

## Private text-cost boundary

### Evidence before decision

`correcao-custos-paragrafo-2026-09-28/before-pdf/` records 66 entries with state
initial.json and before/ against historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. text/mod.rs:1507–1564 validates
Dictionary, per-field Ratio and partial composition; inline/mod.rs:240 reads
shared-scope costs rather than per-fragment costs; inline/linebreak.rs:29–30,961–962
multiplies defaults 135/100 by the ratio. Tekt discarded costs in eval/rules and
accepted them without constructor transport; paragraph_hyphen and
paragraph_select used fixed defaults. Vanilla stderr retains errors/spans,
including accepted negatives. Do not infer intent from that behavior.

### Ownership and boundary criteria

Declare only private mod text_costs; the facade remains static with no new
logic/reexports. `00_nucleo/prompts/core/compiler/text_costs.md` exclusively owns
casting, per-component transport and shared-cost resolution. Global
hyphenation/runt consult configuration; widow/orphan validation/transport and
its limited certified regional effect belong to that owner and the pagination
owner. General pagination remains incomplete.

Partial-fragment costs must not replace shared costs; four validated channels
are glyph-neutral within Styled preparation. Existing realization factors a
whole wrapper before Par. Variable-width support remains limited. Do not infer
cost from glyphs; preserve defaults. This is an ADR-0127 internal parity
correction without a new public contract, flag or phase.

## Realization boundary

Expose compiler::realize as the pure domain facade for realization after common
program execution. Its APIs, coordination and single-evaluation obligations
belong to `00_nucleo/prompts/core/compiler/realize/mod.md`; this compiler hub
adds no execution logic.

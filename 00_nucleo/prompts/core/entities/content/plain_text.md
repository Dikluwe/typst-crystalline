# L0 — core/entities/content/plain_text
Hash do Código: 107a03aa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/plain_text.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own recursive `Content::plain_text` projection. Preserve content order, textual
markers, transparent wrappers, and element-specific delegation without making
rendered bytes or Rust representation part of the language contract.

## Acceptance

Existing plain-text, representation, and morphology tests pass; the operation
remains pure and statically dispatched.


## Realization with one evaluation

Plain text uses the body of ShowRecipe/Located and only the visual portion of
Realized; it must not duplicate consumed origin.

## Ratified source transparency

### Evidence and approval

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`
records HEAD, working tree, UTC, sources, argv, and binaries against historical
pin `9f2b6e8715237cb086899a42873660fe744622e8`. This unit's existing
ShowRecipe/Located/Realized branches supply the corresponding transparent
operation; Source must not leak as an observable element. The exact Source contract remains `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.
Human approval `confirmo` on 2026-10-07 is recorded in the final
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`.

### Contract and acceptance

Source is transparent in this projection and delegates to body; it exposes no
span as language identity and executes no recipes. Segmentation preserves
origin per segment where applicable. Keep exclusive ownership and static
dispatch, with no I/O, new signature/trait/flag, or eval/layout phase change.
The C2/C/macro tests and measurements, builds, and lineage are required before
certification. The historical pin does not certify the current upstream pin.

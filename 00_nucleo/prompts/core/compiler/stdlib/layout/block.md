# L0 — stdlib/layout/block — block
Hash do Código: 514bb08d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/set/declared-properties.toml sha256:b4b40ae19e8c22e62a22af00459e257fa4a6c9d8c518d101cd0798cbf4f97744

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/block.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

block preserves relative sizing, height fractions, inset/outset, radius, clip, fill, stroke, spacing, sticky, parity and all existing defaults.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Declared set properties

Approved on 2026-11-24 (“Aprovo A–F”): `00_nucleo/diagnostics/set-contract-package-2026-11-24.pt-BR.md`.
Parse block arguments through one function shared by the constructor and by
declarations. Record omitted properties, by side for inset and outset. Own the pure
projection that fills omitted properties of a BlockElem from a StyleChain: explicit
sides and strokes fold over declared ones; radius is taken whole; auto restores the
default. Admit a declared value only if that same parser accepts it.

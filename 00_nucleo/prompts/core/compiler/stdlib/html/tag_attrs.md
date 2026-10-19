# L0 — stdlib/html/tag_attrs
Hash do Código: 673205cd

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/html/tag_attrs.rs`. At HEAD `7a813038b9af2eca0eac7c9a0f808ae8f6509d5b`, the source was in `01_core/src/compiler/stdlib/html.rs:475-1398`; the manifest `lab/results/construction/diagnostics/atomizacao-html-2026-09-30-before.json.gz` records exact provenance.

## Contract

Own tag-specific typed-constructor attribute tables and their corresponding local enum values. Preserve every name, category, permitted value and specification order measured in the parent L0 `00_nucleo/prompts/core/compiler/stdlib/html.md`. Separation is internal to the same L1 layer and preserves public contracts and defaults.

## Acceptance

The L1 suite, html tests and `crystalline-lint .` must pass. Do not infer performance gains.

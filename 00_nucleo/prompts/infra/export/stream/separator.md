# L0 — PDF lexical separator emission
Hash do Código: 87cd42a3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

Layer: L3. Exclusive consumer: `03_infra/src/export/stream/separator.rs`.

## Evidence before decision

`03_infra/src/export/stream/draw.rs:164` emits fallback Text at page coordinates;
`:771` uses the local reflection envelope. The RED probes at
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/resultados.json`
show A B C losing real spaces. The public carrier contract in
`00_nucleo/prompts/entities/layout_types.md` was approved on 2026-09-28.
This is language text loss, not a requirement for identical PDF bytes.

## Contract

Own emission of one U+0020 from a causal TextSeparator. Delegate glyph/encoding
and compact/verbose envelopes to existing text helpers with the prepared font
scenario and active style. Use supplied x and PDF baseline; never add advance
to another item. Local and top-level dispatch retain their coordinate conventions.
Do not infer a separator from distance, emit glyph ink, or fuse adjacent source
fragments. Preserve grouping and resource mappings. Tests and PDF extraction
must retain lexical spaces while the adjacency control remains ABC and visible
letter positions remain unchanged.

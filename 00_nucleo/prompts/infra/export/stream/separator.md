# L0 — PDF lexical separator emission
Hash do Código: b1ea9c03

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425

Layer: L3. Exclusive consumer: `03_infra/src/export/stream/separator.rs`.

## Evidence before decision

`03_infra/src/export/stream/draw.rs:164` emits fallback Text at page coordinates;
`:771` uses the local reflection envelope. The RED probes at
`lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/resultados.json`
show A B C losing real spaces. The public carrier contract in
`00_nucleo/prompts/core/entities/layout_types.md` was approved on 2026-09-28.
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

## Terminal literal space in an LTR embedding

### Evidence

`lab/results/construction/diagnostics/espaco-terminal-2026-10-06/before.json`
retains working tree/HEAD/code/L0/binaries. literal-terminal-space in the bidi-
identity batch positions glyphs correctly, but extracted 123 bbox ends at
121.49581pt versus oracle 125.000002pt. PDF emits the space carrier at 121.50342pt;
separator.rs:10–14 emits U+0020, which Poppler drops at run end. Historical vanilla
9f2b6e871 uses ActualText 0020202C to close the export embedding. pdf-boundary-
probe.json shows 0020 alone fails while 0020202C retains bbox to 125.00142pt without
glyph/advance changes. word-envelope-refutation.json refutes wrapping preceding
text plus space across graphic restorations: extracted line changes. These are
observable text identity/geometry, not byte/operator equality requirements. Lost/
duplicated spaces, changed ink or markup/LTR-base regression refutes the correction.

### Contract and acceptance

Internal ADR-0127: no public field/API/phase/flag/dependency/unsafe addition.
U+202C is derived only in PDF ActualText to close the embedding certified by span;
it is not author content/carrier/glyph/bidi input/global ToUnicode. No virtual
control in author representation; preserve the paragraph-bidi-origin L0 intact.

Private emit receives explicit origin and optional BidiSpan. Certify only Literal
with exact U+0020 span at the end of a nonempty Text segment with preceding prefix,
LTR segment direction and RTL input base. Use original segment/range. None,
markup/newline/other ranges/directions and Space without Text segment retain old
emission; never infer boundary from distance/font/visualization.

For the certificate, open ActualText space+PDF, emit original U+0020 exactly once
through text helpers with deferred graphic restoration, close EMC before pending
Q. Compact/Verbose preserve identity through their own helpers. Font/x/baseline/
advance remain unchanged; no document mutation/previous-text clone/reframing.
Positive/negative certificate and EMC-before-Q tests plus raw PDF extraction/bbox/
raster/top/local/markup/LTR controls are required.

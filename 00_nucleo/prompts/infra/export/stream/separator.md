# L0 — PDF lexical separator emission
Hash do Código: 76204f83

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/word-joiner-unicode.toml sha256:3fbe26365aae1904cecc40eb2cb61bd6c73ea0f6a4d84ca9ba7a8820f028075f
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


## Readiness: source joiner prefix beside a causal separator (2026-10-28)

### Measurement before decision

readiness-fixes-2026-10-28/fraction-{candidate,extras}/comparison.json retain
identical original outline PDF/SVG paint after the allocation correction but
failed exact word strings. The immutable list-body-extras zero-inset PDF and
its source-bound build are retained independently. outline-scope-experiment/
receipt.json binds exact QDF/PDF/source/tool/worker hashes, HEAD/dirty stat,
UTC, commands, frozen compiler inputs and independent rasters. Repacked control
and a single wide ActualText remain negative. Two separate scopes pass exact
words and0.001pt boxes (maximum0.00001pt), with identical paint: the existing
space carries space+U+2060 and the original shaped run carries only its suffix.
Both inside-BT and deferred-restoration variants pass; copying a vanilla BT
arrangement is not required. This is observable text identity/bounds, not byte
or operator parity.

wj-before/receipt.json separately binds the new joined/split DejaVu dot/suffix
sources, compiler/oracle bytes and all source hashes. Tekt extracts separate
"." and "U+2060+1" words; pinned oracle extracts ". SPACE U+2060+1" as one word,
then TAIL. Bundled/system DejaVuSans.ttf bytes both hash
 ae7b7855e115a5966d8b1b3f80f254ccc117ec86f9965e202ee2940453837280.
Oracle commit64c5e8c02fe0394d913ead36e42ef8c1334d2107, tree
483bd6f1d8ac8810db47b65336d402647955fd17, binary
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
The earlier glyph-global collision and pure-joiner scale corrections remain
intact. No original source or expected word is normalized to grant a pass.

### Exclusive responsibility and acceptance

Keep normal emission of exactly one original space and the existing RTL terminal-literal/202C certificate unchanged. A private marked-space helper may receive the explicit source replacement from draw's certified adjacent joiner-prefix pair. It uses the SAME ordinary space glyph, font, x/baseline and normal/deferred text kernels; close ActualText before pending graphical restoration as in the existing terminal-space route. The new logical string is original-space+certified-prefix, while the next original shaped leaf supplies the exact suffix in its own scope. No extra advance, inferred separator, merged frame/source objects or wide replacement is allowed. The former no-fusion clause remains in force for every other case; only this explicitly measured export occurrence association is refined. Never bypass a bidi certificate or an authored enclosing replacement.

ADR-0127 continuous internal parity work: no public field/signature/trait,
phase/default/mode/compatibility change. L0 and meaningful real-font PDF RED
precede implementation. Preserve actual source concatenation, all earlier
Unicode/ALT/ligature/space/bidi/UA1 controls and independent PDF/SVG painting.
Require V15/V26 before resealing, forward/reverse lineage, full technical gates
and final historical/JPEG resource validation. This does not certify general
Unicode extraction, PDF-UA, HTML, bibliography or overall product readiness.

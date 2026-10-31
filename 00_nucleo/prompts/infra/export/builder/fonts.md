# L0 — builder/fonts
Hash do Código: 497f4ade

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:eb0ad1967bbbd9ee9ded17582451b09de2820855ba38c8a554bbe7b2db7b1848
- 00_nucleo/prompts/_nuclei/export/word-joiner-unicode.toml sha256:3fbe26365aae1904cecc40eb2cb61bd6c73ea0f6a4d84ca9ba7a8820f028075f
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

## Consumer evidence

`03_infra/src/export/builder/fonts.rs:51` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Font preparation and construction

Own CIDFont/multifont build paths, font usage collection, subsetting and variable
instancing, descriptors, widths, ToUnicode, embedded font streams and CIDSet.
Font identity includes FontList, FontVariant and FontVariations. Merge derived and
explicit axes using the existing precedence; do not extend the single-font variable
instancing scope. Accumulate only the existing subsetting timing into subset_ms.

Collect shaped cluster text once before the per-font loop. Include codepoints and
glyphs from fallback Text, including spaces, in each applicable subset. ToUnicode
must retain shaped clusters and ligatures even when subsetting falls back to full
embedding; remap GIDs only when a subset mapping exists. Prepare the per-font reverse
map for direct mathematical glyphs; do not hardcode their resource to F1.

Compute glyph_to_nominal from the same effective face and axes as /W. Use variable
advances only when instancing succeeded; a default-face fallback uses default-face
advances consistently on both sides of the TJ boundary.

Emit Type0 Identity-H fonts with the correct descendant and stream representation:
TrueType uses CIDFontType2 and FontFile2/SFNT; CFF1 uses CIDFontType0 and
FontFile3/CIDFontType0C with the extracted CFF table; CFF2 uses CIDFontType0 and
FontFile3/OpenType with the complete container. Preserve descriptor metrics and
mapping semantics. Resolve BaseFont/FontName from PostScript name, then full name,
then family, then fallback; remove spaces and use the deterministic AAAAAA subset
prefix without renaming internal Fn resources.

Retain the 256 KiB font-stream compression cutoff and existing raw-data fallback
on compression failure. Every embedded CID font has a CIDSet: set MSB-first bits
for the effective CIDs after subset remapping, or used original CIDs for full
embedding, and allocate its stream outside reserved IDs. Bitmap-only fonts do not
emit an outline font or CIDSet; mixed fonts retain outline embedding for those glyphs
that need it. Prepare bitmap references under the pinned bitmap-embedding nucleus;
resource dictionary merging belongs to the resources child, and painting to the
text child. Unsupported bitmap formats retain the normal font path.

## Verification

Exercise TrueType, CFF1/CFF2, variable fallback, multifont identity, sparse/remapped
CIDSet, ligatures, spaces and bitmap-only/mixed fonts. Verify extracted text and
glyph morphology, not equivalence to an external object-allocation algorithm.

## Separator glyphs and physical page minimum

Source→PDF RED/handler audit fatia-funcional-tekt-2026-09-28 precedes layout_types
public-carrier approval received2026-09-28, recorded provenance. Include U+0020 in
its carrier's corresponding multifont face subset using existing style resolution.
Keep text/geometry/adjacency/pinned invariants, no independent ownership.

correcao-auto-espacos-2026-09-29/{before/results.json, provenance.json, initial.json}
retains30sources/60PDFs/font-path/commands/UTC/HEAD/tree/binaries against historical
9f2b6e8715237cb086899a42873660fe744622e8: literal-only spaces/trailing auto width
and zero MediaBox differ; PDF convert.rs:108–118 mandates physical3×3 minimum.
In CIDFont/multifont use builder's same private3pt-per-axis MediaBox helper,
no copied constant. Preserve font/subset/operators/text/coordinates/other dimensions;
logical-width repair is not exporter's. Internal continuous ADR-0127/L0 first/
RED→GREEN spaces0±/minimum/bilateral causal/prior corpus/build-lint zero errors/
manifest-only seals. Explicit empty text/final newline remain separate completion.

## Variable instance before subset

### Evidence

fonts.rs:933–974 subsets original bytes before instancer; subset.rs:56 uses no
coordinates; font_variant.rs:209–213 returns unchanged bytes when fvar absent,
falsely accepted as successful instancing. eixos-variaveis-export-2026-10-05/probe*.txt
retains isolated-font/full-catalog wght250/900 Hello widths2310/2557 per1000;
old PDF hmtx default/no fvar/H697 both, independent fontTools H689/734.
before.json retains provenance. Historical upstream PDF text.rs:79–94 passes
original+coordinates together, supporting behavior not mechanical copying.
native.rs/native.txt gives H689/734 and no fvar after joint instance/subset.

### Contract and acceptance

Within existing multifont preparation give original bytes and derived+explicit
coordinates to private subset_with_variations adapter. Subset/full-font fallback
must use same requested instance. Compute glyph_to_nominal from it before remap,
/W from subset bytes; do not announce success for frozen-default font. Failed
instancer keeps warning/default fallback consistent nominal↔/W. Preserve GIDs/
cluster mapping/static fonts/unknown axes/APIs/flags/export phase/no unsafe.
Private operation order within same phase is not pipeline-phase change. Do not
expand single-CIDFont scope. After successful subset do not call fontTools; on
fallback call it on original bytes to preserve requested instance. Existing
fontTools dependency/guard remains.

Private multifont coordinate request exists even empty, meaning default instance,
allowing CFF2 default-space freezing. Public no-request APIs/single-CIDFont retain
old scope. PDF controls require no Poppler warning for prepared resources.
Ubuntu Sans PDF RED→GREEN uses explicit axes conflicting with weight700/400,
H/e/l/o=689/547/247/580 and734/584/316/607, both stream modes. Full source→PDF,
nested variations/derived weights/static/TrueType/CFF2/bbox/render versus pin,
build/lint no new errors/valid ownership-pins/owner-only seals. CLI cost measured,
no promised speed gain. Geometry/ink parity, never bytes/algorithm identity.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Collect prepared-cell glyphs and integrate PatternType1 registration/emission in
CID/multifont with same resources/fonts, no duplicated semantics/links.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Transparent type and compression maintenance

`type-complexity-2026-10-15/before.json.gz` retains HEAD/dirty tree/UTC/diffstat/
sources/owners/hashes for fonts.rs:63,67,846 (C040/C041/C043); red-clippy.log records
promoted-lint RED. Inspection distinguishes tuple/repeated-FontKey/existing static
contracts, not functional/AI/parity evidence. Use existing public FontKey from
export/render.rs reexported by export, same RHS
((FontList, FontVariant, FontVariations), Vec<u8>) in the three repeated collection
types. Preserve field/order/borrow/move/selection/instancing/maps/timing, no newtype/
buffer conversion/font clone. Remove unused FontVariant import, cfg(test) only
FontList/FontVariations needed by descendant tests. Transparent abbreviation requires
identical expansion; new public/incompatible type needs another gate. No module/
impl/crate suppression/threshold increase; nominal type_complexity historical
exceptions apply only to their three justified items, not other corrections.
Existing controls unchanged/selected RED→GREEN/unfulfilled expectations as error,
build/all-target Clippy/workspace suite/fmt/Rustdoc/lineage/scoped seals, no alias/
field-name mirror tests.

clippy-expressions-2026-10-16/before.json.gz retains sources/L0/HEAD/tree/status/
diffstat/UTC plus initial-diagnostics.json inspected suggestions/spans at :289
(clippy::single_match). Replace only compress_zlib single-success-arm match with
if let Ok(compressed), keeping256KiB cutoff/stream/success return/raw fallback/data.
Internal representation no new parity/performance. No API/default/phase/dependency/
suppression, L0 first/scoped seals/unchanged controls/tolerances/selected RED→GREEN/
workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

## Named stream font scenarios

clippy-aridade-final-2026-10-16/before.json.gz/callers.txt retains checked sources/
L0/HEAD/tree/UTC/status/diffstat/hashes/workspace-fixtures; MathLayouter homonyms
stay separate, prior22warnings/five arities is representation not parity.
Use with_font_scenario and named FontScenario::{Cidfont,Multifont} in both
PageContext compositions with exact borrows/indices. CID bitmap Some/None remains
bitmap_only-dependent; multifont bindings Some(LookupCache::default()) is once per
context. Preserve with_pdf_tags/with_oracle/tiling_names/emission order/subsetting/
all algorithms. Import FontScenario only from internal stream, no public reexport.
Materializes only named private scope/supersedes old audit-only pending status,
no external/default/phase/compatibility. L0 first/scoped seals/real arity RED→GREEN/
no new suppression or threshold/unchanged body-value/call controls, Grid/Table/
outline/SVG-tiling before-after/full workspace suite/build/all-target Clippy/
Rustdoc/fmt/strict+reverse lineage/Tekt no new warnings under12GiB/zero swap/one job.
No carrier-mirror/global-parity/PDF-UA claim.

## Variable-font fallback warnings

When requested variable-font instancing fails and the existing default-instance
fallback is used, append one detached `SourceDiagnostic::warning` to the current
builder's PDF warning vector. Preserve the exact existing message, font identity,
fallback bytes, mappings, widths, timing and output. Do not write stderr, suppress
the warning, change external-tool detection or turn the successful fallback into
an error. Transport follows the approved
`lab/results/pdf-export-warning-carrier-2026-10-23/review.md` contract.


## R10 — source-preserving raster glyph fallback

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,bitmap-evidence.json,font-tables.json}` binds received R9 code/dirty stat/UTC and fixed bd5ebec2 binaries. Nimbus/Liberation mixed123☃ selects Noto Color Emoji. The received PDF raster route emits only image Do, losing the source character in extraction; SVG glyphs.rs:100-105 requires an outline and emits no raster. Pinned paired PDFs/SVGs preserve source text and the PNG. Font-family names are evidence, never runtime branches; supported actual raster data authenticates the correction. Source character, physical position/advance, PNG pixels and geometry are language observations. Object numbering, Type3 versus CID structures and byte encoding are mechanics.

Classification: private font/export parity correction under ADR-0127 continuous flow, with no new public method/entity/phase/mode/dependency. The previously approved FontMetrics method remains separately bounded. No invisible duplicate source text, fake outline, font-name coefficient, or oracle-byte copying. Build a1016-byte self-authored portable bitmap font fixture from the preserved fontTools generator; it is test data, not a font copied from the oracle or a productive dependency. Authenticate its bytes explicitly in the receipt.

### Exclusive owning obligation

For a complete bitmap-only used glyph set with at most256 original GIDs, construct an extractable Type3 font with deterministic single-byte codes, original shaped cluster ToUnicode, actual face advance widths and bitmap-derived bounding boxes/FontMatrix. CharProcs draw the existing shared prepared image resources; allocate IDs outside reserved image/font/gradient ranges. Preserve single-font and multifont ownership and per-face identity. Unsupported/incomplete raster sets and >256 glyphs retain prior path with an explicit remaining limit. The body stream's font resource still matches its actual source face. No outline font embedding/CIDSet for a bitmap-only Type3 font. Keep ordinary Type0/CID output, subsetting, accessibility wrappers and resource recipe intact. Require real extraction of the portable source glyph in both modes and font paths plus existing bitmap/picture/page/semantic tests.
Require module RED→GREEN before implementation, fixed-pin actual fallback PDF/SVG paint/text/geometry controls, previous A–H/R1–R9, full workspace/lineage/resource gates and byte preservation of received unrelated inputs.


R10 bitmap replacement closure refuter before deciding: the new raster helper
always restores its own q and reports no pending graphic tail. The existing
text-replacement protocol has a measured close-before-Q requirement. Hypothesis:
a source-identical bitmap inside TextReplacement may retain text but lose its
physical bbox. Compare the complete Poppler word record for plain/replaced
source in Compact/Verbose and single/multifont; equality refutes the need for a
runtime change. This is an additional test under the existing acceptance, not
a decision to alter code from source structure alone.


R10 closure decision after actual RED: red-svg-residual.json/log/source archive
in the R10 evidence root confirms the hypothesis for the portable bitmap.
Plain source bbox is(10,13.25,19,22.25); source-identical TextReplacement becomes
(10,20,11,20). The old helper closes Q before the existing ActualText protocol
can close its marked content. For the certified Type3 path, preserve the existing
restore/deferred-tail convention: open the helper q once, emit the actual text,
and let the caller close Q when restore=true or return a pending tail when
restore=false. The existing draw/replacement owner then closes its original
marks/ActualText before Q. Preserve normal/legacy image paths and all source
text, glyph positions, bytes of prepared PNGs, advances, offsets and bitmap
font selection. No new signature, mode, phase or invisible text. The expanded
plain/replacement test must pass in both modes and single/multifont, together
with SVG fractional tracking and all previous gates.


## Readiness: Word Joiner and space CID collision (2026-10-28)

### Evidence before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/before.json`,
raw-outline-build.json and raw-outline-build-source.json.gz bind exact HEAD
212aed0449150dd296ceea13fc2be665cc5db01a, uncommitted diffstat/UTC, source hashes
and release. Oracle remains64c5e8c02fe0394d913ead36e42ef8c1334d2107, binary
22897f93c7c2c8589cf5913b0ce735f1c01ba711c70f55c7a350f7c4b96c0b34.
raw-outline-candidate/comparison.json records identical outline PDF/SVG paint,
but failed words. Its control-outline-tekt.qdf:5568 maps space CID0001 to2060.
unicode-before/receipt.json reproduces ALPHA/U+2060/BETA/U+2060/GAMMA without
outline, before and after the joiner and inside Box; ligatures are a control.
`builder/fonts.rs:534-549,1040-1054` prioritizes shaped cluster strings before
used U+0020. The shaper can reuse the space glyph with zero advance for2060.
This proves a font-global extraction collision, not a need to remove an
invisible language character. Font resource order/GID machinery is mechanical;
source words and occurrence Unicode are observable. Altered paint, lost joiner,
changed ligatures or an overridden authored replacement refute the correction.

### Exclusive responsibility and acceptance

Seed the existing used U+0020 mapping before shaped cluster aliases in both
CIDFont and multifont paths, after the same existing subset remap. Leave every
other cluster/ligature mapping, widths, font choice/instancing/subsetting and
compression unchanged. Do not globally substitute2060 with0020 in source text.
Require a real-font PDF→pdftotext regression including literal spaces and a
zero-advance space glyph authored as2060; cover both modes/font paths and an
explicit enclosing TextReplacement. Other owners preserve occurrence Unicode.

This is ADR-0127 continuous internal parity work: no public field/signature,
mode/default/phase/compatibility change. Existing no-repair/boundary clauses
are superseded only for this collision. V15/V26 must precede resealing; require
runtime RED→GREEN, original outline/code-data and Unicode-first/last/Box and
ligature controls, geometry0.001pt/independent PDF/SVG paint, prior corpus,
full suites/build/Clippy/Rustdoc/fmt/forward-reverse lineage, and preserved JPEG
resource gates. Do not infer global Unicode/PDF-UA or readiness completion.


Seed only when shaped2060 (or a cluster wholly of2060) actually aliases the
same remapped used-space CID. Without that collision retain the old mapping
order and all other clusters; do not extend this correction to NBSP or other
ambiguous Unicode aliases. Character-level collision detection follows the
existing remap, including its empty-map identity fallback.


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

Keep all productive subset/ToUnicode/font-resource construction unchanged. Own the real-font roundtrip RED for the observed dot/space/U+2060+digit/TAIL words, joined and split shaped leaves, top and Group-local coordinates, Compact/Verbose and single/multifont. Derive bounds from the bundled face and actual original positions, and require the original words once with0.001pt boxes; enclosing authored ALT still wins. The previous A/space/joiner/B/C and ALT controls remain unchanged. These integration tests do not authorize another CMap/font policy.

ADR-0127 continuous internal parity work: no public field/signature/trait,
phase/default/mode/compatibility change. L0 and meaningful real-font PDF RED
precede implementation. Preserve actual source concatenation, all earlier
Unicode/ALT/ligature/space/bidi/UA1 controls and independent PDF/SVG painting.
Require V15/V26 before resealing, forward/reverse lineage, full technical gates
and final historical/JPEG resource validation. This does not certify general
Unicode extraction, PDF-UA, HTML, bibliography or overall product readiness.

The measured first-candidate Group/ALT bbox failure is preserved in
wj-readiness-green.json and wj-candidate1-source.json.gz; do not remove that
assertion. Supplement the unchanged bare root/Group cases with transparent
Link/Semantic nesting inside the optional Group. The exact authored words and
translated bounds must agree with the bare case in both modes/font scenarios;
ALT remains authoritative. This verifies recursive tail transport, not a new
public link/tag behavior or a claim of general accessibility conformance.

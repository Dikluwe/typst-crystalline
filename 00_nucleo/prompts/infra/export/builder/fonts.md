# L0 — builder/fonts
Hash do Código: b714220e

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/bitmap-embedding.toml sha256:eb0ad1967bbbd9ee9ded17582451b09de2820855ba38c8a554bbe7b2db7b1848
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

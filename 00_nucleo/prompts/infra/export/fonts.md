# L0 — infra/export/fonts
Hash do Código: ad60f016

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/fonts.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/fonts.rs:26` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF font and Unicode helpers

Own escape_pdf_string, document collectors, char/GID mapping, widths, ToUnicode
CMap, UTF-16BE hexadecimal encoding and shaped cluster reconstruction.
Keep existing sibling visibility and no filesystem or network lookup.

The Helvetica literal helper escapes parentheses and backslash, retains printable
ASCII (including DEL under the current predicate), and replaces non-ASCII or
characters below ASCII space with "?". This is the actual restricted helper,
not a claim of full Latin-1 support. The Identity-H helper emits four-hex-digit
GIDs and maps an absent character to 0000.

Collect codepoints from plain Text and shaped char_code; collect_text_codepoints
reads only plain Text; collect_glyph_ids includes direct Glyph and shaped GIDs.
Traverse background/body/foreground and Group/Link/Semantic descendants through
FrameVisitor. Nonvisual alt never contributes glyphs or text. Ordered sets keep
unique output deterministic. Shaped mapping uses the first char_code per GID and
ignores zero-x_advance marks; shaped cluster-text collection preserves run order.

cluster_text derives boundaries from sorted unique UTF-8 byte offsets plus text
length, independently of visual LTR/RTL glyph order. Invalid/non-boundary ranges
yield an empty substring. Only the first occurrence of a cluster receives its
complete UTF-16BE text; later glyphs in the cluster receive empty hex. Preserve
ligatures and non-BMP surrogate pairs, not truncated u16 codepoints.

widths_array accepts current (gid, hex) pairs and converts the effective face's
advance to rounded 1/1000 text units, retaining missing-advance fallback 500.
to_unicode_cmap accepts GID-to-UTF-16BE strings, Identity-UCS codespace 0000–FFFF,
and batches bfchar entries at no more than 100. Empty input produces a valid
empty map without an empty bfchar batch.
These helpers do not select effective font axes, embed streams or decide remapping;
those belong to `00_nucleo/prompts/infra/export/builder/fonts.md`.

## Verification

Check escaping, ordered recursive collection, marks, ligatures, RTL clusters,
surrogate pairs, notdef and CMap validity. Extracted Unicode and positioned glyph
morphology are observables; subset sizes and exact GID assignment are not parity.

## Author text, separators and visual Unicode maps

RED source→PDF probes in `lab/results/construction/diagnostics/fatia-funcional-tekt-2026-09-28/` and this consumer's handler inspection preceded layout_types public approval on2026-09-28. Collect U+0020 for each TextSeparator into global and textual fallback sets used by subset/ToUnicode.

`lab/results/construction/diagnostics/shy-autoral-2026-09-28/{initial.json,before/resultados.json}` records state/binaries/commands/30 bilateral probes. At 300pt repre<U+00AD>sentation extracts two SHY locally versus one historical oracle; spaces six versus one. shaping.rs:313–325 transports rustybuzz SHY clusters, fonts.rs:318–328 maps that substring to the space GID, builder/fonts.rs:523–535 keeps one identity per GID; prior stream maps CID0001 to00AD and reuses it for a later space. These are text observables. Missing association or already duplicated input refutes the cause; no historical intent inferred.

Use existing approved TextReplacement without new field/public API/flag/phase or break-selection/drawing change. Explicit author U+00AD, never appearance/GID/position/zero advance, causes this correction. Narrow cases still diverge in break selection and do not close SHY policy. cluster_text removes SHY only from global ToUnicode cluster substrings, retaining letters in shared clusters. Omit originally nonempty SHY-only entries rather than reserving a shared space GID with empty hex. Keep full logical text in shaper's per-occurrence envelope, UTF-8 boundaries/repeated marks/ligatures/RTL/surrogate pairs. Delete no additional Unicode controls beyond the bidi set governed below.

`lab/results/construction/diagnostics/controles-bidi-pdf-2026-10-06/before.json` records HEAD/tree/code/L0/binary. bidi-contexto-rtl_literal_embedding-cmap-before.json and bidi-contexto-rtl_forced_mark-cmap-before.json map CID0001 toU+202C/U+200F, replacing later spaces and joining words. block-align-2026-10-06/verification.json.gz preserves bilateral extraction/source/commands. Historical upstream 9f2b6e8715237cb086899a42873660fe744622e8 inline/shaping.rs:992–997 removes Default_Ignorable placeholders after shaping; local fonts.rs:307–328 keeps controls and builder/fonts.rs:523–535 keeps the first mapping. Persistence after removing only visual-map controls/placeholders refutes the hypothesis; no byte/algorithm equality or inferred intent.

Remove only SHY and the pinned bidi-visual-identity set from visual cluster ToUnicode substrings. Omit controls-only clusters without reserving shared GIDs; retain spaces/letters/UTF-16/marks. SHY keeps its occurrence ActualText; bidi controls gain no ActualText because upstream does not copy them as space identity in these probes. Require separated letters/spaces, preserved Unicode clusters and visual controls; extractor-inserted direction marks mean raw extracted text alone cannot prove author identity. Continuous internal parity: no public field/signature/phase/flag/default/unsafe change.

## Ratified bidi and effective face binding


`lab/results/construction/diagnostics/bidi-contexto-2026-10-05/inventory.json`
records unit references/1:1 owners; confirmed manifesto 1ae7513e requires None for
uncertified constructors and `..` read patterns, preserving language observations
under the pinned origin nucleus without phase/API expansion.

`identidade-fonte-shaped-2026-10-05/{initial.json.gz,probe.txt,bilateral.json}`
distinguishes regular GIDs from Oblique selection. Human approval of public
TextShaped.font: Option<Font> is manifesto
`f4a17206500baf5952f898135944d55f9fca905285eafc995315ead1121dd409`;
implementation state/inventory: identidade-fonte-shaped-implementacao-2026-10-05/.

Migrate only this owner's references: legacy constructors None, transforms preserve
Font, shaped comparison/join distinguishes face/axes. The shaper supplies the actual
Font clone even on hits without semantic/style changes to force selection. Collectors/
subset/CMap/PDF bind the same original face and style coordinates. PNG/SVG prepare
local bindings and instance once per face/request before render, keeping original
resource index separate from instanced bytes; None follows legacy resolution.
Preserve public APIs/FontKey, no fabricated fonts/unsafe/globals/new phase; private
functions may receive Font/bindings explicitly.

Association uses original face bytes plus axes, never names/slots alone. Table
lifetime is export-local; preserve supplied resource order and append missing faces.
For multifont distribute items once before per-font collectors; no cross-face
GID/CMap reuse. References never share ownership. Require RED→GREEN/bilateral
miss/hit/variable-font controls without a promised speedup. Shared invariants remain
in pinned shaped-face-identity, no second productive owner.

## Equivalent Unicode fixture expression

fonts.rs:511, `lab/results/construction/diagnostics/ci-simplificacoes-2026-10-14/{before.json.gz,red.log}` preserve owner/source/spans/HEAD/tree/UTC/diffstat and selected lints denied before code; types/usages checked under pinned Rust1.92.0. Construct EcoString once and use it directly as fixture text, removing only identity Into. Keep text/glyph/font/geometry/assertions. This Rust-expression maintenance proves no new upstream parity. Continuous flow, L0 first/scoped lineage, no public/default/phase/new suppression; exact selected RED→GREEN, existing expectations/tolerances, full suites/build/all-target Clippy/fmt/Rustdoc/lineage, no syntax-mirror tests.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [550]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

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

Collect codepoints, glyph IDs, clusters and mappings from prepared cells too, for correct subsetting. Resource collection adds no semantic page content.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

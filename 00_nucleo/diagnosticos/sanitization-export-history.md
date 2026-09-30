# Export L0 sanitization history and ownership audit

## Scope and provenance

This diagnostic archives relevant execution history removed from the L0 owners in
`00_nucleo/prompts/infra/export/`. It is not an L0, a shared nucleus, or an
authorization to change productive behavior. This is a directory-scoped documentary
pass, not a claim that global sanitization, parity, accessibility or conformance is complete.

The audit started at `2026-09-26T21:43:02.527395-03:00` on HEAD
`6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, in an already modified, uncommitted working tree.
The builder, stream, SVG, HTML and images façades and their atomized children were
already present; their English ownership structure was retained rather than
attributed to this pass. The exact consumer snapshot used for clause review is
identified by the source SHA-256 manifest below. Existing code, tests, ADRs, shared
nuclei and Rust repository-relative `@prompt` annotations were not edited.

The initial protected-file digest was
`edb2e845f4c862342e5cea380094ed5ae5b253f55cc6a772132d68db6f8d3a32`. It aggregates sorted tracked/unignored
existing files outside the assigned export prompts and this diagnostic. Subsequent
aggregate snapshots changed while the shared workspace was active; they cannot
certify that other tasks left the entire repository unchanged. A separate comparison
of each assigned consumer against the manifest found no consumer-byte changes.
This task's write operations were restricted to the listed L0s and this diagnostic.
No line counts, performance runs or new upstream parity measurements were made.

## Governance and preservation

Repository instructions and current relevant ADRs were read before editing:
ADR-0107/0108 for language parity and evidence, ADR-0109 for static atomization,
ADR-0121 for measurement provenance, ADR-0126 for production verbose graphics,
ADR-0127 for gates, ADR-0128 for HTML as a semantic target, ADR-0129 for exclusive
ownership/shared nuclei, and ADR-0130 for canonical prose references.

Prose references use complete repository-relative `00_nucleo/prompts/...` paths.
The Portuguese machine-readable `Hash do Código` and `Núcleos Tekt` labels,
existing code-hash values, nucleus paths and complete pins were preserved.
Observable Portuguese panic/test-diagnostic literals were not translated.
Step identifiers retained in filenames or test names identify consumers; they do
not constitute ongoing execution plans.

Production contracts retained include independent Verbose/Compact graphics,
Compact/Pretty serialization, enabled/disabled tags, resolved standards and page
selection; font fallback/subsetting/ToUnicode/CIDSet; binary streams and xref;
archival and UA identification, tables/alt/language and associated files; PDF page
labels; physical/logical page geometry, bleed and layer order; SVG graphs, clip/
glyph/image/paint contexts and numerical acceptance budgets; semantic HTML escaping,
whitespace, tables and exact errors; raster defaults and implemented geometry.

Format validity, extracted text, effective page labels, alpha/geometry, colorants,
profile fields, tagged relationships and explicit diagnostics remain observables.
Whole-file PDF/PNG/XML byte equality, object allocation and external tessellation
are not generic language parity. Narrow byte/order assertions remain where the
existing tests explicitly protect an encoder/observable contract, without being
promoted to universal parity.

## Ownership reconciliation

| Responsibility | Normative owner | Boundary retained |
| --- | --- | --- |
| Public export options/defaults and forwarding | `00_nucleo/prompts/infra/export/mod.md` | No encoding or pipeline validation |
| PdfBuilder state, normalization, dispatch and Helvetica orchestration | `00_nucleo/prompts/infra/export/builder.md` | CID/multifont algorithms stay in fonts |
| Font embedding, effective axes, CIDSet and compression threshold | `00_nucleo/prompts/infra/export/builder/fonts.md` | Subset adapter and Unicode helpers are separate |
| FontSubset/remapper adapter | `00_nucleo/prompts/infra/export/font_subset.md` | Removed instructions to edit builder/fonts/stream and their duplicate contracts |
| Unicode/CMap/cluster helpers | `00_nucleo/prompts/infra/export/fonts.md` | No embedding or font-axis choice |
| Resource objects and dictionaries | `00_nucleo/prompts/infra/export/builder/resources.md` | Spot naming delegates to stream/shape; sampling and decoding remain delegated |
| Physical content-stream envelope | `00_nucleo/prompts/infra/export/builder/streams.md` | No operator generation or graphics defaults |
| Metadata, XMP, structure and output intent | `00_nucleo/prompts/infra/export/builder/metadata.md` | Uses stream MCIDs and navigation annotations rather than renumbering |
| Selection, labels, links and attachments | `00_nucleo/prompts/infra/export/builder/navigation.md` | Original physical selection and logical labels remain distinct |
| Numbered-object/xref/trailer serialization | `00_nucleo/prompts/infra/export/builder/serialization.md` | Uses metadata IDs; no standards selection |
| Borrowed font/resource context and MCID prepass | `00_nucleo/prompts/infra/export/stream.md` | No painting or compression |
| Page ordering and UA1 coalescing policy | `00_nucleo/prompts/infra/export/stream/page.md` | Text run arithmetic belongs to text |
| Recursive coordinates and marked-content envelopes | `00_nucleo/prompts/infra/export/stream/draw.md` | No ParentTree or independent MCID allocation |
| Font/text/glyph operators and mathematical noncoalescing | `00_nucleo/prompts/infra/export/stream/text.md` | Not an oracle-only rule |
| Paint/path/cap primitives and complete-colorant naming | `00_nucleo/prompts/infra/export/stream/shape.md` | Uses prepared resources |
| SVG options/root canvas/static dispatch | `00_nucleo/prompts/infra/export/svg.md` | No child rendering algorithms or intrinsic HTML layout guarantees |
| SVG font/destination context implementation | `00_nucleo/prompts/infra/export/svg/glyphs.md`, `00_nucleo/prompts/infra/export/svg/navigation.md` | Pinned nucleus invariants remain shared, not restated in the hub |
| SVG shape/group/image/paint/scalar encoding | Corresponding `00_nucleo/prompts/infra/export/svg/*.md` owners | No new support promotion or clip deduplication guarantee |
| HTML entrypoints and mode/resolver carriers | `00_nucleo/prompts/infra/export/html.md` | Children own document/inline/table/space/escaping/pretty/tag rules |
| PDF image carriers and zlib helper | `00_nucleo/prompts/infra/export/images.md` | Children own profiles/scanning/JPEG/raster payload/XObjects |
| Gradient discovery and resource keys | `00_nucleo/prompts/infra/export/gradients/mod.md` | Children own coordinates/sampling/relative transforms/functions/mesh/adaptive math |
| Raster rendering and PNG composition | `00_nucleo/prompts/infra/export/render.md` | No implicit glyph/clip/cap expansion |
| Test coverage | Each exclusive test L0 in the manifest | Coverage points to productive owners; it does not create alternate algorithms |

The hubs already lacked most pre-atomization algorithms at audit entry. This pass
resolved remaining documentary overlap, especially the multi-file subset plan,
oracle mathematics, spot naming, UA1 call-site versus text-helper policy and SVG
context construction. Shared carriers and dispatch obligations stay with their
actual source owners; delegating calls do not make a hub the owner of a child's
entire contract.

## History transferred from productive specifications

### Font embedding and bitmap paths

The former `00_nucleo/prompts/infra/export/font_subset.md` combined an execution
plan affecting several source files with its subset adapter contract. It described
the earlier full-font decision (ADR-0027), P515 activation, P520 extra-glyph mapping,
P521 ligature/RTL cluster reconstruction, P560 descriptor work and P797 disabling
CFF1 subsetting after invalid Name-keyed CFF output from oxifont-subset.

The P874 entry recorded replacement by subsetter, conversion to CID-keyed CFF,
old/new mappings from GlyphRemapper rather than the intentionally removed cmap,
and reactivation of CFF1. P882 distinguished bare CFF1/CIDFontType0C from CFF2/
OpenType containers; P883 added font-stream compression. These are archival reasons
for the present descriptor/mapping contracts, not instructions to repeat those edits
or a floating version-number parity target.

P940 recorded unsupported CBDT/CBLC subsetting and a full-font fallback/compression
cost concern. P941 moved supported bitmap glyph rendering to image XObjects, making
outline embedding unnecessary for bitmap-only fonts; mixed fonts still need their
outline path. Bitmap collection later acquired its own owner. The chronological
full-font and XObject statements must not be read as simultaneous mandates.
Current compression cutoff, fallback, bitmap metrics and resource/placement contracts
remain in the actual builder/fonts, bitmap, images and stream/text owners.

The old Helvetica helper described broader Latin-1 support than its predicate
implements; current documentation records ASCII replacement exactly. Old width-map
and CMap examples did not match current GID/UTF-16BE carriers. Cluster ordering,
ligatures, marks and surrogate-pair obligations were retained without retaining the
obsolete multi-file instructions or exact subset-size expectations.

### Oracle investigation and production mathematics

The old oracle L0 contained P980 activation chronology, P982 operator-count
investigation, P983 positional splitting and P1133 refutation of the claim that
mathematical coalescing preserves positions. The zero-integer TJ collapse remains
a diagnostic transformation only. Nonzero adjustments represent real spacing and
must not be removed; q/Q without cm is not itself an error when coordinates are
already absolute.

The decisive P1133 record identified an uncommitted state over HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5` on 2026-08-23. It reported the
same linguistic page/text boxes (184.033 × 245.631 pt), but 59 changed raster
pixels at 288 dpi concentrated in a comma (pixel bbox 370,390–377,404); the oracle
path reported zero changed pixels and bbox=None against the same reference. It attributed the
difference to rounding absolute mathematical positions into integer TJ boundaries.
The archived record did not include the complete contemporaneous dirty-file manifest;
its numerical result was not reproduced in this sanitization and is not new closure
evidence. The measured causal source was the then-monolithic stream's run boundary
and TJ cursor code.

That evidence motivated a production-default gate: positioned mathematical fragments
must remain separate even outside oracle mode. The current consumer confirms this
rule in stream/text. The diagnostic module no longer normatively owns that behavior.
The old operator totals, approval notes and future split plan were removed from L0;
they cannot override the present production contract or support new byte-parity claims.

### Gradient architecture and local parity evidence

P307-era linear/radial/CMYK/relative/function/mesh decomposition notes, obsolete
signatures, fixed execution scopes and suite totals were removed. Current consumers
retain separate PDF sample-count, native interpolation, function dictionary and Type 6
mesh responsibilities. Relative helpers take current tuples; radial helpers take
center/radius/focal/size inputs. Oklab distance is f32 in the PDF helper, regardless
of the reserved space parameter. Conic arc controls do not contain the stale extra
cos factor described by the old prose.

The adaptive L0's P1229 evidence used upstream snapshot `a51e02804`:
`typst-library/src/visualize/gradient.rs:905-964` for midpoint bisection and
`typst-svg/src/paint.rs:246-277` for retaining original/intermediate stops.
It distinguished encoded-sRGB premultiplied error from linear-RGB error.
The threshold/cap and language-level stop/alpha/discontinuity budgets remain normative;
literal stop counts and old success totals do not.

P1262 cited `typst-library/src/visualize/gradient.rs:945-958`,
`typst-library/src/visualize/color.rs:1810-1821` and palette 0.7.6:
FromColor performs conversion then Clamp, before float alpha premultiplication.
This is the evidence for the local Oklab encoded-channel clamp boundary, not a
global Color policy. P1278 additionally cited gradient.rs:1461-1485 for f64
offsets and separate f32 weights, weighted normalization and shortest-hue routing.
Oklch acquired the same local gamut boundary. Hsl/Hsv and LinearRgb retain their
distinct supported conversion paths.

The old polar fixture investigation separated graph closure from numerical closure;
Luma's alpha divergence and unvalidated CMYK remained outside promotion. Existing
SVG support and conic/local-mask budgets were preserved in svg/paints; the adaptive
helper does not itself grant support to a paint pair. None of the archived comparisons
is proof against the currently ratified
`9f2b6e8715237cb086899a42873660fe744622e8` without remeasurement.

### Page constants and raster ancestry

The constants L0 contained conflicting old rounded A4 literals and a later
P1140.20.1 correction. The current source derives production A4 from Paper::A4
using millimetres × 72 / 25.4. Rounded 595/842 emergency fallbacks remain distinct.
ISO 216, ISO 32000-1:2008 and the PostScript language reference supplied documentary
context for page sizes and Bézier geometry; upstream
`lab/typst-original/crates/typst-pdf/src/text.rs` supplied faux-bold evidence.
These references are not new standards-verification measurements.

The raster L0 recorded adaptation from
`lab/typst-original/crates/typst-render/` and accumulated step-specific scope.
Current source, not that ancestry, establishes its supported TextShaped, line,
shape, image and group behavior. Claims of bounded-safe allocation, tofu/direct
glyph rendering or clipping unsupported by that source were not preserved as
implemented guarantees.

## History transferred from test specifications

- The PDF aggregator's extraction/suite totals and numbered feature chronology were
  removed. Existing joint coverage and separately owned child suites were retained.
- P1475–P1478 documented incremental archival metadata, associated files, vendored
  version-specific ICC/CIDSet and empty A4f EmbeddedFiles work. Temporary placement,
  old negative-scope names and raw/raw_bytes parser investigation are history,
  not alternative productive contracts. The vendor v2 profile identity remains
  normative in images/icc.
- P1480's evidence lives in
  `00_nucleo/materialization/typst-passo-1480.md`, sections 0–1. The former L0
  cited veraPDF 1.30.2, ISO 14289-1 §5 and §7.1: missing pdfuaid/DisplayDocTitle
  motivated identification fields; combined archival+UA validation motivated the
  minimal XMP extension schema. The explicit pdfuaid-before-NPages assertion is a
  local metadata serializer contract, not complete PDF-byte parity.
- P1481 recorded a titled fixture and a heading/language/Strong/Em/Link/Figure/
  Artifact corpus whose remaining failures concerned ordinary text structure.
  Existing heading/paragraph boundaries, language carriers and MCID relationships
  remain covered. The historical validator totals were not rerun; they cannot
  establish current full UA conformance or assistive-technology behavior.
- P1483 cited pinned vanilla `586e1bd43`,
  `typst-pdf/src/paint.rs:115-163`, for Separation resources and tint semantics.
  The current complete-colorant identity, escaping, gradients and selected-page
  resources remain normative in their productive owners. This older pin is not
  evidence of parity against `9f2b6e871...`.
- P1486's bilateral fixtures, hashes and extraction commands remain in
  `00_nucleo/diagnosticos/typst-p1486-page-labels.md`. The selected-page
  observation distinguished original physical positions and shifted logical
  counters from renumbering retained pages consecutively. Reader labels are the
  observable, not total bytes or global object IDs.
- P1405 label and P1250 SVG integration instructions now identify their actual
  test-only consumers. They authorize no extra productive consumer. SVG integration
  retains supplied geometry, graph closure and unknown handling, not old approval/
  benchmark narratives.
- The line-cap harness retains its automatic thickness witness, scope-sensitive PDF
  tokenization, UNKNOWN_EXPORT_OBSERVATION and explicit local byte guards. It does
  not establish SVG Line or arbitrary Shape cap equivalence.

## Open ambiguities and implementation limitations

These are recorded, not repaired or promoted into newly accepted support.

- **Decorative-layer tagging:** stream's with_pdf_tags prepass walks `page.items`,
  while drawing paints background/body/foreground. MCID/UA treatment of semantic
  content in decorative layers is not demonstrated by this audit. Structure and
  drawing must not silently be documented as traversing the same full layers.
- **Artifact versions:** Header and Background use fixed property lists; other
  artifact kinds use BMC. The source does not branch on PDF version here. Version/
  assistive-technology applicability remains unproven.
- **Raster support:** group clip_mask and image clip_rect are not applied by the
  dispatcher; plain Text/direct Glyph are omitted; Shape uses Winding and its
  current Butt/Miter stroke settings. The allocation path has no independent safe
  maximum and may panic with the existing Portuguese literal. Line-cap coverage
  must not be generalized to these paths.
- **SVG Line caps:** FrameItem::Line emits butt rather than consuming its cap
  carrier. Shape stroke attributes have a different implemented support boundary.
- **SVG HTML frames:** the exporter wrapper consumes a supplied Page and preserves
  its fill/layers/physical dimensions; intrinsic transparency belongs to the layout
  caller. The wrapper does not independently enforce transparent backgrounds or
  em sizing. Text/font-size output is owned by glyphs.
- **SVG clipping:** accepted clips receive fresh unique IDs; equivalent-clip reuse
  is not implemented. clip_mask lacks an explicit fill-rule carrier, so intended
  evenodd versus representable nonzero geometry cannot generally be recovered.
- **ICC failure envelope:** ICC compression failure falls back to raw profile bytes
  while retaining the existing FlateDecode envelope. Successful-path profile identity
  is preserved, but failure-path validity is not established. No behavior fix was made.
- **PDF standard comments:** legacy comments suggesting header-only effects are
  narrower than actual metadata/output-intent/structure consumers. Sanitized L0s
  follow implementation; complete conformance still requires independent validators.
- **Serialized range:** trailer Size follows maximum emitted ID, not every reserved
  high ID; free entries cover holes within the emitted range. This replaces an
  overbroad allocated-range claim without altering allocation.
- **Timestamp fallback:** selection precedes parsing. An invalid present primary
  epoch falls back to current UTC, not a retry of SOURCE_DATE_EPOCH.
- **Missing metadata:** `00_nucleo/prompts/infra/export/p1431-line-cap-tests.md`
  entered this pass without a Hash do Código field. That absence was preserved;
  ownership/header completeness needs separate review.
- **Historical parity evidence:** older partial pins and execution records lack
  sufficient state to close new claims against the current pin. No remeasurement,
  default change or compatibility decision is authorized by this cleanup.

## Validation and remaining lineage work

`git diff --check -- 00_nucleo/prompts/infra/export/` passed after whitespace
cleanup. A read-only owner/reference audit found one annotated consumer for each
assigned L0, matching Exclusive consumer metadata, and no nonexistent prompt/nucleus
paths. Existing code-hash fields and nucleus blocks were preserved.

`crystalline-lint --checks v15,v26 --fail-on warning .` reported no violations.
The read-only V5 run reported prompt/header drift, including this export scope and
pre-existing changes outside it. The default severity allowed the first V5 invocation
to exit successfully despite warnings; that is not a clean lineage verdict.
A subsequent `crystalline-lint --checks v5 --fail-on warning .` exited 1 and
confirmed export-scope prompt drift. The final per-consumer SHA-256 comparison
still found no consumer changes, and scoped whitespace/metadata checks passed.
No --fix-hashes, snapshot rewrite, global reseal, source edit or test edit was run.
Prompt/header alignment remains pending because this task explicitly forbids
editing consumers. No cargo, renderer, validator or benchmark suite was run.

## Changed L0 and inspected consumer manifest

Every L0 below was edited in this pass. This manifest distinguishes those edits
from pre-existing tracked/untracked changes and records the consumer bytes actually
reviewed. SHA-256 values identify source snapshots, not prompt reseals.

| Changed L0 | Exclusive consumer evidence | Consumer SHA-256 |
| --- | --- | --- |
| `00_nucleo/prompts/infra/export/bitmap_glyphs.md` | `03_infra/src/export/bitmap_glyphs.rs:18` | `b93e415d20ba831c448b752cfe1e3ff55afa0214e091a546383589a3f754d7bf` |
| `00_nucleo/prompts/infra/export/builder.md` | `03_infra/src/export/builder.rs:45` | `aed0f21edc39eb2cfad79af7c71e61c1150f2d3e1d6602c4d0abbb72bc2cf1e9` |
| `00_nucleo/prompts/infra/export/builder/fonts.md` | `03_infra/src/export/builder/fonts.rs:51` | `f01f5117dc7da2ef7591555bfef1974c73dd76c5784b3b16ad35037dce7c9f18` |
| `00_nucleo/prompts/infra/export/builder/metadata.md` | `03_infra/src/export/builder/metadata.rs:22` | `c1c5438cce6701860658500abfe39eef99205e0fd43e10c58468c9c9b603a855` |
| `00_nucleo/prompts/infra/export/builder/navigation.md` | `03_infra/src/export/builder/navigation.rs:32` | `98821412c39d8b532601cd620d2c9e13e4c332dd62b0b830de341d4f1a8b626e` |
| `00_nucleo/prompts/infra/export/builder/resources.md` | `03_infra/src/export/builder/resources.rs:46` | `50e129493ab9e04d5daf228c0a93d1a3892997eaf6a6da44ff3f00b17b7572c9` |
| `00_nucleo/prompts/infra/export/builder/serialization.md` | `03_infra/src/export/builder/serialization.rs:11` | `e75b85be5676ed9c71ca3f6e348ce07df26769d8b6598e0ac94aa88067a47b91` |
| `00_nucleo/prompts/infra/export/builder/streams.md` | `03_infra/src/export/builder/streams.rs:20` | `5b03d1ebce48e4a7a7b7bb34ba84dc1d835c73ef069bfc2ce15373ae8480da44` |
| `00_nucleo/prompts/infra/export/font_subset.md` | `03_infra/src/export/subset.rs:19` | `a9c5312bc2dfd57ca9cd78489e0dc805a7e616bcf533d5c9ff383175e795d906` |
| `00_nucleo/prompts/infra/export/fonts.md` | `03_infra/src/export/fonts.rs:26` | `7151852a9f1afe1d175fbb225102dab3ff2189edf980941dbe9683e86ec64451` |
| `00_nucleo/prompts/infra/export/gradients/adaptive.md` | `03_infra/src/export/gradients/adaptive.rs:42` | `400db121e4add0dd2e0421b7632352ddb608de7293880d63dc14636972a6922a` |
| `00_nucleo/prompts/infra/export/gradients/cmyk.md` | `03_infra/src/export/gradients/cmyk.rs:22` | `6a22582a0ab68e635a22e3d790a8a699b964ae10e7e9ad6b80a0cb69b8c81286` |
| `00_nucleo/prompts/infra/export/gradients/conic.md` | `03_infra/src/export/gradients/conic.rs:17` | `53c5dbecd1d251d67419b3289c6fca995d90e7c5a06b1f284a4755c67d2670d6` |
| `00_nucleo/prompts/infra/export/gradients/function_dict.md` | `03_infra/src/export/gradients/function_dict.rs:12` | `7bc05a7f69cfe565281776cc2c84115e7c01e55e3cef3a50a573da655c2305b0` |
| `00_nucleo/prompts/infra/export/gradients/linear.md` | `03_infra/src/export/gradients/linear.rs:14` | `eadbb79068e27877364d798692fa6669e70fc5dd93d0d7a08c9222059e6567a7` |
| `00_nucleo/prompts/infra/export/gradients/mod.md` | `03_infra/src/export/gradients/mod.rs:54` | `3e8d679d686d1e03f52c9f8038c34afe354db3e500b6685249f1e0910cacfdeb` |
| `00_nucleo/prompts/infra/export/gradients/radial.md` | `03_infra/src/export/gradients/radial.rs:16` | `f3f3e0bb919c7bfec51a571d7799264b046ca941ff38a08d64a6018d2f3f4b67` |
| `00_nucleo/prompts/infra/export/gradients/relative.md` | `03_infra/src/export/gradients/relative.rs:30` | `1bb6f3de598d50bdfe95e6052b7e078308e73a32fac289906bcaed52a06b50c4` |
| `00_nucleo/prompts/infra/export/html.md` | `03_infra/src/export/html.rs:27` | `33662a1cc042b0c487373468b755597a349d46fc9f9f33ea97a78c12eda81366` |
| `00_nucleo/prompts/infra/export/html/document.md` | `03_infra/src/export/html/document.rs:20` | `c548922bf4069de16b048fe27174c7b2afb7b0a12cd7c10bae00ac7b202ae3c6` |
| `00_nucleo/prompts/infra/export/html/encoding.md` | `03_infra/src/export/html/encoding.rs:12` | `d1d65da07ff486f8cc2b4d4f7215a41771375e83e989601f9e0efa0dc26c2c13` |
| `00_nucleo/prompts/infra/export/html/inline.md` | `03_infra/src/export/html/inline.rs:16` | `1555a737aad5dfeb7d707bb2fcd0a9e6fdc954c604c06628475a527ece11cc74` |
| `00_nucleo/prompts/infra/export/html/p1360_ab_tests.md` | `03_infra/src/export/html/p1360_ab_tests.rs:11` | `6b7ab35feb816a2c6e9db2983da658d297124253ab8ca9dc8bd25c12564bcdbd` |
| `00_nucleo/prompts/infra/export/html/pretty.md` | `03_infra/src/export/html/pretty.rs:10` | `7dd06a0df0ccffabb5c38a158655ebe912086a75d09d2a3e1d3a8f137861bccc` |
| `00_nucleo/prompts/infra/export/html/table.md` | `03_infra/src/export/html/table.rs:12` | `401f4ff72827a523a127fdcb72556997d922ff80f9d0f1cb304fb64ae6ddbf20` |
| `00_nucleo/prompts/infra/export/html/tags.md` | `03_infra/src/export/html/tags.rs:63` | `a747d26e3aeb48c0795034ea2dda74b7ea8790b01538080f26ab321a77dbfc66` |
| `00_nucleo/prompts/infra/export/html/tests.md` | `03_infra/src/export/html/tests.rs:13` | `b79d56f271ca248ce2728196aa0a5b3c481cedcc9495f04f6d621e6bd6247092` |
| `00_nucleo/prompts/infra/export/html/whitespace.md` | `03_infra/src/export/html/whitespace.rs:16` | `1627f50e9209714c006ece6412aa761619f4fce37dda1b9a0b9346f1833cf899` |
| `00_nucleo/prompts/infra/export/images.md` | `03_infra/src/export/images.rs:33` | `851d8a9f8889400846bc1486f17e076b33fe3c33e3dcd2847f8ea4ebae086cc3` |
| `00_nucleo/prompts/infra/export/images/icc.md` | `03_infra/src/export/images/icc.rs:37` | `4089b4312056f986039be0d2f1e10f2e40a5f99b429c1a783d36544e8fecc462` |
| `00_nucleo/prompts/infra/export/images/jpeg.md` | `03_infra/src/export/images/jpeg.rs:11` | `2104e3f5177df662b0cf3cdb1f414592d06616b0e7fe2017ae155164b508f0d0` |
| `00_nucleo/prompts/infra/export/images/png.md` | `03_infra/src/export/images/png.rs:24` | `6ab58eade8ed36fe7115358c3548a1dadce1e693e6a70d40dac01dc1f3591c44` |
| `00_nucleo/prompts/infra/export/images/scan.md` | `03_infra/src/export/images/scan.rs:41` | `9db0d5c029ded35636f5965d8c7d39fe4c12a8cb6bc87cbd81fca8674fb79272` |
| `00_nucleo/prompts/infra/export/images/xobjects.md` | `03_infra/src/export/images/xobjects.rs:9` | `0ecb99b5e1073aa2e02fff713ca1b5ea3ba11f3899d601208ffb31fa183c861e` |
| `00_nucleo/prompts/infra/export/mod.md` | `03_infra/src/export/mod.rs:196` | `c96ea36a5b32ecf9a900bd2b0254be7b586541a0df05350239115ef75ed831b4` |
| `00_nucleo/prompts/infra/export/oracle.md` | `03_infra/src/export/oracle.rs:22` | `02289c94ff3d2d1fdde260bb097a4311ac64f036748199d41c4e9d615993aef7` |
| `00_nucleo/prompts/infra/export/p1250_svg_integrated.md` | `03_infra/tests/p1250_svg_integrated.rs:29` | `2bfb58a2447aac6d083043e76d7bcdf03f472a6acadbc9f1da7dc22e85f2344a` |
| `00_nucleo/prompts/infra/export/p1405_pdf_label_tests.md` | `03_infra/tests/p1405_pdf_label.rs:16` | `9aa1312357d868bafa21d8f08bbad959073192aa6174896c806857bd66857fdc` |
| `00_nucleo/prompts/infra/export/p1431-line-cap-tests.md` | `03_infra/src/export/p1431_line_cap_tests.rs:29` | `2680fecbdc621cb91b839078d80583e351c1654d05750c0cd8196253105df870` |
| `00_nucleo/prompts/infra/export/pdf_defaults.md` | `03_infra/src/export/pdf_defaults.rs:` | `7c3b30904fba52be26aef268d13d997046ea579700a641f989f09039afc3d83b` |
| `00_nucleo/prompts/infra/export/render.md` | `03_infra/src/export/render.rs:40` | `fde481ec656cad8d535097abe9a9832b8bf1366f594b2b203963e64024407640` |
| `00_nucleo/prompts/infra/export/stream.md` | `03_infra/src/export/stream.rs:54` | `4ff9db5d67f43e4267c9acec1de7e7e3e77474fd13f78519dafc782205870ced` |
| `00_nucleo/prompts/infra/export/stream/draw.md` | `03_infra/src/export/stream/draw.rs:26` | `d93193cd3e35859c57c24811332809243fd2c0594d37cf5ffc487003afdefba0` |
| `00_nucleo/prompts/infra/export/stream/page.md` | `03_infra/src/export/stream/page.rs:17` | `b01bd9d881f60e58d1869f665d26d1d0195cda28b07ea3fb49e975af04d7b6ba` |
| `00_nucleo/prompts/infra/export/stream/shape.md` | `03_infra/src/export/stream/shape.rs:18` | `06ecd8187a80b5a40db8fa01dcdf8e58526c3030814fdbc3f40c8e1cebd03ce6` |
| `00_nucleo/prompts/infra/export/stream/text.md` | `03_infra/src/export/stream/text.rs:20` | `387e33e705e259ce535d3f9f533ac3dc43ccbfa877e2e76cdc7f2434edc5a333` |
| `00_nucleo/prompts/infra/export/svg.md` | `03_infra/src/export/svg.rs:40` | `da0096f71c82b4093e6b544868790c9671620554f364fa8333ceb71119fbef0c` |
| `00_nucleo/prompts/infra/export/svg/geometry.md` | `03_infra/src/export/svg/geometry.rs:14` | `97e65f142511b8b7edd2f3bf5b509cc003bf74946146063db355c6552b222058` |
| `00_nucleo/prompts/infra/export/svg/glyphs.md` | `03_infra/src/export/svg/glyphs.rs:20` | `6dc3de3a79774719e1ccd018f3ec9df4c38b3a3a95e4ffb195eaa87c9296c933` |
| `00_nucleo/prompts/infra/export/svg/groups.md` | `03_infra/src/export/svg/groups.rs:16` | `60de069ddddeb3d5d22988a55fc2d48494ee04c9c4b02a784cbedfd4b6692f22` |
| `00_nucleo/prompts/infra/export/svg/images.md` | `03_infra/src/export/svg/images.rs:14` | `5f124a2340f4af7f31c4b3317200d9363d51f9222cdbcc58ba8cfc0262f1de34` |
| `00_nucleo/prompts/infra/export/svg/navigation.md` | `03_infra/src/export/svg/navigation.rs:17` | `7bed54a5317ed2c0d708e33741150092f0787f6d77d69a03b1a21e5a4a1e3044` |
| `00_nucleo/prompts/infra/export/svg/p1360_ab_tests.md` | `03_infra/src/export/svg/p1360_ab_tests.rs:9` | `716f9232f025c3be51a9e1cdc18325a2eae40e40e3eada784d98708f741675e9` |
| `00_nucleo/prompts/infra/export/svg/paints.md` | `03_infra/src/export/svg/paints.rs:19` | `2e0c77cc0d020b23398b710583b526cf3f2ffc9f5e2af379b9def5b120a83db5` |
| `00_nucleo/prompts/infra/export/svg/serialization.md` | `03_infra/src/export/svg/serialization.rs:9` | `ef3a71b04e3628c3959e32dfa408466f096a148906143b54f24dcc6e2d952c7e` |
| `00_nucleo/prompts/infra/export/svg/tests.md` | `03_infra/src/export/svg/tests.rs:29` | `f0ac22a003858330d0409104851aea65c7faec256b4c6b4dc03fc5e8ab0f9193` |
| `00_nucleo/prompts/infra/export/tests/mod.md` | `03_infra/src/export/tests/mod.rs:36` | `5e4b16ad4f4fcc05d734c03502f5806a31b87cc5035f1891965bb1148a53bd09` |
| `00_nucleo/prompts/infra/export/tests/p1475_pdfa_conformance_core_tests.md` | `03_infra/src/export/tests/p1475_pdfa_conformance_core_tests.rs:34` | `6b04b54fd1cfc232e13921b52dc349acdc48d784e4d0d6483ae166c792c39f64` |
| `00_nucleo/prompts/infra/export/tests/p1476_pdfa_attachments_af_tests.md` | `03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs:30` | `781875d2d3b8bb539591e086a11e75b1cf69393f28a23399fd7c029b4a1152c0` |
| `00_nucleo/prompts/infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests.md` | `03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs:29` | `1e4bb7b5fee1f8529d0b8c8ef0b8bde7fdd6b563bc1502e9fc72f9065ddfeae4` |
| `00_nucleo/prompts/infra/export/tests/p1478_pdfa4f_empty_embedded_files_tests.md` | `03_infra/src/export/tests/p1478_pdfa4f_empty_embedded_files_tests.rs:32` | `73e4d9717aae70c68051624724c35d5ff9c7a82f38bb5b9b2e2b09be408a419d` |
| `00_nucleo/prompts/infra/export/tests/p1480_pdfua_identification_tests.md` | `03_infra/src/export/tests/p1480_pdfua_identification_tests.rs:17` | `e9b4858b1cad8de31c3c5ed70a4c4111156c445f7175f54de00e46b191f00ced` |
| `00_nucleo/prompts/infra/export/tests/p1481_pdfua_structure_tests.md` | `03_infra/src/export/tests/p1481_pdfua_structure_tests.rs:10` | `d0fb513871698b450a522d244312211aaca597a866f5ebd6402911213ead5eed` |
| `00_nucleo/prompts/infra/export/tests/p1483_pdf_spot_tests.md` | `03_infra/src/export/tests/p1483_pdf_spot_tests.rs:18` | `cac2d9e33dc340226ddb2bc88794754890c42116bd4793d4876ed73b6038dc93` |
| `00_nucleo/prompts/infra/export/tests/p1486_pdf_page_labels_tests.md` | `03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs:13` | `16bb4311022ba850f766f84f3283b0bfffb9ba44eb3105f79d46a98e5ef3725c` |

The only new diagnostic produced by this pass is
`00_nucleo/diagnosticos/sanitization-export-history.md`.

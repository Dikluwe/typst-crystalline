# L0 — infra/export/tests/mod
Hash do Código: ec135bb6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/shaped-face-identity.toml sha256:06ae067763e4c77876f7d999903f2e5e4144df34eb799eb62f2c52a31d8a030b
- 00_nucleo/prompts/_nuclei/paragraph-bidi-origin.toml sha256:a4a76bdb3fa5069232468af724106217bcd1dc85231c9d2cf54b4da69e241425
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/mod.rs:36` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF test aggregation

Own the existing cross-feature PDF cfg(test) suite, shared fixtures/helpers and
declarations of specialized test modules. Preserve meaningful test names/assertions and restricted
access to the public/internal exporter API. Keep genuine joint gradient/image/font/
layout coverage rather than duplicating each child suite in this aggregator.

Specialized test owners are the corresponding files under
`00_nucleo/prompts/infra/export/tests/`: archival core, attachment AF, ICC/CIDSet,
empty A4f, UA identification/structure, spot resources and page labels.
They own coverage, not an alternative productive export contract.
Production behavior is owned by the builder and stream descendants.

## Verification and exclusions

Run the existing export::tests suite and specialized modules without using a
historical fixed test total as an acceptance criterion. Check cross-feature
resource/geometry/text closure. Do not claim global language parity, conformance
or performance from test counts or full-file byte equality. Independent validators
and snapshot harnesses remain separate evidence.

## Ratified bidi and shaped-face fixtures


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

## Variable-subset TJ invariant

export/tests/mod.rs:9622–9626 requires original GID223 in /W; infra-tests.log observes CID1, /W1020 after CFF2→TrueType subset, unlike the original nonsubset era. Acceptance is /W(actual emitted CID)−TJ delta=x_advance, not original GID. Extract the single TJ glyph's hexadecimal CID for width lookup; keep formula/advance/name. Never skip width/delta; productive implementation remains its owner.

## Test observations, independent numeric references and navigation

`lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz` preserves this owner's exact source/spans/compiler output/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty status/diffstat/UTC. Remove only audited unused private imports, keep required public/contract/call imports. Fix Rustdoc paragraph/list continuation without technical/assertion/case change. Continuous flow adds no public/default/phase/formula/acceptance/suppression; all-target/infra names+suite/build/workspace Clippy/Rustdoc/lineage, no import/comment mirror tests.

`lab/results/construction/diagnostics/testes-observaveis-ci-2026-10-13/before.json.gz` preserves state/source/positions: :6348, 6522, 7030, 7395, 7776, 8069, 8490 only assert!(true,...); :8023 P279 checks cm and accepts discarded text, while:8125 P281 already checks(hello)Tj+cm, with CIDFont/multifont/glyph/line/nested group controls. Remove exactly seven constant tests plus redundant stub-accepting P279. This explicit exception to name preservation permits no loss of real observations/reduced other assertions. Keep Block/Boxed/Group bbox, pattern dedup and Shape/Image/Text/Glyph/Line group emission. Test list loses exactly eight names/no additions; all other source identical except lineage/blank lines. History belongs in diagnostics. Infra suite/compile/fmt/lineage/CI inventory, no suppression/productive/public change or test-count/operator-presence parity claim.

mod.rs:3911 and `lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz` preserves library/consumer/L0/manifest-lock/spans/HEAD/tree/UTC/status/diffstat; clippy.toml requires platform-independent math, libm0.2.11 already pinned.90° arc reference uses tan(pi/8)=SQRT_2−1 in f32, independently of productive tangent. Keep inputs/names/tolerances and relevant numeric suites/build/lint/lineage.

labels-fronteira-2026-10-14/reader-probes.json.gz shows MuPDF cannot resolve destinations despite P460/P463 byte searches for /Dests, /sec1, /D /sec1. destinos-pdf-2026-10-14/before.json.gz preserves tests; canonical `00_nucleo/prompts/infra/export/builder/navigation.md` requires Names-tree strings instead of mixed incompatible representations. Keep names/labels/Y-up/GoTo; observe /Names key-array pair and identical UTF-16BE string in destination/action, never invalid /sec1 or /Dests presence alone. Independent reader proves actual navigation, not isolated bytes.

## Conic mesh and parent coordinate controls

conic.rs:142–143,241–242 fixes center/radius, :271–272 ignores CMYK offsets/final sample; resources.rs:508–552 fixes eight-bit/unit Decode. conic-posicionamento-2026-10-14 before/after/refuters expose white corners/ignored center/offsets despite working sample(). conic-geometria-amostragem-2026-10-14/before.json.gz preserves sources/L0/HEAD/tree/diffstat/UTC/binaries; bilateral center/axis-diagonal color/square-rectangle/rotate corpus and pinned typst-pdf/paint.rs:274–299 show left-origin/clockwise, no uniform-vanilla fallback. PDF Reference1.3 table4.31(PDFp250) permits16-bit/per-axis Decode.

Keep Type6/component/flag/determinism/extension/resource coverage; replace obsolete fixed-patch/eight-bit assertions with valid stride(RGB61,CMYK65), flags/components/endpoint samples. Historical helper tests remain explicitly historical. CMYK Decode has six pairs: ordered geometric ranges covering reference plus four[0,1], not a unit-string snapshot. Rename tests promising obsolete patch counts. Conic owner's functional RED covers center/coverage/origin/direction/aspect/offsets/side limits and independent render. L0 first/RED/code/no weakened functional expectations; full corpus/qpdf/raster/regressions/reverse lineage. Reference binary regeneration only after harness/protocol proof.

first-validation/workspace.log fails p273_6_shape_inside_block_carries_parent_bbox_observable_diff; parent-failure-comparison.json shows only Info/XMP timestamps one second apart, identical Shading. Whole-byte equality tests clock, assert_ne may pass falsely. In four Block/Boxed parent/self controls P273.6/P273.7 observe four coordinates of the sole linear Shading: require one ShadingType2/one Coords/exactly four numbers, never silent missing observation. Keep document/spaces/paint/export mode and original relations: smaller parent changes coordinates, explicit page equals fallback, Self ignores parent; measured page coordinate check remains. No clock/production metadata/snapshot/other-test criterion change. Repeat group/full suite.

The initial new tests guessed Y-up Block772/Boxed692; parent-tests-green.log refutes with70/150. Never redefine apply_parent_transform(origin+span) from guessed fixtures or freeze70/150 as physically correct. Original relation scope stays; physical linear-parent Y remains explicitly open for separate causal measurement.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [178,652,698,711,741,790,805,1075,1344,6514,6625,6719,6727,6799,6929,7362,7433,7506,7577,7585,7707,7777,7832,7840,7928,7979,8043,8108,8192,8234,8279,8287]. layout_types.rs:531–539
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

Migrate FrameItem::Shape fixtures to FramePaint/FrameStroke with unchanged values/
semantic assertions. Tiling explicitly constructs a prepared cell; eval fixtures
remain declarative. No total productive conversion or weakened assertion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Real layout bbox observation and explicit fixture values

mod.rs:8205,8550 and field-reassign-2026-10-15/before.json.gz/initial-diagnostics.json preserve source/spans/L0/HEAD/tree/UTC/diffstat. Initialize style_b font and local fill directly in TextStyle with existing remaining defaults; keep font_b.clone(), RGB red/underline Styles/mutations where needed. Do not remove the unused conceptual style/redefine coverage/change documents/resources/groups/underline asserts; no new types/builders/API/suppression/default. L0 first/selected RED→GREEN/unchanged names/inputs/tolerances/full Clippy/workspace/build/fmt/Rustdoc/lineage, no initializer-mirror tests.

mod.rs:6083 and clippy-locais-2026-10-16/before.json.gz preserve audit/source/L0/hash/HEAD/tree/UTC/diffstat. Replace only p273_6_layouter_parent_bbox_consumed_by_block_arm's Some(Rect)/is_some/unwrap smoke body with real layout: Parent-gradient Shape in Content::block200×100, emitted bbox200×100, PDF horizontal span200pt and difference from same Shape outside block. Never feed fabricated bbox. Keep name/adjacent controls; this narrowly supersedes literal preservation for this nonobserving smoke only, not bbox-test removal. Same continuous-flow/private/default/phase/lint gates as prior maintenance, exact inventory/no new warnings/denied expectations/full suite/build/fmt/Rustdoc/lineage12GiB/swap0/jobs1, no derive/Entry/module-mirror tests.

first-validation/validation-architecture.log records new emitted_bbox:6092 `_=>None` increasing V16 baseline327→328; full FrameItem:373–547 checked before correction. Enumerate Shape→bbox; Group/Link/Semantic/TextReplacement→children; TextSeparator/Text/TextShaped/Line/Glyph/Image→None(no bbox/children). No wildcard: enum growth requires compiler decision. Preserve Block/Shape fixtures/bbox+PDF asserts/all other corrections; L0 first/scoped lineage, full gate/suite with no raised baseline/suppression.

## PageContext fixture migration

clippy-aridade-interna-2026-10-16/first-validation/validation-clippy.log records five E0599 at8166, 8229, 8309, 9753, 9947 despite productive build. Source/L0 match initial global hashes; diagnostic preserves prior text/scope extension and inspected callers/expressions. Replace old cidfont/multifont fixture calls with with_font_scenario/existing named FontScenario; preserve expression order for all four resources/maps/fonts/bitmap/mode/Box::leak, no lifetime cleanup. CIDFont bitmap None; Multifont Some(LookupCache::default()) once/context. Add only private FontScenario import. Keep names/fixtures/asserts/exact strings/numbers, no restored wrappers/silenced compile errors/new product API. Compare each old argument/new field and remainder after masking only calls/import/hash. L0 first/scoped lineage, all-target/full workspace/stage gates12GiB/swap0/jobs1, no changed expected/carrier-mirror tests.

## Owned test module declarations

Separately owned: pdfa_core_metadata_tests, pdfa_associated_files_tests, pdfa_icc_version_and_cidset_tests, pdfa4f_empty_embedded_files_tests, pdfua_identification_tests, pdfua_structure_tests, pdf_spot_colors_tests, pdf_page_labels_tests. Delegate bodies; no absorbed ownership.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Migrate legacy Figure fixtures that intend native element kinds to explicit typed Element data; preserve actual PDF/export geometry/text assertions and all other test inputs. String custom-name controls remain Name. This fixture migration is not full Figure rendering parity.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.

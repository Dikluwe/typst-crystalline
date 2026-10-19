# L0 — infra/export/line_cap_tests
Hash do Código: 1a4e09d6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/line_cap_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/line_cap_tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Independent raster/PDF line-cap observations

Own only the existing cfg(test) consumer and its fixtures/assertions. Construct
Butt/Round/Square through math.mat, isolate the transported line and observe actual
exporters. Keep the frozen effective automatic thickness 0.05*11pt=0.55pt.
Raster endpoints must distinguish Butt < Round < Square; automatic augment matches
the explicit Square witness. Preserve the existing deterministic local byte guards
for that witness and the legacy Butt line, not a universal byte-parity claim.

The PDF parser tokenizes operators, verifies balanced q/Q, selects the internal
scope containing w/m/l/S, and attributes its sole J before the line path in that
same scope. Round requires 1 J, Square 2 J, and Butt no redundant 0 J at top/local
drawing. An outer scope alone cannot attribute cap.
Missing/truncated/unattributable stream, pixel or line identity is exactly
UNKNOWN_EXPORT_OBSERVATION and never receives acceptance credit.

## Ownership boundaries

`00_nucleo/prompts/infra/export/render.md` owns raster Line cap mapping;
`00_nucleo/prompts/infra/export/stream/shape.md` owns PDF cap primitives;
`00_nucleo/prompts/infra/export/stream/draw.md` owns their drawing scope.
This suite does not claim equivalent SVG or Shape-stroke cap support.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

The raster witness crops only its isolated line viewport to endpoint bounds padded for the largest cap and antialiasing. Translate both endpoints by a common integral-point offset, preserve 16 pixels per point, thickness, color and caps. Retain Butt < Round < Square and automatic == Square assertions. PDF witness pages and byte guards remain unchanged.

## Ratified compact Value adaptation

`lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/{before.json.gz,inventory.json}` freezes this consumer and every Value::Stroke file:line. sizes.json models272→144 bytes externally, not RSS. The public carrier belongs exclusively to `00_nucleo/prompts/core/entities/value.md`. Owner `confirmo` on2026-10-08 ratified manifest `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`; its earlier pending gate is satisfied only for this package. Change only stroke Value construction to From<Stroke>, preserving cap/offset/augment observations and expectations. Keep tests/oracles, relevant isolation RED→GREEN, owner-only lineage and revalidation; no expanded semantics/defaults/phase.

## Test-only configuration and explicit fixtures

line_cap_tests.rs:5 and `lab/results/construction/diagnostics/infra-imports-docs-ci-2026-10-13/before.json.gz` preserve source/owner/spans/compiler output/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted status/diffstat/UTC. Productive/test/comment references were distinguished; no new parity measurement. Remove only redundant internal cfg(test), since the parent's test module declaration remains gated. Keep test-only status, cap observations and UNKNOWN_EXPORT_OBSERVATION criteria. No public signature/default/phase/formula/acceptance/lint suppression change. Validate all targets, infra test names/execution, build/workspace Clippy/Rustdoc/lineage, scoped resealing; no import/comment mirror tests.

line_cap_tests.rs:78 and `lab/results/construction/diagnostics/field-reassign-2026-10-15/{before.json.gz,initial-diagnostics.json}` preserve source/spans/prior L0/HEAD/tree/UTC/diffstat/initializers. Initialize thickness/cap directly in Stroke with remaining `..Tipo::default()` fields. Keep necessary mutability and specified.thickness/specified.cap=true mutations in their order. Automatic thickness, cap choice and PDF/raster witnesses stay. Remove mut only when unused; no new type/builder/API/suppression/product default. L0 first, selected lint RED→GREEN, unchanged names/inputs/expectations/tolerances and workspace/all-target Clippy/build/fmt/Rustdoc/lineage. No initializer-syntax tests.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [128]. layout_types.rs:531–539
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

## Exhaustive existing FrameItem decisions


### Evidence

`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/
sources/L0/configuration/hashes/status/diffstat. AST spans/patterns/guards/bodies/
complements were checked before code; classification.json records 70 cases across
26 owners under pinned CI AST. Rust exhaustivity is mechanics; product semantics/
morphology/geometry/effects/diagnostics stay. No new architecture/upstream intent.

### Exact owner complements

Only replace first_line's `_` at line 114 (historical F055) with FrameItem::TextReplacement/TextSeparator/Text/TextShaped/Glyph/Image/Shape, using its existing in-scope Rust prefix.
Preserve complete fallback bodies/prior arms/guards/transforms/calls/returns/order.
Callbacks/effects/metadata/clipping/fonts/positions/ink/width follow current owners;
no expanded traversal/wrapper-leaf change/text-or-ActualText geometry replacement.
Tests retain fixtures/expectations/tolerances/counts/Unknown failures; V19 group
notes stay visible without pretending one arm covers all alternatives.

Continuous ADR-0127 adds no enum/field/trait method/public signature/default/
dependency/compatibility/phase and approves no historical pending proposal.
L0 first, scoped bidirectional resealing; no suppression/higher threshold or
match→if-let evasion. Before code run new FrameVisitor control/workspace suite
with unchanged expectations and confirm AST RED for all 70 family warnings.
Afterward zero scoped-family V16/no new warnings; outside V16 remains pending.
For productive core matches reached by cargo check -p typst-core, use a disposable
FrameItem sentinel variant to observe absent E0004 before/present after. That
probe covers only compiled sites, never unbuilt cfg(test) or blocked L3 sites.
For the full batch replay patterns/guards/bodies, verify enum/signatures unchanged,
real build/tests, no stub bodies.

Revalidate full workspace suite/cargo build/strict workspace-all-target Clippy
(-D warnings, -D unfulfilled_lint_expectations)/fmt/Rustdoc -D warnings/four HTML
indexes/strict V1/V5/V15/V26/reverse lineage/SARIF/diff --check. Freeze sources/L0/
configuration during runs; heavy commands sequential under MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1. Structural cleanup is not new global parity,
memory gain or measured AI benefit.

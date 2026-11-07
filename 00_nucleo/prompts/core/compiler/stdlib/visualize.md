# L0 — stdlib/visualize — tiling constructor
Hash do Código: 7201e9b8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896
- 00_nucleo/prompts/_nuclei/tiling-placement.toml sha256:d6e2fba3d2143daec23bcf7a8d959ce59600f3b8ccc5fa35a853c4f6831e51e0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/visualize.rs`

## Evidence before specification

Historical tiling declaration/carrier work first admitted Color/Image/path with Color fallback and stored Parent without implementing it; those constructor/body/layout scope-outs are superseded only by the explicitly approved complete declarative constructor below. The P1140 public type probes againsta51e02804 and structural-replay 22 cases distinguish Type from Func without changing constructor semantics. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Human confirmation of complete declarative public constructor is recorded in historical P1245, prevalidated/materialized P1254. Prior visualize.rs:27-113 Color/Image/path whitelist/no offset-angle contrasts pinned a51e02804 tiling.rs:99-185,250-315 Content/units/unresolved layout frame. This approved public surface is current; a focused SVG fill success does not approve unrelated Stroke/targets.

`lab/results/construction/diagnostics/tiling-transformacao-2026-10-14/before/results.json.gz` preserves dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, source/owners/UTC/stat/binaries and historical 9f2b6e871. angle90/offset 3,5 requires matrix(0 1 -1 0 3 5), not rotated offset(-5,3); pinned tiling 173-174,375-377 and transform 334-343 establish T*R. Full 240×192 white-canvas librsvg/Poppler comparison has quarter-no-offset Tekt PDF/SVG equal oracle SVG but oracle PDF414 pixels>16 different; quarter-offset SVG1764 pixels differ. No cropped/added-tolerance approval or complete Parent/PDF certification.

## Exclusive current constructor contract

Global tiling is Value::Type(Type::Tiling), callable through same native_tiling(ctx,args,world,current_file); preserve type identity and results, no new render/entity logic. Required arbitrary Content body follows normal public Content conversion, no separate Color/Image/Gradient whitelist or implicit world file-loading branch. Preserve tiling(t)==t identity without incompatible overrides. Return declarative Arc-backed Tiling value; injected World pass-through introduces no direct L1 I/O/globals.

Named size/spacing/offset/angle/relative defaults Auto/zero/zero/zero/Auto. Size Auto remains unresolved in eval; spacing finite with language absolute-unit restrictions; offset finite, rejects prohibited font-relative component and preserves percentages for pitch resolution; angle finite. Relative admits Auto/Self/Parent, no constructor-context Auto choice. Distinguish invalid type/nonfinite/prohibited unit errors; exact wording is required when measured public surface. Existing internal extract_size/parse_relative helpers only serve validated declarative construction, without resurrecting superseded whitelist rules.

Eval validates/builds only, no frame/raster/Auto-size resolution/exporter import. Layout alone composes cell Content/derives size/resolves percentages and applies pinned tiling-placement rotation around offset, T(offset)*R(angle), never rotate offset itself. Resolved structures private to phase; exporters consume normal prepared output without body eval/layout repetition. Constructor retains offset/angle declaratively, including percentages, without precomposition/materialization or changed branch/signature/error/default/validation.

## Acceptance and limits

Preserve callable type/identity, arbitrary Content body and declared fields/defaults, Auto versus explicit/pitch-relative geometry and diagnostic distinctions. Superseded Color-fallback/Gradient-body-rejection tests remain archival witnesses, not current acceptance. Meaningful RED→GREEN/restricted reseal/build/suites/lineage follow L0 first. No public/phase change from offset wording correction. Real patterns/Parent resources/Stroke/other targets require current layout/export owners' independently approved integrated gates, never early Color fallback or successful fill SVG as production proof. Do not copy oracle PDF defect as desired language.


## Approved PNG byte source — first raster subset

Evidence: `lab/results/construction/diagnostics/image-input-scope-2026-10-31/`
contains bilateral source/format/reflection/render probes before this decision.
`lab/results/construction/diagnostics/png-bytes-2026-10-31/approval.json` binds
the explicit human approval and exact public proposal. ADR-0127 public/compatibility
gate is satisfied only for path:Option<String>, source:Option<Value> and PNG bytes
with original source. Raster phases, defaults and public constructor signatures
remain. SVG/PDF and format/raw pixels stay pending. JPEG/GIF/WebP byte admission and the measured WebP decode correction are completed by the subsequent explicit user request below.
The partial constructor is incomplete: temporary task admissao-png-bytes owns this
subset; admissao-demais-raster-bytes and cor-webp-pdf complete the named remaining
raster subsets. No claim of full image parity or #8996 closure.

Migrate only existing ImageElem literals to the approved optional file path and explicit source absence for legacy synthetic/test images. Preserve the previous path presence/value as Some, unless it is already a known synthetic empty path, which becomes None. Keep every render/layout/tiling/flow behavior and foreign change. Clones retain the original source; do not add a source from a fabricated filename.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

Extend the existing admitted byte raster set to JPEG/PNG/GIF/WebP while preserving the ratified optional path and original source semantics. The previous temporary pending subset admissao-demais-raster-bytes is completed here; original source contract and legacy absence remain. No new public schema, phase, flag or vector support.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Migrate legacy image literals to the approved optional format/SVG metadata; do not synthesize original source. Preserve format on reconstruction, existing frame geometry and raster behavior.

## Approved image-body storage adaptation — 2026-11-06

The owner explicitly authorized TilingBody::Image(Box<ImageElem>) in this
chat, as specified by the entity owner. Adapt existing construction sites
with Box::new and borrow/clone the contained ImageElem at the legacy bridge.
Preserve every image field, data/source/origin/format, body selection, existing
assertion, layout and export behavior. Do not change the old admitted-input
set, source acquisition, phases or defaults as part of this storage repair.
This clause authorizes only this consumer's mechanical adaptation; entity
representation is exclusively owned by core/entities/tiling.md.

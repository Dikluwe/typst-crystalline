# L0 — entities/content/construction
Hash do Código: 247dfa3a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/figure-placement.toml sha256:86797e01ffab87b98e601e7530074daec3a66252c8dd93eb918af8e775b1b8d0
- 00_nucleo/prompts/_nuclei/image-source-provenance.toml sha256:19a4e9d6333f8000f9f64a3520c15a82f052dca270a845a38c727e630a512b5e
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the residual public `Content` constructors other than sequence normalization. The math, layout, table and reference families are owned respectively by `00_nucleo/prompts/core/entities/content/construction/math.md`, `00_nucleo/prompts/core/entities/content/construction/layout.md`, `00_nucleo/prompts/core/entities/content/construction/tables.md` and `00_nucleo/prompts/core/entities/content/construction/references.md`. They must preserve every signature, default, omission
state, and payload. Construction remains pure L1 and uses only closed native
variants plus the existing explicit `Dynamic` extension boundary. No registry,
property map, I/O, or rendering is allowed.

## Acceptance

All existing constructor, representation, morphology, and parity tests pass;
callers compile unchanged; the module includes local tests.

## Ratified Raw lexical presence

### Evidence before specification

Historical content-serialization-2026-10-14/{before,raw-controls}/results.json.gz
records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree, UTC,
sources/binaries and pin `9f2b6e8715237cb086899a42873660fe744622e8`.
RawElem raw.rs:21–25 lost omission; native_raw markup.rs:78–136 collapsed
absence/false/none. Fields, equality, and encoders distinguished them at that
pin. These are semantics/morphology, not Rust mechanics or a fresh current-pin
measurement.

### Contract and acceptance

Content::raw retains its signature, always marks block present, and marks lang
only for Some. Test getters and maps without changing rendering.
Approved manifesto SHA-256
`b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`;
receipt:
`lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`.
This completes only Raw's previously incomplete presence contract and
supersedes clauses requiring no presence mask. Preserve all other contracts;
no new phase/default/render behavior. Validate RED→GREEN tests, suite, build,
and lineage.


## Approved Figure/Grid occurrence amendment (2026-10-19)

### Evidence before specification

Baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` preserves the measured proposal in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, semantic partition, independent audit and counter receipts. The fixed oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`. Constructor string kind loses function identity; constructor inference precedes occurrence styles; separated Grid fields lose source order. Typed identity, lexical presence, field traversal and counter results are language semantics/morphology. The chosen Rust carriers are implementation mechanics. The owner explicitly approved the linked public fields, contracts, phases and compatibility changes on 2026-10-19 under ADR-0127; this amendment materializes that bounded decision.

### Exclusive contract

Keep Content::figure(body,caption,kind:Option<String>,numbering) unchanged as a public adapter. Some(name) becomes kind Some(Some(FigureKind::Name(name.into()))), None means lexical omission. Initialize resolved_kind, supplement, resolved_supplement to None. Preserve existing numbering Styled transport exactly; no numbering field is restored. Legacy Some("table"/"image") now means a custom name, intentionally requiring typed migration where callers intend actual element identity. Native figure builds the typed entity directly.

### Acceptance and limits

Preserve all unrelated prior obligations and nucleus pins. This amendment supersedes only conflicting Figure/Grid clauses stated above. Require independent semantic RED→GREEN, exhaustive decision extension builds, existing workspace/build/tests/all-targets/Clippy/Rustdoc/fmt, canonical paths, strict forward/reverse lineage and valid V15/V26 before scoped resealing. No suppression, severity downgrade, weakened assertion or global hash repair. The bounded package does not certify full Figure/Grid parity: standalone HTML rendering, complete caption synthesis, supplement-dependent selector preview, external Dynamic enumeration, Grid footer phase and complex Table supplemental target order remain visible debt.


## Approved issue8890 Figure placement — 2026-10-31

### Measurement before decision

`lab/results/construction/diagnostics/issue-8890-2026-10-31/{analysis.json,svg-geometry.json,report.md}` preserves the exact HEAD3c6b0ce7, UTC, clean/dirty state, pinned vanilla64c5e8c0 executable/source/inputs and PDF/SVG measurements. Original Figure placement is rejected in Tekt at stdlib/figure_image.rs:342-360; vanilla loses D after deferring the float. The supported Place control preserves A–E but puts its float on page3 after continuation text, so it does not certify the original trigger. Current source/owner bytes and concurrent dirty state before this amendment are preserved under `lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/before/` and before.json. Geometry/presence/positions are language; queue/carrier mechanics need not copy vanilla.

The user explicitly approved the concrete public-contract proposal with “Aprovo” in this chat; approval.json binds its exact scope. This satisfies ADR-0127 for FigurePlacement and the two public fields only, with existing layout phase/default in-flow behavior retained. The prior placement scope-out is superseded only for this approved subset.

### Exclusive responsibility

Keep Content::figure's signature and numbering chain unchanged. Initialize both approved placement fields to None in this legacy adapter.

### Gates and limits

L0 and scoped reseal precede real RED, code and GREEN; valid V15/V26 precede repairs. Preserve all other ownership, previous clauses/ratifications and concurrent changes. Use only /dev/shm/typst-crystalline-8890-materialization-target for this amendment's compilation/test temporaries, one job, 12GiB virtual ceiling and zero swap. This target supersedes older task-specific target paths only for the present amendment. Record state/hash/UTC around every measurement. Revalidate original and link PDF/SVG, controls and finite progress with all A–E once; page count itself is not an oracle for a known vanilla defect. No new phase/default/compatibility change beyond the approved fields. Do not suppress lint, weaken tests, copy upstream defects, commit or claim global qualification from scoped checks.


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

Keep public Content::image(path:impl Into<String>,...) signature. It makes Some(path) and source None for legacy internal construction, preserving payload, width/height/fit and alt defaults. Native construction may attach the captured source or use a byte entity without a file path.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

Extend the existing admitted byte raster set to JPEG/PNG/GIF/WebP while preserving the ratified optional path and original source semantics. The previous temporary pending subset admissao-demais-raster-bytes is completed here; original source contract and legacy absence remain. No new public schema, phase, flag or vector support.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Preserve Content::image public signature. Its legacy ImageElem literals receive format=None, svg=None and retain original source absence. Do not infer an original source or decode images here.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Migrate legacy image literals to the approved optional format/SVG metadata; do not synthesize original source. Preserve format on reconstruction, existing frame geometry and raster behavior.

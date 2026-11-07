# L0 — compiler/stdlib/tests/figure_image
Hash do Código: ce70c1ea

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/stdlib/tests/figure_image.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Contract

Own only the figure_image regression tests and their local fixtures/assertions. Use the private shared support fixtures. Preserve arguments, diagnostic messages, spans and language-level observations. Expectations must be independent of candidate output. Do not implement namespace behavior or transfer production contracts into this test consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Approved Figure/Grid typed-kind and occurrence-phase migration

At baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85`, this test
consumer asserted constructor-time inferred String kinds. The owner approved
moving inference to occurrence preview/preparation and preserving lexical
omission versus explicit auto, with nominal FigureKind Element/Name identities.
The native Image/Table/Raw and nested Sequence inference observations remain
required, now asserted through the pure occurrence preview with actual native
function identities. Separately assert constructor lexical kind/resolved fields
are absent, explicit auto is Some(None), and explicit strings are Name rather
than element identity. Preserve existing inputs and all independent image tests,
errors and source spans. This migration changes the measured phase and carrier
only as explicitly approved; it does not weaken inferred language outcomes.


## Approved issue8890 Figure placement — 2026-10-31

### Measurement before decision

`lab/results/construction/diagnostics/issue-8890-2026-10-31/{analysis.json,svg-geometry.json,report.md}` preserves the exact HEAD3c6b0ce7, UTC, clean/dirty state, pinned vanilla64c5e8c0 executable/source/inputs and PDF/SVG measurements. Original Figure placement is rejected in Tekt at stdlib/figure_image.rs:342-360; vanilla loses D after deferring the float. The supported Place control preserves A–E but puts its float on page3 after continuation text, so it does not certify the original trigger. Current source/owner bytes and concurrent dirty state before this amendment are preserved under `lab/results/construction/diagnostics/issue-8890-placement-2026-10-31/before/` and before.json. Geometry/presence/positions are language; queue/carrier mechanics need not copy vanilla.

The user explicitly approved the concrete public-contract proposal with “Aprovo” in this chat; approval.json binds its exact scope. This satisfies ADR-0127 for FigurePlacement and the two public fields only, with existing layout phase/default in-flow behavior retained. The prior placement scope-out is superseded only for this approved subset.

### Exclusive responsibility

Before implementation add functional RED tests that the existing Figure native constructor accepts auto/none/top/bottom placement and preserves lexical omission/invalid-cast diagnostics. After approved carrier addition assert actual typed presence and source spans; preserve all existing image/figure expectations.

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

Before implementation prove actual assertion RED for PNG bytes under a World that panics on source/file/path IO, byte payload and source projection, lexical Path/Str source and invalid bytes. Then GREEN and validate the full admitted raster set; vector formats remain pending. Use the existing constructor signature, not compile failure from absent fields as RED.

Cover invalid integer source with the measured admitted Path/Str/Bytes type list.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

Extend the existing admitted byte raster set to JPEG/PNG/GIF/WebP while preserving the ratified optional path and original source semantics. The previous temporary pending subset admissao-demais-raster-bytes is completed here; original source contract and legacy absence remain. No new public schema, phase, flag or vector support.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Before implementation, add module RED assertions for file/bytes SVG/SVGZ admission, original source/type/fields/has/repr, no path load for bytes, alias/show preservation, wrong extension/invalid input and raster controls. A failed compile due to absent API is not a behavioral RED.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.

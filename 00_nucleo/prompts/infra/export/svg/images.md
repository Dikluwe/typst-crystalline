# L0 — svg/images
Hash do Código: 4d8c8dc8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/images.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Consumer evidence

`03_infra/src/export/svg/images.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Embedded images

Own conservative self-contained SVG recognition, MIME/data URLs, raster payload
embedding, image geometry, local cover clips, EXIF and nonvisual fallback markers.
Embed original PNG/JPEG/GIF/WebP bytes with their MIME and resolved physical box;
preserve alpha without decoding/recoding. Recognize SVG only when the first significant
XML element is svg, allowing UTF-8 BOM/whitespace/declaration/initial comments.
Ambiguous/compressed/unrooted data stays Unknown. Do not alter ImageFormat or claim
a general SVG parser. Nested SVG remains an image; do not rewrite its internals.

A finite clip_rect crops cover in the page's local space without reapplying position
or orientation. Missing/malformed carriers remain Unknown, not absent-clip equivalence.
Apply EXIF 1-8 once on the layout-resolved box. preserveAspectRatio=none applies the
supplied stretch/contain/cover geometry, not new fit/layout calculation. Unknown format
emits nonvisual data-crystalline-image-fallback=unknown-format and no image/fictional
MIME. Do not import PDF decoding or add I/O.

## Verification

Check MIME/original payload alpha, physical boxes, cover clips, asymmetric EXIF,
self-contained SVG landmarks and explicit unknown-format behavior.


## Nominal image-leaf arity exception

### Evidence

`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`
retains complete sources/prior L0/signatures/hashes/UTC/status/diffstat at HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree captured
2026-10-15T16:39:18.873477+00:00. classification.json records per-item contract
review/reconfirmed spans. render_image at images.rs:14 has eight arguments
(threshold seven), historical C068. The leaf receives already resolved bytes/
physical box/orientation/clip, writer and shared clip-ID capability; it does not
need the entire fonts/destinations session. This is Rust structure, no new parity,
functional-defect or cognitive-benefit proof.

`clippy-locais-2026-10-16/clippy-inventory.json` retains 79 warnings.
clippy-excecoes-2026-10-16/before.json.gz retains source/spans/existing reasons/
signatures/hashes/HEAD/uncommitted tree/UTC/diffstat checked before deciding.
The nominal-metadata follow-up materializes only the justified exception below,
superseding the old audit-only pending status, not pending corrections.

### Exact exception and limits

Keep signature/body/calls/cfg/order/other attributes and semantics. Only render_image
may carry:
`#[expect(clippy::too_many_arguments, reason="C068: Leaf de imagem recebe bytes, caixa física, orientação e clip já resolvidos, writer e contador de clip IDs")]`.
Use the prescribed exact reason, no module/impl/crate/global application, threshold
increase or other lint. This narrowly supersedes generic suppression bans only
for named item/lint. If no longer over threshold remove/review the expectation;
never silence unfulfilled_lint_expectations. Conditional exceptions requiring
dead-argument removal still require that correction; pending correction cases
(including historical C047/C066) gain no annotation here.

### Acceptance

Continuous ADR-0127 changes only lint metadata, no public contract/executable body/
cfg/default/phase/dependency/architecture. L0 first/scoped resealing. Removing added
attributes and lineage header must recover prior rustfmt-normalized source.
Measure real RED, then all-target inventory: historical batch exactly 46 arities
plus five test warnings satisfied by 49 expectations, no new warnings and
unfulfilled_lint_expectations as error; its 11 correction arities remain visible.
Keep owner functional/call controls without fixture/tolerance relaxation; private
signature corrections migrate callers/L0 before code and new public/compatibility
needs another ADR-0127 gate. Validate build/fmt/strict/reverse lineage/Tekt no new
errors/warnings, existing Counter/coverage tests under 12 GiB, zero swap, one job.
Prior full suite is evidence for identical executable code, not a fresh run;
no lint-attribute mirror tests or new performance comparison for metadata.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Supersede the conservative SVG recognition escape and prohibition on prepared SVG payloads for admitted SVG images only. Embed the self-contained prepared SVG render representation with resolved physical box/cover clips; preserve alpha and gradients. No downstream font selection, filesystem/resource fetch, layout or raw SVGZ embedding. Original language source remains on ImageElem. Raster data URLs and EXIF behavior remain.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.

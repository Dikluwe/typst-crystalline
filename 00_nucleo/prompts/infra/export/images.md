# L0 — images
Hash do Código: c168c91b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/raster-decode.toml sha256:2f0f61bd281ffb77a1c9b62a8b40733e248ef55dff4bf2b81104c3c0d6ce5d0f
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Consumer evidence

`03_infra/src/export/images.rs:33` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Façade contract

Own PdfImagePayload, ImageRef and ImageXObject data carriers, compress_zlib and
compatibility reexports for ICC/JPEG/raster processing, scanning/validation and
XObject writing. Preserve signatures, fields and visibility, including the public
process_png_for_pdf compatibility entrypoint. compress_zlib uses the existing default
zlib compression and string error result; callers own their fallback policy.

Delegate algorithms to the children below. Consume image-format detection from its
existing L1 owner rather than redefining detection here. Do not add format support,
global cache policy or an alternative public image model in this façade.

## Delegated owners

- `00_nucleo/prompts/infra/export/images/icc.md` owns `03_infra/src/export/images/icc.rs`.
- `00_nucleo/prompts/infra/export/images/jpeg.md` owns `03_infra/src/export/images/jpeg.rs`.
- `00_nucleo/prompts/infra/export/images/png.md` owns `03_infra/src/export/images/png.rs`.
- `00_nucleo/prompts/infra/export/images/scan.md` owns `03_infra/src/export/images/scan.rs`.
- `00_nucleo/prompts/infra/export/images/xobjects.md` owns `03_infra/src/export/images/xobjects.rs`.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

Re-export decode_raster_image only inside crate::export for the existing bitmap sibling; retain all public exports and payload signatures. Child png owner implements decode; this unit only composes it.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Dispatch already-prepared Svg payloads to the owning private SVG PDF child, keeping current raster facade/API and measured WebP decoder. No world, source resolution, font selection or layout in export.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.

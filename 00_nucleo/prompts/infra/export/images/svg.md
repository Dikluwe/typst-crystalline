# L0 — 03_infra/src/export/images/svg.rs
Hash do Código: 28e9eb22


**Status:** Approved by owner 2026-11-01; approval at `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`.
**Exclusive consumer:** `03_infra/src/export/images/svg.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Measurement before decision

See `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md` and oracle.json/tekt-before.json, with fixed commit/tree/binary and timestamps. Existing integer raster sizing and missing resource origin cannot faithfully transport measured SVG behavior. Public DTO/backend representation is an implementation choice; source semantics, geometry and visible paint are acceptance observables.

## Contract

Own private conversion/import of already-prepared SVG to a PDF Form XObject with self-contained resources. Coordinate object IDs via existing export owner; handle nested fonts/gradients/clips/transforms/alpha/resources and validate references. Keep vector paths as vector PDF; rasterize only backend-required effects with measured output-scale acceptance. Never substitute a low-resolution bitmap for the whole SVG, and never perform World/font/source IO or layout here. Backend libraries remain L3 only; prove exact version/type interoperability before choosing/pinning dependencies. PDF/A/tagged alt and production default serialization remain governed by existing owners.

## Acceptance

L0 activation and owner/hash/pins precede module behavioral RED then GREEN. See proposal.md matrix and RAM/build/lint gates. Do not claim implementation or assign a code hash before this module exists.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Import svg2pdf normalized form chunks using its exact compatible pdf-writer version. Renumber all references, validate object count and use generated xref offsets for binary-safe object boundaries. No arbitrary endobj scanning, source/World/font IO or layout.


## Nested WebP backend capability

Inspected svg2pdf 0.13.0 Cargo.toml and render/image.rs:50 select image 0.25 WebP decoding, but the default image feature list includes only JPEG/PNG/GIF. Enable WebP on that private image version (L3 alias image-svg-codecs = image 0.25.10), leaving the existing raster image 0.24 path and bounded image-webp 0.2.4 route unchanged. Verify a self-contained SVG with embedded WebP converts to a real Form; do not claim source admission alone verifies this resource.

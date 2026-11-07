# L0 — 01_core/src/entities/svg_image.rs
Hash do Código: dad43662


**Status:** Approved by owner 2026-11-01; approval at `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`.
**Exclusive consumer:** `01_core/src/entities/svg_image.rs`.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Measurement before decision

See `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md` and oracle.json/tekt-before.json, with fixed commit/tree/binary and timestamps. Existing integer raster sizing and missing resource origin cannot faithfully transport measured SVG behavior. Public DTO/backend representation is an implementation choice; source semantics, geometry and visible paint are acceptance observables.

## Contract

Pure SVG preparation DTOs: SvgOrigin::{File(RootedPath),Inline(FileId)}, SvgSource { origin:SvgOrigin, span:Span }, PreparedSvg { data:Arc<Vec<u8>>, width_px:f64, height_px:f64, warnings:Vec<String> }. Debug/Clone on DTOs; PartialEq on SvgOrigin/SvgSource for existing carrier compatibility. Dimensions are finite, positive CSS pixels and preserve fractions. Data is self-contained UTF-8 SVG, fonts outlined and permitted referenced resources embedded. No parser, library backend types, IO or mutable global state. This is the sole owner of these data contracts; method ownership stays with ImageSizer.

## Acceptance

L0 activation and owner/hash/pins precede module behavioral RED then GREEN. See proposal.md matrix and RAM/build/lint gates. Do not claim implementation or assign a code hash before this module exists.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Implement the approved exclusive SVG responsibility; see the full contract above.

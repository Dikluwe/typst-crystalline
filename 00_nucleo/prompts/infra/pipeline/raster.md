# L0 — infra/pipeline/raster
Hash do Código: 69d58f92

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/raster.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:9c4f3ce07c56762e4c588a0f223c3ea1e7e50d6c00f83ff457fad766b0cea3cf
```

## Current evidence

- `03_infra/src/pipeline/raster.rs:12` — `compile_to_png_bytes`.
- `03_infra/src/pipeline/raster.rs:21` — `compile_to_png_bytes_full_error`.
- `03_infra/src/pipeline/raster.rs:33` — `compile_to_png_bytes_with_timings_full_error`.
- `03_infra/src/pipeline/raster.rs:47` — `compile_to_png_bytes_with_timings_full_error_and_features`.
- `03_infra/src/pipeline/raster.rs:61` — `compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/pipeline/raster.rs`.

## Owned contract

- Own PNG facades only. Compile via the common paged path, preserving errors, warnings, explicit features and phase timings.
- Resolve `effective_ppi = ppi_override.unwrap_or(document.format_options.png.ppi)` once and `pixel_per_pt = effective_ppi / 72.0` for both origins. Convert to f32 only at RenderOptions construction; use the same options with and without resolved fonts, with render_bleed=false.
- Compatibility wrappers preserve signatures and pass None. Export only the first page, using the existing empty-page fallback for no pages. Preserve finite nonpositive safe-dimension policy; do not infer behavior for nonfinite inputs.
- Resource errors abort before render. PPI changes raster dimensions, not evaluation or layout in points, other formats, tagging, stream mode or pretty. Raw page-numbering functions must remain separate from realized marginal content in the paged owner.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Export font bindings

`lab/results/construction/diagnostics/identidade-fonte-shaped-2026-10-05/probe.txt`
is the measured basis. Consume private resolve_export_fonts for bound original
fonts under the existing instancing gate. Instancing stays in the exporter at
the same render/export phase, retaining diagnostics/API/page selection. The old
name implied already-instanced bytes; that is not this contract.

## Multi-page PNG package

Add public `compile_to_png_output_files(world, source, full_error, features, ppi_override, pages_override, output)` with the same result shape as `compile_to_svg_output_files`. Compile once, resolve PPI and fonts once, and render every selected page with the same render options. Destinations follow the SVG page template rule shared from the SVG owner, including the stdout and missing-template errors and the empty selection. This supersedes, for the CLI, the first-page-only clause above; the first-page facades keep their signatures and behavior for other callers.

Approved on 2026-11-26 (“Aprovo A–D”): `00_nucleo/diagnostics/png-pages-package-2026-11-26.pt-BR.md`.

A unit test checks that the first page of the collection has the bytes of the first-page facade.

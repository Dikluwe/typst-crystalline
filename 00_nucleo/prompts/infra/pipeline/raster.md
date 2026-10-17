# L0 — infra/pipeline/raster
Hash do Código: abec53af

**Layer:** L3
**Exclusive consumer:** `03_infra/src/pipeline/raster.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/format-option-precedence.toml sha256:68c5c10f124c2c098879318df16c59f6c1477e9172cfa334b603f45ec698ae11
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


## Preparação de bindings de exportação — 2026-10-05

Medição: `00_nucleo/diagnosticos/identidade-fonte-shaped-2026-10-05/probe.txt`. Consumir helper privado resolve_export_fonts: fontes originais vinculadas, com gate de instanciação existente. Instanciação fica no consumidor exportador, na mesma fase de render/export, preservando diagnósticos/API/seleção de páginas. O nome antigo prometia bytes já instanciados e não descreve este contrato.

# L0 — Legacy layout bridge
Hash do Código: e5445a4a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/layout.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030
```

## Current evidence

- `03_infra/src/layout.rs:17` — `layout_with_font`.
- `03_infra/src/layout.rs` — invalid-font, valid-font and unresolved-callback controls.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Preserve `layout_with_font(content: &Content, font_data: &[u8], font_size: f64) -> PagedDocument`. For valid bytes construct `FontBookMetrics` and enter the L1 callback-aware `layout_with_introspector_and_metrics` boundary with an empty introspector and `ImageSizeImageSizer`. A pending math callback cannot be realized without an Engine/World: fail closed through the existing precondition failure rather than return provisional pages, an empty success document or partial geometry.

For invalid bytes use the existing L1 `layout(content)` fallback, preserving its defaults and absence of a font-parse panic. Do not generalize the valid-font callback guard into a new public error return.

Metrics are owned by `00_nucleo/prompts/infra/font_metrics.md`, byte metadata by `00_nucleo/prompts/infra/image_sizer.md`, and productive callback realization/stabilization by `00_nucleo/prompts/infra/pipeline/paged.md` and `00_nucleo/prompts/infra/pipeline/context_stabilization.md`.

## Verification boundary

Use the existing valid/invalid-byte controls and the valid-MATH-font callback case. The latter must not return a provisional `PagedDocument`. Font fixtures resolve under `03_infra/fixtures/fonts/`.

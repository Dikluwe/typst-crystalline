# Prompt L0 — entities/image_sizer
Hash do Código: 6c1cdc72

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/image_sizer.rs`
**Governance:** ADR-0029 (physical purity), ADR-0001 (external whitelist).

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Contract

ImageSizer transports intrinsic image pixel dimensions, DPI, and EXIF
orientation. Actual header/file readers belong to L3. L1 owns only the trait
and its null test implementation; no shared state or I/O is allowed here.

```rust
pub trait ImageSizer {
    fn size(&self, data: &[u8]) -> Option<(u32, u32)>;
    fn dpi(&self, data: &[u8]) -> Option<f64>;
    fn orientation(&self, data: &[u8]) -> Option<u32>;
}
pub struct NullImageSizer;
```

size returns (width_px, height_px), or None for invalid bytes. DPI metadata
precedence is EXIF > JFIF APP0 > PNG pHYs. The consumer applies 72 DPI fallback
when None, not this trait. orientation transports EXIF Orientation tag 0x0112,
values 1–8, with 1 meaning no rotation. PDF applies the visual transformation
through its cm matrix; L1 merely transports the value.

## Acceptance and boundaries

NullImageSizer is the only L1 implementation and returns None for all three
methods. L1 tests needing no intrinsic dimensions consequently use the sizing
consumer's 100×100 pt, 72 DPI and no-rotation fallbacks. Real L3 implementations
use their I/O libraries outside L1. Verify the null behavior and unchanged
public trait signatures.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/image_sizer.md`. Historical observations are not a new parity certification.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Retain size/dpi/orientation signatures and NullImageSizer behavior. Add the default public prepare_svg method defined in proposal.md, returning an explicit unsupported-provider error by default. The method transports pure data only; actual SVG parsing, gzip, font/resource lookup and outlining belong to L3. Preserve fractional CSS dimensions in PreparedSvg; do not project them through u32 raster dimensions.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Transport the approved selected image format and SVG provenance through existing constructors/clones without changing raster behavior or inventing source. Legacy literals use explicit absence; reconstruction preserves the existing format.

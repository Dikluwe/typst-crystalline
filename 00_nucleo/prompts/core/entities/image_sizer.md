# Prompt L0 — entities/image_sizer
Hash do Código: 5cbab6b6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/image_sizer.rs`
**Governance:** ADR-0029 (physical purity), ADR-0001 (external whitelist).

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

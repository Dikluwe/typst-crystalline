# L0 — images/jpeg
Hash do Código: fc298d64

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/jpeg.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/jpeg.rs:11` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## JPEG component inspection

Own jpeg_color_space/jpeg_is_rgb. Inspect SOI and length-bearing markers up to
SOF0/SOF2 without decoding or rewriting data. Stop safely at SOS, malformed lengths
or buffer bounds. One component uses DeviceGray, four DeviceCMYK, otherwise DeviceRGB;
retain the existing unknown-header three-component fallback. RGB classification
requires the resulting three-component path. Do not infer ICC/EXIF from this scanner.

## Verification

Check baseline/progressive headers, gray/RGB/CMYK and malformed/truncated inputs.

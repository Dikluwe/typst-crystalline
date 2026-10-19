# L0 — images
Hash do Código: c691f189

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

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

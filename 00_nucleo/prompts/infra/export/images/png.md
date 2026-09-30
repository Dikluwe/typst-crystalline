# L0 — images/png
Hash do Código: b4191477

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/png.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/png.rs:24` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Raster payload preparation

Own process_png_for_pdf while retaining its compatibility API name. Use the existing
generic image::load_from_memory route for supported PNG/GIF/WebP, including first
GIF frame behavior. Return decoded dimensions, 8-bit DeviceRGB and compressed RGB
bytes. For RGBA split RGB/alpha; emit compressed alpha only when at least one sample
is not fully opaque. Preserve decoder error strings and compression error behavior.
Do not add pointer caching, animation policy or new format support here.

## Verification

Check decoded dimensions/colors, opaque versus partial alpha and malformed raster
diagnostics through the existing public entrypoint.

# L0 — images/png
Hash do Código: 971214ad

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/png.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/export/raster-decode.toml sha256:2f0f61bd281ffb77a1c9b62a8b40733e248ef55dff4bf2b81104c3c0d6ce5d0f

## Consumer evidence

`03_infra/src/export/images/png.rs:24` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Raster payload preparation

Own process_png_for_pdf while retaining its compatibility API name. Use the existing
generic image::load_from_memory route for PNG/GIF and the bounded export-private
WebP decode route specified below, including static first-frame behavior. Return decoded dimensions, 8-bit DeviceRGB and compressed RGB
bytes. For RGBA split RGB/alpha; emit compressed alpha only when at least one sample
is not fully opaque. Preserve decoder error strings and compression error behavior.
Do not add pointer caching, animation policy or new format support here.

## Verification

Check decoded dimensions/colors, opaque versus partial alpha and malformed raster
diagnostics through the existing public entrypoint.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

The direct decoder probe before this decision shows image 0.24.9 gives lossy WebP RGBA [169,101,88,255] where the verified reference gives [255,1,5,255]. Keep process_png_for_pdf signature and payload shape. Add only a private export-visible decode_raster_image helper: PNG/GIF/JPEG retain image::load_from_memory; WebP uses L3-only image-webp pinned =0.2.4, with checked output buffer, existing image Limits allocation bound and decoder memory limit. Preserve straight RGB/RGBA, first frame, errors as strings. PDF validation uses this preparation as before. No IO, source or phase changes. Before implementation add a lossy WebP pixel/alpha RED through the current public payload function.

# Prompt L0 — entities/image_format
Hash do Código: 62666c00

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/image_format.rs`
**References:** ADR-0107, ADR-0108.

## Evidence before scope decision

Historical image probes found unknown/corrupt images silently omitted from
PDF, with only an L3 eprintln, while upstream reported `unknown image format`
(typst_library::visualize::image::mod.rs:344). Detection then lived too late
in export/images.rs to return a SourceDiagnostic tied to the original image
span. Upstream RasterImage::new_impl fully decodes via DynamicImage::from_decoder;
this owner deliberately handles signatures only. Full decoding belongs to L3,
not a new image/decoding dependency in this L1 module. ImageSizer has its own
imagesize/trait boundary.

Later L3 validate_document_images, called before export, addresses recognized
signatures with corrupt payloads using `failed to decode image ({detalhe})`.
Its detached-span limitation remains distinct from upstream's original image
source position. Historical closure applies to that L3 scope, not full L1
decoding or current-pin certification.

## Contract

Preserve public ImageFormat::{Jpeg,Png,Gif,WebP,Unknown}, deriving
Debug/Clone/Copy/PartialEq/Eq, and
`pub fn detect_image_format(data: &[u8]) -> ImageFormat`.
Only compare bytes: JPEG starts FF D8 FF; PNG has eight-byte
89 50 4E 47 0D 0A 1A 0A; GIF starts GIF87a or GIF89a; WebP has RIFF then
WEBP at offset 8. Everything else, including empty, SVG or PDF bytes and
unrecognized corruption, is Unknown. Recognition never certifies payload
integrity. No I/O or external dependency.

This is the shared detector consumed by native_image and L3 PDF export;
do not duplicate ImageFormat/detect_format in export/images.rs. L3 decodes
GIF/WebP with its existing image features; GIF is static at the first frame.
No SVG variant: the separate native-image path uses .svg/.svgz extensions,
not XML content sniffing. Historical absence of an SVG parser is not a new
claim about the entire current repository.

## Acceptance

JPEG prefix maps to Jpeg; exact PNG signature to Png; both GIF signatures to
Gif; RIFF....WEBP to WebP; garbage-not-an-image and [] to Unknown.
Preserve byte-level detector behavior and keep deep-decoding/source-span
responsibilities separate.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/image_format.md`. Historical observations are not a new parity certification.

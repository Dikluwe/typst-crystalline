# L0 — svg/images
Hash do Código: 9084a712

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/images.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/images.rs:14` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Embedded images

Own conservative self-contained SVG recognition, MIME/data URLs, raster payload
embedding, image geometry, local cover clips, EXIF and nonvisual fallback markers.
Embed original PNG/JPEG/GIF/WebP bytes with their MIME and resolved physical box;
preserve alpha without decoding/recoding. Recognize SVG only when the first significant
XML element is svg, allowing UTF-8 BOM/whitespace/declaration/initial comments.
Ambiguous/compressed/unrooted data stays Unknown. Do not alter ImageFormat or claim
a general SVG parser. Nested SVG remains an image; do not rewrite its internals.

A finite clip_rect crops cover in the page's local space without reapplying position
or orientation. Missing/malformed carriers remain Unknown, not absent-clip equivalence.
Apply EXIF 1-8 once on the layout-resolved box. preserveAspectRatio=none applies the
supplied stretch/contain/cover geometry, not new fit/layout calculation. Unknown format
emits nonvisual data-crystalline-image-fallback=unknown-format and no image/fictional
MIME. Do not import PDF decoding or add I/O.

## Verification

Check MIME/original payload alpha, physical boxes, cover clips, asymmetric EXIF,
self-contained SVG landmarks and explicit unknown-format behavior.

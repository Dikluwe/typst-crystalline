# L0 — images/scan
Hash do Código: 1cca7bf1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/scan.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/scan.rs:41` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Validation and document scanning

Own FrameVisitor-based image validation, scan_all_images and page XObject resource
lookup. Validation traverses nested content, fully decodes JPEG and uses the existing
raster processor for other supported formats, stopping at the first
`failed to decode image ({e})` error. Unknown format remains accepted by this
validation path. Validation is not memoized merely because scanning caches payloads.

Scan by encounter order, deduplicating the same Arc data pointer into ImN. Preserve
the existing thread-local payload cache lifetime/key behavior; do not promise new
document-scoped invalidation or merge distinct pointers by equal bytes. RGB JPEG
references the supplied ICC; gray/CMYK keep their own space and JPEG dimensions
retain the existing frame-derived behavior. Raster alpha mask IDs precede main IDs.
Unknown formats are omitted; decode failures retain warning/omission and do not
register successful pointer entries. Build recursive page resource dictionaries
with unique references and return an empty string when no images exist.

## Verification

Check nested validation, first-error diagnostics, repeated-pointer scans, alpha-ID
ordering, RGB ICC routing and failed/unknown omission.

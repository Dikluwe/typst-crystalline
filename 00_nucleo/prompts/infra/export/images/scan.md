# L0 — images/scan
Hash do Código: 86e28332

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

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
Keep the next object ID and image-name counter together in a private scan cursor so
the warning carrier does not expand the image-item helper beyond the linted arity.
Unknown formats are omitted; decode failures retain warning/omission and do not
register successful pointer entries. Build recursive page resource dictionaries
with unique references and return an empty string when no images exist.

## Verification

Check nested validation, first-error diagnostics, repeated-pointer scans, alpha-ID
ordering, RGB ICC routing and failed/unknown omission.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [314]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Validate/register/reference images inside prepared tiling cells and background/
foreground as well, preserving Arc-based byte identity deduplication and existing
decode errors. Use the private visual-resource traversal without changing the
public semantic visitor contract.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Structured omission warnings

The approved
`lab/results/pdf-export-warning-carrier-2026-10-23/review.md` contract makes image
scan receive the current build's warning vector. A malformed supported image
that reaches direct export remains omitted and appends the exact
`failed to decode image ({detail}) — imagem omitida` detached warning. An unknown
format remains omitted and appends the exact
`Formato de imagem desconhecido — imagem omitida` detached warning. Preserve
encounter order, pointer deduplication, validation behavior and successful
registration rules. Do not write stderr, discard a warning, manufacture a source
span or weaken the normal pipeline's pre-export malformed-image error.

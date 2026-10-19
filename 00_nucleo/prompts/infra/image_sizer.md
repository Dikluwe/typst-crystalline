# L0 — Image byte metadata adapter
Hash do Código: c591db42

**Layer:** L3
**Exclusive consumer:** `03_infra/src/image_sizer.rs`

## Current evidence

- `03_infra/src/image_sizer.rs` — `ImageSizeImageSizer`, dimensions/DPI/orientation adapters and TIFF metadata readers.
- `01_core/src/compiler/layout/image.rs` — the L1 sizing consumer of ImageSizer.
- `03_infra/src/export/images.rs` — a separate PDF payload facade, outside this prompt's ownership.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own metadata inspection of already supplied image bytes. Preserve the existing ImageSizer trait implementation and Clone/Copy adapter; do not read paths, load World resources, decode pixels, rotate/re-encode bytes or expose external image-library types to L1. `imagesize::blob_size` yields dimensions as `Option<(u32,u32)>`; invalid/unsupported bytes return `None`.

For PNG DPI, prefer EXIF then pHYs; for JPEG, prefer EXIF then JFIF; other formats return `None`. pHYs uses 0.0254 for pixels/metre and the existing 2.54 conversion for unit 2. JFIF supports unit 1 with positive horizontal density. JPEG segment lengths must include the two length bytes: values below two or beyond the remaining buffer return None without slicing an inverted range, in both DPI and orientation scans. TIFF XResolution supports Rational/SRational with nonzero positive denominator; preserve current parser bounds and conversion. Do not assert a uniform positivity filter: zero PNG/EXIF density can currently be `Some(0.0)`, unlike zero JFIF density. This discrepancy from the old upstream citation remains pending in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`.

Parse TIFF II/MM byte order, marker 42 and IFD0 from the existing PNG/JPEG EXIF containers. Orientation is tag 0x0112, count one, using the accepted BYTE/SHORT/LONG inline forms. Return only values 1 through 8; malformed/absent/unsupported values return `None`. Preserve bounds checking and no new EXIF dependency.

The existing L1 image layout owner interprets absent DPI/default sizing and orientation transforms. `00_nucleo/prompts/infra/world/paths.md` owns World byte acquisition; `00_nucleo/prompts/infra/pipeline/paged.md` owns the pre-export validation call; the excluded `00_nucleo/prompts/infra/export/images.md` and its children own validation/decoding/PDF payloads. This adapter does not duplicate their contracts.

## Verification boundary

Existing byte fixtures control dimensions, density precedence and legal/invalid orientation. Optional `/tmp/p773-dpi300.png` and `.jpg` probes are environmental legacy inputs, not required persistent fixtures or proof when absent. No replacement is assumed.


## Private TIFF reader representation

### Evidence

`lab/results/construction/diagnostics/type-complexity-2026-10-15/before.json.gz`
retains HEAD/uncommitted tree/UTC/diffstat/sources/owners/hashes for image_sizer.rs:268
and :340 (archived C093/C094); red-clippy.log records promoted-lint RED before
editing. Inspection distinguishes positional tuples, repeated FontKey alias and
already explicit static contracts. This is representation maintenance, not AI
benefit, a functional bug or new parity measurement.

### Contract and acceptance

Name the two existing static fn-pointers in private TiffReaders fields read_u16/
read_u32. Private tiff_readers selects II/MM in the same order, None for another
header; remove unused _le. Parsers retain length/magic checks before IFD and call
the same readers/helpers on the same slices. No new parser/bounds/dependency/
precision/DPI/orientation policy.

Continuous ADR-0127 changes no resolved public entity/signature/default/phase.
Type abbreviation requires provably identical expansion; incompatibility/new
public type needs another gate. Nominal type_complexity exceptions do not permit
module/impl/crate suppression or raised threshold. The historical permission
narrowly supersedes generic bans only for its three justified batch items; other
correction cases remain unsuppressible. Preserve functional expectations;
selected gate RED→GREEN includes unfulfilled_lint_expectations as error, followed
by build/all-target Clippy/workspace suite/fmt/Rustdoc/lineage and scoped resealing.
No alias/field-name mirror tests.

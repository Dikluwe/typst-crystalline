# L0 — Document format option domain
Hash do Código: dfcf8fbb

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/document_format_options.rs`

## Decision evidence

The historical `586e1bd43fae6c9a973218163d3165c53ab8d16d` source distinguishes
resolved documentary values from CLI overrides. Retained evidence includes
`lab/typst-original/crates/typst-library/src/layout/page.rs:760-869,806-847`,
`typst-pdf/src/convert.rs:883-898`, `typst-pdf/src/format.rs:552-719`,
and `krilla/src/configure/validate.rs:993-1013,1187-1201` plus
`krilla/src/configure/mod.rs:68-96`. These support page membership and PDF
version/profile compatibility, not fresh parity against the current upstream pin.
Approval history, old staged schemas and probe provenance are preserved in the
sanitation diagnostic. The reduced language-only PDF standards scope remains binding.

## Resolved document options

Public carriers retain Debug/Clone/PartialEq and these fields/defaults:

```rust
pub struct SvgFormatOptions { pub pretty: bool }       // false
pub struct PngFormatOptions { pub ppi: f64 }            // 144.0
pub struct HtmlFormatOptions { pub pretty: bool }      // false
pub enum PdfTagged { Auto, Explicit(bool) }            // Auto
pub struct PdfFormatOptions {
    pub tagged: PdfTagged,                            // Auto
    pub pretty: bool,                                 // false
    pub pages: Option<PageRanges>,                    // None: all pages
    pub standard: Option<PdfStandards>,                // None: undecided
}
pub struct DocumentFormatOptions {
    pub svg: SvgFormatOptions,
    pub png: PngFormatOptions,
    pub html: HtmlFormatOptions,
    pub pdf: PdfFormatOptions,
}
```

SVG/HTML pretty values and PNG ppi are already cascade-resolved, not Options.
PNG stores pixels per inch exactly; it must not store pixels per point or divide
by 72. That conversion belongs to rasterization in L3. No tri-state sentinel or
external override field is introduced. PdfTagged retains the real language
three-state value, not an Option<bool> proxy. Svg/Html/Pdf carriers and PdfTagged
retain Eq; DocumentFormatOptions and PngFormatOptions do not acquire total Eq
because PNG contains f64. Fields/formats and separate instances remain independent.

## Physical page membership

`PageRange` has public optional NonZeroUsize start/end, with inclusive boundaries;
an absent bound is open. Both absent means universal membership. `at(nr)`
constructs a singleton. `PageRanges(Vec<PageRange>)` has a private vector,
`new` preserves it and `includes_page` returns any-range membership. Default
PageRanges is empty and matches no page, distinct from pdf.pages == None.
Keep Clone/PartialEq/Eq; PageRange's existing Copy is preserved.
Do not sort, normalize or deduplicate ranges. Unique physical output order
belongs to the exporter iterating document pages, not to this list.
Dict/array casting, positive-number diagnostics, CLI syntax and link/page
reindexing belong to their evaluator/shell/export owners.

## PDF version and profile domain

```rust
pub enum PdfVersion { V1_4, V1_5, V1_6, V1_7, V2_0 }
pub enum PdfArchival { A1a, A1b, A2a, A2b, A2u, A3a, A3b, A3u, A4, A4f, A4e }
pub enum PdfAccessibility { Ua1 }
pub enum PdfStandard {
    V1_4, V1_5, V1_6, V1_7, V2_0,
    A1a, A1b, A2a, A2b, A2u, A3a, A3b, A3u, A4, A4f, A4e, Ua1,
}
pub struct PdfStandards {
    pub version: PdfVersion,
    pub archival: Option<PdfArchival>,
    pub accessibility: Option<PdfAccessibility>,
}
pub enum PdfValidator {
    Archival(PdfArchival), Accessibility(PdfAccessibility),
}
pub enum PdfStandardsConflict {
    ConflictingVersions(PdfVersion, PdfVersion),
    MultipleArchival,
    MultipleAccessibility,
    NoOverlappingRange { validators: Vec<PdfValidator> },
    VersionOutOfRange { version: PdfVersion, validators: Vec<PdfValidator> },
}
```

Versions/profile enums and PdfStandards retain Debug/Clone/Copy/PartialEq/Eq/Hash;
PdfValidator retains Debug/Clone/Copy/PartialEq/Eq; conflicts retain
Debug/Clone/PartialEq/Eq. PdfVersion default is V1_7; PdfStandards default is
V1_7 with neither profile. Preserve existing raw names/display labels and pure
query methods. `PdfStandards::new(impl IntoIterator<Item=PdfStandard>)`
returns a resolved set or structured conflict, never formatted diagnostics.

Permit at most one explicit version, one archival profile and one accessibility
profile. Retain duplicate-family conflict behavior, including repeated UA-1.
Compatibility intervals are:

- A1a/A1b: 1.4 only.
- A2a/A2b/A2u/A3a/A3b/A3u and UA-1: 1.4 through 1.7.
- A4/A4f/A4e: 2.0 only.

Intersect selected profile intervals. Empty intersection is NoOverlappingRange,
distinct from VersionOutOfRange for an explicit version outside a nonempty
intersection. A valid explicit version wins; without one, selected profiles
resolve to the interval maximum; no profiles/version resolves to 1.7.
Conflict validator data preserves citation/hint order and every involved profile;
diagnostic formatting belongs to the consumer, not this carrier.
`pdf.standard == None` remains distinct from Some(default), including an explicitly
supplied empty standards list.

## Ownership, acceptance and limits

This pure closed domain owns no exporter, CLI precedence, target feature,
style casting, callbacks or I/O. Resolved-option extraction belongs to
`00_nucleo/prompts/compiler/layout/format_options.md`; external override
precedence belongs to `00_nucleo/prompts/infra/pipeline.md`.
Verify defaults, clones/equality, instance and field independence, range membership,
every structured conflict, interval limits and effective version resolution.
Do not test Rust memory layout as language parity.

PDF standards support here is validation/resolution of names and declared version
only. It neither implements nor claims real PDF/A or PDF/UA structural compliance,
profile-specific XMP/color/transparency/font constraints, external certification,
or version-specific PDF body changes. Selecting a profile must not be advertised
as such compliance. Reopening that scope requires a separate architectural decision.

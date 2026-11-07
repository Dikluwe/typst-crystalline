# Prompt L0 — `entities/elements/image` — `ImageElem`
Hash do Código: 27069a53

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/image.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/image-source-provenance.toml sha256:19a4e9d6333f8000f9f64a3520c15a82f052dca270a845a38c727e630a512b5e
- 00_nucleo/prompts/_nuclei/svg-preparation.toml sha256:fe0ab743c1021c51893406cde3ae40925535e1739888ef9b8287f66edfdaf896

## Evidence before specification

Historical extraction anchors content.rs:1740,1896 support text/equality.
The archived image-fit evidence used upstream 0.14.2. Later upstream image
source at `lab/typst-original/crates/typst-library/src/visualize/image/mod.rs:165-181`
provided alt. The archived corpus with alt compiled and passed 106/0 PDF/UA-1
rules; without alt upstream reported `PDF/UA-1 error: missing alt text`.
The old consumer rejected alt as unexpected. These are historical observations,
not a fresh current-pin or general PDF/UA certification.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct ImageElem {
    pub path:   Option<String>,
    pub source: Option<Value>,
    pub data:   PtrEqArc<Vec<u8>>,
    pub width:  Option<Box<Value>>,
    pub height: Option<Box<Value>>,
    pub fit:    EcoString,
    pub alt:    Option<EcoString>,
}
```

The alt field was confirmed on 2026-09-23 under ADR-0127, superseding the
earlier schema without it. Preserve None versus Some("") and include alt in
equality/hash.

## Behavior and acceptance

Preserve `Content::Image(Arc<ImageElem>)` and
`Content::image(path, data, width, height, fit)`; the unchanged constructor sets
alt to None, while stdlib may construct an entity with Some without a new
signature. Fit defaults to cover, with contain/cover/stretch accepted. Its
rendering was an explicit separate scope-out, not certified by this entity.
PtrEqArc and Value belong to entities. PartialEq compares the fields, using
pointer identity for data. Manual Hash remains Debug-based. The historical
claim that PtrEqArc lacks Hash conflicts with its independent current owner
`00_nucleo/prompts/core/entities/ptr_eq_arc.md`; that rationale does not authorize
changing Image's hash. The earlier equality summary omitted fit although the
struct derives equality; preserve all fields, not that incomplete enumeration.
Plain text is empty, is_empty defaults false, maps are terminal, and field/
kind/payload use defaults; non-locatable. Verify terminal carrier preservation,
fit, pointer identity, and distinct absent/empty alt.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/image.md`. Historical observations are not a new parity certification.


## Approved PNG byte source — first raster subset

Evidence: `lab/results/construction/diagnostics/image-input-scope-2026-10-31/`
contains bilateral source/format/reflection/render probes before this decision.
`lab/results/construction/diagnostics/png-bytes-2026-10-31/approval.json` binds
the explicit human approval and exact public proposal. ADR-0127 public/compatibility
gate is satisfied only for path:Option<String>, source:Option<Value> and PNG bytes
with original source. Raster phases, defaults and public constructor signatures
remain. SVG/PDF and format/raw pixels stay pending. JPEG/GIF/WebP byte admission and the measured WebP decode correction are completed by the subsequent explicit user request below.
The partial constructor is incomplete: temporary task admissao-png-bytes owns this
subset; admissao-demais-raster-bytes and cor-webp-pdf complete the named remaining
raster subsets. No claim of full image parity or #8996 closure.

Own the approved exact schema: path:Option<String>, source:Option<Value>. Native original source is Some(Path/Str/admitted raster Bytes); legacy source remains None. Element get_field("source") returns the captured source only. Existing terminal maps/clones preserve source. Data-only entity; no decoder or file operations.


## JPEG, GIF and WebP byte admission — completion

Evidence and explicit user request: `lab/results/construction/diagnostics/raster-bytes-2026-10-31/`.
The original ImageElem schema gate remains ratified. This extension adds no public
field, signature, trait, default, phase or compatibility change. ADR-0127 continuous
flow covers raster admission and the measured internal WebP color correction.
PNG behavior and source invariants remain; SVG/PDF and format/raw pixels stay out.

Extend the existing admitted byte raster set to JPEG/PNG/GIF/WebP while preserving the ratified optional path and original source semantics. The previous temporary pending subset admissao-demais-raster-bytes is completed here; original source contract and legacy absence remain. No new public schema, phase, flag or vector support.


## Approved SVG/SVGZ extension — 2026-11-01

Human ADR-0127 approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Exact contract and phase: `lab/results/construction/diagnostics/svg-image-gate-2026-10-31/proposal.md`. This scoped extension supersedes earlier SVG scope-out/prohibitions only for this feature; preserve unrelated contracts.

Extend ImageElem with format:Option<ImageFormat> and svg:Option<SvgSource>. Preserve path and original source exactly, and retain original data including compressed SVGZ bytes. Native construction captures selected format and SVG origin/span. Legacy constructors retain source=None, format=None and svg=None. No parser/render/IO in this entity. Observation and clones preserve provenance without treating prepared render data as original source.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Apply the approved SVG extension while preserving original source and existing raster behavior.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Transport the approved selected image format and SVG provenance through existing constructors/clones without changing raster behavior or inventing source. Legacy literals use explicit absence; reconstruction preserves the existing format.


## Approved SVG/SVGZ materialization — 2026-11-01

Human approval: `lab/results/construction/diagnostics/svg-svgz-2026-11-01/approval.json`. Preserve unrelated contracts.

Migrate legacy image literals to the approved optional format/SVG metadata; do not synthesize original source. Preserve format on reconstruction, existing frame geometry and raster behavior.

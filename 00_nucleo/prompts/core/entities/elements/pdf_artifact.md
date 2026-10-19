# Prompt L0 — `entities/elements/pdf_artifact` — `PdfArtifactElem`
Hash do Código: 60446855

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/pdf_artifact.rs`
**Governance:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Evidence before specification

Historical upstream pdf/accessibility.rs:36–93 defines kind/body and twelve
kinds. Layout preserves visual/text while PDF uses /Artifact without
MCID/StructElem. The recorded measurement covered other, Header and Background;
real assistive-technology behavior was Unknown. Existing Formula support is
not permission to erase artifact kind. Human approval was recorded on
2026-08-30; the archive preserves the original authorization wording.

## Data contract

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactKind {
    Header,
    Footer,
    Watermark,
    PageNumber,
    LineNumber,
    Redaction,
    Bates,
    Page,
    PaginationOther,
    Layout,
    Background,
    Other,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct PdfArtifactElem {
    pub kind: ArtifactKind,
    pub body: Content,
}
```

ArtifactKind::Other is the language default. Preserve the twelve closed
kebab-case names validated by the native constructor; accept no arbitrary
string and do not collapse distinct kinds. This owner alone owns these data.
The approved fragment is a non-locatable container: element_kind, to_payload,
and dyn_kind_name use defaults. A new ElementKind/query payload needs new
measurement and approval. Plain text and emptiness delegate to body.
map_content transforms body exactly once and rewraps with the same kind;
map_text recurses and preserves kind. get_field("body") returns body;
get_field("kind") returns its kebab-case name; unknown fields return None.
Public repr omits Other and otherwise prints kind before body. Explicit
kind:"other" versus default was not distinguished and remains Unknown.
Equality/hash are structural over kind and body.

## Acceptance and limits

Pure L1 imports no Layouter, stream, or exporter. Never return bare body:
semantic identity must survive to layout/PDF. Body visual/plain text does not
prove AT, reflow, or copy/paste behavior. Nested artifacts, multipage splitting,
and per-PDF-version fallbacks for all twelve kinds remain Unknown outside the
measured fragment. Local tests prove the twelve-name bijection, Other default,
and preserved kind/morphology when mapping. Ownership remains 1:1; this prompt
does not authorize content, layout, or export implementation.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/pdf_artifact.md`. Historical observations are not a new parity certification.

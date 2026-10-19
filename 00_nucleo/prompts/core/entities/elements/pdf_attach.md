# Prompt L0 — `entities/elements/pdf_attach` — `PdfAttachElem`
Hash do Código: 6375d6a2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/pdf_attach.rs`
**Governance:** ADR-0029, ADR-0107, ADR-0108, ADR-0127, ADR-0129.

## Evidence before specification

Historical upstream pdf/attach.rs:31–86 preserves virtual path, bytes,
relationship, MIME and description; typst-layout/src/rules.rs:824 and
typst-pdf/src/attach.rs:13–67 collect globally without visual content.
Recorded tests cover required path, optional bytes as second positional,
byte-identical extraction, and duplicate-path error. Human approval on
2026-08-30 covers Content::PdfAttach(Arc<PdfAttachElem>) without a new Value,
trait or phase; original authorization wording remains archived.

## Data contract

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachedFileRelationship {
    Source,
    Data,
    Alternative,
    Supplement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PdfAttachElem {
    pub path: EcoString,
    pub data: Arc<Vec<u8>>,
    pub relationship: Option<AttachedFileRelationship>,
    pub mime_type: Option<EcoString>,
    pub description: Option<EcoString>,
}
```

path is the resolved virtual name used for deduplication identity and
Filespec. data is the snapshot read/provided during eval; the entity never
queries World, filesystem, or L3. Native pdf.attach validates path, MIME,
types and relationship; this element neither accepts nor silently repairs
invalid payloads. This owner alone owns these data.

The approved fragment is an invisible non-locatable leaf: element_kind,
to_payload, dyn_kind_name, and get_field use defaults. New ElementKind/query
payload/location requires new measurement and approval. Plain text is empty
but is_empty is false so collection cannot prune the marker. Both maps are
terminal and reconstruct Content::PdfAttach(Arc::new(self.clone())), preserving
all fields. Equality/hash compare all five fields; duplicate error identity
uses path, not whole-struct Rust equality.

## Acceptance and limits

Pure L1 may use Arc<Vec<u8>> as RAM; it performs no I/O. Create no frame,
placeholder, text, approximate metadata, or fallback. Distinct elements with
the same path remain distinct here; the global pipeline emits
`attempted to attach file {path} twice`. Compression, all PDF standards and
unmeasured PathOrStr variants remain Unknown; Unknown is never success.
Local tests prove closed relationship parsing, textual invisibility without
pruning, and byte-exact preservation of all five fields through terminal maps.
Ownership is 1:1; this prompt cannot authorize content/layout/pipeline/export
or be reused by another source.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/pdf_attach.md`. Historical observations are not a new parity certification.

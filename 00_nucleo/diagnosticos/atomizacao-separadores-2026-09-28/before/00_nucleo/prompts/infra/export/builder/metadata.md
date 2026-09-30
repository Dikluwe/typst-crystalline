# L0 — builder/metadata
Hash do Código: 7a93975f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/metadata.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/metadata.rs:22` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Metadata and identification

Own Info, XMP, timestamps and document/instance ID generation, archival output
intents, tagged structure trees and PDF/UA viewer preferences. Do not serialize
xref/trailer or attachments; publish the existing stored references for those owners.

Emit Info except in A4/A4f/A4e. Encode Title/Author/Keywords as UTF-16BE hex with BOM
and retain the Typst Crystalline Creator branding. Select the present
CRYSTALLINE_PDF_FIXED_EPOCH value, otherwise SOURCE_DATE_EPOCH, before parsing.
A missing or invalid selected value falls back to current UTC; an invalid primary
value does not retry the secondary variable. Info PDF dates and XMP ISO-8601 dates use that policy.
Always emit a Type Metadata/Subtype XML stream and catalog reference. Escape XML
metacharacters, preserve user metadata presence, application/pdf format, proof
rendition class and selected page count. Generate separate standard-base64 instance
and document identifiers from 16 bytes; tests/fixed-epoch use the existing distinct
constants. An external document ID overrides only the document identifier. Persist
the pair for serialization; do not claim production bytes are globally deterministic.

Use archival identification exactly for the selected mode: A1/A2/A3 have part and
their A/B/U conformance as applicable; A4 has part 4 and rev 2020 without conformance;
A4f/A4e additionally use F/E. Emit the archival namespace only when applicable.
All archival modes receive the output intent and version-selected profile supplied
by the images ICC owner; preserve the GTS_PDFA1 subtype, profile reference and
existing descriptive fields. Do not infer full archival compliance from these fields.

For UA1, emit pdfuaid part 1 before the NPages field and DisplayDocTitle. With archival plus UA1 include
the minimal PDFUAIdentification extension schema, pdfuaid namespace/prefix and
internal Integer part property. Preserve document language in catalog and XMP;
use en only when the document language carrier is absent. Local styled language
does not retroactively replace the document language. Do not add viewer Direction.

## Tagged structure

Build structure only when PdfTags is Enabled. Bind to the page-local MCID table
owned by the stream façade rather than assigning a second numbering algorithm.
Emit per-page StructParents, ordered ParentTree associations, StructTreeRoot and
MarkInfo with Marked true/Suspects false. Disabled tags omit this structure.
Artifacts and descendants never become StructElem/MCID content. Formula alt is
nonvisual; preserve absent versus present-empty alt. Labels use Lbl.

Tables preserve Table/TR/TD, uniform automatic headers THead/TBody/TR/TH/TD,
and direct TR for explicitly mixed rows. Explicit Data in a header remains TD.
Summary belongs only to Table and preserves absent versus empty. TH Scope supports
Row/Column/Both; IDs and header associations follow the logical header carriers,
not a numeric PDF level attribute. Emit span attributes only above one. Repeated
headers across pages reuse the logical TH and do not duplicate IDs or relationships.

UA1 represents a physical heading as one H1-H6 unit and a paragraph as one P unit,
including multi-run content. Preserve Span children, Strong/Em role mapping,
Link/Figure alternative descriptions and language. Do not infer headings from
boldness or font size. Paragraph/line boundaries do not create empty nodes.
Link annotation OBJR and ParentTree relationships bind the references prepared by
navigation; no orphan keys. Keep meaningful content real, not disguised as Artifact.
Structural positives do not establish assistive-technology, reflow or full UA claims.

## Verification

Exercise metadata presence, fixed/external IDs, selected page counts, all archival
variants, UA-only/archival+UA, language, formulas, tables, repeated headers,
paragraph boundaries, links and tag-disabled omission.

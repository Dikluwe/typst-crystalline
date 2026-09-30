# L0 — builder/metadata
Hash do Código: fe5ccd25

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

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

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/builder/metadata.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator pertence à mesma folha textual e idioma/estilo de Span no percurso UA1; não criar um algoritmo distinto de MCIDs ou novo heading/paragraph apenas pelo carrier.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Walker estrutural desce no envelope sem criar estrutura/MCID próprio; estilos/idioma e MCIDs pertencem às folhas visuais.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.

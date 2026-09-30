# L0 — builder/navigation
Hash do Código: 14109b43

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/navigation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/navigation.rs:32` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Selected-page navigation

Own PageIndexConverter, link annotations, named destinations, outline, attachments
and page labels, including URI/name escaping and reference parsing used here.
Select pages by increasing original physical index; use one-based ranges only for
membership, not output ordering or duplicate emission. Without ranges the converter
is identity. Map old zero-based indices to selected zero-based indices everywhere.

Emit annotations only on selected source pages. Convert rectangles to PDF coordinates
using page height - y - height, preserve no-border and URI/GoTo action routing.
Drop links/destinations to a valid but excluded page. Only a target page that never
existed may use the existing last-retained-page fallback. Named destinations retain
label identity and positions with y conversion and PDF-name escaping.

Outlines retain hierarchy, parent/sibling/first/last references, negative direct-child
counts for closed nodes and a positive top-level count. Filter removed-page headings;
reparent retained descendants to an included ancestor or promote them to top-level.
Never reproduce an external outline-count defect as a requirement.

## Attachments and labels

Preserve attachment order, raw EmbeddedFile payloads, Filespec EF/UF, optional
description/MIME and the EmbeddedFiles name tree merged with existing Dests. Duplicate
filename validation stays upstream. AFRelationship and catalog AF apply only to
A3a/b/u and A4f/e; map Source/Data/Alternative/Supplement and default Unspecified.
AF exists only when Filespecs exist. A4f with no attachments still emits an empty
EmbeddedFiles Names array, but no payloads, Filespecs or AF. Do not require an empty
tree for other empty-attachment archival modes.

Emit PageLabels against selected PDF indices from original logical_number and
serializable NumberingPattern. Native styles preserve prefix/suffix; otherwise use
the existing textual label. Never reexecute numbering functions. Without a usable
pattern, emit original physical one-based labels only when pages were actually
omitted; selecting every page needs no corrective label tree. Allocate label and
navigation objects without colliding with pages, fonts, structure or content streams.
UA annotation Contents/StructParent and page Tabs bind to metadata-owned structure;
this owner does not generate the structure tree or decide MCIDs.

## Verification

Cover reordered/overlapping range membership, removed versus invalid destinations,
retained outline descendants, attachment modes, empty A4f, native/text/function labels
and independent annotation/structure references.

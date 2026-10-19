# L0 — builder/navigation
Hash do Código: 89367b95

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/navigation.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/navigation.rs:32` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## URI operand — measured correction 2026-10-14

The PDF probes in links-captura-2026-10-14/probes/results.json.gz record
HEAD, dirty-tree state, UTC and binary digests. qpdf decodes the URL action
as `(https://example.org)` instead of the supplied `https://example.org`.
In navigation.rs:76-77 the helper delegates to serialization.rs:16, which
already returns the complete PDF literal operand; navigation.rs:274 adds
another pair of parentheses. This alters the destination, not merely the
PDF byte representation.

Emit that operand exactly once, preserving the URI's literal parentheses,
backslashes and control escapes through the serializer. A generated PDF
action must decode to the supplied ASCII URI without added delimiters.
Test both a normal URL and escaping-sensitive input through annotation
emission; verify the generated CLI PDF with an independent parser. This
is an internal serialization correction under ADR-0127, with no new API,
mode or phase.

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
label identity and positions with y conversion and the shared PDF string encoding specified below.

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

## Resolvable named destinations — 2026-10-14

### Measurement before decision

labels-fronteira-2026-10-14/reader-probes.json.gz identifies the PDFs,
compiler digests, reader commands and UTC. MuPDF 1.23.10 finds the GoTo
actions but resolves their named destinations to -1 for Tekt and to page
index 0 for the pinned vanilla. navigation.rs:373-408 writes a dictionary
with name keys under Catalog.Names.Dests, while :277 uses name operands
for simple GoTo targets. ISO 32000-1 §§7.9.6/12.3.2.3 requires a name tree
at this location and string references to that tree; the legacy name-key
dictionary belongs directly under Catalog.Dests. Source and owner snapshot:
destinos-pdf-2026-10-14/before.json.gz. This is a reader-observable failure,
not a requirement for matching upstream PDF bytes.

### Contract and acceptance

Emit Catalog.Names.Dests as a root leaf name tree: /Names contains alternating
string keys and destination arrays. Both tree keys and GoTo /D references
use the same UTF-16BE string with BOM, serialized as hex by the existing
serialization owner. Never emit a PDF name object as a reference to this
tree. Preserve every label code point, including slash, parentheses and
supplementary Unicode; delimiters must not become PDF syntax.

Sort tree entries by the unsigned bytes of the encoded keys; since the
BOM is common, lexicographic UTF-16 code-unit ordering is equivalent.
Short prefixes precede longer keys. Keep coordinate conversion, selected
page remapping, excluded-target omission, invalid-page fallback and object
ID allocation. Attachments continue to merge /EmbeddedFiles into the same
Names dictionary without replacing Dests. Do not change capture or logical
page counters in this serialization unit.

Test name-tree topology, matching action operands, ASCII prefixes, Unicode
ordering, retained/excluded targets and coordinates. Verify generated PDFs
through an independent reader resolving internal links, including selected
pages and Unicode labels. Internal correction under ADR-0127: no new public
API, default, flag or pipeline phase.


## Optional retained-index expression

`lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz`
retains sources/L0/HEAD/uncommitted tree/status/diffstat/UTC; initial-diagnostics.json
retains inspected uses/types/suggestions/spans at navigation.rs:583
(clippy::manual_map). This is internal Rust representation, no new parity/performance.

Express only retained-index Option with pic.new_index(page_idx).map, keeping
prior conditions: never-existent target uses last retained page; valid excluded
target remains None. Do not create heading or change stack when excluded.
Continuous ADR-0127 adds no public/default/phase/dependency/suppression. L0 first/
scoped seals, unchanged control inputs/results/tolerances, selected-lint RED→GREEN,
workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage; no syntax-mirror tests.

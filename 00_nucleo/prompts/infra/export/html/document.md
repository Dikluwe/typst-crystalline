# L0 — html/document
Hash do Código: dabfbe2a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/document.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/document.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Document composition

Own export_html_with_frame_resolver, automatic doctype/envelope/metadata,
explicit_document_root and block_sequence with the initial StyleChain. A sole
explicit html root follows the doctype directly without automatic lang/head/metadata;
a sole body retains its attributes inside the automatic html/head envelope. html/body
with other relevant nodes returns `` `<{}>` element must be the only element in the document ``
with the offending tag substituted. Ignore only
the existing introspective/empty-space nodes in root classification; head/title are
ordinary nodes, not extracted metadata.

Buffer top-level paragraph-groupable content according to tags; flush at parbreaks,
block/non-groupable nodes, tables and HtmlFrame. Preserve trimming and paragraph
boundaries through whitespace helpers; do not insert paragraphs inside normal
HtmlElem bodies. Emit compact markup first and call pretty only afterwards.
Inline frames remain block-level without a paragraph wrapper; their markup belongs
to inline and their rendering to the external resolver.

## Verification

Check explicit html/body exclusivity, head as ordinary content, attributes/metadata,
mixed paragraph boundaries, tables/frames and equivalent compact/pretty document DOM.

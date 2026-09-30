# L0 — html/inline
Hash do Código: f1c97acd

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/inline.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/inline.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Inline content dispatch

Own static Content dispatch for text, sequences, Strong/Emph, accumulated Styled
chains, linebreak, table wrappers, HtmlElem and HtmlFrame. Preserve the unsupported
content fallback `HTML export does not yet support this content`. Delegate normal body whitespace,
table serialization and context-dependent escaping to their owners.

HtmlElem preserves typed tag names and attribute order. Empty attribute values emit
only the name; nonempty values are quoted/escaped by encoding. Void tags emit no
closing tag and reject Content bodies even when empty with
"HTML void elements must not have children". Raw/title handling delegates to encoding.
For HtmlFrame, insert the resolver's validated SVG markup unescaped at its occurrence;
without a resolver retain `html.frame must be resolved by the HTML compilation pipeline`. Do not render
layout, wrap frames in p, substitute a frame tag or change style/resolver contracts.

## Verification

Check ordered/empty attributes, void-body rejection, styled chains, raw/title
delegation, unsupported variants and resolved/unresolved inline frames.

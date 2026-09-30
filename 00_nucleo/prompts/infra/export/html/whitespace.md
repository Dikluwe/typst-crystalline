# L0 — html/whitespace
Hash do Código: 088d8a43

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/whitespace.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/whitespace.rs:16` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Structural space handling

Own inline_html_body, ruby recognition, space_between/top_level_space_between and
SpaceSupport classification. Traverse Styled and inline wrappers without modifying
text/raw payloads. Trim only structural Sequence edges and spaces adjacent to block
or br boundaries; preserve supported inline spaces. Distinguish Boundary,
EmptyInline and Supportive, with replaced tags supporting visible space.

An isolated ASCII space lacking supportive content on both sides uses exactly
`<span style="white-space: pre-wrap">&#x20;</span>`. Top-level lookup reaching a
non-groupable node drops the boundary space rather than making a paragraph/span.
Within ruby, drop a space only when both immediate non-space neighbors are rp/rt;
base text or ordinary inline siblings still preserve spaces. Use tags' static
classification rather than duplicating display tables or implementing CSS.

SpaceFromNewline has the same local projection as Space in these helpers. CJK
normalization and textual show matching belong upstream; do not inspect neighboring
CJK characters to hide an upstream language error. Do not indiscriminately trim
raw/title or change preformatted content.

## Verification

Check edges, br/block adjacency, supportive/replaced versus empty inline nodes,
top-level boundaries, ruby counterexamples and direct Space/SpaceFromNewline pairs.

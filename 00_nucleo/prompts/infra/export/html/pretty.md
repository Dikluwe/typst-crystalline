# L0 — html/pretty
Hash do Código: 8ee147b8

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/pretty.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/pretty.rs:10` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Optional structural formatting

Own tokenization/classification, quoted tag termination, protected-context tracking,
pretty_html and structural newline helpers. Operate on already compact markup;
preserve every token's bytes. Insert two-space indentation per depth and final
newline only at structural boundaries. Protect phrasing, p/headings, title,
pre/textarea/script/style and td/th interiors. A > within a quoted attribute must
not end a tag. Retain the current tokenizer's supported scope, including raw SVG/
script limitations; no parser rewrite or escaping belongs here.

## Verification

Compare ordered DOM/escapes and protected text before/after formatting; exercise
quoted >, void elements, raw/title and phrasing spaces.

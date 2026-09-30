# L0 — Document metadata carrier
Hash do Código: 77aa0cfc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/document_info.rs`

## Evidence

Retained upstream evidence at historical `e0e8ca4d` is
`lab/typst-original/crates/typst-library/src/model/document.rs:14-34,157-179`.
Document metadata is embedded by default rather than visibly rendered; this
entity is a partial carrier, not a claim to the entire upstream field surface.

## Contract

`DocumentInfo` has public optional EcoString fields title, author, keywords,
with Debug/Clone/PartialEq/Default. Default and empty produce all None;
`is_empty` is true exactly when all fields are None. Preserve evaluated values,
including explicitly supplied empty strings. Author inputs may be strings or
string arrays, with the latter converted to one comma-separated string by the
evaluation owner; this carrier neither parses nor chooses that conversion.

This is pure metadata transport from evaluation through Module/PagedDocument
to exporters, with no I/O or rendering. Date, description and richer Content/
multiple-author representation remain outside this carrier's declared scope.
Creation date/creator and output-format decisions belong to export owners.
The upstream automatic-date and PDF/UA title evidence
`document.rs:168-178,151-153` must not be turned into an implicit clock or full
PDF/UA claim in this entity. Verify empty/nonempty fields, clone and equality.

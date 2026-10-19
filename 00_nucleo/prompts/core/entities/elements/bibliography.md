# Prompt L0 — `entities/elements/bibliography` — `BibliographyElem`
Hash do Código: 2d2262d7

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/bibliography.rs`

## Evidence before specification

Historical ratified vanilla snapshot `e0e8ca4d`, `crates/typst-library/src/model/bibliography.rs:111–124,428–441`, accepts Hayagriva YAML/YML and BibLaTeX BIB through paths/raw bytes/arrays. JSON is not a bibliography format. The historical local probe on 2026-08-13 rejected `#bibliography("works.json")` with `error: unsupported bibliography format 'json': expected .bib, .yaml or .yml`; upstream reports `unknown bibliography format (must be .yaml/.yml or .bib)`. Thus the old JSON title was documentary drift, while the differing error wording remains an observable-language issue. Upstream `:146–154,159–163` accepts built-in names, CSL paths or raw bytes and defaults to `ieee`; `model/cite.rs:39–42` establishes reference syntax `@key`. Current-file-relative resolution and local error design are implementation decisions, not claims from that documentation.

The archived 2026-08-13 explicit-`ieee` probe produced citations `A [1] B [1] C [2] D [1]` and an entry `[1] J. Doe, At what cost. Fake Press, 2020.`, with byte-identical output for that fixture. Omitted style differed; that unresolved default issue belongs to `00_nucleo/prompts/core/compiler/layout/bibliography.md`. This is historical evidence, not current-pin or general CSL certification.

## Carrier and Element contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BibliographyElem {
    pub entries: Vec<BibEntry>,
    pub path: Option<EcoString>,
    pub title: Option<Content>,
    pub style: Option<EcoString>,
    pub locale: Option<EcoString>,
}
```

Use `Content::Bibliography(Arc<BibliographyElem>)`. Preserve literal-entry compatibility through `Content::bibliography(entries, title)`, CSL configuration through `bibliography_with_style(entries, title, style, locale)`, and eval-time loading through `bibliography_from_path(path, title, style, locale)`. `BibEntry` supplies Eq/Hash and Content its manual Hash. Derived PartialEq covers the declared carrier fields; the older entries/title-only explanation predates added fields and is not permission to ignore them.

This is a locatable container. `plain_text` emits title plus newline when present, then each entry as `"[{key}] {author}. {title} ({year}).\n"`. `is_empty` is exactly `entries.is_empty() && title.is_none()`. Both maps recurse into title while retaining entries and all other fields. Default `get_field` returns None. `element_kind` returns `Some(ElementKind::Bibliography)`; `to_payload` returns `Some(ElementPayload::Bibliography { entries: self.entries.clone() })`. Preserve the payload-based Bibliography→BibStore consumer; it does not inspect Content directly.

## Loading and CSL integration boundaries

Use hayagriva for supported bibliography parsing and the CSL engine. Keep render behavior in the bibliography layout owner as a free function under ADR-0109; this carrier does not perform rendering or I/O. The eval bibliography owner uses `World::read_bytes(current_file, path)` for L3 byte access and `hayagriva::io` for pure parsing. Relative paths resolve against current_file through World; absolute paths are used directly. Populate entries during eval and retain the internal literal-entry API. The obsolete JSON claim is withdrawn; supported file extensions are `.bib`, `.yaml`, `.yml`.

Style remains `Option<EcoString>`; eval distinguishes built-in names and CSL paths without changing this struct. Its `resolve_style`, `load_csl_from_path`, `resolve_csl_path` first try `hayagriva::archive::ArchivedStyle::by_name`, then relative/absolute path loading through World and XML parsing through `hayagriva::citationberg::IndependentStyle::from_xml`. Preserve readable unknown-style, file-not-found, encoding and malformed-XML errors. Language acceptance is bibliography/citation behavior, not copying upstream mechanics.

## Acceptance and unresolved scope

Preserve title traversal, literal entries, field retention, locatable payload extraction, plain text, emptiness and equality/hash behavior. Verify real CSL and path-loaded citation/bibliography integration while keeping the default-style discrepancy visible.

The original rendering slice explicitly excluded URLs, multiple bibliography files, customized title rendering and CSL locale support despite storing title/locale. These are integration debts, not permission to remove the fields. Loading excludes a new generic loader, network/watch/reload, encoding detection and complex BibTeX macros. Custom CSL excludes URLs, system directories such as `~/.csl/`, simultaneous styles, hot reload, full RelaxNG validation and persistent cross-run caches. Internal Vec<BibEntry> conversion remains in eval/layout. Historical exclusions do not establish current support in other owners.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/bibliography.md` preserves the original evidence and chronology.

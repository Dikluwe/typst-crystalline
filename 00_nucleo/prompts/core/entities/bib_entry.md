# Prompt L0 — BibEntry
Hash do Código: 60f817bb

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/bib_entry.rs`
**Governance:** ADR-0034, ADR-0037, ADR-0054, ADR-0060, ADR-0065.

## Evidence and bounded scope

The archived bibliography diagnostic records the choice of four minimal
universal fields instead of the richer hayagriva::Entry parsing/typography
surface. This is a graded subset, not a claim to every upstream field/style.

## Data and constructor contract

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    pub key:    String,
    pub author: String,
    pub title:  String,
    pub year:   u32,
}
```

`BibEntry::new(key: impl Into<String>, author: impl Into<String>,
title: impl Into<String>, year: u32) -> Self` preserves all values. Key is the
unique bibliography identifier; author/title/year are the universal subset.
Year accepts nonnegative u32, with zero meaning no year. Derived equality
compares all four fields; Clone/Debug retain pure data.
The boundary consumer uses Vec<BibEntry> in bibliography content; stdlib's
private extract_bib_entries validates required fields from Array<Dict> input.
That parsing belongs to its owner.

## Deferred scope and acceptance

Volume, pages, journal, publisher, URL, DOI and richer CSL integration remain
outside this subset. Hayagriva integration is a future candidate under the
applicable promoted ADR-0062 decision, not reserved implementation space.
The old suggestion that adding public Option<String> fields is nonbreaking
must not waive ADR-0127: no public field is added by this cleanup.
Verify `new("k", "A", "T", 2024)` exposes all four fields; a difference in
any field breaks equality; Debug includes all four readably.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/bib_entry.md`. Historical observations are not a new parity certification.

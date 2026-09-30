# L0 — compiler/eval/bibliography
Hash do Código: 234f05e8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bibliography.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bibliography.rs:34` — `pub fn load_bib_entries_from_path`.
- `01_core/src/compiler/eval/bibliography.rs:54` — `pub fn parse_bibliography`.
- `01_core/src/compiler/eval/bibliography.rs:102` — `pub fn resolve_style`.
- `01_core/src/compiler/eval/bibliography.rs:115` — `fn load_csl_style_from_path`.
- `01_core/src/compiler/eval/bibliography.rs:143` — `fn hay_entry_to_bib_entry`.
- `01_core/src/compiler/eval/bibliography.rs:186` — `fn person_to_string`.

## Contract

Own bibliography loading/parsing and CSL style resolution. Obtain bytes through
World, preserve relative FileId/path resolution, decode UTF-8 and select BibTeX
or YAML/YML by the existing extension policy. Preserve unsupported-format,
encoding, parse, empty-key and missing-field diagnostics. Convert Hayagriva
entries and persons into the existing BibEntry domain representation.

Resolve built-in CSL styles or load a relative custom CSL through World with the
current validation and errors. L1 performs no direct filesystem/network I/O.
Do not change citation realization, layout, registry or the public signatures.

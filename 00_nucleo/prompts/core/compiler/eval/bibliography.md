# L0 — compiler/eval/bibliography
Hash do Código: 961d1831

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

## Unused local helper boundary

### Evidence before decision

`01_core/src/compiler/eval/bibliography.rs:34` contains the historical Clippy
inventory item. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working
tree; `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
preserves sources, owners, references, UTC, status and diffstat. Call-site and
productive-path research precedes this classification; this is neither new
parity evidence nor measured performance.

### Contract and acceptance

Compile internal load_bib_entries_from_path only under `#[cfg(test)]`, preserving
its signature/body for existing tests. This legacy read wrapper is called only
by the UTF-8 test. Its parent module is pub(crate), without external reexport.
parse_bibliography and resolve_style remain productive; the bibliography native
continues resolving/reading its source and calling parse_bibliography.

Preserve productive routes, existing test names/bodies, external APIs, defaults
and phase. ADR-0127 continuous flow applies. Revalidate build/check for all
workspace targets, the core suite, test inventory, Clippy and lineage; do not
remove expectations to pass the gate.

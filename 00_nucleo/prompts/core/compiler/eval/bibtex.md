# L0 — Pure BibTeX parser
Hash do Código: 51e7074f

**Layer:** L1 (the owner remains here; filesystem loading belongs to L3)
**Exclusive consumer:** `01_core/src/compiler/eval/bibtex.rs`

## Current evidence

- `01_core/src/compiler/eval/bibtex.rs:62` — `parse_bibtex`.
- `01_core/src/compiler/eval/bibtex.rs:85` — entry parsing.
- `01_core/src/compiler/eval/bibtex.rs:205` — nested braced values.
- `03_infra/src/world/paths.rs` — the bibliography loading adapter.

These are current local source anchors, not a new upstream parity measurement.

## Contract

This prompt owns only the pure parser in its exclusive consumer. It accepts a UTF-8 string and returns `Result<Vec<BibEntry>, BibTeXError>`; `BibTeXError.message` remains a public string and its existing Portuguese diagnostic text is unchanged. `BibEntry` belongs to `01_core/src/entities/bib_entry.rs`. No filesystem, network, clock or environment access belongs to this parser.

Supported entry types are `article`, `book`, `inproceedings`, `misc`, `phdthesis` and `techreport`, normalized without case sensitivity. Keys end at whitespace, comma or closing brace. Field names are normalized to lowercase; repeated names retain the last value. `title`, `author` and `year` are required. `year` must parse as `u32`. Recognized optional fields are `doi`, `url`, `journal`, `booktitle`, `volume` and `pages`; `booktitle` supplies `journal` only when `journal` is absent.

Values use balanced braces, quoted strings or decimal digits. Braced values preserve inner braces and literal LaTeX macros. The implemented quoted-string scan ends at the next quote. Escaped-quote and negative-number support has unresolved implementation gaps recorded in `lab/results/construction/diagnostics/sanitization-infra-wiring-history.md`; this sanitation does not change the parser or silently certify those claims.

Authors split on lowercase ` and `. A comma separates surname from given name; canonical output trims each part and joins authors with ` and `. Percent comments and whitespace are skipped; other text outside an entry is a syntax error. Unsupported types, macros such as `@string`/`@preamble`/`@comment`, concatenation and arbitrary LaTeX expansion are not added.

File decoding and `.bib`/YAML selection belong to `00_nucleo/prompts/infra/world/paths.md` and the bibliography evaluation owner. They are dependencies, not additional consumers of this prompt.

## Verification boundary

The existing inline cases cover entries, required/optional fields, canonical authors, nested braces, unsupported types and unexpected text. Test parser semantics and diagnostic fragments without moving I/O into L1.

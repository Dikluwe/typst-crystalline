# L0 — Pure BibTeX parser
Hash do Código: 088ce7f3

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


## Measured unbraced natural-name canonicalization

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
bibliography-names-before/receipt.json retains thirteen exact BibTeX assets,
font/style/source/argv/UTC/HEAD/dirty-stat/archive/binary identities against
immutable math-private-build2 releasefeffd641 and oracle64c5e8c0. With explicit
English/ieee/title:none, natural Ana Silva/Bruno Souza, Ada Lovelace, Alan
Mathison Turing, Émilie Du Châtelet and Jean-Luc Picard render full given names
locally but native initials. Existing commas, initials, single name, lower-name
and particle-comma controls pass their author text. Ludwig van Beethoven also
requires a given/family boundary; corporate braces remain an independent
negative. Current bibtex.rs:297 gives every comma-free name no given part.
Pinned dependency biblatex0.12.0 types/person.rs:95–187 explicitly distinguishes
leading capitalized words, lowercase prefix and last name. These are name
semantics, not Rust data-structure equality or a string copied from candidate.
The source/data receipts retain the exact evidence used here; no full name/
CSL/locale/title parity is inferred.

### Exclusive bounded correction

Keep the existing lowercase ` and ` author split and all comma-form handling.
For an unbraced name without a comma or backslash, split whitespace into words.
A single word remains a family-only author. For multiple words, given names
are the initial capitalized sequence, excluding the mandatory last word; the
remaining words form the existing family/prefix string. If the first word is
not uppercase, preserve family-only meaning. Canonicalize through the existing
Author.to_canonical, retaining every word and its characters. Keep braced or
macro-containing names on their current path; do not strip corporate braces or
pretend that the existing author-string representation supports full structured
prefix/suffix/extended-name/APA semantics. Explicit comma/initial/single/lower/
protected inputs remain in controls. Existing parser field/type/year/error and
all other values are unchanged; introduce no external crate/parser or I/O.

Add actual parser RED for natural/two-author/middle/Unicode/hyphen/particle and
initial forms; retain comma/single/lower/protected/macro controls. GREEN and
real unchanged bilingual/original bibliography render controls must show the
actual author words, with full core and final strict/workspace/visual/history/
JPEG gates. Locale, automatic title and corporate/structured names remain open.
Internal parity correction follows continuous ADR-0127, L0 first; no public
field/trait/signature/default mode or phase change. This supersedes only the
historical comma-free family-only parsing clause, not other parser contracts.

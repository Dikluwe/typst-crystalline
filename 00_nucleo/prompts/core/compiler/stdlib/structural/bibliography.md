# L0 — stdlib/structural/bibliography
Hash do Código: ab78a56c

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/bibliography.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/bibliography.rs:48`, `01_core/src/compiler/stdlib/structural/bibliography.rs:266`, `01_core/src/compiler/stdlib/structural/bibliography.rs:381` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own bibliography/cite construction, entry extraction and strict citation-form/style parsing. bibliography accepts pre-parsed Array<Dict> entries or Path|Str loaded through the owning eval bibliography helper; Path identity goes directly to World::read_path, while Str resolves relative to the caller before reading through the same helper. No direct file I/O or local BibTeX parser; injected World access through the loader is not a filesystem dependency of this unit. Preserve cross-file rooted identity.

bibliography preserves title/style/locale and registers the same entries. Omitted style defaults to "ieee"; explicit Str selects it, explicit none disables it. Do not promote the textual style parameter to Path without a separately specified contract. cite accepts key and optional supplement/form/style, resolving only registered entries. Form/style matching is strict and case-sensitive; Auto/None becomes None, "prose" is valid but "Prose" is not. Preserve existing casts, diagnostics and entry semantics; loading/parsing and CSL/rendering remain with their owners.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Explicit optional locale extension

### Evidence before decision

At baseline HEAD `91ea6ee911`, `compiler/stdlib/structural/bibliography.rs:331-335`
retains Str locale and projects None or every other Value to absent locale.
The current pinned upstream BibliographyElem in
`lab/typst-original/crates/typst-library/src/model/bibliography.rs:143-224`
has no locale argument; text language/region supplies its locale at lines
1098-1108. The bilateral `lab/results/v16-corrections-2026-10-19/batch-f/agents/adapters/baseline.json`
confirms valid-source calls with supplied locale reject it as unexpected upstream,
and missing-source errors precede locale rejection. Tekt's existing locale surface
is therefore an extension, not a vanilla wrong-type cast to invent.

### Contract and acceptance

Keep this extension's current behavior explicitly: absent locale and every
non-Str Value produce None; any Str is retained verbatim without validation or
normalization. Enumerate the present Value variants exhaustively so future types
require a new decision. Preserve source loading/parsing before locale projection
and style resolution afterward, including failures and registration. Preserve
pre-parsed entries, rooted paths, title, default ieee and explicit style none.
Do not remove this extension or add validation in a V16 repair. Tests cover omitted,
None, valid/invalid-looking strings, Bool/Int/Content and collection-valued locale,
and source/style error precedence. A disposable new Value must produce E0004 at
this decision. ADR-0127 continuous flow applies to nominal coverage only.

# L0 — stdlib/structural/bibliography
Hash do Código: 724bb68b

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/bibliography.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/bibliography.rs:48`, `01_core/src/compiler/stdlib/structural/bibliography.rs:266`, `01_core/src/compiler/stdlib/structural/bibliography.rs:381` identifies the ownership boundary. Inspection is not new bilateral parity evidence or upstream intent. Historical evidence retains its original provenance in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own bibliography/cite construction, entry extraction and strict citation-form/style parsing. bibliography accepts pre-parsed Array<Dict> entries or Path|Str loaded through the owning eval bibliography helper; Path identity goes directly to World::read_path, while Str resolves relative to the caller before reading through the same helper. No direct file I/O or local BibTeX parser; injected World access through the loader is not a filesystem dependency of this unit. Preserve cross-file rooted identity.

bibliography preserves title/style/locale and registers the same entries. Omitted style defaults to "ieee"; explicit Str selects it, explicit none disables it. Do not promote the textual style parameter to Path without a separately specified contract. cite accepts key and optional supplement/form/style, resolving only registered entries. Form/style matching is strict and case-sensitive; Auto/None becomes None, "prose" is valid but "Prose" is not. Preserve existing casts, diagnostics and entry semantics; loading/parsing and CSL/rendering remain with their owners.

## Acceptance

Preserve syntax, semantics, morphology, diagnostics/hints/spans, bindings, defaults and identities. No productive code or compatibility change is authorized by this sanitation. Preserve pinned shared claims and exclusive ownership under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

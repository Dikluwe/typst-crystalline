# L0 — bib csl
Hash do Código: edfa8951

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/bib_csl.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/bib_csl.rs:39`, `01_core/src/compiler/layout/bib_csl.rs:47`, `01_core/src/compiler/layout/bib_csl.rs:58`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Construct pure citation/bibliography caches from supplied BibEntry values, built-in styles or a supplied parsed IndependentStyle, locale and optional citation order. Preserve the Normal/Prose/Author/Year mappings and the existing unsupported-form behavior. Built-in style lookup uses the archive and the cache builder's `ieee` default; the hub decides whether a cache is requested at all. Parse supplied custom CSL XML without filesystem or URL access. Convert rendered CSL nodes into Content while preserving supported links, strong/emphasis, small caps, underline and block breaks. Serialize textual YAML scalars through total JSON-string escaping so controls, Unicode, empty and YAML-looking strings remain text. Normalize a maximal run of at least two ASCII hyphens in the pages field to one hyphen; single hyphens, Unicode dashes, and other fields are unchanged. Preserve diagnostics for malformed styles/entry conversion and the existing locale fallback.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

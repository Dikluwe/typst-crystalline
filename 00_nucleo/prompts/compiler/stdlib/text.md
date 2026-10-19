# L0 — stdlib/text
Hash do Código: a8732f7f

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/mod.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/mod.rs:1` (@prompt) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only the text module's declarations and direct native reexports. Preserve constructor, case, deco, lorem, shift, smallcaps and smartquote descendants and their existing public function paths.

Native text construction, case/replace, underline/strike/overline/highlight, lorem, sub/super, smallcaps and smartquote semantics belong to the respective owners under `00_nucleo/prompts/compiler/stdlib/text/`. Private casts and validators remain inside their unit. Tests are owned by the actual atomized test consumers rather than a historical monolithic stdlib suite.

There is no dispatch table or native wrapper in this hub. Regex lives in `00_nucleo/prompts/compiler/stdlib/foundations/regex.md`. Bindings belong to `00_nucleo/prompts/compiler/stdlib/registry/text.md`; layout belongs to layout owners. Do not translate or erase an existing step-named error literal merely because its wording is obsolete: it is a public diagnostic discrepancy, recorded for separate correction.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

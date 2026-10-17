# L0 — stdlib/foundations
Hash do Código: 3e91f683

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/mod.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/mod.rs:1` (@prompt) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only module declarations and direct reexports in foundations/mod.rs. Preserve the current visibility/signatures and paths through the stdlib façade. Declare cast, color, datetime, float, int, path, query, regex_constructor, repr, selector, str and ty; keep private/internal method-dispatch linkage as currently exposed.

Each child owns its casts, native behavior, diagnostics and defaults under `00_nucleo/prompts/compiler/stdlib/foundations/`. The datetime constructor remains in cast; datetime fields belong to datetime. Regex construction belongs to `00_nucleo/prompts/compiler/stdlib/foundations/regex.md`, not str or text. Morphological value serialization remains with `00_nucleo/prompts/compiler/eval/repr.md`.

Do not reintroduce a global len function, add constructor logic, calendar/formatting, World reads or new registry entries here. Existing string/array/dictionary len methods remain separate language operations.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

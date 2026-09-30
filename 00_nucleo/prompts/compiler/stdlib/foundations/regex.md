# L0 — stdlib/foundations/regex
Hash do Código: f6e6f18f

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/regex_constructor.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/regex_constructor.rs:16` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_regex and its local tests, not string instance methods or evaluation dispatch. Require exactly one positional Str and no named arguments. Compile eagerly through the L1 Regex entity, returning Value::Regex; reject invalid patterns, wrong types/arity and named arguments with the existing exact diagnostics, including the regex engine's error text. Preserve pure execution without a global mutable cache, new engine options or public signature changes. The global binding's established Type::Regex identity and dispatch linkage are supplied by their registry/call owners; do not revert it to a function binding.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

# L0 — stdlib/primitives-constructors/duration
Hash do Código: 19629e85

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/duration.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/primitives_constructors/duration.rs:17`, `01_core/src/compiler/stdlib/primitives_constructors/duration.rs:114` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_duration has two exclusive forms. A single positional string has canonical d/h/m/s components, optional leading sign and fractional seconds. Reject empty/malformed strings, unknown/out-of-order suffixes, wrong types or mixing positional and named forms. The named form sums integer weeks, days, hours, minutes, seconds, milliseconds, microseconds and nanoseconds, including negatives, in i128 nanoseconds. No arguments produces zero. Return Value::Duration through Duration::from_nanos, preserving units/sign/diagnostics.

Do not narrow arithmetic or compare a represented i128 total against impossible outside-i128 bounds. Retain the existing string compatibility route; do not add casts, .in(unit), .display() or component extractors. Callable-type dispatch delegates to this same pure free function.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

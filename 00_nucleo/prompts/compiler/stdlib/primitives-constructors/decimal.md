# L0 — stdlib/primitives-constructors/decimal
Hash do Código: 6d6478fc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/decimal.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/primitives_constructors/decimal.rs:19` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_decimal accepts exactly one positional Str and no named arguments, parses Decimal::from_str and returns Value::Decimal. Reject invalid strings, wrong types and wrong cardinality with existing diagnostics. Do not read World/current_file, add casts, Value variants or arithmetic. The callable-type entry delegates to this same free function without a semantic wrapper.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

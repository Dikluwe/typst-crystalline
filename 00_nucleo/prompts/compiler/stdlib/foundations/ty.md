# L0 — stdlib/foundations/ty
Hash do Código: 7d9c475b

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/ty.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/ty.rs:20` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_type accepts exactly one positional value and returns `Value::Type(value.type_of())`, not a string. Preserve `type(1) == int`, `type(int) == type`, `type(none)` returns `Value::Type(Type::None)` and `repr(type(1)) == "int"`. Missing/extra values retain the diagnostic prefix `type() requer 1 argumento`; do not translate language-visible diagnostic literals. Named arguments remain rejected through expect_no_named. The old examples omitted the current suffix `, recebeu {n}`; retain the actual diagnostic text without treating the abbreviated examples as a new parity result.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

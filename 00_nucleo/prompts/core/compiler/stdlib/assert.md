# L0 — stdlib/assert
Hash do Código: b5c2633a

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/assert.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/assert.rs:23` (pub fn native_assert); `01_core/src/compiler/stdlib/assert.rs:91` (fn values_equala: &Value, b: &Value) -> bool {); `01_core/src/compiler/stdlib/assert.rs:106` (fn assert_eq_ne_args<'a>) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own assert, assert.eq and assert.ne argument handling and result/diagnostic construction. Registry wiring belongs to `00_nucleo/prompts/core/compiler/stdlib/registry/diagnostics.md`.

assert requires one Bool condition and accepts only message as a named argument. Resolve String directly, Content through plain_text, and the existing other-value type-name fallback. Success returns None with no output. Failure is `assertion failed`, or `assertion failed: MESSAGE`. Preserve missing/invalid-condition and unknown-named diagnostics.

eq/ne require two values and use language equality, including Int/Float equality and content morphology, not a new Rust structural comparison. Success returns None. Defaults are `equality assertion failed: value LEFT was not equal to RIGHT` and `inequality assertion failed: value LEFT was equal to RIGHT`, using the current canonical repr. With a message use `equality assertion failed: MESSAGE` or `inequality assertion failed: MESSAGE`. Preserve spans, severity, hints and outer traces; no native backtrace mechanism is added.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

## Canonical language equality prerequisite

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, UTC/working tree and initial binary
are preserved in lote-a1-callbacks-reflexao-seletores-2026-10-17.
assert-prerequisite-before.json.gz confirms assert.eq(1pt,0%+1pt) and
assert.eq((1,),(1.0,)) fail in Tekt and pass at vanilla bd5ebec285.
assert.rs:91–101 duplicates a shallow comparator; operators/equality.rs already
preserves Relative/Length coercion and recursive/counter semantic equality.
Correct Box reflection returns Relative even for an absolute dimension; changing
the getter to an incorrect type to bypass assert is unacceptable.

### Contract and acceptance

Private values_equal delegates to canonical
compiler::eval::operators::equality::values_eq. Do not create another equality
relation or alter Rust PartialEq. Preserve validation, repr and messages. Use
the frozen RED cases above and an inequality control. This restricted dependency
does not correct the separate supplement/std-identity equality issue. ADR-0127
flow adds no API/phase; L0 before code, then suite and gates.

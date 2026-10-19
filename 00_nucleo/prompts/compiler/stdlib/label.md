# L0 — stdlib/label
Hash do Código: de3fed13

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/label.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/label.rs:19` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

native_label requires exactly one nonempty positional Str, rejects named arguments, and returns Value::Label equal to the code literal <name>, not Content. Reject missing/extra values, non-strings and empty names. Preserve the established Type::Label constructor binding. label(name, body) is not public language surface. Internal Content::label/label_auto association and markup attachment remain owned outside this native.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

# L0 — stdlib/foundations/repr
Hash do Código: d65301d3

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/repr.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/repr.rs:22` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only native_repr's argument validation and exposure of repr_value. Require one value and return Value::Str with the language representation from `00_nucleo/prompts/compiler/eval/repr.md`; missing arguments retain `repr() requer 1 argumento`. Do not implement serialization or render formatting here. Preserve the integration's scalar, quoted-string, unit and Content/Symbol morphology; Rust Debug is not an acceptable substitute. The historical markup, Symbol, spacing/break and PageRun decision evidence is retained in the diagnostic, not duplicated here as ownership of repr_content.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

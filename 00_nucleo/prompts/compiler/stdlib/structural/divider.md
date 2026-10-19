# L0 — stdlib/structural/divider
Hash do Código: fb0b6772

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/divider.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/divider.rs:25` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

divider accepts zero positional and zero named arguments and emits Content::divider(). Any positional fails `divider() não aceita argumentos posicionais`; any named uses the façade expect_no_named helper. Preserve pure pass-through context/World/file arguments, with no private parser. Thickness, color, spacing and drawing belong to layout; do not add cosmetic arguments silently or substitute the unrelated drawing line primitive.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

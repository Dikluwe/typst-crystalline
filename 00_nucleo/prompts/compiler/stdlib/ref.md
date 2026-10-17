# L0 — stdlib/ref
Hash do Código: 99ba5612

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/ref.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/ref.rs:23` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only native_ref's construction and validation. Require a nonempty first positional Str label name and emit Content::Ref with the name and optional explicit Content supplement. Omitted supplement remains None, allowing the referenced element's default; reject non-string names and non-content supplements with the existing contextual diagnostics. Preserve form: "normal" | "page", default Normal, rejecting other strings. Explicit supplement may suppress or override the page supplement. Numeric resolution, introspection, numbering, render defaults and legacy fallbacks belong to the layout/reference owners, not this constructor; preserve their received carrier without doing those operations here.

Contract/code discrepancy: the current native also accepts Value::Label names and Str/None supplements, and does not enforce a closed leftover-argument policy. These observed extensions are not authorization to broaden the original constructor contract. Preserve code during this sanitation and resolve the discrepancy separately before any implementation change.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

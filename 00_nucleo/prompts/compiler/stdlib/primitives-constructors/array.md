# L0 — stdlib/primitives-constructors/array
Hash do Código: 6508a7d5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:761d5adeca09f6a60ba2960f8492aa6c0d934798bbc826f341004cf993ba8736

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors/array.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/primitives_constructors/array.rs:15` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Deliberately incomplete: this unit owns only array(Bytes), not Array/Version conversions or other upstream forms; no completion step is authorized for them. Own the internal native_array_bytes using the pinned constructor ABI, with already evaluated Args and no expression reevaluation, callbacks or I/O. Public dispatch selects this route only when its first positional is Bytes.

Consume exactly that positional and reject the first causal leftover: `unexpected argument` positional, `unexpected argument: <name>` named, at occurrence span with aggregate fallback only when detached. Return Value::Array with one unsigned Int in 0..255 per byte, in unchanged order/length. Empty bytes produce an empty array; never mutate Bytes, reverse, apply endian, convert text or truncate. Defensive non-Bytes input may fail `type array does not have a constructor` at Args.span without expanding dispatch.

Do not add Array methods, public Rust API, Type/Value/Args changes, array(version), array(array), missing/named-only routes or Func identity/With expansion. Preserve productive local tests; retired externally included p1339_observation harnesses must not return or acquire substitute instrumentation without a separately specified formal cfg. This prerequisite does not establish general array or float-conversion parity.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

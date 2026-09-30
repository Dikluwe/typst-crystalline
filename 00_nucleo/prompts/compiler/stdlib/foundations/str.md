# L0 — stdlib/foundations/str
Hash do Código: 890a94d9

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/str.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/str.rs:24`, `01_core/src/compiler/stdlib/foundations/str.rs:96`, `01_core/src/compiler/stdlib/foundations/str.rs:119` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own native_str, native_str_from_unicode and native_str_to_unicode. str supports none, bool, int, float, str, auto, length, ratio, angle, UTF-8 Bytes, Label and Symbol, but rejects Color. base applies only to Int and must be 2–36. Preserve 3.0 -> "3.0", integer base 16 lowercase output, compound length formatting, and `bytes are not valid UTF-8` / `str() não suporta color`. Label becomes its internal name without angle brackets; Symbol clones its entire value, including multiple codepoints. Never substitute repr, canonical symbol name, first scalar or normalization.

str.from-unicode requires an integer Unicode scalar; reject outside the scalar range and surrogates. U+0000 is valid. str.to-unicode requires a string containing exactly one Unicode scalar and returns its codepoint; reject empty/multiple-scalar strings through the char cast. Do not use bytes or grapheme clusters. The bound string method in collections delegates here with character as first positional; preserve the same Func identity. Regex construction belongs only to `00_nucleo/prompts/compiler/stdlib/foundations/regex.md`.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

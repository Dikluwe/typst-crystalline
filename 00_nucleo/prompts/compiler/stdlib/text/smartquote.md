# L0 — stdlib/text/smartquote
Hash do Código: 9f555810

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/smartquote.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/smartquote.rs:37` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

smartquote accepts zero positionals and named double: Bool=true, enabled: Bool=true, alternative: Bool=false and quotes: Auto|string|array|dictionary=Auto. Positional error retains `smartquote() não aceita argumentos posicionais (recebeu {n})`; unknown named `smartquote(): argumento nomeado inesperado '{name}'`; wrong boolean preserves the parameter-specific bool error.

When enabled, emit a Content::SmartQuote leaf retaining explicitly supplied optional fields, not Content::Styled: public content.func() must retain smart.quote identity. When disabled, emit literal ASCII double/single quote according to double, ignoring alternative/quotes and never updating the layout quoter. Preserve omission versus explicit quotes:auto so inherited overrides can be erased. Set rules continue using Styles. Direct-field/set-chain/default resolution and localized glyph/open-close state belong to the quote/layout owners, not this native.

Use the localized-quotes owner's validation/canonicalization: strings contain exactly two Unicode grapheme clusters, arrays exactly two items, dictionaries only single/double, each Auto/string/array. Preserve `expected 2 characters, found {n} character{s}`, `expected 2 quotes, found {n} quote{s}` and `unexpected key "{key}", valid keys are "double" and "single"`. Explicit quotes take priority over alternative; absent/auto dictionary members select localized pairs. No language read, EvalContext quoter counter or mutable global state. Do not infer unmeasured content.fields()/BCP-47-region behavior from a repr wrapper that happens to be transparent. The historical public-carrier gate is archived, not newly approved by this sanitation.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.

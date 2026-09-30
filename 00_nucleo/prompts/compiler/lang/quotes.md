# L0 — lang/quotes

Hash do Código: a2383787

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lang/defaults.toml sha256:c8f920865f8895a89d9c42659c15e773e9bb2603bd614e390805f620834babd9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lang/quotes.rs`

## Consumer evidence

`01_core/src/compiler/lang/quotes.rs:57` locates `fn pair(open: &str, close: &str) -> SmartQuotePair`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Localization and validation

Own pure quote-pair lookup, validation and resolution, using the public types
owned by `00_nucleo/prompts/entities/elements/smartquote.md`.
`DEFAULT_QUOTES` is `“`/`”`; `DEFAULT_SINGLE_QUOTES` is `‘`/`’`.

Primary double pairs: pt/es/it `«`/`»`, en `“`/`”`, de `„`/`“`,
fr `«\u{00A0}`/`\u{00A0}»`. Single pairs: en `‘`/`’`, de `‚`/`‘`.
Uncovered or missing language uses the corresponding English curly pair.
German alternative pairs are single `›`/`‹` and double `»`/`«`.
Exact language lookup does not infer BCP47 regions.

`parse_smartquote_quotes(value, span)` accepts auto/none, string, array or
dictionary. Auto/none selects automatic localization. A string must have
exactly two grapheme clusters; an array must have exactly two strings.
A bare string/array overrides the double pair only. A dictionary accepts only
`single` and `double`; each member is auto/none or a valid pair.
Explicit overrides affect only their member; auto preserves its localized
alternative. Resolution does not implement nesting, opening/closing context,
apostrophes or primes.

Preserve span-anchored diagnostics, including singular/plural forms:
`expected 2 characters, found {n} character{s}`,
`expected 2 quotes, found {n} quote{s}`,
`expected string, found {type}`,
`unexpected key "{key}", valid keys are "double" and "single"`,
`expected auto, string, or array, found {type}` and
`expected auto, string, array, or dictionary, found {type}`.
Regional and nesting parity is not established by this owner.

# L0 — lexer/markup
Hash do Código: 232437b4


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/markup.rs`

## Consumer evidence

`01_core/src/compiler/lexer/markup.rs:23` locates `pub(super) fn markup(&mut self, start: usize, c: char) -> SyntaxKind`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Markup grammar

Own text, structural markers, links, labels, references, escapes and raw
mechanics. Recognize escapes/linebreaks, HTTP(S) autolinks, labels, reference
markers, shorthands, content/math delimiters, smart quotes and heading/list/
enumeration/term markers at their existing whitespace/end boundaries.
Star and underscore are delimiters only outside words; CJK scripts do not
act as Western alphanumeric word interiors for that decision.

Text groups contiguous content up to a lexical boundary. It absorbs an ASCII
space exactly when the following scalar is alphanumeric. Tabs, newlines,
Unicode whitespace and space before punctuation remain hub trivia.
Incomplete special prefixes remain text rather than fabricating links,
references or shorthands. Quotes yield SmartQuote only in this mode;
localization and contextual quote choice belong to downstream owners.

Inline raw preserves non-newline whitespace and emits RawTrimmed for newlines.
Block raw recognizes a language tag, computes minimum character-based dedent
and retains delimiter/text/trimmed nodes. Preserve unterminated-raw and escape
diagnostics. Autolinks reject unbalanced brackets; labels require nonempty
content and closing `>`; references leave likely trailing dot/colon for
adjacent text. Preserve `unclosed Unicode escape sequence`,
`label cannot be empty` and `unclosed label` diagnostics.

Acceptance covers text/trivia morphology, marker boundaries, raw node
preservation, dedent, quotes and malformed special forms without changing
Code or Math behavior.

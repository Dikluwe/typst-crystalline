# L0 — lexer/code

Hash do Código: d31bc0eb

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/code.rs`

## Consumer evidence

`01_core/src/compiler/lexer/code.rs:19` locates `pub(super) fn code(&mut self, start: usize, c: char) -> SyntaxKind`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Code-mode grammar

Own `Lexer::code`, code strings, identifiers, numbers and invalid-operator
diagnostics. The hub owns the cursor and universal trivia; consume only the
contiguous suffix after the first scalar supplied by the hub.

Recognize braces, brackets, parentheses, dollar, comma, semicolon, colon, dot
and simple operators. Compound operators win over their prefixes:
`==`, `!=`, `<=`, `>=`, `+=`, `-=`, `*=`, `/=`, `..`, `=>`.
Minus accepts ASCII hyphen and U+2212.

Identifiers start with `is_id_start` and continue with `is_id_continue`.
Recognize keywords except after a single dot or at-sign. An isolated underscore
is Underscore; other identifiers are Ident.

Numbers support binary, octal, decimal and hexadecimal bases, decimal floats,
exponents and units `pt`, `mm`, `cm`, `in`, `deg`, `rad`, `em`,
`fr` and `%`. Distinguish Int, Float and Numeric. Do not consume range
`..` or method access as a decimal separator. Preserve separate invalid-base,
hexadecimal, incomplete-float and invalid-suffix diagnostics.

Strings end at an unescaped double quote; unterminated strings report an error.
Invalid characters preserve the existing diagnostic and hints for redundant
hash, `&&`, `||`, `!` and `~=`, without accepting incompatible aliases.
Preserve `unclosed string` and the operator hints (including code-word spelling):

```text
in Typst, `and` is used for logical AND
in Typst, `or` is used for logical OR
in Typst, `not` is used for negation
or did you mean to write `!=` for not-equal?
in Typst, `!=` is used for not-equal
```

Code quotes produce Str, not SmartQuote. Acceptance checks compound precedence,
valid/invalid numerics, token text and observable diagnostics.

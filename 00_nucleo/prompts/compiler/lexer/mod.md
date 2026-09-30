# L0 — lexer/mod

Hash do Código: 33a1fe9b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:aae80d538980eeec87b884712269e3b777fe44c0dae6b3503b5fa9ee4f9ad76f

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/mod.rs`

## Consumer evidence

`01_core/src/compiler/lexer/mod.rs:14` locates `pub mod scanner;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Lexer hub

Own `Lexer<'s>`, universal trivia, token assembly and mode dispatch in
`01_core/src/compiler/lexer/mod.rs`, not the child mode grammars.
Preserve its scanner, current SyntaxMode, newline flag and pending SyntaxError;
construction, mode/set_mode, cursor/jump, newline/column and next interfaces
remain unchanged. Column is measured in characters from the last newline,
while cursor positions are byte offsets.

The parser controls mode changes; lexical handlers do not change modes.
Before mode dispatch, recognize end, whitespace, initial shebang, line comments,
nested block comments and raw outside Math. Whitespace left outside Markup
text yields Space, or Parbreak for multiple newlines in Markup. Preserve
newline tracking and error-to-SyntaxNode conversion, original token text,
offsets and diagnostic hints, including `unexpected end of block comment`.

Delegate each remaining token to the active mode exactly once:
`00_nucleo/prompts/compiler/lexer/markup.md`,
`00_nucleo/prompts/compiler/lexer/code.md` or
`00_nucleo/prompts/compiler/lexer/math.md`.
Raw mechanics live with Markup despite universal entry outside Math.
`00_nucleo/prompts/compiler/scanner.md` owns cursor safety.
Quote localization and evaluation are not lexer responsibilities.

Acceptance covers universal precedence, nested comments, lossless token
boundaries, mode isolation, newline/column behavior and pending diagnostics.
Do not infer language parity from matching tokenization code.

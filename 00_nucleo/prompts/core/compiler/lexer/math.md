# L0 — lexer/math
Hash do Código: c9ae54cd


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:8ac65bcca69910f497bdf8a35ad55ac1fdeb628ae05aca729295caf22d213a31

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/math.rs`

## Consumer evidence

`01_core/src/compiler/lexer/math.rs:20` locates `fn starts_multigrapheme_math_identifier(c: char, scanner: &Scanner<'_>) -> bool`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Math grammar

Own mathematical shorthands, atoms, delimiters, graphemes and inferred arguments.
Long arrow/relation/dot/operator shorthands precede their prefixes and yield
MathShorthand. Preserve dedicated kinds for punctuation, hash, underscore,
dollar, slash, hat, alignment points, roots and bang. Consecutive apostrophes
form one MathPrimes; double quotes reuse Code string parsing, never SmartQuote.

Parentheses have their dedicated kinds. Mathematical Opening/Closing
delimiters yield LeftBrace/RightBrace, including `[|` and `|]`.
Multigrapheme mathematical identifiers yield MathIdent, dotted accesses form
FieldAccess, and a single grapheme remains MathText. Consume complete numeric
and grapheme clusters; a numeric decimal point requires following digits.

`maybe_math_named_arg` recognizes an identifier directly followed by colon,
excluding `:=` and `::=`; underscore produces an error node.
`maybe_math_spread_arg` recognizes `..` only with a following argument and
without colliding with `...`, separators or math end. Failed inference
restores the original cursor. Universal dispatch and mode changes are not
owned here.

Acceptance preserves shorthand precedence, structured field accesses,
prime grouping, grapheme boundaries, error nodes and cursor rollback.

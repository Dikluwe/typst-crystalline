# L0 — scanner
Hash do Código: a4b620f0


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/scanner.rs`

## Consumer evidence

`01_core/src/compiler/lexer/scanner.rs:24` locates `pub struct Scanner<'a>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Cursor and matching contract

Own the pure UTF-8 scanner in `01_core/src/compiler/lexer/scanner.rs`,
inlined from unscanny (Apache-2.0, ADR-0014). A `Scanner<'a>` borrows a
string and maintains a byte cursor, initially zero, within bounds and always
on a Unicode scalar boundary.

Preserve `new`, `string`, `cursor`, `done`, `before`, `after`,
`parts`, `from`, `to`, `get`, `peek`, `at`, `scout`, `locate`,
`eat`, `uneat`, `eat_if`, `eat_while`, `eat_until`,
`eat_whitespace`, `expect` and `jump`. Views borrow the original string;
from/to are relative to the current cursor. Eat/uneat advance/retreat by
whole UTF-8 scalars. Scout/locate navigate relatively; scout(0) equals peek.
Jump snaps to an in-bounds scalar boundary; jumping beyond the end reaches
the end. Done means cursor equals string length.

The sealed `Pattern<T>` abstraction supports char, string, fixed char
arrays, char slices and the existing char/reference-char predicates.
Conditional/loop consumption respects match length and UTF-8 boundaries.
Expect panics on mismatch with the pattern's descriptive diagnostic; preserve
the existing empty-slice and closure diagnostics.

ADR-0116 permits the existing unchecked slice hot paths; bounds, snapping and
pattern-returned lengths must uphold their safety invariants. This is not
permission for unsafe code elsewhere or for caller-supplied arbitrary indices.
Lexer-specific scanner helpers may remain here. Acceptance exercises multibyte
navigation, slices, snapping, matching and failure behavior; it does not claim
byte-for-byte language parity from implementation similarity.

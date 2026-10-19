# L0 — lexer/mod
Hash do Código: 68015269


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/lexer/mode-boundaries.toml sha256:8ac65bcca69910f497bdf8a35ad55ac1fdeb628ae05aca729295caf22d213a31

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/lexer/mod.rs`

## Consumer evidence

`01_core/src/compiler/lexer/mod.rs:14` locates `pub mod scanner;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

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
`00_nucleo/prompts/core/compiler/lexer/markup.md`,
`00_nucleo/prompts/core/compiler/lexer/code.md` or
`00_nucleo/prompts/core/compiler/lexer/math.md`.
Raw mechanics live with Markup despite universal entry outside Math.
`00_nucleo/prompts/core/compiler/lexer/scanner.md` owns cursor safety.
Quote localization and evaluation are not lexer responsibilities.

Acceptance covers universal precedence, nested comments, lossless token
boundaries, mode isolation, newline/column behavior and pending diagnostics.
Do not infer language parity from matching tokenization code.


## Domain method and Rust trait boundary

### Evidence before decision

Lexer.next appears in the historical Clippy inventory of 13 warnings. The
existing signature/body were read before this decision;
`lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/before.json.gz`
preserves complete sources/L0, global hashes, HEAD, uncommitted working tree,
UTC and diffstat. Its return contract differs from the suggested trait. This
is Rust-interface comparison, not new parity evidence or a measured AI-reading
benefit.

### Exclusive responsibility and acceptance

Preserve next(&mut self) -> (SyntaxKind, SyntaxNode), explicit End tokens even
on repeated calls after EOF, errors/newline and parser-controlled mode changes.
Do not implement Iterator or change the return to Option. Correct only the
type's description to tokenizer with an explicit sentinel; next receives the
named should_implement_trait expectation.

Use only the exact attribute
`#[expect(clippy::should_implement_trait, reason="S001: tokenizer com token End explícito e modo/cursor controlados pelo parser; retorno não é Option")]`
on the named method. The Portuguese reason is machine metadata and remains
verbatim. Do not suppress the module/impl/crate, raise a threshold or extend
this exception to Styles::from_iter. Implementing the suggested trait here would
require a new contract or different semantics; the exception preserves the
existing contract. ADR-0127 continuous flow applies to metadata/documentation,
with no new API, default or phase. L0 first and restricted resealing; measure
lint RED and require GREEN with unfulfilled_lint_expectations as an error.
Preserve bodies and existing functional tests without changing expectations.
Validate workspace/build/Clippy/doc/fmt/lineage under 12 GiB, zero swap and one job.

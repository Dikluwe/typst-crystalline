# L0 — parse/markup
Hash do Código: acb5a934

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/markup.rs`

## Evidence and classification

`01_core/src/compiler/parse/markup.rs:24` (pubsuper) fn markup); `01_core/src/compiler/parse/markup.rs:39` (pubsuper) fn markup_exprsp: &mut Parser, mut at_start: bool, stop_set: SyntaxSet) {); `01_core/src/compiler/parse/markup.rs:54` (fn reparse_markup) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own markup productions, strong/emphasis, headings, list/enum/term items, references and equations. Start-of-line constructs are recognized only when at_start is true; preserve wrap_trivia and the existing indentation/newline rules. Nested brackets remain text until the corresponding nesting level.

Enter Math for dollar-delimited equations and Code after hash, using the state owner's scoped restoration. Closed empty strong/emphasis delimiters produce the existing warnings and hints; an unclosed delimiter remains an error. Grammar-specific wording, severity, hints, delimiter positions and zero-length warning morphology remain unchanged. Generic error insertion belongs to `00_nucleo/prompts/compiler/parse/parser/diagnostics.md`.

Whitespace token aggregation belongs to the lexer, not this grammar. Do not normalize Content or change token origins to reproduce an old probe. Preserve ordinary markup, references, line structures, empty/open strong/emphasis and equation CSTs.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

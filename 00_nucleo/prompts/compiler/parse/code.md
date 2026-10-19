# L0 — parse/code
Hash do Código: 257d8503

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/code.rs`

## Evidence and classification

`01_core/src/compiler/parse/code.rs:31` (pubsuper) fn codep: &mut Parser, stop_set: SyntaxSet) {); `01_core/src/compiler/parse/code.rs:38` (pubsuper) fn code_exprsp: &mut Parser, stop_set: SyntaxSet) {); `01_core/src/compiler/parse/code.rs:61` (pubsuper) fn embedded_code_exprp: &mut Parser) {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own code sequences, expression precedence/postfix chaining, embedded code, code/content blocks and balanced block reparsing. Consume expressions until the supplied stop set and recover unexpected tokens without losing progress. Preserve CST morphology and trivia.

Unary and binary precedence/associativity use the existing operator entities; do not define a competing numeric table here. Calls and directly adjacent content blocks bind through the current postfix path; field/method access and the atomic-expression restrictions remain distinct. Preserve the special `not in` expectation and existing closure/destructuring interpretation.

A code block enters Code mode; a content block enters Markup mode. Scoped mode/newline transitions restore state through `00_nucleo/prompts/compiler/parse/parser/state.md`. Reparse succeeds only for a balanced block ending exactly at the requested range. Delegate statements to `00_nucleo/prompts/compiler/parse/rules.md`, parenthesized/groups/arguments/patterns to `00_nucleo/prompts/compiler/parse/patterns.md`, markup and math transitions to their own grammar owners. Do not evaluate expressions or resolve bindings.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

# L0 — parse/math
Hash do Código: 1de5191a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/math.rs`

## Evidence and classification

`01_core/src/compiler/parse/math.rs:24` (pubsuper) fn mathp: &mut Parser, stop_set: SyntaxSet) {); `01_core/src/compiler/parse/math.rs:32` (pubsuper) fn math_exprsp: &mut Parser, stop_set: SyntaxSet) -> usize {); `01_core/src/compiler/parse/math.rs:50` (fn math_exprp: &mut Parser) {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own math expressions, attachments, fractions, roots, delimiter groups, implicit calls and math arguments. Preserve Math, MathAttach, MathFrac, MathRoot and MathDelimited morphology according to the existing token/trivia and precedence rules.

Preserve hat/underscore/primes chains, alphabetic-function recognition and implicit call behavior. Math function/root precedence remains 2 in this consumer. Retain unparenthesizing, delimiter classification and positional/named/semicolon-array distinctions. Duplicate named arguments retain their diagnostic and location. Embedded code delegates to `00_nucleo/prompts/compiler/parse/code.md`; mode/lookahead restoration belongs to `00_nucleo/prompts/compiler/parse/parser/state.md`. No evaluation or mathematical layout is owned here.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

# L0 — parse/patterns
Hash do Código: 0a2898dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/patterns.rs`

## Evidence and classification

`01_core/src/compiler/parse/patterns.rs:22` (pubsuper) fn expr_with_parenp: &mut Parser, atomic: bool) {); `01_core/src/compiler/parse/patterns.rs:90` (fn parenthesized_or_array_or_dictp: &mut Parser) -> SyntaxKind {); `01_core/src/compiler/parse/patterns.rs:148` (pubsuper) struct GroupState {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own parenthesized expression/array/dictionary disambiguation, argument and parameter groups, closures, destructuring and reassignment patterns. Preserve comma/colon/spread rules, duplicate checks, distinct named/positional keys, and the single-sink restrictions.

Speculative parsing uses the existing checkpoint/memo interface from `00_nucleo/prompts/core/compiler/parse/parser/checkpoints.md`; this grammar chooses the interpretation, not the storage/restore algorithm. Keep preceding error nodes during memo hits and restore the cursor/nodes for rejected interpretations. Recovery must preserve progress to the terminator and the existing CST form and messages. Do not introduce a reflective parser, normalize patterns into runtime values or infer a new grammar from successful implementation tests.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

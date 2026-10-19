# L0 — core/compiler/parse/mod
Hash do Código: 21b4ce8f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:b6a95a7684745e45903417a81eabc019a17eb3b8fab9fe246e8840978d5d42c8

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/mod.rs`

## Evidence and classification

`01_core/src/compiler/parse/mod.rs:25` (pub fn parsetext: &str) -> SyntaxNode {); `01_core/src/compiler/parse/mod.rs:33` (pub fn parse_codetext: &str) -> SyntaxNode {); `01_core/src/compiler/parse/mod.rs:41` (pub fn parse_mathtext: &str) -> SyntaxNode {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only the public parser façade and its local integration tests. Keep the private grammar/engine modules and the four pure entrypoints with their existing signatures.

`parse`, `parse_code` and `parse_math` create a Parser in the corresponding SyntaxMode, invoke the owning grammar, and return Markup, Code and Math roots respectively. Syntax errors remain recoverable nodes, not a new exceptional return path. `parse_anchored` dispatches by SyntaxMode and synthesizes spans for a non-detached anchor; a detached anchor preserves the parser's spans.

Grammar belongs to `00_nucleo/prompts/core/compiler/parse/code.md`, `00_nucleo/prompts/core/compiler/parse/markup.md`, `00_nucleo/prompts/core/compiler/parse/math.md`, `00_nucleo/prompts/core/compiler/parse/patterns.md` and `00_nucleo/prompts/core/compiler/parse/rules.md`. Engine types belong to `00_nucleo/prompts/core/compiler/parse/parser.md`; its children own state, checkpoint and diagnostic algorithms. The façade must not duplicate productions, precedence or recovery.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

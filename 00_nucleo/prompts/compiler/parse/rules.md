# L0 — parse/rules
Hash do Código: fc0630be

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:ffba4f0f6d7276a3beac03c1bd2bef40135d74681be616112a1e1f172d2f24b0

Layer: L1
Exclusive consumer: `01_core/src/compiler/parse/rules.rs`

## Evidence and classification

`01_core/src/compiler/parse/rules.rs:21` (pubsuper) fn let_bindingp: &mut Parser) {); `01_core/src/compiler/parse/rules.rs:52` (pubsuper) fn set_rulep: &mut Parser) {); `01_core/src/compiler/parse/rules.rs:71` (pubsuper) fn show_rulep: &mut Parser) {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own let/set/show/context, if/while/for, import/include and break/continue/return grammar productions. Wrap each statement with its existing SyntaxKind and preserve source order, colon/pattern/import/block diagnostics and hints. Keep import-list and compound-structure newline policies.

Expressions and blocks delegate to `00_nucleo/prompts/compiler/parse/code.md`; arguments/parameters/patterns delegate to `00_nucleo/prompts/compiler/parse/patterns.md`. This unit produces CST only: no evaluation, name resolution, import I/O or runtime control-flow execution.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

# L0 — compiler/parse/parser/state
Hash do Código: 5932cfa0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/parse/core.toml sha256:b6a95a7684745e45903417a81eabc019a17eb3b8fab9fe246e8840978d5d42c8

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/parse/parser/state.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own parser creation/finalization, lookahead queries, Marker Index/IndexMut,
consumption, trivia, CST wrapping and the lexer bridge. The current token is
outside nodes. Preserve source offsets and complete text, clamp wrapping
markers before trivia, scope SyntaxMode/AtNewline transitions and restore fake
End tokens on exit. Preserve ContextualContinue and StopParBreak rules and
RequireColumn's column <= min_col comparison. Preserve InvalidHexNumber and
InvalidUnicodeCodepoint lexer errors across mode exit. Do not implement grammar
productions, checkpoint algorithms or new diagnostics. Verify scoped newline
restoration and marker access locally.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

# L0 — entities/syntax-node
Hash do Código: dc567ae6

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_node.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Declare private errors and representation modules. Explicitly reexport
SyntaxNode, SyntaxError, SyntaxErrorKind, LinkedNode, LinkedChildren and Side
through entities::syntax_node. NumberingResult/Unnumberable remain pub(crate).
Do not own internal node representations, diagnostic algorithms or navigation.
Own inline facade tests for public paths, UTF-8/trivia, bidirectional iteration,
find/numbering, spanless equality and clone isolation. Children remain private.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

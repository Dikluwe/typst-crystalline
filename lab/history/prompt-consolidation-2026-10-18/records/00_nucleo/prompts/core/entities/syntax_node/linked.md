# L0 — entities/syntax-node/linked
Hash do Código: a556bb69

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_node/linked.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own LinkedNode, Side, LinkedChildren and navigation implementations.
LinkedNode retains private node reference, Option<Rc<Self>> parent, index and
offset. Do not substitute Arc parents, impose Send or introduce global state.
Remain a private descendant of representation so find accesses private
NodeKind/InnerNode without widened fields or new getters. Offsets and ranges
are byte-based. Children retain indices, parents, front/back boundaries and
Iterator/DoubleEndedIterator/ExactSizeIterator/size_hint behavior under mixed
next/next_back. Ordinary sibling traversal skips trivia; with_trivia traversal
does not. Preserve Before/After behavior at edges, ends and empty leaves.
Find prunes by numbering and compares full Span. Preserve get/Deref/Debug,
parent/parent_kind, siblings/kinds, leaf_at, prev/next_leaf and extreme-leaf
behavior. Tests cover nested leaf traversal, trivia and UTF-8 byte ranges.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

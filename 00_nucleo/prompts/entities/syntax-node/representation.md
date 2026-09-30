# L0 — entities/syntax-node/representation
Hash do Código: a91e664b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_node/representation.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own SyntaxNode, closed NodeKind (Leaf/Inner/Error), LeafNode, InnerNode and
ErrorNode and the cohesive SyntaxNode implementation: construction, reads,
tree errors, Arc::make_mut COW, spanless equality, editing, counts, numbering,
Debug and Default. Leaf is inline, Inner/Error use Arc and text uses SyntaxText.
Internal fields and enum stay private. Linked navigation is a private child,
not a sibling; reexport its types to the facade. Receive diagnostic types from
the errors consumer. Do not define their behavior here.

Leaf/inner constructors reject Error kinds in debug; placeholder Error panics.
Length counts bytes; inner nodes aggregate descendant/error caches. Inner read
text is empty and into_text concatenates children. Errors/hints/synthesize and
editing preserve clone isolation. spanless_eq ignores only spans and preserves
kind/message/hints/text distinctions. Preserve numbering failure on insufficient
intervals, replace_children/update_parent boundaries, caches and spans of equal
edge children. Preserve Clone/Eq/PartialEq/Hash behavior. Local tests cover COW
cache updates and equal-edge spans.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

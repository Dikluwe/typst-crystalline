# L0 — entities/syntax-node/representation
Hash do Código: e51d3ff0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/syntax_node/representation.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Evidence before the nominal equality decision

At reviewed HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`01_core/src/entities/syntax_node/representation.rs:27-34` defines private
NodeKind with Leaf, Inner and Error. SyntaxNode::spanless_eq at lines 191-198
matches `(&self.0, &other.0)`: the three same-kind pairs delegate to their
existing representation-specific comparisons; its open complement returns
false. The V16-082 inventory and causal ledger in
`lab/results/v16-cause-analysis-2026-10-19/` agree with the consumer and this
owner's pre-edit hashes. Leaf comparison at lines 362-364 retains kind/text;
Inner at lines 461-473 retains structure/caches and recursive equality;
Error at lines 596-598 retains text and the error owner's spanless comparison
(`01_core/src/entities/syntax_node/errors.rs:66-70`). This is inspection of
the current Rust syntax-tree contract, not a new language parity measurement
or a claim about language Content equality.

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

Keep the exact three same-kind delegates in SyntaxNode::spanless_eq. Name
the six heterogeneous pairs explicitly: (Leaf, Inner), (Leaf, Error),
(Inner, Leaf), (Inner, Error), (Error, Leaf) and (Error, Inner); each returns
false. Identical visible text or empty children do not equate unlike syntax
representations. This closed 3×3 decision ignores spans only; it does not
ignore kind, byte text, child order/structure, error category/message/hints
or cached structural distinctions. Preserve both operand directions.

Require a new decision when NodeKind grows on either tuple axis, including
new/new. Do not substitute discriminant equality, a hash comparison,
if-let/matches! or a helper carrying the same open false fallback. Do not
duplicate equal bodies merely to hide V19 or change Eq/PartialEq/Hash,
syntax representation, COW storage, numbering/editing or public paths.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

Preserve existing integration tests `spanless_eq` and
`representation_clone_and_spanless_equality_are_preserved` in
`01_core/src/entities/syntax_node.rs`, and this consumer's
`editing_clone_updates_its_caches_without_mutating_original` and
`replacement_retains_spans_of_unchanged_edges`. The error owner's
`spanless_error_equality_keeps_hints_and_kind` remains an integration control,
not test ownership transferred here.

Local controls must cover all nine Leaf/Inner/Error pair categories.
The six heterogeneous pairs stay false even when they expose identical
text. Same-kind structural equals with different synthesized spans stay
true, while changed leaf kind/text, inner child order/structure and error
category/message/hints stay unequal. Preserve clone isolation and the
existing equal-edge span behavior. These are current syntax-tree API
observations, not a replacement oracle for Typst Content equality.

In a disposable complete workspace only, extend copied NodeKind with a
sentinel. Structural baseline lacks an E0004 obligation at this selected
match; the nominal correction must yield attributable E0004 in
the tuple match at
`01_core/src/entities/syntax_node/representation.rs:192-197`, with coverage
of new/old, old/new and new/new. Function-level errors elsewhere are insufficient.
Other matches may also fail on the added variant, but those failures alone
do not prove this match closed. Do not alter the productive enum, use stubs,
suppress V16, change scan scope or weaken expected test values. Review any
informative V19/V20 notes without treating one grouped arm as every executed
case. ADR-0127 continuous flow applies to this internal preservation change:
L0 first, no public/default/phase/compatibility change, pure L1 and existing
1:1 ownership. Valid V15/V26 precedes restricted resealing
and regression/build/lint gates.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

### Checked numbering failure

The pre-edit source inventory and snapshot under
`lab/results/panic-elimination-2026-10-22/baseline-reconciled/` record
unconditional Span::from_number unwraps and unchecked subtraction, division,
addition, multiplication and child slicing in the existing numbering functions.
Before Source can map failure to a diagnostic, both numbering implementations
must return existing Unnumberable for an invalid midpoint, reversed/empty or
insufficient interval, invalid child interval, zero/inconsistent descendant count
or arithmetic overflow. Preserve successful numbering values, FULL layout,
existing equal-edge renumbering, COW/caches and partial-mutation contract on Err.
Use checked operations/borrows and existing NumberingResult; do not saturate or
replace invalid spans with detached values. Add direct RED controls without
allocating enormous trees, retaining all valid/clone/edge tests. Leaf/inner kind
and placeholder fallibility is a separate pending contract; this bounded change
does not claim the entire syntax representation panic-free.

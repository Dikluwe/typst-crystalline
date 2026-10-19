# Prompt L0 — `entities/elements/math_op` — `MathOpElem`
Hash do Código: a5d0c14d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_op.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

`lab/corpus/docs/math/op.typ:7–27` preserves documentation examples;
`lab/typst-original/crates/typst-library/src/math/op.rs` defines text/limits,
and `lab/typst-original/crates/typst-layout/src/math/ir/resolve.rs::resolve_op`
consumes limits. Historical Content anchors 1655,1568,2151,2637,1838 describe
text, emptiness, maps and equality. Actual layout use belongs to
`00_nucleo/prompts/core/compiler/math/layout/attach.md` (historical attach.rs:78).

## Contract and acceptance

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathOpElem {
    pub text: Content,
    pub limits: bool,
}
```

Preserve Content::MathOp(Arc<MathOpElem>) and
Content::math_op(text: Content, limits: bool). Plain text delegates to text;
is_empty defaults false. map_content recurses into text, propagates errors,
and preserves limits. map_text is terminal and clones the same carrier.
Field/kind/payload default None, non-locatable. Derived equality compares both
text and limits. The earlier phrase "limits does not alter eq" cannot override
this explicit two-field equality: it described the unchanged dispatch, not
permission to ignore the flag. Layout uses limits to affect the parent
MathAttach; no layout logic belongs to this entity. Verify text transparency,
recursive child map, terminal text map, flag preservation and equality.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_op.rs:30` preserves traversal but materializes the element even when
descendants do not change. `01_core/src/entities/content/traversal.rs:111–192`
unconditionally wraps results from these owners in Some. This is mechanical
cost; language behavior and element semantic identity remain preserved.
Refutable inference: propagating explicit absence of change reduces allocations
and temporary retention. Probes and timing controls may refute that gain.

Internal refinement under ADR-0127: provide only a crate-private
map_content_if_changed helper for elements already owned by this consumer.
Return None only when no actually visited child was replaced; childless
terminals return None. Transform children once in the existing map_content
order and stop at the first error. A callback Some, even containing equal
content, always means change. Preserve every field, value, explicit presence,
omitted/none/present slot, flag, span, and metadata item. Do not use equality,
hash, or address as a certificate. Allocate vectors/rows only after the first
changed child/row; unchanged siblings retain shared content. Public signatures
and trait methods remain unchanged, with internal adaptation to this proof;
map_content_pre and map_text retain their previous contracts. This refinement
does not expand any ratification of historically proposed contracts.

Specific logic remains with this owner; Content's exhaustive match only
delegates statically. No vtable, new enum/fields, global state, I/O, or new
phase. Acceptance requires RED→GREEN for reuse, order, errors, equal-value
replacement, optional fields/slots, and metadata; core suite, builds and lint;
text/morphology/render/query qualification against the previous release and
pinned vanilla; and paired allocation/timing measurements with provenance.
Do not infer a universal improvement from isolated cases.

The shared proof of absence of change follows the pinned nucleus claims.
This prompt and its sole consumer still own the algorithm, transformed fields,
visit order, and their specific preservation obligations.

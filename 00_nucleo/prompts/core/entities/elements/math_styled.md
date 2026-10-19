# Prompt L0 — `entities/elements/math_styled` — `MathStyledElem`
Hash do Código: 8c315d27

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_styled.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

ADR-0102/0103 specify the variant mechanism; historical Content anchors
1642,2145,2636,1826 record text/maps/equality. The original math-layout handler
and apply_math_style used these fields with outer-wins Option::or composition;
that boundary behavior remains with its layout owner, not implemented here.

## Contract and acceptance

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathStyledElem {
    pub kind: Option<MathStyleKind>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub body: Content,
    pub cramped: Option<bool>,
}
```

Preserve grouped fields and Content::MathStyled(Arc<MathStyledElem>).
Plain text delegates to body; is_empty defaults false. map_content recursively
transforms body and preserves all four optional style fields. map_text is
terminal, cloning the same carrier. Field/kind/payload default None;
non-locatable. Derived equality compares all five fields through structural
Arc equality. Verify recursive body mapping with flags preserved, terminal
text map, equality and the unchanged math-layout suite; the historical count
of roughly 58 tests is not a current suite-size claim. Do not alter layout
composition while changing carrier access.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_styled.rs:34` preserves traversal but materializes the element even when
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

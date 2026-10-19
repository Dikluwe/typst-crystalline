# Prompt L0 — `entities/elements/math_root` — `MathRootElem`
Hash do Código: 01722197

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_root.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence and approval

Historical Content anchors 1625,1568,2107,2623 specify text, emptiness and maps.
The owner explicitly approved causal index presence on 2026-09-16 under
ADR-0127, as recorded in the original specification.

## Contract

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathRootElem {
    pub index: Option<Content>,
    pub radicand: Content,
    pub index_explicit: bool,
}
```

Preserve Content::MathRoot(Arc<MathRootElem>) and
Content::math_root(index:Option<Content>, radicand:Content, index_explicit:bool).
Index presence has three states: omitted None/false (sqrt), explicit none
None/true, and supplied Some(_)/true including empty content. The bit changes
neither text nor geometry but survives clone/hash/equality/walkers as causal
morphology. Derived equality compares all three fields.

Text is `sqrt({radicand text})` for None, otherwise
`root({index text}, {radicand text})`. is_empty defaults false. map_content
recurses through optional index using transpose/error propagation, then
radicand, preserving index_explicit. map_text is terminal. Field/kind/payload
default None, non-locatable. Public math.sqrt(radicand:Content) belongs to stdlib
and produces the same None-index carrier, distinct from numeric calc.sqrt;
no reverse evaluator dependency belongs here (ADR-0109).

Verify both text forms, optional-index preservation and ordered child mapping,
terminal text map, structural equality, and all three causal presence states.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_root.rs:35` preserves traversal but materializes the element even when
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

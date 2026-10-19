# Prompt L0 — `entities/elements/boxed` — `BoxedElem`
Hash do Código: 8fc0fef5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/boxed.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The archived extraction classified boxed content as an element-shaped inline
container, not a primitive. Historical Content anchors 1768,1635,2054,2235,
1904 establish text, emptiness, maps and equality. The later fractional width
extension supports the language morphology returned by outline.entry.inner.

## Contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BoxedElem {
    pub body: Content,
    pub width: Option<Length>,
    pub height: Option<Length>,
    pub inset: Sides<Length>,
    pub baseline: Length,
    pub outset: Sides<Length>,
    pub radius: Corners<Length>,
    pub clip: bool,
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub width_fraction: Option<f64>,
}
```

Preserve Content::Boxed(Arc<BoxedElem>) and the five-argument historical helper
Content::boxed(body,width,height,inset,baseline), which defaults the remaining
original fields and sets width_fraction None. Full-field construction uses
Content::Boxed(Arc::new(BoxedElem { ... })) rather than losing those fields.
Historical constructors/literals use width_fraction None; the owner-local
outline helper may use Some(weight), and native_box(width:Nfr) is covered by
its original public ADR-0127 gate, not newly authorized here. In valid public
construction width and width_fraction are mutually exclusive. Preserve weight
sign and magnitude in walkers, equality and hash without normalization.

Plain text and emptiness delegate to body. Both maps recurse into body and
preserve every other field. Field/kind/payload retain defaults; non-locatable.
Derived equality compares every field, including the later fractional one.
Manual Hash uses Debug because Length/Sides/Corners/Color/Stroke contain f64;
retain the signed-zero caveat. Verify full metadata/fraction preservation,
partial-helper defaults, recursive maps, body transparency and equality.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/boxed.rs:53` preserves traversal but materializes the element even when
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

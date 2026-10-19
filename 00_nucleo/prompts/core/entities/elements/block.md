# Prompt L0 — `entities/elements/block` — `BlockElem`
Hash do Código: 98716040

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/block.rs`
**Architecture:** ADR-0105 model D, nonlocatable container, related to Boxed.

## Evidence before relative/fractional dimensions

Historical compiler/stdlib/layout.rs:861-874 reduced Ratio to Length::ZERO;
native_block:1535-1539 used it for height. In two profiles, After appeared
upstream page1/yMin7.404 and local page0/yMin42.230; lone/control matched within
each product, isolating lost height rather than cardinality/metadata.
Upstream container.rs:257-284,474-505 retains relative height as Sizing.
The relative-height slice explicitly deferred relative width/fractional height.
The subsequent measurement found upstream width Smart<Rel<Length>> and height
Sizing::{Auto,Rel,Fr}; local Option<Length> width and Option<Rel<Length>> height
could not carry both percentages and fractions.

## Carrier and preservation contract

The final listed carrier is:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct BlockElem {
    pub body:      Content,
    pub width:     Option<Rel<Length>>,
    pub height:    Option<BlockHeight>,
    pub inset:     Sides<Length>,
    pub breakable: bool,
    pub outset:    Sides<Length>,
    pub radius:    Corners<Length>,
    pub clip:      bool,
    pub fill:      Option<Color>,
    pub stroke:    Option<Stroke>,
    pub spacing:   Option<Length>,
    pub above:     Option<Length>,
    pub below:     Option<Length>,
    pub sticky:    bool,
}
```

BlockHeight is Relative(Rel<Length>) or Fractional(f64). None means auto;
absolute relative lengths have rel0, pure percentages abs0, mixed values keep
both. Construction boundaries reject negative/nonfinite fractional weights.
Equality/hash distinguish variant/payload. Never encode percentages or fr
through em,NaN,infinity,sign or another sentinel.

The original relative-height stage had Option<Rel<Length>> and explicitly
excluded fr; the later proposed schema above supersedes that type only within
its approved scope. The archive does not provide an explicit public-gate
receipt for that later proposal. Preserve its full requirements and approval
provenance gap; implementation or later reuse alone is not ratification.
Its finite-region/direct-Sequence distribution was explicitly incomplete:
transparent wrappers,columns,multiregion were assigned a separate follow-up
(original P1375), otherwise Unknown with no acceptance credit.

Content::Block holds Arc. Existing Content::block(body,width,height,inset,
breakable) covers only five fields and defaults outset/radius/clip/fill/stroke/
spacing/above/below/sticky. Full-field reconstruction uses Arc::new(BlockElem)
and preserves every field, not the narrow convenience constructor.

plain_text and is_empty delegate to body. Both maps recurse only into body,
preserving the other 13 fields. get_field/kind/payload retain defaults;
introspection descends into body without a Block payload. PartialEq compares
all 14 fields. The original manual Hash uses Debug because of floating fields
and explicitly warns about -0/+0 mismatch. Retain this unresolved hash/equality
caveat rather than silently endorsing a new canonical algorithm.

## Acceptance and limits

Preserve absolute/percentage/mixed values through both maps/equality/hash and
all full-field reconstructions. Keep nonlocatability and current text/emptiness.
Dimension changes are public ADR-0127 gates; this cleanup neither adds API nor
claims the finite-region slice closes wrappers/columns/multiregion.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/block.rs:65` preserves traversal but materializes the element even when
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

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/block.md` preserves the original evidence and chronology.

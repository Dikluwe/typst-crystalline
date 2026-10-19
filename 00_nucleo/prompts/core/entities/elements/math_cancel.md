# Prompt L0 — `entities/elements/math_cancel` — `MathCancelElem`
Hash do Código: e56b151f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_cancel.rs`
**Architecture:** ADR-0105 model D, nonlocatable structural math.

## Evidence and supersession

The old body-only carrier at math_cancel.rs:18-21 was explicitly replaced by
the full payload/callback realization. Historical upstream a51e02804
math/cancel.rs:18-115 declares body,length,inverted,cross,angle,stroke,background;
angle accepts angle/function/auto. The 2026-08-31 probes establish that lexical
field order and diagnostic domains, with omitted/explicit repr separately
owned. Do not resurrect a second body-only element or callback runtime.

## Complete contract

```rust
pub enum MathCancelAngle {
    Auto,
    Angle(Angle),
    Func(Func),
}

pub struct MathCancelElem {
    pub body: Content,
    pub length: Rel<Length>,
    pub inverted: bool,
    pub cross: bool,
    pub angle: MathCancelAngle,
    pub stroke: Option<Stroke>,
    pub background: bool,
    pub span: Span,
    pub explicit: MathCancelExplicit,
}
```

Defaults: length100%+0.3em,inverted=false,cross=false,angle=Auto,stroke=None,
background=false. None stroke exclusively means a MATH-derived line using
0.05em thickness and active text paint, never no line. Explicit Stroke retains
all fields. Cross wins geometrically over inverted but both remain observable
arguments/set-rule data.

Span anchors late callback errors and is not a language field. Explicit bits
separately track supplied length/inverted/cross/angle/stroke/background in
call/set rule; they do not alter layout but preserve explicit defaults in
morphology/identity/repr. Compatible Content::math_cancel(body) uses detached
span/defaults/no bits. The native path uses Args::span and exact supplied bits.
The full typed Content constructor transports seven values,span,presence
without reexecuting or reinterpreting callbacks.

Equality/hash ignore span and include all seven language values plus bits;
the later generic 'all fields' sentence cannot override this explicit span
exclusion. Use existing canonical floating-scalar conventions for Rel/Angle/
Stroke, never strings or dropped fields to obtain derive(Hash).
Content::MathCancel holds Arc; plain_text delegates to body,is_empty defaults
false,map_content recurses into body and preserves metadata,map_text remains
terminal. get_field/kind/payload retain nonlocatable defaults unless separately
specified by their owners.

## Callback and acceptance boundary

MathCancelAngle::Func uses the existing pure transcript owned by
`00_nucleo/prompts/core/compiler/math/layout/callbacks.md` and realized between
passes by the infrastructure pipeline. Do not execute callbacks inside the
entity/stdlib/layout,ignore them or reinterpret as Auto in exported documents.
Content construction,structural-math casts,cancel geometry and repr retain
exclusive ownership. Acceptance preserves transparent text,recursive body,
terminal math text mapping,all values/presence and canonical identity; the
old body-only structural test is superseded only by the explicit full payload.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_cancel.rs:109` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_cancel.md` preserves the original evidence and chronology.

# Prompt L0 — `entities/elements/math_attach` — `MathAttachElem`
Hash do Código: 557d5cb1

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_attach.rs`
**References:** ADR-0105, ADR-0107, ADR-0129.

## Attachment contract

MathAttachSlot derives Debug/Clone/PartialEq/Hash with Omitted,
ExplicitNone and Present(Content). MathAttachElem derives the same traits and
has public base:Content plus t,b,tl,bl,tr,br:MathAttachSlot.
Content::MathAttach(Arc<MathAttachElem>) is the sole payload;
Content::math_attach(base,t,b,tl,bl,tr,br) is canonical construction.

Omitted means absent argument; ExplicitNone means Typst none; Present includes
Content::Empty for []. Never replace these three morphological states with
Content/text/span sentinels or heuristics. sup() prefers Present tr then
Present t; sub() prefers Present br then Present b. Omitted/none never blocks
fallback.

plain_text concatenates ^tl _bl base ^t _b ^tr _br, including only Present
slots. map_content recursively transforms base and Present content while
preserving all states. map_text is terminal and clones the element. is_empty,
get_field, element_kind and to_payload preserve nonlocatable defaults.
Equality/hash include base and all six slots.

## Acceptance

Syntax and math.attach share the payload. repr omits Omitted, prints none for
ExplicitNone and [] for present empty content. Layout materializes only
Present, so empty markup still reserves SpaceAfterScript; omission and none
are layout-equivalent but remain morphologically distinct.

## Prime carrier

The historical upstream attachment source defines PrimesElem with public
count:usize; text glyphs alone cannot expose its identity/count/equality.
The archived contract records MathPrimesElem as implemented in this cohesive
owner. Preserve public count:usize and Debug/Clone/PartialEq/Hash.
get_field("count") yields its integer; all other fields are absent.
plain_text uses U+2032/U+2033/U+2034/U+2057 for counts 1–4, repeats U+2032
above 4 and is empty for 0. There are no children; both maps preserve count and
identity. No geometry here and no representation as MathText.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_attach.rs:45` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_attach.md` preserves the original evidence and chronology.

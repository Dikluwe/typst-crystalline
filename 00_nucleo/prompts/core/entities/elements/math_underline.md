# Prompt L0 — `entities/elements/math_underline` — `MathUnderlineElem`
Hash do Código: 4b8e4da4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_underline.rs`

## Evidence before identity contract

Historical upstream a51e02804, math/underover.rs:4-14, separates body-only math
underline from text underline with stroke/offset/extent. The 2026-08-31 probe
found math.underline == underline false. Partial visual equivalence never
licenses an identity alias. Upstream also separates body-only math Overline
from both math Underline and textual overline.

## Ratified contract

MathUnderlineElem has public body:Content and derives Debug/Clone/PartialEq/Hash.
It is structural, nonlocatable math. plain_text delegates to body; map_content
recurses and reconstructs MathUnderline; map_text is terminal; equality/hash
include body and introspection uses nonlocatable defaults.
Content::math_underline(body) is the public constructor.

math.underline requires exactly one positional Content, with canonical string
to Content cast. Missing body reports `missing argument: body`; an uncastable
value reports `expected content, found <tipo vanilla>`; a second positional
reports `unexpected argument`; any named reports `unexpected argument: <nome>`.
The placeholders retain the historical diagnostic convention. No ignored args.

Do not reuse textual Content::Underline, its stroke/offset/extent fields or
function identity. Geometry belongs to the math underline layout owner and
the Content variant to its owner. math.underline([x]) retains MathUnderline,
plain text x, one body and public repr underline(body: [x]); equality with
the text underline function stays false.

The 2026-09-18 ADR-0127 approval adds MathOverlineElem { body:Content } in
this cohesive owner. plain_text delegates; map_content descends/reconstructs
MathOverline; map_text preserves the variant; equality/hash include body.
Never reuse textual Content::Overline or infer identity merely from a visible
rule above content. The archived contract records both slices as implemented.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_underline.rs:33` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_underline.md` preserves the original chronology and evidence.

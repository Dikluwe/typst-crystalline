# Prompt L0 — `entities/elements/math_frac` — `MathFracElem`
Hash do Código: 73d1c514

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_frac.rs`
**Architecture:** ADR-0105 model D, nonlocatable math.

## Evidence before fraction contract

The 2026-08-22 measurement at HEAD 781b207b4a5d with uncommitted tree found
crystalline binom represented as MathMatrix with 13.156pt baseline spacing;
upstream measured 14.993pt and used frac's FractionItem with line=false
(ir/resolve.rs:715-759). Empirical matrix padding is not the specified repair.
Later pinned math/frac.rs:24-123 defines num/denom, public vertical/skewed/
horizontal style and two internal outer-parenthesis-removal bits, beyond
the earlier local num/den/line carrier. These are historical observations.

## Existing carrier

MathFracElem derives Debug/Clone/PartialEq/Hash with public num:Content,
den:Content and line:bool; Content::MathFrac carries Arc.
Content::math_frac(num,den) sets line=true; internal math_frac_unlined sets
false for binom, wrapped in MathDelimited('(',...,')'). line contributes to
morphology/equality. The original public-field change required ADR-0127 approval;
this consolidation introduces no additional approval.

plain_text is format!("({})/({})",num.plain_text(),den.plain_text()).
is_empty defaults false. map_content recurses into numerator then denominator
and preserves line; the historical abbreviated reconstruction omitted line
and does not override field preservation. map_text is terminal and clones.
get_field/element_kind/to_payload default None. Equality/hash include num,
den and line. Acceptance checks text format, both recursive maps, terminal
map_text and structural equality.

## Complete fraction proposal and approval provenance

The archived complete-carrier proposal remains distinct from the initial
carrier: MathFracStyle::{Vertical,Skewed,Horizontal}, optional explicit style,
and num_deparenthesized/denom_deparenthesized. Omitted style is causally distinct
from explicit Vertical. line remains solely for internal binom; when false,
style/bits neither convert it into public frac nor restore its bar.
map_content/equality/reconstruction preserve every field, transforming only
num and denom. The source spells the old field den and proposed prose denom;
retain that naming reconciliation rather than silently inventing a public rename.

The original header labels this proposal pending ADR-0127. An implementation
or later container-reuse optimization alone does not prove public ratification;
resolve the approval receipt before promoting the proposal's status. Preserve
its complete requirements and present the provenance gap explicitly.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_frac.rs:39` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_frac.md` preserves the original evidence and chronology.

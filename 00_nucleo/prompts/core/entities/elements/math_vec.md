# Prompt L0 — `entities/elements/math_vec` — `MathVecElem`
Hash do Código: 880b14e0

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_vec.rs`

## Evidence before contract

The historical crystalline eval/math.rs:1030-1062 degraded vec to MathMatrix.
Upstream a51e02804, math/matrix.rs:18-68, defines a distinct VecElem with
delimiters, horizontal alignment, relative gap and variadic children.
2026-08-31 probes confirmed zero children preserve vec(children: ()) identity.
The archived contract records this slice as implemented; this is not new parity
certification against the current pin.

## Public contract

MathVecElem has public children:Vec<Content>, delim:(char,char), align:HAlign,
gap:Rel<Length>, and explicit:MathVecExplicit. Defaults are ('(',')'), Center,
and Rel { rel:0, abs:0.2em }. Explicit has one bit per delim/align/gap named
argument. Bits do not alter layout but participate in morphology/identity so
explicit defaults survive repr. The absent-delimiter sentinel follows the
typed matrix/cases convention; none never becomes a glyph. Start and End
remain distinct payload values.

This structural math element is nonlocatable. plain_text concatenates children
in order; map_content recursively transforms each and preserves delim/align/gap
and presence; map_text is terminal. Equality/hash include every field under
canonical scalar rules, including presence. Zero children never normalizes to
empty Content or matrix. The canonical constructor transports all fields;
the Rust convenience without named arguments marks all three bits omitted.

Gap percentage remains unresolved data. Geometry belongs to the math vec
layout owner and its region-gap contract; using em as the percentage base is
forbidden. Casts/errors belong to structural-math and eval-math owners.

## Language acceptance

Preserve vector identity, child cardinality/order, inferred delimiters,
alignment and gap. Default repr is vec(children: ()) or
vec(children: ([a], [b])). Present named arguments precede children in order
delim/align/gap. Delimiters normalize to pairs and gap to relative length:
delim:"[" renders delim:("[","]"); gap:1em renders gap:0% + 1em.
Singleton children retain the comma.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_vec.rs:60` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_vec.md` preserves the construction chronology.

# Prompt L0 — `entities/elements/math_delimited` — `MathDelimitedElem`
Hash do Código: 1a006427

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_delimited.rs`
**Architecture:** ADR-0105 model D, nonlocatable math.

## Mechanical delimiter contract

MathDelimitedElem derives Debug/Clone/PartialEq/Hash with public open:char,
body:Content,close:char. Content::MathDelimited holds Arc;
Content::math_delimited(open,body,close) preserves its signature.
plain_text is format!("{}{}{}",open,body.plain_text(),close);
is_empty defaults false. map_content recurses only into body, preserving
open/close; map_text is terminal and clones. get_field/kind/payload default
None. Structural equality/hash include all three fields. Acceptance verifies
that text order, recursive body-only map and terminal map_text.

## Ratified public lr carrier

Pinned upstream LrElem has size:Rel<Length>,body:Content; the mechanical
open/body/close carrier cannot preserve unfenced lr, exact variadic body,
cascade and explicit size presence. Human approval 2026-09-17 authorizes
separate MathLrElem with body:Content,size:Rel<Length>,size_explicit:bool,
deriving Debug/Clone/PartialEq/Hash over all three fields.
plain_text delegates; map_content transforms only body and preserves size/
presence exactly; map_text stays terminal. Casts,positional joining,cascade
and stretch remain outside this entity. Do not infer lr from delimiter equality
or alter MathDelimited's existing mechanical constructor.

Field projection always exposes stored body, and size as Value::Relative only
when size_explicit. Defaults/geometric cascade cannot manufacture presence.
This applies to every MathLr construction including math.abs, without abs-only
fields or changes to MathDelimited.

## Separately ratified mid carrier

Pinned MidElem has one required positional body, preserves repr identity and
participates in nearest-lr resolution, distinct from a bare body or mechanical
delimiters. MathMidElem { body:Content } derives Debug/Clone/PartialEq/Hash;
plain_text delegates, map_content transforms only body, map_text is terminal.
Casts, Relation class and stretching belong to eval/layout.

`lab/history/construction/records/00_nucleo/materialization/typst-passo-1437.md:141`
records explicit owner confirmation after proposal commit
b0f48017d29f476357279a678aea3ff2e4c9df70, with implementation 2026-09-17 under
executed_without_isolation_attestation. It supersedes the stale pending gate
without inventing isolation evidence or broadening the approval.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_delimited.rs:65` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_delimited.md` preserves the original evidence and chronology.

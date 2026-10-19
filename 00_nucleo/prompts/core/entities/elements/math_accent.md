# Prompt L0 — `entities/elements/math_accent` — `MathAccentElem`
Hash do Código: da44d952

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_accent.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

The earlier carrier stored accent as Content and could not establish a single
normalized codepoint, orientation, relative size, dotless, or explicit presence.
The recorded upstream comparison uses normalized Accent(char), size default
100%, and dotless true. Historical Content anchors 1645,1568,2131,2631,1831
support the earlier carrier's projection/traversal/equality.

## Earlier carrier contract

The original MathAccentElem has public base:Content and accent:Content,
deriving Debug/Clone/PartialEq/Hash, with Content::MathAccent(Arc<...>) and
Content::math_accent(base:Content, accent:Content). Text concatenates base and
accent text; is_empty defaults false; map_content recursively transforms both;
map_text is terminal. Field/kind/payload default None, non-locatable. Equality
compares both fields. Its acceptance requires those projections/maps/equality.

## Complete carrier proposal and approval-evidence gap

The later complete contract specifies base:Content, accent:char,
size:Rel<Length>, dotless:bool and public MathAccentExplicit { size, dotless }
with Copy/Eq/Hash. Element observational equality/hash include values and
presence bits with the established relative-carrier float zero normalization.
map_content visits only base; map_text remains terminal. Plain text concatenates
base text and canonical accent codepoint. All reconstructions preserve values
and presence. These clauses explicitly replace the two-Content model within
the proposed scope; they must not be combined into a hybrid schema.

The original labels the replacement proposed under ADR-0127. The construction
record `lab/history/construction/records/00_nucleo/materialization/typst-passo-1429.md`
retains that gate; the following cases record describes accent as certified,
but this review has not located a direct human ratification receipt. Preserve
the complete proposal and its acceptance, with approval provenance unresolved.
Current code or extraction work alone does not supply that approval. The later
internal reuse refinement below does not expand the proposed public scope.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_accent.rs:70` preserves traversal but materializes the element even when
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

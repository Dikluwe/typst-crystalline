# Prompt L0 — `entities/elements/math_stretch` — `MathStretchElem`
Hash do Código: d2670e78

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_stretch.rs`

## Ratified public carrier

The owner explicitly approved this bounded math.stretch contract on
2026-09-18 after proposal commit `503e6416c78563b0d3949dfb81a58412da79dccb`,
as recorded in §7 of
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1446.md`.
That receipt supersedes the stale proposed heading; internal reuse does not
expand its ratification. The specified carrier is
`MathStretchElem { pub body: Content, pub size: Rel<Length>, pub size_explicit: bool }`.
All three fields participate in clone/equality/hash; Rel floats use the
existing mathematical-carrier zero normalization.
Plain text delegates to body; map_content transforms only body and preserves
size/size_explicit. map_text is terminal. The element is non-locatable.
Causal presence distinguishes omitted default from explicit size:100%; it is
not a third public language field. Acceptance preserves identity, body, size,
and presence through clone/equality/hash/reconstruction; changing body does
not change the stretch target.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_stretch.rs:44` preserves traversal but materializes the element even when
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

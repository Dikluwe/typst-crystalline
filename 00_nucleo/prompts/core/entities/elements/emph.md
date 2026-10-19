# L0 — EmphElem
Hash do Código: fdb9d779

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/emph.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/entities/element-boundary.toml sha256:eddb0b1b680907d9db343169884ebcc6e668d4d46a7e94146c6ddc018d4fe4f7

## Evidence before specification

The consumer represents `_italic_` as its own semantic element with one body;
render style is owned by layout. Historical upstream `a51e02804` at
`model/emph.rs:26` declares Locatable and Tagged. The prior baseline
`2f42d64253547734564513a1159ee6b584c1c4b4` lacked the payload override; that
chronology is diagnostic history, not a current gate or fresh parity measurement.

## Owned contract

`EmphElem::new(body)` preserves Content. `plain_text` and `is_empty` delegate
to body. `map_content` and `map_text` recurse and reconstruct `Content::emph`,
preserving this semantic occurrence rather than creating additional wrappers.
`get_field("body")` returns Content; other fields return None.
`to_payload` returns `Some(ElementPayload::NativeElement)` for this occurrence,
even with empty body, not for a descendant's visual style. Preserve kind, fields,
constructor and existing trait signatures. Functional identity and occurrence
snapshots belong to compiler owners, not this entity.
The trait boundary is owned by
`00_nucleo/prompts/core/entities/elements/mod.md` and shared invariants by the
pinned nucleus. Verify morphology, text/body transparency, recursive maps and
the own-occurrence payload. Language query/counter integration distinguishes
Emph, Strong and mere rendering styles; a unit constructor alone is not proof.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/emph.rs:45` preserves traversal but materializes the element even when
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

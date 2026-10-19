# Prompt L0 — `entities/elements/math_underover` — `MathUnderoverElem`
Hash do Código: 4acfeb9b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_underover.rs`
**Architecture:** ADR-0105 model D; ADR-0054 bounded generic carrier.

## Generic carrier

MathUnderoverElem derives Debug/Clone/PartialEq/Hash with public base:Content,
under:Option<Content>,over:Option<Content>. Content::MathUnderover holds Arc;
Content::math_underover(base,under,over) preserves that signature.
plain_text follows over+base+under, absent optional text empty. is_empty
defaults false; map_content visits base then under/over, preserving None and
error propagation. map_text is terminal and clones; get_field/kind/payload
are default None (nonlocatable). Equality/hash include all three fields.
Acceptance preserves visual text order, all recursive children/None states,
terminal text mapping and structural equality.

## Evidence and ratified public spreaders

Historical upstream preserves each named spreader function and distinguishes
omitted annotation from explicit none; generic geometry alone erased both.
Each separate carrier below has body:Content,annotation:Option<Content>,
annotation_explicit:bool. Constructors preserve presence. Equality/hash
include all three fields. map_content descends into body and annotation;
map_text preserves variant. For Underparen/Undershell the original walker
clause requires transforming both children while preserving variant/presence;
keep this explicit qualification rather than reducing it to a generic terminal
rule. Do not infer native identity from generic MathUnderover geometry.

| Public carrier | Glyph | plain_text order | ADR-0127 approval |
|---|---|---|---|
|MathOverbraceElem|U+23DE ⏞|annotation+glyph+body|2026-09-17|
|MathOverbracketElem|U+23B4 ⎴|annotation+glyph+body|2026-09-18|
|MathOverparenElem|U+23DC ⏜|annotation+glyph+body|2026-09-18|
|MathOvershellElem|U+23E0 ⏠|annotation+glyph+body|2026-09-18|
|MathUnderbraceElem|U+23DF ⏟|body+glyph+annotation|2026-09-18|
|MathUnderbracketElem|U+23B5 ⎵|body+glyph+annotation|2026-09-18|
|MathUnderparenElem|U+23DD ⏝|body+glyph+annotation|2026-09-18|
|MathUndershellElem|U+23E1 ⏡|body+glyph+annotation|2026-09-18|

These dated approvals supersede the original proposed-section labels within
their individual scopes. All public spreaders and the generic carrier remain
separate identities; none collapses omitted and explicit-none annotation.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_underover.rs:105` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_underover.md` preserves the original evidence and chronology.

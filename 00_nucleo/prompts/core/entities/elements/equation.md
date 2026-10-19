:warning: **Prompt L0 — `entities/elements/equation` — `EquationElem`**
Hash do Código: b23dc783

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/equation.rs`

## Evidence before specification

Historical upstream `e0e8ca4d:crates/typst-library/src/math/equation.rs:59–75` documents block default false and numbering of block-level equations through a pattern or function, with optional Numbering and set math.equation(numbering: "(1)"). This supports the language gate, not mandatory storage in the element. Upstream supplement synthesis at equation.rs:94–110,173–188 and tagged Formula alt consumption at typst-pdf/tags/resolve/mod.rs:303–306 likewise identify observable obligations without requiring data-to-exporter coupling.

## Carrier and public morphology

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct EquationElem {
    pub body: Content,
    pub block: bool,
}
```

Use Content::Equation(Arc<EquationElem>) and Content::equation(body,block). Both `$...$` syntax and math.equation(body,block:false) produce the same morphology. Body is required positional Content; block is boolean default false. repr omits false block and places true block before body. Derived equality/hash cover body/block.

Numbering stays in style chain `equation.numbering`, not an artificial struct field. Public numbering accepts string/function/none; omission inherits outer style and explicit none disables. Active numbering requires block && numbering active; do not reduce a Func to textual pattern. equation_numbered wraps Content::Styled with custom("equation.numbering",Value::Str("(1)")). This supersedes the original producer-only phase's rejection of numbering.

number-align is validated Align2D stored in `equation.number-align`; omission resolves to end+horizon. Its constructor wrapper is visible in repr without changing structural element equality. Without active numbering it is inert and does not move the body. Supplement remains `equation.supplement`, synthesized by Location for references; callbacks receive equivalent public content/block/body/effective styles. Alt remains `equation.alt`; constructor/field access/layout preserve public morphology, with export semantics delegated to a separate frame wrapper. The old absence-of-structure-tree observation is historical, not a current global prohibition. These later clauses complete the originally named deferred producer fields without adding fields here.

## Element contract and acceptance

Equation is locatable: kind Equation and corresponding Equation payload retain block and automatic unit counter update. The old unit Step notation is historical; the ratified CounterUpdate owner supplies its positive-level action shape. The payload consumer retains the effective block/numbering gate and inspects payload rather than Content.

plain_text is body text inline, or newline+body+newline for block. is_empty stays the default false, never delegates to body. map_content recurses through body retaining block. map_text is terminal and returns self; preserve the combined terminal hub arm rather than splitting it into a recursive text path. get_field's base default is None; public styled properties use their dedicated projection owner. No numbering/number-align/supplement/alt field is added to Hash/PartialEq/maps here.

Preserve constructor/repr, block numbering, wrapper morphology, explicit-none versus inherited numbering, inert unnumbered alignment, supplement callback morphology and alt transport tests. Do not equate upstream struct layout with language parity. The pinned internal no-change refinement below preserves this traversal asymmetry and does not expand any public approval.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/equation.rs:58` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/equation.md` preserves the original evidence and chronology.

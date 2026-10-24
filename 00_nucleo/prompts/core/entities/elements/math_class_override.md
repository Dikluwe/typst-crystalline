# Prompt L0 — `entities/elements/math_class_override` — `MathClassOverrideElem`
Hash do Código: 4fcb6915

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_class_override.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence and language intent

The recorded upstream ClassElem in math/mod.rs has required class and body.
math.class overrides inferred class for spacing between neighboring math
symbols, without changing body's own layout. The example
`#let loves = math.class("relation", sym.suit.heart)` requires
`$x loves y$` to receive the same THICK relation spacing as `$x = y$`.
Classification and MathLayouter::layout_node remain with math-layout owners.

## Contract and acceptance

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathClassOverrideElem {
    pub class: MathClass,
    pub body: Content,
}
```

Preserve Content::MathClassOverride(Arc<...>) and
Content::math_class_override(class:MathClass, body:Content).
Text and emptiness delegate to body (unlike MathCancel's nonempty default).
map_content recursively transforms body, propagates errors and preserves
class. map_text is terminal; the Content dispatcher may clone directly as
for other math carriers. Field/kind/payload default None, non-locatable.
Derived equality compares class and body.

## Cross-owner integration obligations

Keep exhaustive hub Debug/Display (`math.class({:?})`), construction, text,
equality, recursive content map and terminal text-map dispatch. Introspection
walk and locatable classification treat it as non-locatable/terminal;
non-math-context layout uses layout_math_fallback. Repr is
`class("<name>", <repr(body)>)`. Infrastructure query helpers retain their two
terminal/no-own-text branches. These belong to their respective owners and
are not reimplemented in this entity. Verify body transparency, recursive
child mapping, terminal text map, preserved class, structural equality and
unchanged spacing/body-layout boundary.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_class_override.rs:40` preserves traversal but materializes the element even when
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


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Override existing Element::get_field with class as math_class_name string and body as retained Content. Other fields return None. This supersedes prior field-default-None for these two fields; no trait/entity declaration change.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.

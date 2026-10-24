# Prompt L0 — `entities/elements/math_cases` — `MathCasesElem`
Hash do Código: 1b2db001

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_cases.rs`
**Trait owner:** `00_nucleo/prompts/core/entities/elements/mod.md`

## Evidence before specification

Historical Content anchors 1639,1568,2123,2626,1828 support the earlier
rows-based projection/traversal/equality. The complete-carrier comparison
records that an intermediate carrier had delim/reverse/gap but pre-resolved
rows, mandatory (char,char), Option<Length>, and no explicit presence.
Upstream retains variadic children, optional delimiter sides, and relative gap;
branch resolution belongs to its consumer.

## Earlier carrier contract

The original public MathCasesElem { rows:Vec<Vec<Content>> } derives
Debug/Clone/PartialEq/Hash, uses Content::MathCases(Arc<...>) and
Content::math_cases(rows:Vec<Vec<Content>>). Its layout boundary uses a fixed
left brace and no right delimiter. Plain text joins cells with ` & ` and rows
with `, `; is_empty defaults false; map_content visits every cell in row order;
map_text is terminal. Field/kind/payload default None, non-locatable. Equality
compares rows. Keep those requirements as the earlier model, not as a second
simultaneous implementation.

## Complete carrier proposal and approval-evidence gap

The replacement specifies children:Vec<Content>,
delim:(Option<char>,Option<char>), reverse:bool, gap:Rel<Length>, and public
MathCasesExplicit { delim, reverse, gap } with Copy/Eq/Hash. Element values,
explicitness, delimiter sides and gap all participate in equality/hash with
existing relative-float normalization. map_content visits each child in order;
map_text remains terminal. Plain text joins branches with `, `, leaving
MathAlignPoint inside each child. Every reconstruction preserves values,
order and bits. This supersedes pre-resolved rows and mandatory delimiters
within the proposed scope; branch/cell realization must not leak back in.

The original replacement is labelled proposed under ADR-0127;
`lab/history/construction/records/00_nucleo/materialization/typst-passo-1430.md`
retains that gate. The subsequent record says cases was certified, but this
review found no direct human ratification receipt. Retain the complete
proposal, tests and approval-evidence gap, without treating source code or
later extraction as authorization. Internal reuse below cannot expand it.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_cases.rs:66` preserves traversal but materializes the element even when
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

Override the existing Element::get_field method: children always returns ordered Content array; delim/reverse/gap only when the existing corresponding explicit bit is true. Delimiter absent sides become None; gap is full Relative. Unknown/unset returns None. No entity field or trait declaration changes.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.

# Prompt L0 — `entities/elements/math_limits_override` — `MathLimitsOverrideElem`
Hash do Código: de54ac83

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/math_limits_override.rs`

## Evidence before specification

Upstream math/attach.rs distinguishes ScriptsElem(body) and LimitsElem(body,inline:true); math IR resolve_scripts/resolve_limits normally resolve the body and then override limit placement. The documented examples in `lab/corpus/docs/math/attach.typ:18–28` and archived tests at math/layout/tests.rs:7560–7765 establish scripts(sum)_1^2 versus sum_1^2, limits(A)_1^2 versus A_1^2, inline mode, class transparency and default italic. In particular `p992_limits_forca_empilhamento_mesmo_inline`, `p992_limits_inline_false_respeita_modo`, `p992_scripts_nao_muda_classe_do_body` and `p992_limits_preserva_italico_por_defeito` preserve those controls. The language requires these effects, not two distinct Rust structs.

The later named-argument probes `/tmp/e1..e3.typ` measured `error: unexpected argument: foo` for scripts/limits(A,foo:1), and `error: expected boolean, found content` for limits(A,inline:5) in the math-native context where 5 evaluates to Content. The owner requested that repair on 2026-08-11. Errors are language observables; silently dropping named inputs is not acceptable.

## Carrier and semantics

```rust
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct MathLimitsOverrideElem {
    pub body: Content,
    pub limits: bool,
    pub inline: bool,
    pub inline_explicit: bool,
}
```

Use Content::MathLimitsOverride(Arc<MathLimitsOverrideElem>) and `Content::math_limits_override(body: Content, limits: bool, inline: bool, inline_explicit: bool)`. scripts uses limits=false, conventional inline=true (ignored) and inline_explicit=false because it has no inline argument. limits uses limits=true; inline=true forces stacked scripts even inline, while false stacks only in block/display mode. scripts always places them laterally. Do not change the body's math class or spacing.

inline_explicit distinguishes omission from an explicit value, including true equal to default. Bare and qualified math.limits derive it from causal argument presence, never by comparing value to default. The public field/constructor gate was confirmed on 2026-09-16 in the original P1425 record. Preserve it in walkers/equality/reconstruction; it only controls repr emission of inline, never geometry/class/emptiness/text.

## Element, dispatch and acceptance

plain_text and is_empty delegate to body. map_content recursively transforms body, preserving all three flags; map_text is terminal. get_field, element_kind and to_payload default to None: this wrapper is nonlocatable. Derived structural equality/hash include body and every flag.

The Content hub delegates Debug/Display (`math.limits({:?})`), construction, plain_text, equality and recursive/terminal map paths. Introspection treats it as nonlocatable/terminal; outside math, layout uses layout_math_fallback. repr distinguishes `limits(<repr(body)>, inline: <bool>)` with the later presence rule from `scripts(<repr(body)>)`. Infra query helpers treat it as terminal without own text. These cross-owner paths remain independently specified.

Unlike the historical MathClassOverride default-italic gap, apply_math_default recurses into this body's single-letter bases through normal layout_node. base_math_class/node_math_class remain body-transparent. apply_math_style for bb/bold retains its previously scoped unchanged fallback; this contract does not silently close that gap.

Reject unknown named arguments with SourceDiagnostic::error and the measured unexpected-argument text. Evaluate inline and reject non-Bool using type_name rather than silently keeping true. Require transparent text/emptiness, recursive content mapping, terminal text mapping, all-field retention/equality, default italic, class transparency, inline/display placement and exact diagnostic controls. Internal parity corrections follow ADR-0127 continuous flow; the no-change refinement below does not expand the public approval.

## Container reuse without replacement

Evidence before specification:
`lab/results/construction/diagnostics/content-container-reuse-2026-10-10/before.json.gz`
records HEAD, working tree, UTC, complete sources, owners, and release.
`01_core/src/entities/elements/math_limits_override.rs:39` preserves traversal but materializes the element even when
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

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_limits_override.md` preserves the original evidence and chronology.

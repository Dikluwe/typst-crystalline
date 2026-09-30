# L0 — math/layout/spacing

Hash do Código: 074768bc

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/spacing.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/spacing.rs:29` locates `fn base_math_class(content: &Content) -> MathClass`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Mathematical classes and automatic gaps

Own node_math_class, promote_vary, spacing_between and compute_gaps.
Classify named MathIdent from its resolved Unicode symbol, not the first letter
of the name; unresolved identifiers and MathText retain their existing
first-scalar classification. TextItem is Alphabetic; MathOp is Large regardless
of its limits flag. MathStyled and MathLimitsOverride preserve the body's math
class; MathAttach inherits the base's math class. Mathematical underline,
overline and public spreaders preserve their body's boundary classes.
MathClassOverride explicitly supplies its class. MathMid is Relation;
MathPrimes is Normal. Optional matrix/vector delimiters expose Opening/Closing
only on present sides, otherwise Normal. Preserve existing fenced/fence
boundary handling and sequence edge classes.

Space and SpaceFromNewline remain neutral Normal; newline origin does not
trigger textual fallback, CJK normalization or show/regex processing.
First-scalar classification and composite accent/cancel/underover inheritance
limitations are audit items, not a language-wide definition.

Promote Vary to Binary after Normal, Alphabetic, Closing or Fence; otherwise
it stays unary-like without fabricating a Binary gap. Apply ordered spacing
rules (size in em):
1. Before Punctuation: zero; after Punctuation: 1/6.
2. After Opening or before Closing: zero.
3. Relation–Relation: zero; other Relation boundaries: 5/18.
4. Binary boundaries: 2/9.
5. Large boundaries: 1/6 except the established Opening/Fence exclusions.
6. No explicit rule: zero, subject to spaced-item fallback.

Unary has no dedicated override that would defeat a later Binary rule.
Keep an explicit zero rule distinct from absence of a rule. Only absence
allows fallback to the active font's measured text-space advance for spaced
TextItem/atomic textual markup or Fence. Do not hardcode a point width or
trigger fallback from SpaceFromNewline.
Compute n-1 gaps sequentially with Vary promotion; in script size, class
spacing is suppressed, not merely reduced. The caller supplies script state
when math_script or Script/ScriptScript applies. Preserve alignment-boundary
spacing delegation and explicit-zero priority there too.

Acceptance observes class overrides, boundary asymmetry, Large operators,
integrals, symbol names, textual/fence fallback, punctuation and script
suppression using attributable geometry. Generic spaced frames, normal-class
spaced wrappers, per-item mixed sizes and composite inheritance remain
limitations requiring separate evidence; do not encode their defects as defaults.

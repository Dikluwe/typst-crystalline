# L0 — Explicit OpenType variation axes
Hash do Código: c8585ebf

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_variations.rs`

## Evidence

Historical upstream evidence in `text/font/variations.rs:196-214,234`,
`text/mod.rs:846-850` and `tag.rs:22-27,91-112` supports normalization, folding,
settable axes and tag validation. The former probe provenance is preserved in
the sanitation diagnostic, not represented as current-pin parity proof.

## Public contract

`FontVariations(pub Vec<([u8; 4], f32)>)` derives Debug/Clone/PartialEq/Default.
`is_empty` checks axes. `from_value(&Value, Span)` returns
`Result<Self, Vec<SourceDiagnostic>>`; `from_validated_dict(&IndexMap<EcoString,
Value, FxBuildHasher>) -> Self` ignores unexpected entries without failing.
`fold(&self, outer: &FontVariations) -> FontVariations` gives inner tags
precedence and retains outer tags missing inside. Both construction paths
normalize by tag, sort and deduplicate with later occurrence winning.

## Validation and observable diagnostics

Input must be a dictionary; otherwise use `expected dictionary, found {type}`
without a tag hint. For each entry in dictionary order, preserve this precedence:

1. Reject characters outside printable ASCII 0x20..=0x7E with
   `tag may contain only printable ASCII characters` and the hint
   `found invalid cluster` followed by the quoted offending cluster.
2. Require tag byte length 1..=4 with
   `tag must be one to four characters in length` and `found {n} characters`.
3. Permit spaces only as trailing padding, never leading/interior, with
   `spaces may only appear as padding following a tag` and no specific hint.
4. Convert Int/Float to f32; reject other values with
   `expected float, found {type}`. Map short type names int/str/bool to
   integer/string/boolean in these diagnostics.

Append the following hint after each entry-specific hint:

```text
occurred in tag at index {i} (`"{key}"`)
```

The invalid-cluster hint is exactly:

```text
found invalid cluster `"{cluster}"`
```
No range validation is introduced; clamping belongs to shaping/instancing.
Tags are [u8;4] padded with ASCII space, without a ttf_parser dependency.
The existing chars-versus-grapheme diagnostic limitation remains explicit;
it does not establish full combining-sequence parity.

The owner is pure L1, with no I/O or parser/runtime types in its public boundary.
Structural PartialEq supports resource keys; style resolution and physical font
instancing remain separate owners. Verify normalization, fold precedence,
lenient validated input, diagnostic text/hint order and supplied spans.

## Lenient numeric projection and enum evolution

Evidence precedes this internal classification. At reviewed HEAD
`841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`01_core/src/entities/font_variations.rs:92-105` implements
`FontVariations::from_validated_dict`; its Value match is at lines 95-99.
The frozen V16-078 inventory and causal ledger are in
`lab/results/v16-cause-analysis-2026-10-19/`. The consumer and this owner's
pre-edit SHA-256 match that inventory. `01_core/src/entities/value.rs:82-231`
defines the current closed Value domain, and
`01_core/src/entities/style_chain.rs:759-770` calls the lenient path while
folding dictionary-valued style levels. This is source/contract evidence,
not an executed test, new vanilla measurement or inference of upstream intent.

Keep Int and Float as the only numeric cases, with their existing `as f32`
conversion. The intentional omission class is None, Bool, Str, Array, Dict,
Module, Datetime, Func, Content, LocatedContent, Auto, Length, Relative, Ratio,
Angle, Color, Stroke, Fraction, Align, Location, Gradient, Regex, Tiling, Bytes,
Decimal, Duration, Version, Selector, Symbol, Args, State, Counter, Label, Dir,
Path, Type and SpotColorant. Name each present alternative in the match and
continue to the next entry; do not replace the open decision with another
wildcard, an if-let/matches! projection, a helper or a suppression. A future
Value variant must require a decision here. LocatedContent has two payloads;
None and Auto are unit variants, and the other omitted variants have one
payload. Do not broaden numeric admission merely because a value can be
formatted or converted elsewhere.

Omission is not an error in this function. Do not call strict from_value,
change the public Self return into Result, add diagnostics or invent a default
axis value. Numeric entries still undergo the existing tag validation;
invalid tags are independently ignored. Preserve numeric-before-tag processing,
empty-result validity, padding, normalization order, later normalized-tag wins,
fold precedence and existing f32/nonfinite behavior. Add no range/clamp policy.
The strict from_value validation and all diagnostic precedence remain unchanged.

Regression controls in this owner's consumer must cover
a mixed dictionary of valid Int/Float axes, nonnumeric values and invalid tags;
all current nonnumeric categories must be omitted without discarding valid
neighbors. Include an empty dictionary, numeric/non-numeric entries with
colliding padded tag identities, and later valid normalized-tag precedence.
Contrast the strict route's error/hints/span with the lenient route's omission.
Retain `p836_from_validated_dict_leniente`,
`p836_from_value_valido_normaliza_e_ordena`,
`p836_valor_string_erro_verbatim`,
`p836_indice_no_hint_segue_posicao_do_dict` and
`p836_fold_interno_vence_por_tag_externo_sobrevive` unchanged. Existing focused controls alone do not establish every omitted category.
Functional expectations stay
green for this preservation change; the missing enum-growth obligation supplies
the structural RED.

In a disposable complete workspace only, add a sentinel to copied Value and
compare compiler JSON before/after the nominal correction. Require E0004 at
`01_core/src/entities/font_variations.rs:95-99`, the selected
from_validated_dict match, not merely elsewhere in its function or workspace.
Do not modify the productive enum, create stubs, change scan scope or suppress
V16. Review any resulting V19/V20 alternatives without treating one executed
arm as proof of every payload. This is ADR-0127 continuous internal work:
L0 first, unchanged public contract/default/phase/compatibility and pure L1.
Keep 1:1 ownership and valid V15/V26 before restricted resealing and
regression/build/lint gates.

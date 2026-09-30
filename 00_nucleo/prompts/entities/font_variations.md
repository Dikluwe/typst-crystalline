# L0 — Explicit OpenType variation axes
Hash do Código: 8669fc16

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

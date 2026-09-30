# L0 — infra/export/gradients/function_dict
Hash do Código: 1cdb3061

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/function_dict.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/function_dict.rs:12` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF interpolation functions

Own RGB, CMYK and one-channel spot function dictionary emitters. Consume already
sampled stops; do not sample gradients or allocate unrelated PDF objects.
Return the main dictionary plus subfunction objects, updating the caller-supplied
sub_first_id for each newly emitted subfunction.

RGB/CMYK require at least two samples. Two samples use Type 2, Domain [0 1],
C0/C1 and N 1 without subobjects. More use Type 3 stitching of adjacent Type 2
segments, uniform Bounds and repeated Encode [0 1]. RGB uses three components;
CMYK uses four and Range [0 1 0 1 0 1 0 1]. Preserve current component precision.

Spot uses tint directly, not RGB projection, with Range [0 1] and one component.
Zero-to-two samples retain the current endpoint/default-zero Type 2 behavior.
Additional samples use Type 3 adjacent segments and current eight-decimal tint/bounds.
Output Separation resource/shading routing belongs to
`00_nucleo/prompts/infra/export/builder/resources.md`.

## Verification

Check components, domains/ranges, segment endpoints, subreference closure and
collision-free IDs supplied by the caller. Tint and component identity are output
contracts; exact external object IDs or stop-allocation mechanics are not parity.

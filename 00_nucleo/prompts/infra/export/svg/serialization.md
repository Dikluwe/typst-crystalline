# L0 — svg/serialization
Hash do Código: a65dc621

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/serialization.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/serialization.rs:9` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Scalar serialization

Own color_to_css, fmt_num and fmt_pair. Preserve sRGB/alpha conversion, nine-decimal
rounding with factor 1_000_000_000, integer fast path and ryu representation.
XmlWriter owns XML escaping; paints owns adaptive offset representation. Do not
generalize this numeric formatter into those other contracts.

## Verification

Check integers, fractional/negative coordinates, pairs and color/alpha output.

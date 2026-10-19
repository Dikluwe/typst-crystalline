# L0 — stdlib/foundations/cast
Hash do Código: 66cf18e0

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/cast.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/cast.rs:27` (pub fn native_int); `01_core/src/compiler/stdlib/foundations/cast.rs:126` (pub fn native_float); `01_core/src/compiler/stdlib/foundations/cast.rs:151` (pub fn native_range) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own int/float/range/bytes/datetime/symbol constructors, not registry discovery or instance methods. Preserve current native ABIs and diagnostic ordering.

int accepts Int, Bool, Float, Decimal and String; Float/Decimal truncate toward zero and reject overflow. Explicit base is string-only and must be 2..36. float accepts Float, Int and parseable String. range preserves start/end, inclusive and step defaults; incompatible direction yields empty, zero step fails `number must not be zero`, and terminal overflow must not loop.

bytes accepts UTF-8 String, Bytes and Array of integer octets 0..255, not Int. The old bytes((0xFF,)) rejection example contradicts its allowed range and must not be treated as the contract: preserve 255 acceptance; the discrepancy is archived.

datetime takes year/month/day/hour/minute/second named components, requiring a complete date, complete time or both; preserve validation order, limits and literal diagnostics. Calendar operations remain with the Datetime entity and static fields with its child stdlib owner.

symbol variants are strings of exactly one extended grapheme cluster or (modifiers,value) arrays. Preserve complete Unicode strings, including ZWJ, selectors, tone modifiers and flags. Reject empty/multiple graphemes with `invalid variant value: REPR` and `variant value must be exactly one grapheme cluster`. Never truncate through chars().next().

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

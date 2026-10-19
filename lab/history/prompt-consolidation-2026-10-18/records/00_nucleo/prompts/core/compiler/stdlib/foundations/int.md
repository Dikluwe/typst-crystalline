# L0 — stdlib/foundations/int
Hash do Código: 6df70ab9

**Baseline**: vanilla ratificado `a51e02804`

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/foundations/int.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/foundations/int.rs:16` (type NativeResult = SourceResult<Value>;); `01_core/src/compiler/stdlib/foundations/int.rs:18` (pub fn int_type_fieldfield: &str) -> Option<Value> {); `01_core/src/compiler/stdlib/foundations/int.rs:35` (pubcrate) fn is_int_instance_methodname: &str) -> bool {) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own int_type_field, static operations and the self-first instance bridge; the int constructor remains with `00_nucleo/prompts/core/compiler/stdlib/foundations/cast.md`. Preserve min/max integer constants, signum, bit-not/and/or/xor/lshift/rshift, from-bytes and to-bytes. Discovery is closed, not reflective.

Use signed 64-bit bit operations. Left shift >=64 fails `the result is too large`; overflowing bits otherwise leave the 64-bit result. Right shift defaults logical:false and saturates excessive shifts to 0/-1; logical:true reinterprets unsigned and saturates to 0. from-bytes accepts at most eight octets, default endian little and signed true, with sign extension. to-bytes defaults little/size 8 and truncates or extends to the requested size. from-bytes is static-only; self operations preserve static/bound equivalence.

Args with Some are projected to positional occurrences without losing Values, order or either individual span, using the supplied aggregate. None keeps legacy positional views. Bound forwarding injects a synthetic receiver with detached individual spans, retaining the aggregate and all original origins. Do not invalidate Some, copy removed named occurrences, infer origin from equal Ints or broaden every native's duplicate-named policy. Keep logical/endian/size casts and exact diagnostics.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.

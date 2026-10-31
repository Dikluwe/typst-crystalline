# L0 — scan/reader: borrowed JPEG byte reader
Hash do Código: e0f96fd1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/scan/reader.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130, ADR-0131

## Evidence before classification

Received product/source/UTC/dirty stat and exact inputs are frozen in
`lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/before.json`.
The pinned zune-core revision2f9086b07cdf5a36621ce18f79d310f99b751de9,
bytestream/reader/std_readers.rs:11-41, routes ZCursor through BufRead+Seek and
loops on exact reads, including EOF rollback. reader/no_std_readers.rs:56-128
contains direct slice operations, but its trait implementation is disabled
when std is enabled. The existing dependency remains pinned and unmodified.

`reader-probe/receipt.json` plus `reader-probe/source-provenance.json` bind
same-process alternating probes, source/manifest/lock/binary hashes and exact
uncommitted source state. Direct/adapter medians: noise22.864/26.073ms,
gradient52.993/64.977ms, uniform28.877/28.888ms. Both use full native reconstruction,
strict options and SIMD. This supports a direct-reader candidate; it does not
certify whole-product improvement. Mechanical transport/complexity is separate
from language observables; parser outcomes, diagnostics and render are verified
by the owning scan consumer and full compatibility campaign.

The integrated default-recipe pilot is source-bound in reader-pilot/receipt.json:
noise/prior0.98742924 (paired95% upper0.99550460), gradient0.89627856,
uniform1.01632125, RGB1.02379592 and all RSS<=1.03. This refutes the
received performance RED under the same predicate. Final actual-default and
complete validation remain promotion gates, not assumptions from this pilot.

## Scope and contract

Own a module-private BorrowedJpegReader<'a>, exposing only pub(super) constructor
and type to its parent. Borrow an immutable byte slice and keep one usize cursor;
no input copy, heap allocation, I/O, unsafe code, cache or mutable globals.
Implement the existing zune_core::bytestream::ZByteReaderTrait directly, without
implementing std::io::Read/BufRead/Seek or adding any method to the external trait.
This is a private adapter, not a public contract/mode/phase/compatibility change;
ADR-0127 continuous flow applies. The owner is bijective and independent from
scan's JPEG policy; no duplicated decoding invariant or new Tekt nucleus.

Provide checked single-span reads, avoiding the std retry loop for immutable
complete memory input. Partial reads consume/copy min(request,remaining).
Exact reads copy the available prefix even on EOF, return NotEnoughBytes with
actual/requested counts, and advance only on complete success. Thus failure
keeps position unchanged, matching the received std adapter's rollback contract.
All peek operations copy the same bytes without advancing, including on error.
read_byte_no_error returns zero at/beyond EOF without moving; it advances once
only when a byte exists. Empty reads succeed without movement, even beyond EOF.

Support Start/Current/End seeks, including beyond EOF. Reject negative or
unrepresentable/overflowing positions without changing cursor, with checked
integer conversions/addition; never wrap or clamp a successful position.
Position reports the cursor, EOF means position>=input length. read_remaining
appends the remaining suffix to the caller's Vec, advances by the appended count,
and returns zero without movement when already at/beyond EOF. Allocation is
owned only by that existing sink when it grows; no intermediate Vec is permitted.
Errors of this private adapter are not new user-facing diagnostics: scan keeps
its existing authoritative fallback. Do not alter byte framing or JPEG decoding.

## Tests and acceptance

Materialization acceptance requires module tests, product integration and the
full resource/language gates. Write module tests first; retain the compile
RED for the absent new adapter, separately from the genuine performance RED
(resource-red2.json) on the unchanged received product. Require GREEN for six
semantic cursor tests: exact EOF/prefix/rollback, partial/peek behavior, byte EOF
and remaining suffix, reference seeks, extreme checked positions, and seeded
mixed operations versus the received ZCursor std adapter. Compare outcome,
position and destination bytes; allocation/layout/Rust identity are not language
parity claims. Test extreme overflow safely without copying old cursor defects.

The parent owns whole-image eligibility, full entropy/IDCT/Complete, errors,
allocation controls and integration. Preserve all old tests and add a noisy
odd-dimension JPEG/entropy-tail preservation case there. The performance GREEN
must satisfy the unchanged median/CI/RSS predicate and all predeclared nine-case,
936-process, compatibility and full validation gates from its owning L0.
Reseal after V15/V26. Preserve received files, failed candidates and source/binary
provenance; do not promote an isolated probe or an experimental binary.

# P1327 C2 — independent A/B result

**PASS_SCOPED.** Independently recomputed full exit/stdout/stderr equality for
all 432 observations: original corpus 336/336 and R2 corpus 96/96. Their 112
and 32 keys occur exactly once in each normal/repeat/reverse order; outputs
are stable. No violations, missing observations, Unknown or integrity failures.

Candidate SHA-256:
`75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`.
Current executable matches both candidate receipts. Preserved C1, original
baseline and pinned vanilla binaries also match their declared identities.

Original fixtures and frozen R0/R1/R2 oracles, runners, measurements and
identified historical artifacts remain intact. All three normative L0 hashes
match the R2 manifest after removing only the reciprocal code-hash metadata
line. The original C1 `Violated` verdict and its 24 failing transcripts remain
immutable; this is an additive verdict for a distinct candidate binary.

The 432 comparisons include 48 explicit debt-preservation observations:
redundant import rename, import type error, and raw serialization failure with
or without a preceding warning. Those match full frozen crystalline outputs
and do **not** claim vanilla parity. The other 384 compare full pinned vanilla
transcripts. No diagnostics are stripped, normalized or reordered by the
acceptance comparator.

Detailed verdict: `p1327-ab-c2-verdict.json`, SHA-256
`f18b357d4f2277d9fc656d91333174b41db7ae7e6082561615dda223515171a8`.
It records both receipt hashes, exact measurement intervals, executable and
oracle identities, all integrity checks and per-corpus counts.

Executed as A/B without technical isolation attestation. No runtime source or
full state receipt was read. Acceptance covers the frozen observable fragment,
not general parity or architectural gates delegated to the final reviewer.

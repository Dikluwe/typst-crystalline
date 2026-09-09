# P1327 A/B — independent candidate verdict

**Violated.** Recomputed all 336 full exit/stdout/stderr comparisons: 312
Preserved and 24 Violated, with no missing observations or integrity failures.
All 112 profile/case keys occur exactly once in each normal/repeat/reverse run;
their observations are stable across those orders.

The failures are exactly `later-error` and `alias-later-error`, in all four
profiles and all three orders. Candidate stderr emits the warning block first
and the error block second. Frozen vanilla emits the error block first and the
warning block second. Both blocks are otherwise byte-identical, including
messages, anchors and spacing; exit remains 1 and stdout remains empty.
The verifier proves this exact block reversal independently for all 24 failures;
it does not normalize or remove warnings to accept them.

Frozen fixtures, oracle, runner, original R0 artifacts, R1 formatting successor,
and normative L0 hashes remain intact. Current candidate binary matches its
receipt: SHA-256
`5f00502b5055fedf8ca1dc2c879112ac1300e12621e11fdb2c731364450a62f9`.

Candidate receipt SHA-256:
`6ffa5fc5ce6925eb0c85e40749adff5d1688ce006fe2400cdad3a4edeb47fafe`.
Independent verdict `p1327-ab-verdict.json` SHA-256:
`976a2d160c8fc9095650f39b0f79ba33cc496656ba3ea14b08962ddccc1ed6e9`.
The JSON retains exact failing transcripts, timestamps, commands through the
referenced candidate receipt, binary identity, checks and immutable input hashes.

The CLI presentation owner is a possible cause, not a confirmed source-level
attribution: this role did not read runtime or candidate source. The frozen
contract is not satisfied, so modules-only sufficiency is unproven and any
additional owner requires explicit reconsideration. No expectation was adapted;
the original failure corpus remains preserved.

Executed as A/B without technical isolation attestation. This verdict concerns
only the frozen fragment and makes no claim of general parity.

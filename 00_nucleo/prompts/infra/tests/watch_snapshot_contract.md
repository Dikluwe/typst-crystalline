# L0 — External watch snapshot contract tests
Hash do Código: 566a5c21

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/watch_snapshot_contract.rs`

## Current evidence

- `03_infra/tests/watch_snapshot_contract.rs` — transition controls, type checks and pinned rustdoc JSON inspection.
- `03_infra/src/watch.rs` — public product owned by `00_nucleo/prompts/infra/watch.md`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own this integration test only. A captured snapshot must precede the stimulus; `wait_for_change_since` receives that token. Preserve the legacy `wait_for_change` API. Observe public behavior without fixing private fields, layout, fingerprint algorithm or collection representation.

Each case uses its own temporary directory and RAII cleanup. Detect equal-length content replacement, creation of an initially absent path and removal of an initially present path after capture. Type-check the three snapshot/wait entrypoints. On Linux, retain the public pseudo-file control whose content changes between reads.

Complete each transition before starting the waiting thread. `recv_timeout` bounds failure; it is not readiness evidence. Do not add sleep, retry, artificial load or a corrective second mutation.

Pinned-toolchain rustdoc JSON must show public WatchSnapshot with nonempty private state, no public fields/inherent items/direct traits, and only snapshot/wait_for_change_since among public functions whose signatures reference the token. Accept named or tuple representation without choosing field names/types. Missing identity, unsupported version/schema or invalid JSON fails the test.

## Verification boundary

Verify transitions, signatures and public inventory. This test does not prove general watch equivalence or replace the CLI process suite.

## Exhaustive JSON signature-reference classification

### Source evidence before decision

At HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`03_infra/tests/watch_snapshot_contract.rs:174–191` contains the private
`signature_mentions_id` helper; its wildcard false is at `:189`.
Array values recurse; Object checks `resolved_path.id` through `as_u64` and
recurses through object values. Scalars cannot contain that structured path.
`Cargo.lock:1895–1896` pins serde_json 1.0.150; its closed six-variant Value
is defined at `src/value/mod.rs:116–175`, read from
`/home/dikluwe/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.150/src/value/mod.rs`.
Earlier analysis cited cached 1.0.149; use the locked 1.0.150 source for this
batch's evidence and any copied dependency. The variants are identical in those
inspected definitions; no dependency upgrade or schema change is proposed.
The V16 inventory and per-case plan are under
`lab/results/v16-cause-analysis-2026-10-19/`. This source observation is not an
executed test or a new rustdoc-schema compatibility certificate.

### Decision and preserved complement

Replace the helper's wildcard false with explicit Null, Bool, Number and String
patterns sharing false. Keep Array/Object recursion, short-circuiting and exact
`resolved_path.id`/`as_u64` detection unchanged. A standalone target number,
string containing the target, or unrelated object `id` is not a type reference.
Do not search scalar text/numbers, accept a missing or unsupported rustdoc schema,
relax the public inventory, move this test helper into productive Watch code or
exclude the helper from the check. The existing strict WatchSnapshot public-surface
contract remains authoritative, including its exact snapshot/wait_for_change_since exposure.

### Acceptance and exact-site evolution proof

Propose a focused helper test with nested arrays/objects containing a genuine
resolved_path reference; unrelated resolved_path ids; direct numeric/string target;
plain id property; empty collections; null/bool; malformed or absent path/id; and
nested positive alongside negative siblings. Require the same result before and
after classification, without inferring opaque token identity from JSON shape.
Preserve `rustdoc_json_pinado_prova_superficie_opaca_e_fechada` and every existing transition/signature test unchanged,
including snapshot-before-stimulus behavior and equal-length/create/remove transitions. Missing identity, unsupported version/schema or invalid JSON
must continue failing the independent public-surface test. No sleep/retry/load or
corrective stimulus is introduced.

Copy locked serde_json 1.0.150 into a disposable workspace and
adds a Value sentinel only in that copied dependency. Compile the actual
`typst-infra` integration-test target `watch_snapshot_contract` without running tests as part
of the structural proof. Baseline lacks E0004 at this helper's match; repaired
source must report E0004 at this same match when the consumer is type-checked.
A copied-dependency build failure, another match's error or a lib-only build
omitting this integration test is inconclusive. Retain exact dependency/source
hashes, target/argv and compiler JSON. No sentinel, patch or dependency change
enters the productive checkout. Functional tests and pinned rustdoc inspection
remain separately required; structural compilation does not certify their results.

This is an internal classification correction under ADR-0127 continuous flow:
no public field, trait, signature, product default, compatibility rule, dependency
version or pipeline phase changes. Preserve the owning layer and static dispatch.
No suppression, severity change, `if let`/`matches!` replacement, or helper that
relocates the unchecked complement is an acceptable repair. Validate V15/V26
before restricted forward/reverse resealing. Freeze source/dependency identity during structural and functional validation;
record exact targets, compiler diagnostics, resource settings and outcomes before
claiming acceptance. A structural proof is not a functional test result.

### Attributable actual-domain omission proof

For V16-105, an equally strict structural alternative avoids a new enum variant
preventing core or external-dependency metadata emission before the real infra
consumer is type-checked. Use two disposable copies with the same unchanged real
dependencies: one with the original wildcard decision and one with its repaired
exhaustive classification. At the signature_mentions_id match at watch_snapshot_contract.rs:175–190, remove only
the complete unguarded Value::Array(values) arm in each copy. This arm is unique and unguarded; no other arm
in that match handles Array. With the omission, the wildcard baseline
still covers Array and must lack E0004 at this site; the repaired copy must
emit E0004 at this exact match. Retain function/target identity, mutation diff,
source/dependency hashes, compiler JSON and no-omission controls. An unrelated
error or dependency failure remains inconclusive. No replacement stub, new public
API, linter or target-scope change is allowed.

This paired omission experiment proves that the actual-domain classification no
longer has an implicit complement. The normal repaired match must remain fully
nominal and exhaustive, so future enum growth still forces an explicit owner
decision. This alternative changes only how the structural obligation is measured;
it weakens neither the future-variant obligation nor the independent functional
controls, and no probe result is claimed by the specification.

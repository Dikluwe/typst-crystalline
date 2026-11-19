# L0 — bounded immutable zlib reuse
Hash do Código: dc3011bd

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/compression.rs`

## Measurement before decision

Frozen primaryproof source (HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2,
uncommitted laboratory tree; evidence/freeze-primaryproof.json) retains
export/images.rs:47–51 fresh default-ZlibEncoder construction on every call.
export/builder/streams.rs:27 and builder/fonts.rs:291/322 invoke that function.
primaryproof-stream-reuse/receipt.json binds exact PDF inputs, UTC and actual
byte comparisons: on each measured edit 8/10 streams, 267541/268864 decoded
bytes, remain identical in the hundred-paragraph input; outline has19/21.
This is mechanism evidence, not a language equality rule or measured CPU saving.
controlprimary-cpu-stacks/ places compression near6.5% of observed nearest-project
samples, with truncation/inlining limits. primaryproof-native-phases/ locates
render elapsed near4.4ms; it does not attribute that whole phase to compression.

## Internal decision and falsification

Own private default-zlib computation and bounded reuse for exact immutable byte
inputs, returning the existing Result<Vec<u8>,String>. No public API, mode,
phase/order, language semantics, compression level or error-policy change.
ADR0127 continuous internal flow; laboratory only, no integration authorized.
A single process-wide L3 Mutex stores at most32 entries and1048576 bytes total
owned input+output payload. Metadata capacity is additionally bounded by entry
count. L1 remains pure; no source, font, document, file, World, object ID,
metadata or dictionary identity enters the key. This pure operation needs only
exact input bytes and its fixed current default compression configuration.
No hash-only acceptance; compare full byte slices, including length and empty.
Owned copies prevent caller mutation. Clone the cached compressed bytes on return.

Lookup/update hold the lock briefly; compress misses outside the lock. Poisoning
falls back to fresh compression and never changes the caller result. Retain only
successful results; failures remain retriable. Oversized inputs or combined
input/output bypass retention. Use least-recently-used eviction under both bounds;
deduplicate races by exact input after computation. Concurrent misses may compute
independently but must not produce duplicate retained entries. Do not hold file
handles, clocks, global layout state or callbacks. The cache has process lifetime
but deterministic strict bounds, independent of watch eviction or source epochs.
Keep compression implementation exclusively here; images facade only delegates.

## Acceptance

Tests first: repeated inputs must compute once, same-length different bytes must
remain independent under ABBA, mutation of caller inputs/returned outputs cannot
poison retained data, failure must retry, both bounds and eviction must hold,
concurrent requests and poisoned locks must preserve fresh results. Inflate
all tested outputs to their exact original payloads; byte identity is a codec
mechanical check, not a substitute for Typst language acceptance.
Run complete infra tests, workspace targets, Clippy/Rustdoc/fmt, release build,
strict lint and provenance freeze. Core may be reused only with byte identity.
Compare text, semantic styling, geometry, annotations, queries and allpage paint
on the established55 controls, nominal watch outputs, lexical known debts and
real late/replaced fonts. Measure twelve alternating pairs against primaryproof
and the pinned64c5e8c0 vanilla with unchanged recipe, observer off; memory and
size must be recorded. No nominal improvement means no promotion despite fewer
codec calls. Unknown/public/phase expansion requires its own decision.

## Productive adoption scope — 2026-11-18

The user explicitly authorizes the cumulative packages with measured gain. This
owner adopts only its finalized implementation in the preserved capture candidate
29115606, qualified together with its required dependency owners; earlier
laboratory-only exclusions are superseded only for this specific implementation.
Existing current project ADR scope clarifications and conservative fallbacks
remain in force. No additional public method, default or phase is invented here;
already specified optional capabilities keep their explicit authority/None paths.
Diagnostic census, no-retain ablations and blocking watch markers are not product
features. Current adoption uses disk-backed task targets, one Cargo job,12GiB AS,
8GiB tree RSS and no swap; prior task-specific temporary-target paths are scoped
to their original correction. Require real capture RED→GREEN, complete suites,
lineage/ownership, source/effect/invalidation controls and measured before/after
and pinned vanilla comparison. No commit is authorized by this adoption.

## Validated cache budget for bitmap Unicode (2026-11-19)

The preceding Unicode census reproduced the exact LRU decisions in both sessions. At 32 entries and 1 MiB, all 44 bitmap buffers in a comment cycle repeated byte for byte and all 44 ran Deflate again. Both limits triggered eviction. The replay with 64 entries and 2 MiB retained the repeated buffers in those sessions. The result specifies an experiment and predicts no cycle time.

In the productive implementation, cap the same LRU at 64 entries and 2,097,152 total owned input-plus-output bytes. This supersedes the earlier two constants for the productive cache. Preserve exact-byte matching, current encoder and settings, eviction order, ownership, errors and admission rules. Exports preserve source text, fonts, effects and current invalidation. No change to public contracts or pipeline phases.

Run the recorded all-caller stream through the actual cache logic before and after the change. Use recorded compressed lengths in that structural witness, separate from the timed release. Verify real Unicode publications against fresh baseline, including effects, dependencies and font mutation. Qualify CPU benefit only with balanced A/A and complete end-to-end watch cycles. Keep build and runtime bounded .

### Productive adoption — 2026-11-19

Adopt the bounded implementation qualified in
lab/results/construction/diagnostics/unicode-deflate-cache-64-2026-11-19/report.md.
The 2026-11-19 integration is explicitly authorized and supersedes isolation
restrictions for this validated implementation. Prior experiment gates and
provenance remain historical. Preserve the specified refusals, source identity,
effects and invalidation. Keep measurement barriers and private observers in the
laboratory. Reseal this owner and verify the integrated release and the pinned
vanilla comparison.

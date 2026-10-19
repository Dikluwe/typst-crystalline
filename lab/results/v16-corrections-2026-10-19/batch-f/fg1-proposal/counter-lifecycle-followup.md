# Figure counter lifecycle: a specific refutation and a narrower next design

Status: **not ready for approval**. This source review refutes the proposed
shared-result cache as presently specified; it does not refute typed Figure kind.
No productive source, active L0 or pending decision changed. No compilation or
runtime probe was performed.

The earlier candidate treated callback-once as an undifferentiated requirement.
That is too strong mechanically: current contextual validation deliberately
reexecutes reads in an isolated transaction. The architectural requirement is
correct values/errors and publication of retained contributions once, with
validation/discarded effects absent. Eliminating every internal callback replay
would contradict the current validation model unless separately redesigned.

## Evidence and counterexample

`01_core/src/compiler/eval/context_reads.rs:247–276` constructs a fresh context,
clones the candidate introspector and invokes read replay with a new local Sink.
`03_infra/src/pipeline/context_stabilization.rs:488–516` also supplies an explicit
replay transaction whose sink is never published. Production execution instead
creates its own sink at `:279–294`, retains it in Execution at `:329–337`, and
publishes retained execution warnings in causal order at `:416–421`. The current
L3 owner explicitly requires discarding validation/replaced/discovery sinks and
prohibits implementing counter folds in L3.

Consequently, consider a callback that emits a diagnostic while producing a
valid counter state. A validation replay can compute a result for candidate I
before a later retained execution reads the same key in I. If a shared cache
stores only the result, that later execution sees a hit and loses its diagnostic
because the computing transaction's sink was discarded. Conversely, caching a
published flag shared with retained execution can let a discarded validation
claim ownership of the contribution. This is a source-derived counterexample,
not a newly executed language probe. It refutes “same key/generation means safe
result reuse” without contribution ownership. Sink dedup cannot repair this:
`01_core/src/entities/sink.rs:35–69` deduplicates only inside that individual sink;
it does not transfer diagnostics between discarded and retained transactions.

A post-layout or postpass-only cache is also insufficient: explicit filtered
reads already call the complete fold through
`01_core/src/compiler/stdlib/counter/filtered_reads.rs:78–107`, and the finalizer
owns another mutable introspector. It cannot retroactively supply those reads.
A single global callback-once flag would additionally suppress required replay
against changed causal events or selected automatic locations.

## A narrower coherent direction, still conditional

Keep the existing replay transactions and retained-warning policy. First define
a pure, immutable *input description* for a demanded counter: typed semantic
key, selected automatic Locations, ordered manual actions and callback identity,
plus target/features and causal ordering. A fold's output must include its
result and any contribution that must be attached to a retained demand; validation
must neither publish nor consume ownership of that contribution. Cache reuse is
optional. Recompute in discarded validation transactions until reuse is proven;
never use Rust Func name equality, raw Location ordering or a hash as the proof.

The compiler owner, not L3, must define the fold/input equivalence and output
projection. L3 can own transaction lifetime through an approved pure L1 facade,
as it already does for occurrence frames; prior occurrence approval does not
approve this new contract. This gives a concrete division of responsibility
without changing the five-attempt budget or copying business logic into L3.
It is not yet a complete proposal: exact contribution capture/replay APIs and
causal diagnostic origins need measurement and a current-owner audit.

Before promotion, distinguish a retained read after validation-only computation,
two retained demands, a replaced producer, changed manual update, changed
where(kind) membership, and a callback failure after the requested prefix.
Check values, original spans/traces and published warnings, not invocation counts
alone. Retain the measured no-caption/numbering:none/hidden no-demand controls.
Separately, the already concrete nth-counted-Figure → Location → prefix lookup
remains necessary and does not require interpreting update-event index as a
Figure index. Preview/final kind preparation and custom-name supplement policy
remain independent unresolved boundaries in readiness.md.

## Provenance

Read at UTC 2026-10-19T21:28:46.533114+00:00, HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, uncommitted working tree. This is static evidence, not a new runtime measurement. Exact reviewed inputs (SHA-256):

- `01_core/src/compiler/eval/context_reads.rs`: `5589d056fd9fa8a7f40e44de568a2e477e9f184af089b1cff93b3df0a800c830`
- `01_core/src/compiler/stdlib/counter/filtered_reads.rs`: `fafe8489b4424fcca2ac3cc7a1dc89095c9d9c7a1ec3e7193cc8484a99cf9bb1`
- `01_core/src/entities/sink.rs`: `00b6b480ed37c6ea502ce5a9b2bf4ca5c4a09866108cd5c1b99d24d8053ba661`
- `03_infra/src/pipeline/context_stabilization.rs`: `abb5e6e36882a3c3edc68002c44bbc33c5c080622621d00b976004e207281d2a`
- `00_nucleo/prompts/infra/pipeline/context_stabilization.md`: `a7aa969c374eec487ad51f42027b121a2d1b3a385c6a8064266bd8e13d60b4d6`
- `00_nucleo/prompts/core/entities/sink.md`: `4dbaf4102421bd4948e7aaef02567c14a6031624e053b7c10b2127eba8cbd6b2`

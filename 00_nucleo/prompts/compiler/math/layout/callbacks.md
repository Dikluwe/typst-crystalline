# L0 — math/layout/callbacks

Hash do Código: 55473c8e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/callbacks.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/callbacks.rs:20` locates `pub struct MathCancelRequestId`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Validated in-memory callback transcript

Own MathCancelRequestId (equation Location, structural occurrence, line),
MathCancelRequest (id, Func, default Angle, Span, StyleChain),
MathCancelResolution, SealedMathCallbacks, MathCallbackPassState and the
closed Pending/Complete outcome. This module neither executes Func nor
depends on Engine, World, infrastructure or I/O.

Each concrete layout attempt has a fresh state: optional validated read
store and local RefCell transcript. Never use global/thread-local state,
cross-attempt cache or shared mutable runtime. Reserve cancel occurrence at
dispatcher entry before descending; cross uses the same occurrence and
line=0/1. Span is diagnostic data, not identity.

Resolve a request only when id, function, default and collapsed styles match.
Absent, stale, different or unconsumed resolutions prevent completion;
do not accept partial store success. Pending carries requests only, never a
PagedDocument. Complete is available only after integral transcript validation.
Rejected internal layout attempts discard their transcripts. Only the accepted
candidate participates in later realization.

Snapshot styles by retaining the lexical StyleChain (including custom) and
pushing effective TextStyle fields with representable StyleDelta slots,
without reapplying defaults or inventing properties for internal math flags.
Matching uses collapsed semantic styles, not pointer identity or a textual hash.

Cross emits two requests and therefore two calls, each with the same positive
default angle measured from the body. Reflect only the returned second angle.
Without cross, inverted is applied after resolution; with cross it is ignored.
Angles change line geometry, not MathBox dimensions.

`finish_transcript(self) -> Result<(), Vec<MathCancelRequest>>` performs
integral validation independently of document ownership. Existing
`finish(document)` delegates to it and preserves MathLayoutPassOutcome.
The general coordinator may combine independent families without introducing
par.line execution or dependence into this owner.

Execution, return-type casting, finite convergence and span-anchored function
diagnostics belong to the pipeline's existing realization owner. Those callers
must not export a provisional document. Acceptance covers stable identity,
cross cardinality, style matching, stale/extra/missing store entries, discarded
attempts, repeated/reflowed requests and document-free Pending.

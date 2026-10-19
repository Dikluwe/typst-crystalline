# L0 — math/layout/callbacks
Hash do Código: 4029b4ca


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:4e7dbee80ff21b100bdac46801ecf462152c6743aad29b6cb6119d409c8db030

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/callbacks.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/callbacks.rs:20` locates `pub struct MathCancelRequestId`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

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


## Outcome storage and named lint boundary

### Evidence before decision

`lab/results/construction/diagnostics/clippy-enums-2026-10-16/before.json.gz`
retains sources/L0/global hashes/HEAD/uncommitted working tree/UTC/diffstat;
callers.txt records workspace uses. probe-details.json/measurements.json retain
rustc 1.92/x86_64, commands/binary/rlib, isolated candidate sizes/allocations before
code. The private enum is faithfully mirrored; public types are measured from
rlib. An unsafe observer exists only in the diagnostic executable, never
productive layers. This is Rust representation, not language equality, upstream
intent or global benefit.

### Exclusive responsibility and acceptance

Keep public MathLayoutPassOutcome::Pending(Vec<MathCancelRequest>) and
Complete(PagedDocument) directly by value. Productive uses return/match a single
outcome, not collections. Complete-only probes with empty document/black_box
measure no inline allocation versus one per boxed Complete: 20,000 extras per
sample. The enum is 440 bytes, boxed candidate 24, but its 440-byte document still
exists separately. A smaller container does not mean less total result memory.
Keep the direct validated path without Box. Use only the exact enum attribute:
`#[expect(clippy::large_enum_variant, reason="E003: resultado único de math layout por valor; Box adiciona alocação ao Complete, sem reduzir o documento")]`.
The Portuguese reason is machine metadata and remains verbatim. This is not
real-layout benchmarking or all-workload characterization.

ADR-0127 continuous flow changes only crate-private representation/constructor
or nominal metadata in its named batch; external outcome/entity contracts stay
unchanged. L0 first, restricted resealing. The private-size RED→GREEN and
large_enum_variant gates confirm three selected spans, fixing one through code
and two through fulfilled expectations. Do not raise thresholds or suppress
module/impl/crate, and do not extend this exception to Tag/Transformation or the
private request. Preserve fixtures/expectations and existing outline controls
(eight P1405 cases), context/replay/callback/pipeline tests. Validate full workspace/
build/Clippy/expectations/Rustdoc/fmt/lineage without new Tekt warning under
12 GiB/zero swap/one job. Tag/Transformation measurement is no authorization for
a public change in this package.

## Ratified complete text size

### Evidence before decision

`lab/results/construction/diagnostics/text-argumentos-2026-10-17/before.json.gz`,
before-measurement.json.gz, morph-before-measurement.json.gz and audit.json
preserve HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC, status/diffstat, sources/L0/binaries/argv and historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. named-em measures A at 0pt versus
24pt; affine 3pt versus 27pt; nested over 18pt measures 0pt versus 36pt. All six
Size/Color/body permutations are accepted by the pin and rejected by Tekt.
Positional set 18pt is ignored (12 versus 18pt). Pinned text/mod.rs:290,303,
924–930,1136–1142 and args.rs:178–247 establish typed search, named preference,
remaining body and composition. style_chain.rs:513–517 and constructor.rs:123
discard em; collapse at style_chain.rs:394–395,419–422 uses top-wins where size
needs a fold. Size/scope is observable language; Rust representation is an
internal choice. Infer no historical intent. Restoration, Raw, heading, math,
capture/show, duplicates/spans, zero/negative or metric failures can refute it.
The equality probe refuted comparing declared values: the pin returns true for
text(size:2em)[A] == text(size:24pt)[A]. Pinned StyledElem::eq at
content/mod.rs:779–782 compares child. That equality defect requires independent
correction; a complete size type does not close that axis.

### Ratification and limits

The human approved the complete-size package and its two public contracts in
manifest `771f5fd3d36545eb4b4fb7f0c0f2c91061470d1f010fc9a887b001a636d8feba`.
Require implementation and S1–S16 tests under 12 GiB/zero swap; no release/merge/
push. Reseal only after implementation and a valid graph. RG1 remains independent.

### Exclusive callback-snapshot adapter

Already-resolved TextStyle.size Pt becomes absolute Length in StyleDelta.size.
Preserve shape/payload, callback demand and replay without multiplying em again.

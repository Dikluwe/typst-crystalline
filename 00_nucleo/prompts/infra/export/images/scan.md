# L0 — images/scan
Hash do Código: 4b754dfd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/scan.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/scan.rs:41` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Validation and document scanning

Own FrameVisitor-based image validation, scan_all_images and page XObject resource
lookup. Validation traverses nested content, fully decodes JPEG and uses the existing
raster processor for other supported formats, stopping at the first
`failed to decode image ({e})` error. Unknown format remains accepted by this
validation path. Successful validation is deduplicated only within the current
invocation by shared Arc identity, under the contract below; payload caching
does not imply validation caching.

Scan by encounter order, deduplicating the same Arc data pointer into ImN. Preserve
the existing thread-local payload cache lifetime/key behavior; do not promise new
document-scoped invalidation or merge distinct pointers by equal bytes. RGB JPEG
references the supplied ICC; gray/CMYK keep their own space and JPEG dimensions
retain the existing frame-derived behavior. Raster alpha mask IDs precede main IDs.
Keep the next object ID and image-name counter together in a private scan cursor so
the warning carrier does not expand the image-item helper beyond the linted arity.
Unknown formats are omitted; decode failures retain warning/omission and do not
register successful pointer entries. Build recursive page resource dictionaries
with unique references and return an empty string when no images exist.

## Verification

Check nested validation, first-error diagnostics, repeated-pointer scans, alpha-ID
ordering, RGB ICC routing and failed/unknown omission.

## JPEG validation without a duplicate pixel destination (2026-10-26)

### Measurement and classification

Received HEAD `212aed0449150dd296ceea13fc2be665cc5db01a` plus uncommitted
working tree, exact inputs/diffstat/UTC and preserved release recorded in
`lab/results/construction/diagnostics/jpeg-validation-buffer-2026-10-26/before.json`.
`scan.rs:79` calls image 0.24.9 load_from_memory and discards DynamicImage.
Dependency image.rs:708 decoder_to_vec allocates the destination before
codecs/jpeg/decoder.rs:109 read_image decodes to a second vector and copies it.
The same decoder's into_reader at line 98 completes entropy/pixel decoding
before returning the owned result. io/free_functions.rs:107 reserves total_bytes
from default Limits before set_limits. Export images/xobjects.rs:25 embeds the
original compressed bytes, not decoded pixels. The previous resync's JPEG RSS
observation motivates remeasurement; it does not certify a future improvement.

Buffer duplication/lifetime is mechanics; image acceptance, diagnostics and
visible PDF content are language observables. The hypothesis is reduced peak
memory without weakening complete decode, refuted by unchanged/worse memory or
any acceptance/error/render regression. Internal correction, same phase and
signatures/defaults; continuous flow under ADR-0127, no new public contract.

### Contract and verification

The legacy fallback uses the existing image JPEG decoder's completed owned-reader path and discards
its result, avoiding DynamicImage's separate pixel destination. Preserve the
old default Limits total_bytes reservation and subsequent set_limits order,
all decoding/conversion errors and the failed-to-decode diagnostic envelope.
Do not scale, validate only headers, cache decoded
pixels or change PNG/GIF/WebP decoding, traversal or PDF byte transport.
This remains full-frame decoding, not a scanline/constant-memory promise.
image 0.24.9 marks into_reader deprecated for future removal. Permit a narrowly
scoped deprecated allowance at that single call: read_image allocates/copies the
extra destination in this version. Record the dependency-upgrade limitation;
do not suppress warnings at module/crate scope. The scanline clause below owns
the newly authorized decoder dependencies; the legacy fallback stays intact.

The first VmHWM guard failed on both paths (red-final.log and
memory-investigation.log); RGB output removal alone does not establish an RSS
bound when decoder component buffers dominate. Do not relax that RSS threshold
to claim GREEN. Instead demonstrate RED with an isolated Linux child counting
full RGB-size allocation requests around validation of a generated 6000x4000
JPEG, using a test-only forwarding System allocator. Alteration 1 reduced two
full RGB destinations to one in the legacy path; the scanline clause below
further reduces qualifying inputs to zero. Forward alloc,
alloc_zeroed, realloc and dealloc unchanged; counters are activated only inside
the isolated child after fixture loading. Keep fixture encoding in the parent.
Treat this as an allocation regression, not proof of peak RSS reduction.
GREEN must retain complete decode and old success/errors
for RGB/gray, damaged headers/entropy and oversized dimensions. Compare CMYK,
progressive, EXIF/ICC, malformed inputs and PNG control externally as well.
Measure before/after release memory and time on identical frozen inputs and
compare PDF rendering, extraction and embedded JPEG data. Record exact UTC,
dirty state and hashes; report repeated samples and limitations. Reseal this
owner only, and run build/full tests/clippy/fmt/doc/full crystalline-lint.

## Invocation-local shared-image validation (2026-10-26)

### Evidence and classification

Received HEAD `212aed0449150dd296ceea13fc2be665cc5db01a` plus uncommitted
working tree, exact UTC/diffstat/file hashes and release after JPEG alteration 1
are frozen in `lab/results/construction/diagnostics/image-validation-dedup-2026-10-26/before.json`.
scan.rs:49-72 visits every image occurrence without an identity set; repeated
FrameItem::Image values share immutable Arc<Vec<u8>>. The resource scanner
already keys identities by Arc::as_ptr, while export/tiling.rs:95-126 traverses
groups, page roots and prepared fill/stroke cell resources. PagedDocument owns
all those Arcs throughout the call. pipeline/paged.rs:1160 and bundle.rs:864
already validate their completed document; neither phase/call site changes.
Avoided repeated work is mechanics; first-error meaning/order, document
acceptance and rendered/embedded content are language observables. No new
signature, field, dependency, flag or phase: continuous flow under ADR-0127.

### Contract and verification

Own a fresh private set of Arc byte-buffer pointer identities inside each
validate_document_images call. Skip only an identity successfully validated
earlier in that call; insert after successful full validation. The set spans
all pages, backgrounds, items, foregrounds, nested carriers and prepared tiling
fills/strokes reached by the existing walk. Same bytes in distinct Arcs remain
distinct validations. Never hash/compare image contents or keep the set after
returning. Preserve the first error and stop later validation, including after
repeated valid occurrences. Unknown-format acceptance remains unchanged.
Every later call/export starts fresh, including after failure or byte mutation
between calls. Do not change payload caches, resources or decoder behavior.

First demonstrate RED by counting actual full RGB decode-output allocations
in an isolated test child: shared occurrences across page/group/tiling roots
must decode once, a distinct Arc with equal bytes twice, and repeated calls
must each decode again. Reuse the forwarding test allocator with a selectable
size only in the isolated child; retain alteration 1's default target and test.
Test first-error order and fresh-call behavior after a formerly valid Arc is
mutated between calls. GREEN must preserve prior JPEG full-decode/limits/error
tests. Measure alternating release runs against the preserved alteration-1
binary for single/shared/distinct images, multiple pages and JPEG/PNG controls.
Freeze inputs and record UTC/code/binary hashes, time/RSS and raster/text/JPEG
preservation. Do not infer speedup for unique images or lower peak memory just
from fewer decode calls. Reseal owner only and run full build/tests/lint/clippy/
fmt/doc; preserve all received changes.

## Conservative zune scanline experiment (2026-10-26)

### Measurements before decision

Received HEAD `212aed0449150dd296ceea13fc2be665cc5db01a` plus uncommitted
working tree, UTC/diffstat/2165 input hashes and alteration-2 release are frozen
in `lab/results/construction/diagnostics/jpeg-scanline-2026-10-26/before.json`.
The isolated probe pins zune-image commit
`2f9086b07cdf5a36621ce18f79d310f99b751de9` (zune-jpeg 0.5.16-rc3,
zune-core 0.5.3). decoder.rs:963 finish consumes unread rows, while progressive
and multi-SOS retain coefficients. An unrestricted strict scanline decoder
accepted malformed inputs rejected by the existing decoder: 1 of the first
624 mutations and 119 of 6000 additional mutations. A baseline-only eligibility
filter still exposed two invalid DC-category cases in 7729 header mutations.
The refined eligibility filter plus legacy fallback had zero acceptance
mismatches across all 14364 frozen cases (fixtures and generated mutations).
See compat-*.json/tsv and the pinned probe sources/lockfile in that directory.
The productive validator then matched legacy acceptance and exact errors on
the same 14364 cases. An additional 240 entropy/trailing-byte cases matched in
the probe; include them in the productive replay. The small progressive test
fixture has SHA-256 `5936023a53de7c410467d4f56596bfd338a650d20780a932a35ce6461dbf9de9`.

These measurements refute unconditional replacement and a constant-memory
claim for progressive JPEG. Select a conservative internal fast path instead;
no new public signature/field, product mode/default flag or phase. Decoder
choice is mechanics, acceptance/error order and rendered output are language.
This user-authorized experiment follows continuous ADR-0127 flow; do not
weaken the legacy acceptance/error policy to obtain a speedup.

### Contract

Add the two L3-only Rust dependencies at that exact Git revision. Preserve
alteration 1's legacy decoder and default total_bytes reserve/set_limits before
trying the fast path. Stream only byte-bounded SOF0, 8-bit, 1/3-component images
with one full interleaved SOS and final EOI exactly at input end. Walk declared
header segment lengths safely; allow ordinary APP/COM only before SOF,
allow DQT/DHT and DRI headers, and fall back for other markers/forms. The first
productive GREEN exposed image 0.24.9's encoder placing DQT after SOF (encoder.rs
frame/table emission); retain eligibility there by validating complete nonzero
8-bit quantizer tables with IDs <=3. Other quantizer forms use legacy. Validate DHT
boundaries, nonempty symbol lists, class/id <=1, DC categories <=11 and baseline
AC size <=10 (zero-size only EOB/ZRL). The entropy eligibility pass accepts
stuffed FF00 bytes and the final EOI only; restart/multiscan/trailing/unknown
structures conservatively use legacy. This filter admits an optimization; it
does not replace full entropy decoding.

Use zune strict mode with width/height ceilings 65535 (JPEG's existing u16
dimensions), start scanline output and require finish Complete. Never treat
NeedMoreInput, Ready, an error or unsupported input as success. Finish must
consume the image, not merely its headers. On any non-success, drop zune state
and run the retained legacy decoder, reusing its preflight instance. Thus
legacy diagnostics and accepted non-fast forms, including progressive/CMYK,
remain authoritative. Preserve Arc invocation-local dedup and all other
formats, metadata transport, pixel geometry and PDF bytes.

### RED→GREEN and validation

First add an isolated allocation regression: a qualifying 6000x4000 JPEG must
not request a full RGB output allocation (old path does). Retain the previous
no-duplicate guard as at-most-one allocation. Count actual validation calls
for the existing dedup tests via test-only thread-local instrumentation, since
decoded-buffer allocations no longer represent a scanline decoder invocation.
Test fast admission, malformed DHT/postscan tables, unsupported/progressive
fallback, old limits and exact errors, including the frozen progressive
counterexample. Keep all allocated instrumentation out of the product build.

Repeat the frozen differential corpus against the productive implementation,
including error strings, before promotion. Compare alternating release builds
against alteration 2 on single/shared/distinct, gray/RGB, progressive/CMYK,
ICC/EXIF and PNG controls; record time/RSS and all-page render/text/JPEG
preservation. No claim for untested photos or universal memory bounds. Run
full build/tests/lint/clippy/fmt/doc and preserve the received working tree.

## Grayscale scanline conversion cost (2026-10-26)

### Measurement and classification

Received HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, uncommitted tree,
UTC/diffstat/input hashes and scanline release `c6ff873c095d21fa21a4fbb397e9e4551370454c7301b92bb33020c60d86a417`
are frozen in `lab/results/construction/diagnostics/jpeg-grayscale-2026-10-26/before.json`.
The preceding campaign measured gray-large CLI wall time +7.0% with RSS -72.3%
against alteration 2. Pinned zune-core options/decoder.rs:720 defaults to RGB;
zune-jpeg worker.rs:113 and 144 duplicate Luma into three channels. Luma output
uses worker.rs:47, retaining full entropy/IDCT and scanline completion, without
the discarded RGB conversion. In the isolated paired probe, 35 samples per
mode after three warmups measured medians 35.393 ms RGB and 28.012 ms Luma for
the same frozen 6000x4000 grayscale input. Initial grouped timings oscillated;
retain them as inconclusive and use the alternating probe receipt instead.
Commands, source/binary/fixture hashes and UTC are in `probe-paired.json`.

These are internal conversion mechanics; acceptance, diagnostics and visible
content remain observables. Hypothesis: selecting Luma only for actual L8
input removes avoidable conversion cost without full-frame allocation. Refute
by paired release slowdown, increased memory, or any admission/error/render
regression. This is a private correction under continuous ADR-0127 flow, not
a public contract/default mode/phase change. Dependencies and eligibility stay
at the existing pin and subset; no upstream source patch is authorized here.

The first productive runtime-option candidate reduced gray-large time 12.0%
but raised RGB-large time 9.4% in the three-version campaign. A separate
31-pair repeat reproduced RGB medians 50.756 -> 55.654 ms and gray medians
47.092 -> 41.544 ms. Preserve this superseded candidate's source, binary and
receipts in `attempt-runtime/`; it does not certify a final improvement.
Runtime color selection may impede compiler constant propagation; that is an
inference, refuted if separately instantiated constant options do not recover
RGB time. Experiment with private compile-time specialization of the existing
complete scanline helper for Luma versus RGB, without changing its algorithm.
Compare the resulting release afresh; do not dismiss the RGB slowdown as noise.

### Contract and verification

Reuse the legacy preflight decoder's color type. Dispatch privately on its L8
classification into compile-time Luma/RGB specializations of the eligible
scanline helper; set zune output to Luma only for L8. All other inputs retain
the original constant RGB options. Do not convert RGB input
to Luma (that could skip chroma processing), change the admission guard, omit
entropy/IDCT work, scale pixels, or weaken start Ready / finish Complete.
Preserve fallback, limits, exact errors, Arc-local dedup, compressed bytes and
other formats. No full-frame buffer may be introduced.

Before implementation, extend the isolated forwarding-allocation regression
with an odd-width baseline grayscale input: observe a discarded three-channel
stripe before and require zero such allocations after, a one-channel stripe,
and no full grayscale destination. Retain RGB zero-full-buffer and dedup tests.
Check grayscale boundary dimensions and malformed tails against the legacy
decoder. Replay all 14604 frozen compatibility inputs with exact diagnostics.
Repeat paired release CLI time/RSS on grayscale, RGB, shared/distinct, fallbacks
and PNG; compare all-page renders/text/JPEG payloads. Measure fresh against both
the received scanline binary and alteration 2, so elimination of the old 7%
regression is demonstrated by a current run rather than historical timings.
Preserve incoming files, reseal this owner only and run full validation.

## Gradient grayscale: native reconstructed stripes (2026-10-26)

### Measurements before decision

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, dirty file list/diffstat,
UTC and inputs are frozen in
`lab/results/construction/diagnostics/jpeg-gradient-2026-10-26/before.json`.
Received release SHA-256
`74c8a978ee5c326c217951f9fab36e35d7b601fbc496524f9fb302abe3f28cb8`
had gradient CLI median 113.37 ms versus alteration-2 90.51 ms (+25.3%).
Those are historical observations, to be remeasured for closure.

At pinned zune revision `2f9086b07cdf5a36621ce18f79d310f99b751de9`,
decoder.rs:963 finishes converted scanlines; mcu.rs:1633 post_process dispatches
and converts each reconstructed row into the converted staging buffer.
decoder.rs:1242 decode_next_imcu_row instead reconstructs a native plane stripe
through the same entropy/IDCT path, returning RowReady/NeedMoreInput/Complete.
It retains row checkpoints, not a whole plane, for single-SOS baseline input.

The isolated probes compare legacy, converted Luma, scalar IDCT and raw-plane
stripes on gradient/uniform/noise. Scalar was substantially slower; optimizing
only zune-jpeg did not help. Consumer opt2/3 plus raw stripes improved some
probes but changes more code; no recipe change is adopted here. Cargo 1.92's
local reference/profiles.html:501-522 explains generic instantiation's effect
on package overrides. Static size-optimized gradient medians were 64.520 ms
converted and 59.107 ms native; uniform 28.406 -> 22.993 ms. A later prototype
with memchr produced almost equal gradient times (59.772/59.749 ms), refuting
a stable universal win. The guard alone cost about 2.3 ms on gradient, and
memchr did not improve that case. All receipts, binaries and negative results
are retained under the diagnostic directory; they do not close the product gate.

Stage the smaller native-stripe change with the received size-oriented recipe
and no new dependency. Less discarded postprocessing is a mechanical hypothesis;
acceptance, exact diagnostics and visible output remain the observables.
Require a fresh release median at least 5% faster on gradient than received,
peak RSS no more than 103% of received on large gradient/uniform/noise/RGB,
no full frame allocation, and unchanged language/error/render gates. A failure
refutes promotion; do not lower these resource bounds after seeing results.
This private implementation change is continuous under ADR-0127, with no public
contract, mode, phase, dependency, upstream patch or recipe change.

### Contract and RED→GREEN

After the existing legacy preflight/limits and conservative eligibility guard,
try a private native-plane path only for L8 with one component and sampling
factors 1x1. Decode headers using the same strict options/ceilings and full size.
Use raw_output layout without allocating a whole plane; allocate exactly
width*8 bytes of reusable output with checked arithmetic. Pull every reconstructed
iMCU stripe through entropy and full IDCT, including final partial rows.
Track remaining logical rows and accept only Complete after all rows; require
each RowReady count to match the expected stripe and bound the number of calls.
NeedMoreInput, unexpected layout/count/status or errors are not success.

On any non-success, drop native state and retain the previous converted-Luma
path before the existing authoritative legacy fallback. Thus excluded gray
sampling does not lose its scanline memory gain. RGB retains its existing path;
progressive/CMYK/multiscan/restart/trailer forms retain prior admission/fallback.
The earlier converted-Luma clause remains authoritative for those gray fallbacks;
this native Complete contract replaces it only for successful 1x1 native decoding.
Do not skip IDCT/chroma, scale, weaken limits/EOI/error policy, cache pixels or
change export bytes, Arc-local dedup, other formats or phases.

The initial gradient fixture exposed legacy acceptance after an entropy cut
followed by EOI; the first failing cut was the half-length cut, not the final
three bytes as initially inferred. Log each actual outcome and compare exact
results instead of assuming a rejection threshold. Do not infer upstream intent
from admission or change the existing error policy in this performance task.

Strengthen the existing isolated gray allocator test before implementation:
received counts [RGB9, converted-Luma9, native-Luma8, full-gray] must RED as
[0,1,0,0]; GREEN must be [0,0,1,0]. This preserves the old no-RGB/no-frame
requirements while proving the native path; it alone is not a time/RSS claim.
Add semantic tests for nontrivial gradient entropy and truncated tails, unusual
single-component sampling with old results, and stuffed/final markers across
search boundaries. Keep RGB allocation/dedup/error tests.
Replay all 17882 prior comparisons against the productive validator, including
exact diagnostics. Measure alternating frozen received/alteration-2/new release
CLI processes on gradient, uniform, noise, RGB, shared/distinct, fallbacks and
PNG; record time/RSS/size, source states and all-page renders/text/JPEG payloads.
Preserve received files, reseal this owner only, then run complete validation.

## Residual gradient cost: marker framing batches (2026-10-27)

### Measurement before decision

`lab/results/construction/diagnostics/jpeg-gradient-residual-2026-10-26/before.json`
freezes HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, uncommitted stat/list,
UTC, inputs and received release
`a424337b066631ee0da841687f0c9804739531c74b87954e13502dad97d888ae`.
The preceding window measured remaining gradient overhead +16.4% against the
alteration-2 release. It is historical, not a current acceptance measurement.

Bateia's current source was built without editing it. Full-tree local analysis
exceeded its byte budget; a hashed Cargo/src-only view of the pinned zune crate
completed as INCONCLUSIVE/CATALOG_GAP. This is not evidence of absent capability
or an optimization recommendation. Dossier and provenance are in the diagnostic
directory. Inspection/profiling and product experiments remain our responsibility.

Same-source recipe controls changed only Cargo CLI overrides. Infra opt2 won
30/31 gradient pairs (103.346 -> 99.734 ms), but its R6 control failed the fixed
RSS geomean cap (0.971941 > 0.97), with one unqualified baseline variation
(0.0348 > 0.03). Infra opt3 gained less and was larger; infra2+zune3 slowed the
gradient. None is adopted. Root manifests and main.rs remain unchanged, and the
received executable was restored. Keep all failed controls/thresholds verbatim.

The current eligibility guard searches entropy bytes individually for FF,
accepts FF00 stuffing and only a terminal FFD9. The admitted Huffman decode and
IDCT remain separate. A size-optimized isolated source probe comparing the same
header guard with SSE2 marker batches measured gradient framing medians
2.135086 ms scalar -> 0.134889 ms vector, uniform 0.151468 -> 0.009692 ms,
noise 1.319615 -> 0.081294 ms. Source/binary hashes, 31 paired samples with
20 iterations each and UTC are in `marker-results.json`. This does not prove
whole-CLI improvement; require a fresh independent product comparison.

These are internal mechanics, not new image semantics, mode, flag, phase,
public contract, dependency or recipe. Hypothesis: fewer scalar framing branches
reduce residual latency while preserving memory. Refute promotion if the paired
CLI comparison does not establish lower gradient time, RSS exceeds the existing
103% large-case cap, or any admission/error/render/source-preservation gate fails.
For the new received baseline above, require lower median gradient time and a
seeded paired-bootstrap 95% upper bound below 1.0 (61 samples). The preceding
5% gate against the older Luma baseline remains historical and is not relaxed.

### Contract and RED→GREEN

Keep the existing header eligibility parser, preflight, limits, native eight-row
reconstruction, converted/legacy fallbacks, RGB, Arc-local dedup and compressed
export bytes. Replace only the admitted entropy marker walk with a private
framing helper. It must require terminal FFD9 and accept a body iff every FF
byte is immediately followed by 00. Earlier EOI, restart, tables, padding FF,
trailing bytes and incomplete framing retain the previous eligibility result.
This helper is not validation of Huffman entropy and cannot replace full decode.

On targets with x86_64 and SSE2 enabled, compare 16 adjacent byte pairs at a
time: current==FF AND next!=00 must be empty. Use unaligned loads, no allocation,
no feature upgrade beyond the target's enabled baseline. All other targets keep
the old scalar search algorithm. Require explicit bounds before each load:
body_end=len-2; i<=body_end; body_end-i>=16 ensures both current[i..i+16] and
next[i+1..i+17] are within the original input, including the EOI lookahead.
SSE2 support follows cfg(target_feature="sse2"); do not call this branch when
disabled. Document these safety conditions next to the unsafe block. Process
at most 15 remaining body bytes scalarly and keep the same framing predicate.

Before implementation, add test-only scalar-comparison instrumentation to the
old walk and a regression on 8192 stuffed FF00 pairs: RED exceeds the bounded
15-byte tail; GREEN must satisfy it while eligibility stays true. Instrumentation
is absent from the product. Add differential tests against the received scalar
walk, exhaustive byte pairs around a vector boundary and deterministic random
payloads across lengths/offsets, plus all existing marker/decoder/allocation tests.
Replay the 17902 previous productive comparisons with exact diagnostics.

Compare frozen alteration-2/received/new releases with no competing build:
gradient (61 samples), uniform/noise, horizontal/vertical, RGB/shared/distinct,
progressive/CMYK and PNG. Record ratios, confidence interval, all samples,
RSS/size, full-page rasters/text/JPEG payloads and code/recipe identities.
Preserve all received changes, root recipe and earlier bounds. Reseal only this
owner after V15/V26; require complete build/tests/Clippy/fmt/rustdoc/lineage and
full crystalline-lint with no warning/error before closure.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [314]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Validate/register/reference images inside prepared tiling cells and background/
foreground as well, preserving Arc-based byte identity deduplication and existing
decode errors. Use the private visual-resource traversal without changing the
public semantic visitor contract.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Structured omission warnings

The approved
`lab/results/pdf-export-warning-carrier-2026-10-23/review.md` contract makes image
scan receive the current build's warning vector. A malformed supported image
that reaches direct export remains omitted and appends the exact
`failed to decode image ({detail}) — imagem omitida` detached warning. An unknown
format remains omitted and appends the exact
`Formato de imagem desconhecido — imagem omitida` detached warning. Preserve
encounter order, pointer deduplication, validation behavior and successful
registration rules. Do not write stderr, discard a warning, manufacture a source
span or weaken the normal pipeline's pre-export malformed-image error.


## Grayscale noise: direct borrowed byte transport (2026-10-28)

### Measurement before decision

`lab/results/construction/diagnostics/jpeg-gray-noise-2026-10-28/before.json`
freezes HEAD212aed0449150dd296ceea13fc2be665cc5db01a plus exact dirty source/list/
stat/UTC and received binary4ec7ab5e12336c0b00ac8660776ddce0d60c7d5b8784ea4aef68a2e9d11058c2.
The previous task's noise+5.48% against5c6e11a8 is historical. The formal fresh
resource RED2 uses61 balanced3-role samples: received/prior1.05374182,
paired95%[1.05062819,1.05896200], failing the predeclared2% median/3% upperCI.
The initial2-role sampler cancelled alternation; that run is indicative only,
excluded from formal acceptance and preserved with its correction record.

Compiler controls opt2/branch32/s32/z32/opt3-32 and the native helper no-inline
boundary were rejected. No recipe or helper boundary from those trials survives.
GDB localizes noise validation24.009->26.350ms; scalar IDCT is refuted by the
isolated probe. Those are diagnostics, not whole-product acceptance claims.

Pinned zune-core2f9086b07cdf5a36621ce18f79d310f99b751de9,
bytestream/reader/std_readers.rs:11-41, routes ZCursor through BufRead+Seek with
an exact-read retry loop. The direct implementation used by no_std is unavailable
when std is enabled. This is a transport mechanism, not intended JPEG semantics.
`reader-probe/receipt.json` and source-provenance supplement measure a safe direct
borrowed reader with unchanged SIMD/strict/full reconstruction: noise26.073->22.864ms,
gradient64.977->52.993ms, uniform effectively unchanged.

`reader-pilot/receipt.json` records the integrated default-recipe candidate with
61 noise/gradient and31 uniform/RGB samples in six balanced role permutations.
Candidate/prior-fast ratios: noise0.98742924 (paired95% upper0.99550460),
gradient0.89627856, uniform1.01632125, RGB1.02379592; all RSS<=1.03.
The same resource predicate changes RED->GREEN; this licenses the implementation,
not promotion without a fresh frozen actual-default build and final qualification.

### Contract and gates

Delegate native gray byte transport to the module-private child owned by
`00_nucleo/prompts/infra/export/images/scan/reader.md`. Replace only ZCursor in
jpeg_gray_native_complete with BorrowedJpegReader::new. Preserve the strict
preflight/header/plane/sampling/layout checks, checked width*8 stripe, row counts,
full Huffman entropy and IDCT reconstruction, Complete condition and converted/
legacy fallback sequence. Keep the original native inlining behavior, separate
SSE framing boundary, converted/RGB readers, output bytes, Arc dedup, other
formats, recipe, dependencies, target features and phase. Add no public surface,
new trait method, mode or default export path: ADR-0127 continuous flow applies.
Reader interface mechanics belong exclusively to its owner, not a shared prompt.

Tests are written before materialization. Six private-reader semantic tests
first compile RED because the new type is absent; that is not a functional
failure of the received decoder. The actual product regression RED is the
resource assertion above. Preserve every allocator/marker/fallback/error/RGB/
shared-image test and add a noisy odd-dimension JPEG/full native reconstruction/
entropy-tail+EOI case compared to exact legacy results. Module GREEN is28 tests;
final complete Rust count must rise only by the seven added tests.

Before promotion, require the unchanged noise median<=1.02 and paired95% upper
<=1.03 versus fast prior, noise time<=.98 received, and all9 median times/RSS
<=1.03 against both received and fast prior. Keep the existing936-process gradient/
pairedCI/native-Luma/nine-RSS limits unchanged. Refute promotion on any failure;
no threshold relaxation or candidate-byte promotion as final proof. Freeze and
rebuild the actual default recipe and require17922 compatibility cases, exact
errors/full reconstruction/allocation, JPEG output/paint/payload observations,
latest display-math and old Stack/width/notes/language/eight-matrix/raster corpora,
R6 size/RSS/repeatability/math-time, all12 validation gates and integral lint
with zero warnings/errors. Preserve all received files, rejected controls and
historical seals; record every state/source/binary/UTC and scoped resealing after
V15/V26. No commit/push or dependency/oracle resync is part of this change.

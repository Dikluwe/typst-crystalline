# L0 — entities/font_book — exact Unicode coverage and font metadata
Hash do Código: 2252d563

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/font_book.rs`
**Applicable ADRs:** ADR-0022, ADR-0030, ADR-0107, ADR-0108, ADR-0129.

## Evidence before the construction decision

- `03_infra/src/fonts.rs:403` collects codepoints from every Unicode cmap subtable and calls the existing vector-based constructor. L3 owns font bytes, discovery and acquisition.
- Historical pinned vanilla `9f2b6e8715237cb086899a42873660fe744622e8`, `lab/typst-original/crates/typst-library/src/text/font/info.rs:119-134`, builds compact coverage for each ordered Unicode subtable and unions intervals. Its general vector constructor at `info.rs:296` still sorts and deduplicates individual codepoints.
- `01_core/src/entities/font_book.rs:161` stores alternating outside/inside run lengths. The observable contract is exact Unicode membership and ordered iteration, including gaps and duplicates; the collection algorithm is internal mechanics.

The existing vector API must be retained. Compact contiguous input sequences into intervals, order those intervals and merge overlapping or adjacent intervals before encoding the canonical runs. The earlier refinement kept this API without adding a public range builder or importing font bytes into L1. The range-constructor decision below explicitly updates that restriction; font bytes remain outside L1. Performance claims belong to `lab/results/construction/diagnostics/font-lifecycle-experiment.md` and its reproducible dataset.

## Recorded fixed-cost evidence and internal refinement

`lab/results/construction/diagnostics/custo-fixo-2026-10-05/audit.json.gz` records the measured dirty-tree/binary/input provenance. Its catalog diagnostic composition measures roughly 46 ms in this constructor, after collecting 28,689,553 codepoint occurrences for 1,112 system faces. This is a nested component measurement, not a decomposition of plain CLI medians. `compression.json.gz` compares the current real constructor with an isolated scalar-interval implementation across three processes and nine batches each: about 44–45 ms versus 23–25 ms, with canonical encoding equal for every face. These measurements precede the production refinement; final CLI gains remain to be established.

Keep the vector input and the existing sorting, merging and gap/count encoding. During the initial compaction, retain the active inclusive interval in local scalar start/end variables instead of accessing and updating the last Vec entry for every codepoint. Flush an interval only when a point is disjoint and nonadjacent, then flush the final interval. Empty input has no active interval. Check forward adjacency first for ordered cmap streams, then membership and reverse adjacency; arbitrary input order, overlap, duplicates and both interval edges remain supported. Checked adjacency arithmetic and the Unicode upper boundary remain valid. No API, acquisition policy, pipeline phase, unsafe or mutable global changes.

Classification: internal mechanics, with set membership/ordered iteration as semantic acceptance. Existing independent set-oracle tests are run before materialization and after; they already pass on the baseline. Do not invent a semantic RED for a performance-only change or place wall-clock thresholds in ordinary unit tests. The preimplementation compression measurement is the cost baseline; the isolated candidate, complete CLI comparison and output controls decide acceptance. Preserve original sources and results even if final performance refutes the experiment.

## Evidence and decision for a pure range constructor

`lab/results/construction/diagnostics/cmap-intervalos-2026-10-05/measurement-direct.json.gz` and `measurement.json.gz` preserve exact source/binary/input/dirty-tree provenance for the independent experiments. `03_infra/src/fonts.rs:404–415` currently enumerates and retains individual codepoints. The installed-font control has 1,112 faces and 28,689,553 point occurrences; extraction into native ranges has 1,520,598 input intervals. Simple range sorting costs about 20 ms per catalog in the first experiment. Compacting adjacent/overlapping incoming intervals before sorting costs about 10.8–10.9 ms against about 53 ms for the existing point-extraction/constructor composition in the second. Every supported face had equal canonical encoding; no legacy face fallback occurred in this host control. This is an isolated coverage experiment, not a measured CLI gain.

Measured parser anchors: ttf-parser 0.25.1 `tables/cmap/format4.rs:103–115` enumerates declared inclusive segment bounds and stops at the singleton FFFF sentinel; `format12.rs:68–75` and `format13.rs:41–48` enumerate inclusive group bounds without checking the glyph ID. Raw byte access and eligibility remain L3 concerns. The experimental adapter lives under diagnostics, not in productive L1/L3. The pinned vanilla mechanism at `lab/typst-original/crates/typst-library/src/text/font/info.rs:119–134` streams declared codepoints into a builder and unions subtable coverage; it does not prove direct range extraction equivalent.

Decision/classification: add a public pure constructor for neutral integer ranges, keeping union/canonicalization in L1 and byte parsing in L3. This extended the public contract; its ADR-0127 ratification is recorded below. Existing public signatures, metadata representation, discovery policy and pipeline phases remain unchanged. No foreign parser types, unsafe, I/O, mutable globals or new public entities are introduced.

New constructor:

```rust
pub fn from_ranges(
    ranges: impl IntoIterator<Item = std::ops::Range<u32>>,
) -> Self;
```

Ranges are half-open: start is included and end excluded. Empty and reversed ranges contribute no members. Nonempty ranges may be unordered, overlapping, adjacent or repeated. Build their exact union, preserving the existing canonical gap/count encoding, ordered iteration, membership/equality/hash behavior. Unicode zero and the complete supported upper boundary are representable through `0..0x110000` and `0x10ffff..0x110000`. Range endpoints are ordinary u32 values; do not clamp to Unicode blocks or introduce char/glyph approximation. The exclusive-end representation cannot express a member at u32::MAX; this does not broaden the existing point API's supported Unicode domain.

Compact connected input ranges using local start/end scalars, flushing only at a disjoint/nonadjacent range or end of input. Two ranges connect when their closed endpoint tests permit overlap or adjacency (`next.start <= active.end && active.start <= next.end` for half-open bounds); update with min(start)/max(end). Ignore empty/reversed inputs before compaction. Sort the resulting intervals by start, merge globally and encode the existing gaps/counts. Shared normalization/encoding remains owned by this module; from_codepoints may delegate its already compacted intervals without changing its public vector signature or exact supported point semantics.

Acceptance: implement the public constructor through a genuine runtime RED with a temporary empty implementation, then GREEN with independent set-oracle tests of unordered/overlapping/reversed/empty intervals, gaps, adjacency, zero and the Unicode maximum. Preserve all existing point-constructor and catalog tests. The adapter owner's separate L0 must precede any productive byte/range extraction; synthetic controls cover formats 4/12/13, fallback formats, Unicode eligibility, glyph-zero declarations, sentinel behavior, invalid-record termination and legacy fallback. Validate complete CLI/PDF/font outputs and paired performance before asserting production gain. Measurements support the experiment, not an unconditional speedup promise.

## Owned coverage contract

Preserve `Coverage(Vec<u32>)` as an immutable exact set after construction, with the existing `Clone`, equality, hash, debug and default behavior. Cloning copies its vector; no constant-time clone is promised.

Preserve these public signatures:

```rust
pub fn new() -> Self;
pub fn from_codepoints(codepoints: impl Into<Vec<u32>>) -> Self;
pub fn from_ranges(ranges: impl IntoIterator<Item = std::ops::Range<u32>>) -> Self;
pub fn contains(&self, codepoint: u32) -> bool;
pub fn is_empty(&self) -> bool;
pub fn iter(&self) -> impl Iterator<Item = u32> + '_;
```

The input may be unordered and may contain duplicates. Preserve exact set membership throughout Unicode `0..=0x10FFFF`, including zero and the upper boundary. Empty input yields empty coverage. Iteration yields each member once in ascending order. Never replace exact coverage with a block approximation.

Encode alternating lengths outside and inside the set, starting outside at zero. The set `{2,3,4,9,10,11,15,18,19}` encodes as `[2,3,4,3,3,1,2,2]`. Canonical encoding must be independent of input order, subtable overlap and duplicate occurrences.

During construction, compact adjacent input points into inclusive intervals. An interval may extend at either edge; a repeated point already inside it changes nothing. Sort intervals by their start, merge overlap/adjacency and emit the existing gap/count pairs. Arithmetic at the Unicode upper boundary must remain valid. Contains scans runs linearly, stopping once its cursor exceeds the requested codepoint. Iteration expands inside runs. No I/O, mutable globals or external font parser types are allowed.

## Font metadata and catalog contracts

Preserve `FontInfo { family: String, variant: FontVariant, flags: FontFlags, coverage: Coverage }`, existing font style/weight/stretch types, clamps, symbolic names, defaults, distance functions and trait behavior. `FontFlags` retains its existing monospace and serif fields. L3 populates metadata and owns whether coverage is extracted during discovery or on demand. A populated coverage is exact and immutable.

`FontBook` remains a pure ordered collection of `FontInfo`. Push appends; indices preserve acquisition order. Existing new/default, infos, len and is_empty behavior and public signatures remain unchanged.

Family selection is case insensitive and chooses by weight distance, then style distance, preserving the existing tie behavior. Pattern selection uses the existing literal/regex matching rules and the same variant ranking. Family enumeration preserves insertion order. `candidates_for_char` returns precisely the indices whose populated coverage contains that character, in catalog order, without opening font faces. Its cost is linear in catalog size with a linear run scan per candidate.

Fallback selection accepts already covering indices, skips invalid indices and preserves the first candidate on a complete score tie. Preserve existing ranking: likeness by monospace, serif, shared family-name prefix words and shorter family name, then variant distance by style, stretch and weight. Do not introduce vanilla's variable-font preference or axis fields here.

## Boundaries and verification

Font storage, mmap-backed bytes and the `Font` API belong exclusively to `00_nucleo/prompts/core/entities/world_types.md`, whose consumer is `01_core/src/entities/world_types.rs`. This prompt does not own that consumer. System catalog lifecycle belongs to `00_nucleo/prompts/infra/world/fonts.md`; byte parsing and extraction are delegated by `00_nucleo/prompts/infra/fonts.md` to its private owner `00_nucleo/prompts/infra/fonts/coverage.md`. Neither L3 acquisition policy nor shaping is decided here.

Verify exact membership and ordered iteration against an independent set oracle for duplicate and overlapping subtables, reversed ranges, arbitrary order, gaps, zero and the Unicode maximum. Preserve catalog selection and fallback tests. Acceptance is set/selection semantics; wall-clock thresholds are not unit-test requirements. The historical bitmap and vector-sort decisions remain evidence in the diagnostic dataset, not current acquisition policy.


Human ratification: the owner confirmed the coverage-ranges package with `confirmo` on 2026-10-07; manifesto SHA-256 `f8f7805c6d2ca08d1d14296ed5c93767c416b536fa9c17532eaa8310ca93a549`. The proposed range constructor is therefore part of the current contract.

## Explicit range fixture acceptance

Measurement: Rust 1.92 Clippy reports the intentional reversed range at
`01_core/src/entities/font_book.rs:913` and the singleton range collections at
`:969` and `:973`. The owned constructor contract above requires reversed
ranges to contribute no members and covers the complete Unicode domain.
Decision: express the invalid interval with explicit Range start/end fields,
and a single valid interval with `std::iter::once`. Preserve all bounds and
assertions. These are test-fixture spellings only; never turn the invalid range
into a descending iterator or expand a range collection into codepoints.


## R6 immutable coverage storage sharing — 2026-10-24

### Evidence before classification

`lab/results/construction/diagnostics/pdf-resources-r6-2026-10-24/coverage-before.json`
and its full source archive retain HEAD/dirty diffstat/UTC and exact inputs.
font_book.rs:160–161 derives Clone over a private Vec<u32>; world/fonts.rs:207–217
clones populated system/math/project FontInfo while composing the ordered catalog.
The exact set is immutable, but this copies every run allocation. The received
allocation profile attributes974768 large requested bytes to fresh_catalog;
requests are not peak live heap. Recipe-only "z" still fails the fixed aggregate
RSS3% gate (ratio0.97259); no threshold reduction is permitted.

### Private representation correction and acceptance

Supersede only the earlier requirement to retain private Vec storage and deep
Clone. Store completed canonical runs as a private Arc<[u32]>; builders still use
local Vec then seal immutable runs. Clone shares storage in constant time. Retain
Coverage's public name/constructors/signatures, FontInfo public field types,
FontBook order/ranking, exact membership/ordered iteration, canonical lengths,
content equality/hash/debug/default. No public field, new API, mutation method,
dependency, global state, I/O or pipeline change. This private representation
correction is continuous under ADR-0127 and required RAM discipline ADR-0029/0030.
No repr/ABI is promised by this opaque type. Do not expose Arc as a new API.

Before implementation add a nonempty sparse coverage clone resource test that
preserves membership/equality/hash and then requires no second runs allocation;
confirm runtime RED and GREEN. Pointer sharing observes only the mechanical
no-copy requirement, never language identity. Existing canonical-vector tests
may borrow a slice to observe the same exact values independently of private
storage; preserve all values and assertions. Keep independent set-oracle,
metadata/fallback and catalog tests. Shared backing is immutable after source
drop. Retain only with source-bound full-output preservation and the original
size/RSS gates, including historical single/plural stress controls; an unchanged
or worse resource result refutes the performance hypothesis.


## Laboratory indexed literal family selection — 2026-11-17

### Measured basis before decision

math-regression-isolation-2026-11-16/{initial,profile-analysis,activity,final}.json
pin dirty HEAD48e5a2570159c6fc325babe883bac7d1c1fbd4b2, full sources/ELFs/UTC.
The two CPU periods997/1237us put37-38% of mathematical samples in
FontNamePattern::is_match. Direct counts show this workload uses the legacy
index and does not call the new semantic uniqueness helper. Local instruction
placement alone changes that workload by about4-6%; this is a confound to
control, not the optimization proposed here. No earlier NOP overlay is used.

Current font_book.rs:452-476 filters every FontInfo through pattern.is_match,
collects matching indices into a temporary Vec, then min_by_key ranks weight
and style. world/fonts.rs:178-194 invokes it on either the existing prefix or
current full book. infos are private, appended only by push, and exposed by
immutable slice. These measured boundaries permit a book-local derived lookup
without extending a World proof/cache or changing font acquisition. The existing
regex comment incorrectly says first matching face; the observed algorithm
ranks ALL matches. Preserve that algorithm, not the misleading comment.

### Classification and scope

Internal pure RAM mechanics under ADR0029/0030/0107/0108 and0127 continuous flow.
Only this owning prompt and its consumer may change in the isolated laboratory.
No public entity field/signature, trait method, product/default flag, phase,
shared Nucleus, dependency, unsafe, I/O, clock/environment or global state.
FontBook is opaque and promises no Rust memory layout. Prior sources, ELFs,
root edits and receipts remain unchanged; no integration or commits.

### Derived family index and exact selection

Retain infos in original acquisition order and keep every FontInfo/slot index,
family spelling, variant, flags and exact coverage unchanged. A private Vec of
slot indices may order entries by ASCII-folded family bytes, with original slot
order within equal keys. ASCII folding maps only A-Z; every non-ASCII UTF-8 byte
remains unchanged. Do not use Unicode lowercase, normalization, locale rules,
font-name aliases, prefixes, hashes, pointer identities or regex grouping as an
equality substitute. It must coincide exactly with eq_ignore_ascii_case.

new/default start empty. Every push first appends the SAME info and inserts its
new slot after all previous equal-family entries. No query result is cached;
no negative result persists. Another book/world/catalog/prefix has its own index.
No metadata/face/byte acquisition or World epoch behavior changes. An appended
better variant must be visible immediately, including after a previous miss.
Construction is O(log N) family comparisons and O(N) index movement per push,
thus potentially O(N^2) total movement; explicitly measure cold/build cost.
Index storage is O(N), one slot integer per face plus Vec capacity slack.

Only select_pattern's Literal branch may use lower-bound binary search then
visit the contiguous equal-family slot range, without allocating a query key
or candidate vector. Preserve the original min_by_key(weight distance, style
distance) and FIRST catalog slot on complete tie. Stretch is not a new tiebreak.
Missing family remains None, including empty/non-ASCII names under the existing
rules. Keep select, select_family, regex scan/all-match ranking, fallback scoring
and coverage methods unchanged. Preserve duplicate and mixed-case family entries.

Private derived data never changes public equality or Hash: both remain exactly
ordered infos, delegating to the same fields as the prior derive. Manual
implementations may replace those existing derives, not add a public contract.
No retained physical/shaping result or new invalidation certificate is authorized.

### Tests first and refuters

Scaffold the existing literal scan through a private pure comparison visitor;
production uses no-op. This is not acceleration. Execute RED: a query in4096
unique families exceeds a logarithmic comparison budget on that actual path.
Compile errors and timer thresholds are not RED. After implementation require
logarithmic range-location work plus the actual matching-family size, for hits
and misses at several catalog sizes. Keep the old select method as independent
literal oracle and a test-only old pattern scan for regex/differential controls.

Test catalog append/new/independent books, misses then insertion, better variant
and ties after insertion, mixed ASCII case, empty names, Unicode byte distinctions,
raw non-normalized Literal queries, identical/duplicate families, all style/weight/
stretch combinations, regex expressions matching multiple families and none,
unchanged info/coverage/order and Eq/Hash-by-infos. Observe actual private table
capacity within a linear bound; counts/addresses are mechanics, not language proof.
Preserve all existing core/infra/font/coverage/source tests and refusals.

### Complete-cycle acceptance and resource bounds

Use matched fresh release baseline and candidate recipes, not previous patched
ELFs. Preserve previous algorithm ELF as provenance/reference only. No global
alignment/optimization/debug setting changes; opt=s/ThinLTO/one codegen unit and
existing alignment. Serial builds and no overlap with benchmarks. Counter work
and paired full cycles distinguish the algorithm improvement from small layout
effects; do not claim all37-38% recoverable or sum old gains.

Twelve blocks, six orders twice for baseline/candidate/byte-identical baseline
alias, prior14 inputs, startup/same bytes/two comments/visible edit, PDF default
with tags and no profiler/debugger/census/timing JSON in nominal. Include seven
fresh-process rounds (discard first), separate source/style/include restoration,
state/counter/query/aliases/error publication, literal/regex/Unicode/missing-font
cases and current pinned oracle controls. Exact fresh/watch input hashes and
text/style/0.001pt/link/bookmark/tag plus auxiliary pixel comparisons are required.
Measure catalog construction/index bytes and full startup, not just lookup.
Retain nonresponses/regressions and exploratory A/A/bootstrap uncertainty.

One job/build;12GiB AS/8GiB process-tree RSS, abort MemAvailable<6GiB. Runtime
AS1GiB/stack64MiB/files64MiB, disk targets only, never tmpfs. Full manifests,
dirty changed-file list and UTC at every source transition. No integration/commits.


## Laboratory sparse coverage query index — 2026-11-17

### Measurement before decision

unicode-regression-isolation-2026-11-17/{analysis,activity,report}.json/md retain
source/ELFs/dirty HEAD/UTC: Unicode109.57ms baseline,123.11ms current link candidate,
108.44ms local placement diagnostic. All four executions make463 ordered candidate
queries with identical ordered returned slots. Placement is not a Rust improvement.
world/fonts.rs:220–244 scans catalog slots and calls Coverage::contains; this
owner's contains at font_book.rs:263–280 linearly scans alternating runs. The
observed native hotspot contains that inner accumulation loop. No native padding
or compiler/global alignment change is authorized as this experiment's candidate.

### Exclusive private correction and unchanged semantics

This narrowly supersedes the earlier mandatory linear membership scan, the earlier
no-coverage-change restriction of the literal-family experiment, and the private
single-Arc representation only to add one immutable sparse derived index. Retain
canonical gap/count lengths in the original Arc and preserve all constructors,
public methods/types, equality, exact Hash writes, Debug, default and ascending
iteration. Opaque Coverage has no repr/ABI promise; no public field/signature,
trait, mode/default, phase, compatibility, I/O or dependency change. Internal
ADR-0127 continuous flow, L0 first; ADR-0029/0030 permit pure shared RAM data.

Each constructed coverage may own a second private Arc<[u32]> of cumulative ends,
one for each consecutive block of16 runs, including a final partial block.
Sixteen is even, so each block starts outside. Derive it only after canonical
construction; the last checkpoint equals the exact last exclusive endpoint.
Membership rejects beyond that endpoint, chooses the first checkpoint greater
than the codepoint by binary search (first-block shortcut permitted), then scans
only that block of at most16 canonical runs from the preceding checkpoint.
Zero-length first gap, gaps, both boundaries, zero, surrogate integers, Unicode
maximum, ordinary supported u32 range endpoints, empty/reversed/duplicate and
unordered inputs retain their exact existing set behavior. Do not broaden the
unsupported point u32::MAX construction by overflow masking or clamp codepoints.

Index has ceil(run_count/16) u32 entries, plus constant Arc/struct overhead; never
expand all codepoints or retain per-character results. Clone shares both immutable
backings in constant time without allocation. Equality/Hash delegate only to the
original canonical runs, preserving exactly the prior derived Hash behavior;
Debug retains the original one-field tuple format. Index is not language identity,
source certificate, font generation/epoch or a new cache authority. No results
survive a replaced Coverage,book or World except existing owned immutable clones.
No World, font loader, lazy missing-coverage map or invalidation code changes.

### RED first, refuters and resource acceptance

Scaffold the current real linear scan through a private pure entry-visit callback;
production uses no-op. This is instrumentation, not acceleration. RED requires
that the actual lookup on4096 isolated codepoints exceeds a32-entry examination
budget for late hits/misses/high absent codepoints. Compile errors or timers are
not RED. GREEN requires logarithmic checkpoint work plus at most16 run entries.
Keep an independent set oracle and frozen linear reader for semantic differential;
mechanical reads/pointers are not language parity. No timing thresholds in tests.

Tests precede implementation: both constructors; all boundaries,zero,empty,adjacent,
large/sparse/unordered/duplicate/reversed ranges, Unicode and larger supported
u32 values; full ordered iteration/canonical runs; equality/hash/debug parity with
canonical backing for arbitrary Hasher writes; shared clone backing after source
drop; independent books and full FontBook candidates ordering with sparse coverages.
Test bounded actual index storage; no deep-copy regression of R6. Report measured
catalog-wide extra bytes and complete startup cost, not just lookup work. Keep
R6's no-copy and existing size/RSS discipline; do not hide memory regression by
loosening prior thresholds. Retain only with source-bound full-output and full-
cycle benefit. Query-index memory must remain sparse, never a duplicate full index.

Use matched fresh release baseline/current Rust candidate, one opt=s/ThinLTO/
codegen1/alignment3 recipe; preserve old byte-identical baseline alias. Include
12 balanced blocks across14 cases and all5 watch states, seven fresh-process
rounds (discard first), plus a separate held-out Unicode comparison against the
prior diagnostic corrected-placement ELF. The Rust candidate contains no overlay.
Gain only over the regressed ELF is insufficient: show benefit beyond the native
placement reference and report all regressions/A/A/bootstrap uncertainty.

Run full core/infra suites, focused tests, workspace build, strict production
Clippy, Rustdoc, owner fmt and unfiltered lineage. Preserve inherited baseline-
confirmed all-target Clippy/fmt debts without suppression/unrelated repairs; those
global gates remain open and this experiment authorizes no integration. Preserve
source/style/include/font mutation and restore, actual query codepoints/ordered
slots, effects/state/counters/aliases/error publication, physical producer/hit/
refusal census, text/style/0.001pt/links/bookmarks/tag-presence and auxiliary raster.
Keep pinned oracle64c5e8c02fe0394d913ead36e42ef8c1334d2107 debts explicit.
One build job,AS12GiB/tree RSS8GiB,abort MemAvailable<6GiB; runtime AS1GiB/stack64MiB/
files64MiB,disk only. Source manifests, exact dirty stat, ELFs and UTC at every
transition; preserve old source/results. No integration and no commits.

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
